use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use ori_driver::pipeline::{run_build, run_check, run_compile, CheckOutput};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let id = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ori_driver_test_{}_{}_{}",
            std::process::id(),
            id,
            name,
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }

    fn write(&self, name: &str, source: &str) {
        std::fs::write(self.path(name), source).unwrap();
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn diagnostic_codes(out: &CheckOutput) -> Vec<&'static str> {
    out.diagnostics.iter().map(|d| d.code).collect()
}

#[test]
fn check_loads_default_import_alias_and_imported_return_type() {
    let dir = TestDir::new("default_import");
    dir.write("util.orl", r#"namespace app.util

func answer() -> int
    return 11
end
"#);
    dir.write("main.orl", r#"namespace app.main

import app.util

func main()
    const value: int = util.answer()
end
"#);

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_resolves_imported_types_in_signatures_and_builds_c() {
    let dir = TestDir::new("imported_types");
    dir.write("model.orl", r#"namespace app.model

struct User
    id: int
end

func same(user: User) -> User
    return user
end
"#);
    dir.write("main.orl", r#"namespace app.main

import app.model as model

func pass(user: model.User) -> model.User
    return model.same(user)
end

func main()
end
"#);

    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("ORI__app_main_pass"));
    assert!(build.c_source.contains("ORI__app_model_same"));
    assert!(build.c_source.contains("int main(void)"));
}

#[test]
fn build_handles_same_type_name_in_distinct_imported_namespaces() {
    let dir = TestDir::new("same_type_name");
    std::fs::create_dir_all(dir.path("left")).unwrap();
    std::fs::create_dir_all(dir.path("right")).unwrap();
    dir.write("left/user.orl", r#"namespace left.user

struct User
    id: int
end

func same(user: User) -> User
    return user
end
"#);
    dir.write("right/user.orl", r#"namespace right.user

struct User
    id: int
end

func same(user: User) -> User
    return user
end
"#);
    dir.write("main.orl", r#"namespace app.main

import left.user as left
import right.user as right

func take_left(user: left.User) -> left.User
    return left.same(user)
end

func take_right(user: right.User) -> right.User
    return right.same(user)
end

func main()
end
"#);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert_eq!(build.c_source.matches("\nstruct ori_def_").count(), 2);
    assert_eq!(build.c_source.matches("typedef struct ori_def_").count(), 2);
    assert!(build.c_source.contains("ORI__left_user_same"));
    assert!(build.c_source.contains("ORI__right_user_same"));
}

#[test]
fn build_uses_qualified_names_for_imported_constants() {
    let dir = TestDir::new("imported_constants");
    dir.write("config.orl", r#"namespace app.config

const LIMIT: int = 21
"#);
    dir.write("main.orl", r#"namespace app.main

import app.config as config

func main()
    const value: int = config.LIMIT
end
"#);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("static const int64_t app_config_LIMIT = INT64_C(21);"));
    assert!(build.c_source.contains("int64_t value = app_config_LIMIT;"));
}

#[test]
fn compile_uses_imported_constant_value() {
    let dir = TestDir::new("compile_imported_constant");
    dir.write("config.orl", r#"namespace app.config

const LIMIT: int = 21
"#);
    dir.write("main.orl", r#"namespace app.main

import app.config as config
import ori.io as io

func main()
    io.print(string(config.LIMIT))
end
"#);

    let exe = dir.path(if cfg!(windows) { "const_main.exe" } else { "const_main" });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "21");
}

#[test]
fn check_reports_type_error_inside_imported_top_level_const() {
    let dir = TestDir::new("imported_const_type_error");
    dir.write("config.orl", r#"namespace app.config

const LIMIT: int = "bad"
"#);
    dir.write("main.orl", r#"namespace app.main

import app.config as config

func main()
end
"#);

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.type_mismatch"));
}

#[test]
fn check_uses_imported_top_level_const_type_at_use_site() {
    let dir = TestDir::new("imported_const_use_type");
    dir.write("config.orl", r#"namespace app.config

const LIMIT: int = 21
"#);
    dir.write("main.orl", r#"namespace app.main

import app.config as config

func main()
    const value: string = config.LIMIT
end
"#);

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.type_mismatch"));
}

#[test]
fn check_reports_missing_local_import() {
    let dir = TestDir::new("missing_import");
    dir.write("main.orl", r#"namespace app.main

import app.missing

func main()
end
"#);

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.import_not_found"));
}

#[test]
fn check_reports_import_namespace_mismatch() {
    let dir = TestDir::new("namespace_mismatch");
    dir.write("util.orl", r#"namespace app.other

func answer() -> int
    return 1
end
"#);
    dir.write("main.orl", r#"namespace app.main

import app.util

func main()
end
"#);

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.import_namespace_mismatch"));
}

#[test]
fn compile_runs_entry_namespace_main_with_imported_call() {
    let dir = TestDir::new("compile_import");
    dir.write("util.orl", r#"namespace app.util

func answer() -> int
    return 13
end
"#);
    dir.write("main.orl", r#"namespace app.main

import app.util as util
import ori.io as io

func main()
    io.print(string(util.answer()))
end
"#);

    let exe = dir.path(if cfg!(windows) { "main.exe" } else { "main" });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "13");
}

#[test]
fn compile_loads_transitive_local_imports() {
    let dir = TestDir::new("transitive_imports");
    dir.write("c.orl", r#"namespace app.c

func value() -> int
    return 8
end
"#);
    dir.write("b.orl", r#"namespace app.b

import app.c as c

func value() -> int
    return c.value()
end
"#);
    dir.write("a.orl", r#"namespace app.a

import app.b as b

func value() -> int
    return b.value()
end
"#);
    dir.write("main.orl", r#"namespace app.main

import app.a as a
import ori.io as io

func main()
    io.print(string(a.value()))
end
"#);

    let exe = dir.path(if cfg!(windows) { "transitive.exe" } else { "transitive" });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "8");
}
