use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use ori_driver::pipeline::{
    run_build, run_check, run_compile, run_compile_with_options, run_doc, run_fmt, run_test,
    CheckOutput, CompileOptions,
};

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
        let path = self.path(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, source).unwrap();
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

fn ori_path_literal(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn exe_path(dir: &TestDir, name: &str) -> PathBuf {
    let file_name = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    dir.path(&file_name)
}

fn compile_c_source(dir: &TestDir, name: &str, source: &str) {
    let c_path = dir.path(&format!("{name}.c"));
    let obj_path = dir.path(&format!("{name}.o"));
    std::fs::write(&c_path, source).unwrap();
    let output = match Command::new("cc")
        .arg("-std=gnu11")
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
    {
        Ok(output) => output,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return,
        Err(err) => panic!("failed to run cc: {err}"),
    };
    assert!(
        output.status.success(),
        "generated C did not compile\nstderr:\n{}\nsource:\n{}",
        String::from_utf8_lossy(&output.stderr),
        source,
    );
}

fn normalize_stdout(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap().replace("\r\n", "\n")
}

fn compile_and_run(dir: &TestDir, exe_name: &str) -> String {
    let exe = exe_path(dir, exe_name);
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    normalize_stdout(output.stdout)
}

#[test]
fn check_loads_default_import_alias_and_imported_return_type() {
    let dir = TestDir::new("default_import");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func answer() -> int
    return 11
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util

func main()
    const value: int = util.answer()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        !diagnostic_codes(&out).contains(&"bind.unused_import"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn doc_extracts_markdown_from_documentation_comments() {
    let dir = TestDir::new("doc_extracts_markdown");
    dir.write(
        "main.orl",
        r#"namespace app.main

--|
Computes an area.

@param width Width in pixels.
@param height Height in pixels.
@returns The computed area.
|--
public func area(width: int, height: int) -> int
    return width * height
end
"#,
    );

    let out = run_doc(&dir.path("main.orl")).unwrap();

    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.markdown.contains("# Ori API Documentation"));
    assert!(out.markdown.contains("## app.main.area"));
    assert!(out.markdown.contains("- Kind: function"));
    assert!(
        out.markdown
            .contains("public func area(width: int, height: int) -> int"),
        "{}",
        out.markdown
    );
    assert!(out.markdown.contains("Computes an area."));
    assert!(out.markdown.contains("- `width`: Width in pixels."));
    assert!(out.markdown.contains("- `height`: Height in pixels."));
    assert!(out.markdown.contains("Returns: The computed area."));
}

#[test]
fn doc_lists_stdlib_modules_collection_signatures_and_constraints() {
    let dir = TestDir::new("doc_stdlib_collections");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
end
"#,
    );

    let out = run_doc(&dir.path("main.orl")).unwrap();

    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.markdown.contains("## Standard Library"));
    assert!(out.markdown.contains("### Modules"));
    assert!(out.markdown.contains("- `ori.map`"));
    assert!(out.markdown.contains("- `ori.heap`"));
    assert!(out.markdown.contains("### Collection Signatures"));
    assert!(
        out.markdown.contains("queue.new<T>() -> queue.Queue<T>"),
        "{}",
        out.markdown
    );
    assert!(
        out.markdown
            .contains("graph.topological_sort<N>(g: graph.Graph<N>) -> list<N>"),
        "{}",
        out.markdown
    );
    assert!(
        out.markdown
            .contains("maps.new<K, V>() -> map<K, V> where K is Hashable and K is Equatable"),
        "{}",
        out.markdown
    );
    assert!(
        out.markdown
            .contains("heap.new<T>() -> heap.Heap<T> where T is Comparable"),
        "{}",
        out.markdown
    );
}

#[test]
fn check_resolves_public_reexported_import() {
    let dir = TestDir::new("public_reexport_check");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func answer(value: int) -> int
    return value + 1
end
"#,
    );
    dir.write(
        "facade.orl",
        r#"namespace app.facade

public import app.util as util
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.facade as api

func main()
    const value: int = api.util.answer(40)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        !diagnostic_codes(&out).contains(&"bind.unused_import"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn build_lowers_public_reexported_import_to_real_symbol() {
    let dir = TestDir::new("public_reexport_build");
    dir.write(
        "model.orl",
        r#"namespace app.model

public struct User
    id: int
    name: string
end

public func make_user() -> User
    return User(id: 34, name: "Ada")
end
"#,
    );
    dir.write(
        "facade.orl",
        r#"namespace app.facade

public import app.model as model
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.facade as api

func make_user() -> api.model.User
    return api.model.make_user()
end

func main()
end
"#,
    );

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("ORI__app_dot_model_dot_make_user"));
    assert!(build.c_source.contains("ORI__app_dot_main_dot_make_user"));
    assert!(!build
        .c_source
        .contains("ORI__app_dot_facade_dot_model_dot_make_user"));
    compile_c_source(&dir, "public_reexport_build", &build.c_source);
}

#[test]
fn check_reports_imported_function_argument_count_mismatch() {
    let dir = TestDir::new("imported_arg_count");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func add_one(value: int) -> int
    return value + 1
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as util

func main()
    const value: int = util.add_one()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.arg_count_mismatch"));
}

#[test]
fn check_reports_imported_function_argument_type_mismatch() {
    let dir = TestDir::new("imported_arg_type");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func add_one(value: int) -> int
    return value + 1
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as util

func main()
    const value: int = util.add_one("bad")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.arg_type_mismatch"));
}

#[test]
fn check_resolves_imported_types_in_signatures_and_builds_c() {
    let dir = TestDir::new("imported_types");
    dir.write(
        "model.orl",
        r#"namespace app.model

public struct User
    id: int
end

public func same(user: User) -> User
    return user
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.model as model

func pass(user: model.User) -> model.User
    return model.same(user)
end

func main()
end
"#,
    );

    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("ORI__app_dot_main_dot_pass"));
    assert!(build.c_source.contains("ORI__app_dot_model_dot_same"));
    assert!(build.c_source.contains("int main(int argc, char** argv)"));
}

#[test]
fn check_resolves_imported_struct_field_type() {
    let dir = TestDir::new("imported_struct_field");
    dir.write(
        "model.orl",
        r#"namespace app.model

public struct User
    id: int
    name: string
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.model as model

func user_id(user: model.User) -> int
    return user.id
end

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_reports_missing_field_on_imported_struct() {
    let dir = TestDir::new("imported_missing_field");
    dir.write(
        "model.orl",
        r#"namespace app.model

public struct User
    id: int
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.model as model

func user_name(user: model.User) -> string
    return user.name
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.no_such_field"));
}

#[test]
fn build_lowers_imported_struct_literal() {
    let dir = TestDir::new("imported_struct_literal");
    dir.write(
        "model.orl",
        r#"namespace app.model

public struct User
    id: int
    name: string
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.model as model

func make_user() -> model.User
    return model.User(id: 7, name: "Ada")
end

func main()
end
"#,
    );

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("ORI__app_dot_main_dot_make_user"));
    assert!(build.c_source.contains(".id = INT64_C(7)"));
    assert!(build.c_source.contains(".name = ORI_STR(\"Ada\")"));
}

#[test]
fn build_lowers_imported_enum_variants() {
    let dir = TestDir::new("imported_enum_variant");
    dir.write(
        "model.orl",
        r#"namespace app.model

public enum Status
    Ready
    Done(code: int)
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.model as model

func ready() -> model.Status
    return model.Status.Ready
end

func done() -> model.Status
    return model.Status.Done(code: 2)
end

func main()
end
"#,
    );

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("__Ready"));
    assert!(build.c_source.contains("__Done"));
    assert!(build.c_source.contains(".payload.Done"));
}

#[test]
fn check_reports_field_access_on_non_struct() {
    let dir = TestDir::new("field_non_struct");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const value: int = 1.id
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.field_on_non_struct"));
}

#[test]
fn build_handles_same_type_name_in_distinct_imported_namespaces() {
    let dir = TestDir::new("same_type_name");
    std::fs::create_dir_all(dir.path("left")).unwrap();
    std::fs::create_dir_all(dir.path("right")).unwrap();
    dir.write(
        "left/user.orl",
        r#"namespace left.user

public struct User
    id: int
end

public func same(user: User) -> User
    return user
end
"#,
    );
    dir.write(
        "right/user.orl",
        r#"namespace right.user

public struct User
    id: int
end

public func same(user: User) -> User
    return user
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

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
"#,
    );

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert_eq!(build.c_source.matches("int64_t id;").count(), 2);
    assert!(build.c_source.matches("\nstruct ori_def_").count() >= 2);
    assert!(build.c_source.matches("typedef struct ori_def_").count() >= 2);
    assert!(build.c_source.contains("ORI__left_dot_user_dot_same"));
    assert!(build.c_source.contains("ORI__right_dot_user_dot_same"));
}

#[test]
fn build_uses_qualified_names_for_imported_constants() {
    let dir = TestDir::new("imported_constants");
    dir.write(
        "config.orl",
        r#"namespace app.config

public const LIMIT: int = 21
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.config as config

func main()
    const value: int = config.LIMIT
end
"#,
    );

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build
        .c_source
        .contains("static const int64_t app_dot_config_dot_LIMIT = INT64_C(21);"));
    assert!(build
        .c_source
        .contains("int64_t value = app_dot_config_dot_LIMIT;"));
}

#[test]
fn compile_uses_imported_constant_value() {
    let dir = TestDir::new("compile_imported_constant");
    dir.write(
        "config.orl",
        r#"namespace app.config

public const LIMIT: int = 21
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.config as config
import ori.io as io

func main()
    io.print(string(config.LIMIT))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "const_main.exe"
    } else {
        "const_main"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "21");
}

#[test]
fn compile_uses_top_level_constant_global_data() {
    let dir = TestDir::new("compile_global_const_data");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

const LIMIT: int = 31

func main()
    io.print(string(LIMIT))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "global_const.exe"
    } else {
        "global_const"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "31");
}

#[test]
fn compile_updates_top_level_mutable_global() {
    let dir = TestDir::new("compile_global_var_data");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

var counter: int = 2

func bump()
    counter = counter + 5
end

func main()
    bump()
    io.print(string(counter))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "global_var.exe"
    } else {
        "global_var"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "7");
}

#[test]
fn compile_runs_top_level_managed_globals_native() {
    let dir = TestDir::new("compile_global_managed_data");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.map as maps

const PREFIX: string = "start"
var current: string = "one"
var values: list<string> = ["a", "b"]
var labels: map<string, string> = { "x": "old" }

func update()
    current = current + "-two"
    lists.push(values, current)
    maps.set(labels, "x", current)
end

func main()
    update()
    io.print(PREFIX)
    io.print(current)
    io.print(lists.get(values, 2))
    io.print(maps.get(labels, "x"))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "global_managed.exe"
    } else {
        "global_managed"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "start\none-two\none-two\none-two\n"
    );
}

#[test]
fn compile_runs_string_stdlib_len_concat_and_slice() {
    let dir = TestDir::new("compile_string_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.string as str

func main()
    const joined: string = str.concat("ab", "cdef")
    const part: string = str.slice(joined, 1, 4)
    io.print(part)
    io.print(string(str.len(part)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "string_stdlib.exe"
    } else {
        "string_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "bcd\n3\n");
}

#[test]
fn compile_runs_native_to_string_parts() {
    let dir = TestDir::new("compile_native_to_string_parts");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    io.print(string(-120))
    io.print(string(0))
    io.print(string(true))
    io.print(string(2.5))
    const stored: string = string(55)
    const stored_bool: string = string(false)
    const stored_float: string = string(3.25)
    io.print(stored)
    io.print(stored_bool)
    io.print(stored_float)
    io.print(f"{true} {2.5}")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "native_to_string_parts.exe"
    } else {
        "native_to_string_parts"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "-120\n0\ntrue\n2.5\n55\nfalse\n3.25\ntrue 2.5\n"
    );
}

#[test]
fn compile_runs_displayable_string_conversion_native() {
    let dir = TestDir::new("displayable_string_conversion_native");
    dir.write(
        "main.orl",
        r##"namespace app.main

import ori.core as core
import ori.io as io

struct Resource
    id: int
end

implement core.Displayable for Resource
    func display(self) -> string
        return "Resource#" + string(self.id)
    end
end

func main()
    const resource: Resource = Resource(id: 7)
    io.print(string(resource))
    io.print(string("ready"))
    io.print(f"value={resource}")
end
"##,
    );

    let exe = dir.path(if cfg!(windows) {
        "displayable_string_conversion.exe"
    } else {
        "displayable_string_conversion"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "Resource#7\nready\nvalue=Resource#7\n"
    );
}

#[test]
fn build_c_backend_displayable_string_conversion() {
    let dir = TestDir::new("c_backend_displayable_string_conversion");
    dir.write(
        "main.orl",
        r##"namespace app.main

import ori.core as core
import ori.io as io

struct Resource
    id: int
end

implement core.Displayable for Resource
    func display(self) -> string
        return "Resource#" + string(self.id)
    end
end

func main()
    const resource: Resource = Resource(id: 7)
    io.print(string(resource))
    io.print(string("ready"))
    io.print(f"value={resource}")
end
"##,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source
            .contains("ORI__app_dot_main_dot_Resource_dot_Displayable_dot_display"),
        "{}",
        out.c_source
    );

    compile_c_source(
        &dir,
        "c_backend_displayable_string_conversion",
        &out.c_source,
    );
}

#[test]
fn check_rejects_non_displayable_f_string_interpolation() {
    let dir = TestDir::new("non_displayable_f_string");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Secret
    id: int
end

func main()
    const secret: Secret = Secret(id: 7)
    const text: string = f"value={secret}"
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(diagnostic_codes(&out).contains(&"type.arg_type_mismatch"));
}

#[test]
fn compile_runs_native_interpolation_with_string_length_helper() {
    let dir = TestDir::new("compile_native_interpolation_string_len");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const name: string = "Ori"
    io.print(f"{name} language")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "native_interpolation_string_len.exe"
    } else {
        "native_interpolation_string_len"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "Ori language");
}

#[test]
fn compile_runs_extended_string_stdlib() {
    let dir = TestDir::new("compile_extended_string_stdlib");
    dir.write("main.orl", r#"namespace app.main

import ori.io as io
import ori.string as str

func main()
    const trimmed: string = str.trim("  Abc Def  ")
    const lower: string = str.to_lower(trimmed)
    const upper: string = str.to_upper(lower)
    const replaced: string = str.replace(upper, "DEF", "XYZ")
    io.print(replaced)
    if str.contains(replaced, "XYZ") and str.starts_with(replaced, "ABC") and str.ends_with(replaced, "XYZ")
        io.print("ok")
    else
        io.print("bad")
    end
end
"#);

    let exe = dir.path(if cfg!(windows) {
        "extended_string_stdlib.exe"
    } else {
        "extended_string_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "ABC XYZ\nok\n");
}

#[test]
fn compile_runs_more_string_and_conversion_stdlib() {
    let dir = TestDir::new("compile_more_string_conversion_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.convert as conv
import ori.io as io
import ori.string as str

func main()
    const parts: list<string> = str.split("a,b,c", ",")
    io.print(str.join(parts, "|"))
    io.print(str.repeat("ha", 3))
    io.print(str.pad_left("7", 3, "0"))
    io.print(str.pad_right("x", 3, "."))
    io.print(str.trim_start("  left"))
    io.print(str.trim_end("right  "))
    io.print(string(str.index_of("abcdef", "cd")))
    io.print(conv.float_to_string(2.5))
    io.print(conv.bool_to_string(false))
    if some(n) = conv.string_to_int("41")
        io.print(string(n + 1))
    end
    if some(f) = conv.string_to_float("3.5")
        io.print(conv.float_to_string(f))
    end
    match conv.string_to_int("not a number")
        case some(n):
            io.print(string(n))
        case none:
            io.print("int:none")
    end
    match conv.string_to_float("not a number")
        case some(f):
            io.print(conv.float_to_string(f))
        case none:
            io.print("float:none")
    end
    match str.parse_int("42")
        case success(n):
            io.print(string(n + 1))
        case error(message):
            io.print(message)
    end
    match str.parse_int("not a number")
        case success(n):
            io.print(string(n))
        case error(message):
            io.print(message)
    end
    match str.parse_float("6.25")
        case success(f):
            io.print(conv.float_to_string(f))
        case error(message):
            io.print(message)
    end
    match str.parse_float("not a number")
        case success(f):
            io.print(conv.float_to_string(f))
        case error(message):
            io.print(message)
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "more_string_conversion_stdlib.exe"
    } else {
        "more_string_conversion_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "a|b|c\nhahaha\n007\nx..\nleft\nright\n2\n2.5\nfalse\n42\n3.5\nint:none\nfloat:none\n43\ninvalid int\n6.25\ninvalid float\n"
    );
}

#[test]
fn compile_runs_list_index_set_and_len() {
    let dir = TestDir::new("compile_list_index_set_len");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists

func main()
    var values: list<int> = [10, 20, 30]
    values[1] = values[0] + values[2]
    io.print(string(values[1]))
    io.print(string(lists.len(values)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "list_index.exe"
    } else {
        "list_index"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "40\n3\n");
}

#[test]
fn compile_runs_io_read_line() {
    let dir = TestDir::new("compile_io_read_line");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const line: string = io.read_line()
    io.print(line)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "read_line.exe"
    } else {
        "read_line"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let mut child = Command::new(&exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"hello stdin\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "hello stdin\n");
}

#[test]
fn check_reports_stdlib_call_type_error() {
    let dir = TestDir::new("stdlib_call_type_error");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.string as str

func main()
    const bad: string = str.concat("a", 1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.arg_type_mismatch"));
}

#[test]
fn compile_runs_math_stdlib() {
    let dir = TestDir::new("compile_math_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.math as math

func main()
    io.print(string(math.abs(-9)))
    io.print(string(math.min(8, 3) + math.max(8, 3)))
    if math.sqrt(9.0) == 3.0
        io.print("sqrt")
    else
        io.print("bad")
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "math_stdlib.exe"
    } else {
        "math_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "9\n11\nsqrt\n");
}

#[test]
fn compile_runs_more_math_stdlib() {
    let dir = TestDir::new("compile_more_math_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.math as math

func main()
    const floored: int = math.floor(3.9)
    const ceiled: int = math.ceil(3.1)
    const rounded: int = math.round(3.5)
    io.print(string(floored + ceiled + rounded))
    io.print(string(math.clamp(15, 0, 10)))
    if math.pow(2.0, 3.0) == 8.0 and math.log(1.0) == 0.0 and math.log2(1.0) == 0.0
        io.print("powlog")
    else
        io.print("bad")
    end
    if math.sin(0.0) == 0.0 and math.cos(0.0) == 1.0 and math.tan(0.0) == 0.0
        io.print("trig")
    else
        io.print("bad")
    end
    if math.pi > 3.0 and math.e > 2.0 and math.is_nan(math.nan) and math.is_infinite(math.infinity)
        io.print("special")
    else
        io.print("bad")
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "more_math_stdlib.exe"
    } else {
        "more_math_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "11\n10\npowlog\ntrig\nspecial\n"
    );
}

#[test]
fn compile_runs_math_float_overloads() {
    let dir = TestDir::new("math_float_overloads");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.math as math

func main()
    const a: float = math.abs(-2.5)
    const b: float = math.min(1.0, 2.0)
    const c: float = math.max(1.0, 2.0)
    if a == 2.5 and b == 1.0 and c == 2.0
        io.print("float-overloads")
    else
        io.print("bad")
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "math_float_overloads.exe"
    } else {
        "math_float_overloads"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "float-overloads\n");
}

#[test]
fn build_c_backend_compiles_math_stdlib_surface() {
    let dir = TestDir::new("c_backend_math_stdlib_surface");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.math as math

func main()
    const floored: int = math.floor(3.9)
    const clamped: int = math.clamp(15, 0, 10)
    const absf: float = math.abs(-2.5)
    const minf: float = math.min(1.0, 2.0)
    const maxf: float = math.max(1.0, 2.0)
    const logged: float = math.log2(1.0)
    const special: bool = math.is_nan(math.nan) and math.is_infinite(math.infinity)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.c_source.contains("ori_math_clamp"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_math_abs_float"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_math_min_float"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_math_max_float"),
        "{}",
        out.c_source
    );
    assert!(out.c_source.contains("ori_math_log2"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_math_is_nan"), "{}", out.c_source);
    compile_c_source(&dir, "c_backend_math_stdlib_surface", &out.c_source);
}

#[test]
fn compile_runs_string_split_and_chars() {
    let dir = TestDir::new("compile_string_split_chars");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.string as str

func main()
    const parts: list<string> = str.split("red,blue", ",")
    const chars: list<string> = str.chars("abc")
    io.print(parts[0])
    io.print(parts[1])
    io.print(chars[2])
    io.print(string(lists.len(parts) + lists.len(chars)))
end
"#,
    );

    let stdout = compile_and_run(&dir, "string_split_chars");
    assert_eq!(stdout, "red\nblue\nc\n5\n");
}

#[test]
fn compile_runs_set_and_map_stdlib() {
    let dir = TestDir::new("compile_set_map_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.map as maps
import ori.set as sets
import ori.core as core

struct Token
    id: int
end

implement core.Hashable for Token
end

func main()
    const seen: set<int> = sets.new()
    sets.add(seen, 4)
    sets.add(seen, 4)
    sets.add(seen, 9)
    const scores: map<int, int> = maps.new()
    maps.set(scores, 4, 40)
    maps.set(scores, 9, 90)
    maps.set(scores, 4, 44)
    if sets.contains(seen, 9) and maps.contains(scores, 4)
        io.print(string(sets.len(seen) + maps.len(scores)))
        io.print(string(maps.get(scores, 4) + maps.get(scores, 9)))
    else
        io.print("bad")
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "set_map_stdlib.exe"
    } else {
        "set_map_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "4\n134\n");
}

#[test]
fn check_accepts_string_map_keys_string_set_values_and_rejects_unsupported_hash_inputs() {
    let dir = TestDir::new("map_set_string_and_reject_unsupported_hash");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.map as maps
import ori.set as sets

func main()
    const ok_map: map<string, int> = maps.new()
    maps.set(ok_map, "a", 1)
    const ok_lit: map<string, int> = { "b": 2 }

    const ok_set: set<string> = sets.new()
    sets.add(ok_set, "a")
    const ok_set_lit: set<string> = set { "b" }

    const bad_map: map<list<int>, int> = maps.new()
    maps.set(bad_map, [1], 1)

    const bad_map_lit: map<list<int>, int> = { [2]: 2 }
    const bad_set: set<list<int>> = sets.new()
    sets.add(bad_set, [1])
    const bad_set_lit: set<list<int>> = set { [2] }

    const bad_named_map: map<Token, int> = maps.new()
    const bad_named_set: set<Token> = sets.new()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"type.collection_hash_unsupported"),
        "got: {:?}",
        codes
    );
}

#[test]
fn check_list_stdlib_preserves_element_types() {
    let dir = TestDir::new("list_stdlib_element_types");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.list as lists

func main()
    var values: list<int> = [1, 2]
    lists.push(values, "bad")
    lists.set(values, 0, "bad")
    lists.insert(values, 0, "bad")
    const has_bad: bool = lists.contains(values, "bad")
    const bad_index: int = lists.index_of(values, "bad")
    const first: string = lists.get(values, 0)
    const slice: list<string> = lists.slice(values, 0, 1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected list generic mismatches");
    let mismatch_count = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.code, "type.type_mismatch" | "type.arg_type_mismatch"))
        .count();
    assert!(
        mismatch_count >= 6,
        "expected several element-type mismatches, got {:?}",
        out.diagnostics
    );
}

#[test]
fn check_map_set_stdlib_preserves_key_value_and_element_types() {
    let dir = TestDir::new("map_set_stdlib_generic_types");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.map as maps
import ori.set as sets

func main()
    const labels: map<int, string> = maps.new()
    maps.set(labels, "bad", "one")
    maps.set(labels, 2, 20)
    const got_int: int = maps.get(labels, 1)
    const bad_keys: list<string> = maps.keys(labels)
    const bad_values: list<int> = maps.values(labels)
    const bad_entries: list<tuple<string, int>> = maps.entries(labels)

    const seen: set<int> = sets.new()
    sets.add(seen, "bad")
    const has_bad: bool = sets.contains(seen, "bad")
    sets.remove(seen, "bad")
    const other: set<int> = sets.new()
    const wrong_union: set<string> = sets.union(seen, other)
    const bad_union_arg: set<int> = sets.union(seen, "bad")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected map/set generic mismatches");
    let mismatch_count = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.code, "type.type_mismatch" | "type.arg_type_mismatch"))
        .count();
    assert!(
        mismatch_count >= 10,
        "expected several key/value/element mismatches, got {:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_unsupported_optional_result_helper_forms() {
    let dir = TestDir::new("unsupported_optional_result_helper_forms");
    dir.write(
        "main.orl",
        r#"namespace app.main

func maybe() -> optional<int>
    return some(1)
end

func parse() -> result<int, string>
    return success(1)
end

func main()
    const early: int = maybe().or_return(none)
    const wrong_context: result<int, string> = parse().or_wrap(123)
    const wrong_receiver: optional<int> = maybe().or_wrap("context")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(
        out.has_errors,
        "unsupported helper forms should not type-check"
    );
    let messages = out
        .diagnostics
        .iter()
        .map(|d| d.message.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(messages.contains("`or_return`"), "{messages}");
    assert!(messages.contains("`.or_wrap()` context"), "{messages}");
    assert!(messages.contains("`.or_wrap()` can only"), "{messages}");
}

#[test]
fn compile_runs_optional_result_or_helper_native() {
    let dir = TestDir::new("optional_result_or_helper_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func maybe() -> optional<int>
    return some(7)
end

func empty() -> optional<int>
    return none
end

func parse(flag: bool) -> result<int, string>
    if flag
        return success(9)
    end
    return error("bad")
end

func unexpected() -> int
    io.print("unexpected")
    return 99
end

func main()
    io.print(string(maybe().or(unexpected())))
    io.print(string(empty().or(2)))
    io.print(string(parse(true).or(unexpected())))
    io.print(string(parse(false).or(4)))
end
"#,
    );

    let stdout = compile_and_run(&dir, "optional_result_or_helper");
    assert_eq!(stdout, "7\n2\n9\n4\n");
}

#[test]
fn build_c_backend_optional_result_or_helper() {
    let dir = TestDir::new("c_backend_optional_result_or_helper");
    dir.write(
        "main.orl",
        r#"namespace app.main

func maybe(flag: bool) -> optional<int>
    if flag
        return some(1)
    end
    return none
end

func parse(flag: bool) -> result<int, string>
    if flag
        return success(2)
    end
    return error("bad")
end

func main()
    const first: int = maybe(false).or(10)
    const second: int = parse(false).or(20)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains(".has_value ?"), "{}", out.c_source);
    assert!(out.c_source.contains(".is_ok ?"), "{}", out.c_source);
    compile_c_source(&dir, "c_backend_optional_result_or_helper", &out.c_source);
}

#[test]
fn compile_runs_result_or_wrap_helper_native() {
    let dir = TestDir::new("result_or_wrap_helper_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func parse(flag: bool) -> result<int, string>
    if flag
        return success(7)
    end
    return error("bad")
end

func wrapped(flag: bool) -> result<int, string>
    return parse(flag).or_wrap("loading")
end

func main()
    match wrapped(true)
    case success(value):
        io.print(string(value))
    case error(message):
        io.print(message)
    end

    match wrapped(false)
    case success(value):
        io.print(string(value))
    case error(message):
        io.print(message)
    end
end
"#,
    );

    let stdout = compile_and_run(&dir, "result_or_wrap_helper");
    assert_eq!(stdout, "7\nloading: bad\n");
}

#[test]
fn build_c_backend_result_or_wrap_helper() {
    let dir = TestDir::new("c_backend_result_or_wrap_helper");
    dir.write(
        "main.orl",
        r#"namespace app.main

func parse(flag: bool) -> result<int, string>
    if flag
        return success(1)
    end
    return error("bad")
end

func main()
    const first: result<int, string> = parse(true).or_wrap("load")
    const second: result<int, string> = parse(false).or_wrap("load")
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("ori_string_concat"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_result_or_wrap_helper", &out.c_source);
}

#[test]
fn check_reports_map_set_literal_element_mismatches() {
    let dir = TestDir::new("map_set_literal_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const bad_map: map<int, int> = { 1: 10, 2: "two" }
    const bad_set: set<int> = set { 1, "two" }
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"type.map_value_mismatch"),
        "got: {:?}",
        codes
    );
    assert!(
        codes.contains(&"type.set_element_mismatch"),
        "got: {:?}",
        codes
    );
}

#[path = "multifile_imports/collections.rs"]
mod collections;

#[test]
fn build_lowers_default_parameter_arguments_to_c() {
    let dir = TestDir::new("build_default_parameter");
    dir.write(
        "main.orl",
        r#"namespace app.main

func add(base: int, step: int = 5) -> int
    return base + step
end

func main()
    const value: int = add(7)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source
            .contains("ORI__app_dot_main_dot_add(INT64_C(7), INT64_C(5))"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "default_parameter", &out.c_source);
}

#[test]
fn compile_runs_default_parameters_native() {
    let dir = TestDir::new("compile_default_parameter_native");
    dir.write(
        "math.orl",
        r#"namespace app.math

public func scale(value: int, factor: int = 2) -> int
    return value * factor
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.math as math
import ori.io as io

func add(base: int, step: int = 5) -> int
    return base + step
end

func main()
    io.print(string(add(7)))
    io.print(string(add(7, 3)))
    io.print(string(math.scale(4)))
    io.print(string(math.scale(4, 3)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "default_parameter.exe"
    } else {
        "default_parameter"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "12\n10\n8\n12\n");
}

#[test]
fn compile_runs_p4_grammar_forms_native() {
    let dir = TestDir::new("p4_grammar_forms_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Point
    x: int
    y: int
end

func bounded(value: int = 4 if it > 0) -> int
    return value
end

func main()
    const pair: tuple<int, string> = tuple(7, "seven")
    io.print(string(pair.0))
    io.print(pair.1)

    const p: Point = .{ x: bounded(), y: bounded(5) }
    io.print(string(p.x + p.y))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "p4_grammar_forms.exe"
    } else {
        "p4_grammar_forms"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "7\nseven\n9\n");
}

#[test]
fn build_lowers_named_arguments_to_c_order() {
    let dir = TestDir::new("build_named_arguments");
    dir.write(
        "main.orl",
        r#"namespace app.main

func combine(left: int, right: int) -> int
    return left * 10 + right
end

func main()
    const value: int = combine(right: 2, left: 4)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source
            .contains("ORI__app_dot_main_dot_combine(INT64_C(4), INT64_C(2))"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "named_arguments", &out.c_source);
}

#[test]
fn build_lowers_variadic_parameters_to_c() {
    let dir = TestDir::new("build_variadic_parameters");
    dir.write(
        "main.orl",
        r#"namespace app.main

func sum(seed: int, values: int...) -> int
    var total: int = seed
    for value in values
        total = total + value
    end
    return total
end

func main()
    const parts: list<int> = [2, 3]
    const value: int = sum(1, ..parts, 4)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source
            .contains("ORI__app_dot_main_dot_sum(INT64_C(1),"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "variadic_parameters", &out.c_source);
}

#[test]
fn check_reports_spread_outside_variadic_parameter() {
    let dir = TestDir::new("spread_outside_variadic");
    dir.write(
        "main.orl",
        r#"namespace app.main

func take(value: int)
end

func main()
    const parts: list<int> = [1]
    take(..parts)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.spread_non_variadic"));
}

#[test]
fn compile_runs_named_arguments_native() {
    let dir = TestDir::new("compile_named_arguments_native");
    dir.write(
        "math.orl",
        r#"namespace app.math

public func mix(first: int, second: int = 2, third: int = 3) -> int
    return first * 100 + second * 10 + third
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.math as math
import ori.io as io

func pair(left: int, right: int) -> int
    return left * 10 + right
end

func main()
    io.print(string(pair(right: 8, left: 6)))
    io.print(string(math.mix(third: 9, first: 1)))
    io.print(string(math.mix(third: 7, second: 4, first: 2)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "named_arguments.exe"
    } else {
        "named_arguments"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "68\n129\n247\n");
}

#[test]
fn compile_runs_native_parameter_contracts() {
    let ok_dir = TestDir::new("compile_native_param_contract_ok");
    ok_dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func require_positive(value: int if it > 0) -> int
    return value
end

func gap(value: int, start: int if it < value) -> int
    return value - start
end

func main()
    io.print(string(require_positive(3)))
    io.print(string(gap(7, 5)))
end
"#,
    );

    let ok_exe = ok_dir.path(if cfg!(windows) {
        "param_contract_ok.exe"
    } else {
        "param_contract_ok"
    });
    let ok_out = run_compile(&ok_dir.path("main.orl"), Path::new(&ok_exe)).unwrap();
    assert!(!ok_out.has_errors, "{:?}", ok_out.diagnostics);

    let ok_output = Command::new(&ok_exe).output().unwrap();
    assert!(ok_output.status.success(), "{:?}", ok_output);
    let stdout = String::from_utf8(ok_output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "3\n2\n");

    let fail_dir = TestDir::new("compile_native_param_contract_fail");
    fail_dir.write(
        "main.orl",
        r#"namespace app.main

func require_positive(value: int if it > 0) -> int
    return value
end

func main()
    require_positive(0)
end
"#,
    );

    let fail_exe = fail_dir.path(if cfg!(windows) {
        "param_contract_fail.exe"
    } else {
        "param_contract_fail"
    });
    let fail_out = run_compile(&fail_dir.path("main.orl"), Path::new(&fail_exe)).unwrap();
    assert!(!fail_out.has_errors, "{:?}", fail_out.diagnostics);

    let fail_output = Command::new(&fail_exe).output().unwrap();
    assert!(!fail_output.status.success(), "{:?}", fail_output);
}

#[test]
fn compile_runs_native_struct_field_contracts() {
    let ok_dir = TestDir::new("compile_native_field_contract_ok");
    ok_dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Positive
    value: int if it > 0
end

func main()
    const item: Positive = Positive(value: 4)
    io.print(string(item.value))
end
"#,
    );

    let ok_exe = ok_dir.path(if cfg!(windows) {
        "field_contract_ok.exe"
    } else {
        "field_contract_ok"
    });
    let ok_out = run_compile(&ok_dir.path("main.orl"), Path::new(&ok_exe)).unwrap();
    assert!(!ok_out.has_errors, "{:?}", ok_out.diagnostics);

    let ok_output = Command::new(&ok_exe).output().unwrap();
    assert!(ok_output.status.success(), "{:?}", ok_output);
    let stdout = String::from_utf8(ok_output.stdout).unwrap();
    assert_eq!(stdout.trim(), "4");

    let fail_dir = TestDir::new("compile_native_field_contract_fail");
    fail_dir.write(
        "main.orl",
        r#"namespace app.main

struct Positive
    value: int if it > 0
end

func main()
    const item: Positive = Positive(value: 0)
end
"#,
    );

    let fail_exe = fail_dir.path(if cfg!(windows) {
        "field_contract_fail.exe"
    } else {
        "field_contract_fail"
    });
    let fail_out = run_compile(&fail_dir.path("main.orl"), Path::new(&fail_exe)).unwrap();
    assert!(!fail_out.has_errors, "{:?}", fail_out.diagnostics);

    let fail_output = Command::new(&fail_exe).output().unwrap();
    assert!(!fail_output.status.success(), "{:?}", fail_output);
}

#[test]
fn compile_runs_variadic_parameters_native() {
    let dir = TestDir::new("compile_variadic_parameters_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func sum(seed: int, values: int...) -> int
    var total: int = seed
    for value in values
        total = total + value
    end
    return total
end

func count(values: int...) -> int
    var total: int = 0
    for value in values
        total = total + 1
    end
    return total
end

func main()
    const parts: list<int> = [6, 7]
    io.print(string(sum(10, 1, 2, 3)))
    io.print(string(sum(1, ..parts, 8)))
    io.print(string(count()))
    io.print(string(count(4, 5)))
    io.print(string(count(..parts)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "variadic_parameters.exe"
    } else {
        "variadic_parameters"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "16\n22\n0\n2\n2\n");
}

#[test]
fn compile_runs_generic_function_monomorphization_native() {
    let dir = TestDir::new("compile_generic_monomorph_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func identity<T>(value: T) -> T
    return value
end

func pick_second<T>(first: T, second: T) -> T
    return second
end

func wrap<T>(value: T) -> T
    return identity(value)
end

func main()
    const answer: int = identity(41) + 1
    const label: string = identity("ok")
    io.print(string(answer))
    io.print(label)
    io.print(string(pick_second(3, 7)))
    io.print(wrap("done"))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "generic_monomorph.exe"
    } else {
        "generic_monomorph"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "42\nok\n7\ndone\n");
}

#[test]
fn compile_runs_managed_generic_trait_and_any_native() {
    let dir = TestDir::new("compile_managed_generic_trait_any_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

trait Labelled
    func label(self) -> string
end

struct Tag
    label: string
end

implement Labelled for Tag
    func label(self) -> string
        return self.label
    end
end

func choose<T>(first: T, second: T) -> T
    return second
end

func generic_label<T>(value: T) -> string where T is Labelled
    return value.label()
end

func any_label(value: any<Labelled>) -> string
    return value.label()
end

func same_any(value: any<Labelled>) -> any<Labelled>
    return value
end

func main()
    const picked: Tag = choose(Tag(label: "old"), Tag(label: "new"))
    const boxed: any<Labelled> = picked
    io.print(generic_label(picked))
    io.print(any_label(picked))
    io.print(same_any(boxed).label())
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "managed_generic_trait_any.exe"
    } else {
        "managed_generic_trait_any"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "new\nnew\nnew\n");
}

#[test]
fn compile_runs_transitive_imports_with_generic_traits_native() {
    let dir = TestDir::new("compile_transitive_generic_traits_native");
    dir.write(
        "traits.orl",
        r#"namespace app.traits

public trait Named
    func name(self) -> string
end

public func read_name<T>(value: T) -> string where T is Named
    return value.name()
end
"#,
    );
    dir.write(
        "models.orl",
        r#"namespace app.models

public import app.traits as traits

public struct User
    name: string
end

implement traits.Named for User
    func name(self) -> string
        return self.name
    end
end

public func make_user(name: string) -> User
    return User(name: name)
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.models as models
import ori.io as io

func main()
    const user: models.User = models.make_user("Ada")
    io.print(models.traits.read_name(user))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "transitive_generic_traits.exe"
    } else {
        "transitive_generic_traits"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "Ada\n");
}

#[test]
fn compile_runs_any_trait_dynamic_dispatch_native() {
    let dir = TestDir::new("compile_any_trait_dispatch_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Player
    score: int
end

struct Booster
    score: int
end

trait Scored
    func score(self) -> int

    func bonus(self) -> int
        return 5
    end
end

implement Scored for Player
    func score(self) -> int
        return self.score
    end
end

implement Scored for Booster
    func score(self) -> int
        return self.score
    end

    func bonus(self) -> int
        return 9
    end
end

func add_bonus(item: any<Scored>) -> int
    return item.score() + 5
end

func identity(item: any<Scored>) -> any<Scored>
    return item
end

func main()
    const player: Player = Player(score: 37)
    const booster: Booster = Booster(score: 20)
    const item: any<Scored> = player
    const boosted: any<Scored> = booster
    io.print(string(item.score()))
    io.print(string(add_bonus(player)))
    io.print(string(identity(player).score()))
    io.print(string(player.bonus()))
    io.print(string(item.bonus()))
    io.print(string(boosted.bonus()))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "any_trait_dispatch.exe"
    } else {
        "any_trait_dispatch"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "37\n42\n37\n5\n5\n9\n");
}

#[test]
fn build_c_backend_compiles_any_trait_dynamic_dispatch() {
    let dir = TestDir::new("build_any_trait_dispatch");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Player
    score: int
end

struct Booster
    score: int
end

trait Scored
    func score(self) -> int

    func bonus(self) -> int
        return 5
    end
end

implement Scored for Player
    func score(self) -> int
        return self.score
    end
end

implement Scored for Booster
    func score(self) -> int
        return self.score
    end

    func bonus(self) -> int
        return 9
    end
end

func add_bonus(item: any<Scored>) -> int
    return item.score() + 5
end

func identity(item: any<Scored>) -> any<Scored>
    return item
end

func main()
    const player: Player = Player(score: 37)
    const booster: Booster = Booster(score: 20)
    const item: any<Scored> = player
    const boosted: any<Scored> = booster
    io.print(string(item.score()))
    io.print(string(add_bonus(player)))
    io.print(string(identity(player).score()))
    io.print(string(player.bonus()))
    io.print(string(item.bonus()))
    io.print(string(boosted.bonus()))
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    compile_c_source(&dir, "any_trait_dispatch", &out.c_source);
}

#[test]
fn check_allows_any_trait_equality() {
    let dir = TestDir::new("any_trait_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main

trait Scored
    func score(self) -> int
end

struct Player
    score: int
end

implement Scored for Player
    func score(self) -> int
        return self.score
    end
end

func main()
    const a: any<Scored> = Player(score: 1)
    const b: any<Scored> = Player(score: 1)
    const same: bool = a == b
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    compile_c_source(&dir, "any_trait_equality", &out.c_source);
}

#[test]
fn check_reports_function_value_equality() {
    let dir = TestDir::new("function_value_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Handler
    run: func(int) -> int
end

func first(x: int) -> int
    return x
end

func second(x: int) -> int
    return x + 1
end

func main()
    const f: func(int) -> int = do(x: int) => x
    const g: func(int) -> int = do(x: int) => x + 1
    const closures_equal: bool = f == g
    const functions_equal: bool = first == second
    const a: Handler = Handler(run: f)
    const b: Handler = Handler(run: g)
    const handlers_equal: bool = a == b
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert!(
        codes
            .iter()
            .filter(|code| **code == "type.comparison_not_supported")
            .count()
            >= 2,
        "{:?}",
        out.diagnostics
    );
    assert!(
        codes.contains(&"type.equality_unsupported_field"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn compile_runs_struct_structural_equality_native() {
    let dir = TestDir::new("struct_structural_equality_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Address
    city: string
    zip: int
end

struct User
    id: int
    name: string
    scores: list<int>
    address: Address
end

func main()
    const left: User = User(
        id: 1,
        name: "Ada",
        scores: [10, 20],
        address: Address(city: "Recife", zip: 50000),
    )
    const same: User = User(
        id: 1,
        name: "Ada",
        scores: [10, 20],
        address: Address(city: "Recife", zip: 50000),
    )
    const different_name: User = User(
        id: 1,
        name: "Bia",
        scores: [10, 20],
        address: Address(city: "Recife", zip: 50000),
    )
    const different_nested: User = User(
        id: 1,
        name: "Ada",
        scores: [10, 20],
        address: Address(city: "Olinda", zip: 50000),
    )
    const different_list: User = User(
        id: 1,
        name: "Ada",
        scores: [10, 21],
        address: Address(city: "Recife", zip: 50000),
    )

    io.print(string(left == same))
    io.print(string(left != different_name))
    io.print(string(left != different_nested))
    io.print(string(left == different_list))
end
"#,
    );

    let stdout = compile_and_run(&dir, "struct_structural_equality");
    assert_eq!(stdout, "true\ntrue\ntrue\nfalse\n");
}

#[test]
fn compile_runs_list_structural_equality_native() {
    let dir = TestDir::new("list_structural_equality_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const left: list<int> = [1, 2, 3]
    const same: list<int> = [1, 2, 3]
    const different_value: list<int> = [1, 2, 4]
    const different_len: list<int> = [1, 2]

    io.print(string(left == same))
    io.print(string(left == different_value))
    io.print(string(left != different_value))
    io.print(string(left != different_len))

    const words: list<string> = ["ori", "lang"]
    const same_words: list<string> = ["ori", "lang"]
    const other_words: list<string> = ["ori", "runtime"]

    io.print(string(words == same_words))
    io.print(string(words != other_words))
end
"#,
    );

    let stdout = compile_and_run(&dir, "list_structural_equality");
    assert_eq!(stdout, "true\nfalse\ntrue\ntrue\ntrue\ntrue\n");
}

#[test]
fn compile_runs_set_map_structural_equality_native() {
    let dir = TestDir::new("set_map_structural_equality_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.map as maps
import ori.set as sets

func main()
    const left_set: set<int> = sets.new()
    sets.add(left_set, 1)
    sets.add(left_set, 2)

    const same_set: set<int> = sets.new()
    sets.add(same_set, 2)
    sets.add(same_set, 1)

    const different_set: set<int> = sets.new()
    sets.add(different_set, 1)
    sets.add(different_set, 3)

    io.print(string(left_set == same_set))
    io.print(string(left_set != different_set))

    const words: set<string> = sets.new()
    sets.add(words, "red")
    sets.add(words, "blue")

    const same_words: set<string> = sets.new()
    sets.add(same_words, "blue")
    sets.add(same_words, "red")

    const other_words: set<string> = sets.new()
    sets.add(other_words, "red")
    sets.add(other_words, "green")

    io.print(string(words == same_words))
    io.print(string(words == other_words))

    const scores: map<int, int> = maps.new()
    maps.set(scores, 1, 10)
    maps.set(scores, 2, 20)

    const same_scores: map<int, int> = maps.new()
    maps.set(same_scores, 2, 20)
    maps.set(same_scores, 1, 10)

    const changed_scores: map<int, int> = maps.new()
    maps.set(changed_scores, 1, 10)
    maps.set(changed_scores, 2, 99)

    io.print(string(scores == same_scores))
    io.print(string(scores != changed_scores))

    const labels: map<string, int> = maps.new()
    maps.set(labels, "a", 1)
    maps.set(labels, "b", 2)

    const same_labels: map<string, int> = maps.new()
    maps.set(same_labels, "b", 2)
    maps.set(same_labels, "a", 1)

    io.print(string(labels == same_labels))

    const buckets: map<int, list<int>> = maps.new()
    maps.set(buckets, 1, [1, 2])

    const same_buckets: map<int, list<int>> = maps.new()
    maps.set(same_buckets, 1, [1, 2])

    const changed_buckets: map<int, list<int>> = maps.new()
    maps.set(changed_buckets, 1, [1, 3])

    io.print(string(buckets == same_buckets))
    io.print(string(buckets != changed_buckets))
end
"#,
    );

    let stdout = compile_and_run(&dir, "set_map_structural_equality");
    assert_eq!(
        stdout,
        "true\ntrue\ntrue\nfalse\ntrue\ntrue\ntrue\ntrue\ntrue\n"
    );
}

#[test]
fn build_c_backend_list_structural_equality() {
    let dir = TestDir::new("c_backend_list_structural_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const left: list<int> = [1, 2, 3]
    const same: list<int> = [1, 2, 3]
    const different: list<int> = [1, 2, 4]
    const words: list<string> = ["ori", "lang"]
    const same_words: list<string> = ["ori", "lang"]

    const ints_equal: bool = left == same
    const ints_different: bool = left != different
    const words_equal: bool = words == same_words
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("ori_list_at") && out.c_source.contains("ori_string_eq"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_list_structural_equality", &out.c_source);
}

#[test]
fn build_c_backend_struct_structural_equality() {
    let dir = TestDir::new("c_backend_struct_structural_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Address
    city: string
    zip: int
end

struct User
    id: int
    name: string
    address: Address
end

func main()
    const left: User = User(id: 1, name: "Ada", address: Address(city: "Recife", zip: 50000))
    const same: User = User(id: 1, name: "Ada", address: Address(city: "Recife", zip: 50000))
    const different: User = User(id: 1, name: "Ada", address: Address(city: "Olinda", zip: 50000))

    const users_equal: bool = left == same
    const users_different: bool = left != different
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("ori_string_eq")
            && out.c_source.contains(".address")
            && out.c_source.contains(".city"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_struct_structural_equality", &out.c_source);
}

#[test]
fn build_c_backend_set_map_structural_equality() {
    let dir = TestDir::new("c_backend_set_map_structural_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.map as maps
import ori.set as sets

func main()
    const left_set: set<int> = set { 1, 2 }
    const same_set: set<int> = set { 2, 1 }
    const different_set: set<int> = set { 1, 3 }
    const set_same: bool = left_set == same_set
    const set_different: bool = left_set != different_set

    const words: set<string> = set { "red", "blue" }
    const same_words: set<string> = set { "blue", "red" }
    sets.add(same_words, "red")
    const words_same: bool = words == same_words

    const labels: map<string, int> = { "a": 1, "b": 2 }
    const same_labels: map<string, int> = { "b": 2, "a": 1 }
    const labels_same: bool = labels == same_labels

    const buckets: map<int, list<int>> = maps.new()
    maps.set(buckets, 1, [1, 2])

    const same_buckets: map<int, list<int>> = maps.new()
    maps.set(same_buckets, 1, [1, 2])

    const changed_buckets: map<int, list<int>> = maps.new()
    maps.set(changed_buckets, 1, [1, 3])

    const nested_same: bool = buckets == same_buckets
    const nested_different: bool = buckets != changed_buckets
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("ori_map_set_string_value")
            && out.c_source.contains("ori_set_contains_string")
            && out.c_source.contains("ori_map_value_at"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_set_map_structural_equality", &out.c_source);
}

#[test]
fn compile_runs_optional_result_inequality_native() {
    let dir = TestDir::new("optional_result_inequality_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func fail_a() -> result<int, string>
    return error("a")
end

func fail_b() -> result<int, string>
    return error("b")
end

func main()
    const maybe_one: optional<int> = some(1)
    const maybe_two: optional<int> = some(2)
    const missing: optional<int> = none

    io.print(string(maybe_one != maybe_two))
    io.print(string(maybe_one != missing))
    io.print(string(missing != none))

    const ok_one: result<int, string> = success(1)
    const ok_two: result<int, string> = success(2)
    const err_a: result<int, string> = fail_a()
    const err_b: result<int, string> = fail_b()

    io.print(string(ok_one != ok_two))
    io.print(string(ok_one != err_a))
    io.print(string(err_a != err_b))
    io.print(string(err_a != fail_a()))
end
"#,
    );

    let stdout = compile_and_run(&dir, "optional_result_inequality");
    assert_eq!(stdout, "true\ntrue\nfalse\ntrue\ntrue\ntrue\nfalse\n");
}

#[test]
fn check_reports_non_numeric_ordering() {
    let dir = TestDir::new("non_numeric_ordering");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const bool_order: bool = true < false
    const string_order: bool = "a" < "b"
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert_eq!(
        codes
            .iter()
            .filter(|code| **code == "type.comparison_not_supported")
            .count(),
        2
    );
}

#[test]
fn build_lowers_operator_overloads_through_core_traits() {
    let dir = TestDir::new("operator_overload_traits");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.core as core

struct Score
    value: int
end

implement core.Addable for Score
    func add(self, other: Score) -> Score
        return Score(value: self.value + other.value)
    end
end

implement core.Subtractable for Score
    func subtract(self, other: Score) -> Score
        return Score(value: self.value - other.value)
    end
end

implement core.Equatable for Score
    func equals(self, other: Score) -> bool
        return self.value == other.value
    end
end

implement core.Comparable for Score
    func compare(self, other: Score) -> int
        return self.value - other.value
    end
end

func main()
    const left: Score = Score(value: 3)
    const right: Score = Score(value: 5)
    const sum: Score = left + right
    const diff: Score = right - left
    const same: bool = left == right
    const different: bool = left != right
    const smaller: bool = left < right
    const at_most: bool = left <= right
    const larger: bool = right > left
    const at_least: bool = right >= left
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out
        .c_source
        .contains("ORI__app_dot_main_dot_Score_dot_Addable_dot_add"));
    assert!(out
        .c_source
        .contains("ORI__app_dot_main_dot_Score_dot_Subtractable_dot_subtract"));
    assert!(out
        .c_source
        .contains("ORI__app_dot_main_dot_Score_dot_Equatable_dot_equals"));
    assert!(out
        .c_source
        .contains("ORI__app_dot_main_dot_Score_dot_Comparable_dot_compare"));
}

#[test]
fn compile_runs_operator_overloads_native() {
    let dir = TestDir::new("operator_overload_traits_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.core as core
import ori.io as io

struct Score
    value: int
end

implement core.Addable for Score
    func add(self, other: Score) -> Score
        return Score(value: self.value + other.value)
    end
end

implement core.Subtractable for Score
    func subtract(self, other: Score) -> Score
        return Score(value: self.value - other.value)
    end
end

implement core.Equatable for Score
    func equals(self, other: Score) -> bool
        return self.value == other.value
    end
end

implement core.Comparable for Score
    func compare(self, other: Score) -> int
        return self.value - other.value
    end
end

func main()
    const left: Score = Score(value: 3)
    const right: Score = Score(value: 5)
    const sum: Score = left + right
    const diff: Score = right - left
    io.print(string(sum.value))
    io.print(string(diff.value))
    io.print(if left == Score(value: 3) then "eq" else "bad")
    io.print(if left != right then "ne" else "bad")
    io.print(if left < right then "lt" else "bad")
    io.print(if left <= right then "le" else "bad")
    io.print(if right > left then "gt" else "bad")
    io.print(if right >= left then "ge" else "bad")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "operator_overloads.exe"
    } else {
        "operator_overloads"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "8\n2\neq\nne\nlt\nle\ngt\nge\n"
    );
}

#[test]
fn compile_runs_managed_operator_overloads_native() {
    let dir = TestDir::new("managed_operator_overload_traits_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.core as core
import ori.io as io
import ori.string as strings

struct Label
    text: string
end

implement core.Addable for Label
    func add(self, other: Label) -> Label
        return Label(text: self.text + other.text)
    end
end

implement core.Equatable for Label
    func equals(self, other: Label) -> bool
        return self.text == other.text
    end
end

implement core.Comparable for Label
    func compare(self, other: Label) -> int
        return strings.len(self.text) - strings.len(other.text)
    end
end

func main()
    const left: Label = Label(text: "ori")
    const right: Label = Label(text: "-lang")
    const joined: Label = left + right
    io.print(joined.text)
    io.print(if joined == Label(text: "ori-lang") then "eq" else "bad")
    io.print(if left < right then "lt" else "bad")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "managed_operator_overloads.exe"
    } else {
        "managed_operator_overloads"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "ori-lang\neq\nlt\n");
}

#[test]
fn build_c_backend_compiles_string_equality() {
    let dir = TestDir::new("c_string_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const same: bool = "ori" == "ori"
    const different: bool = "ori" != "ora"
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("ori_string_eq"));
    compile_c_source(&dir, "c_string_equality", &out.c_source);
}

#[test]
fn check_accepts_lazy_type_and_stdlib_once_force() {
    let dir = TestDir::new("lazy_type_once_force");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.lazy as lz

func main()
    const delayed: lazy<int> = lz.once(do() => 41)
    const value: int = lz.force(delayed)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(!diagnostic_codes(&out).contains(&"type.lazy_not_implemented"));
}

#[test]
fn build_c_backend_compiles_lazy_once_force() {
    let dir = TestDir::new("c_lazy_once_force");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.lazy as lz

func main()
    const delayed: lazy<int> = lz.once(do() => 41)
    const first: int = lz.force(delayed)
    const second: int = lz.force(delayed)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("ori_lazy_i64_t"));
    assert!(out.c_source.contains("->forced"));
    compile_c_source(&dir, "c_lazy_once_force", &out.c_source);
}

#[test]
fn build_c_backend_emits_json_parse_extern_without_c_lowering() {
    let dir = TestDir::new("c_backend_json_extern_only");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.json as json

func main()
    const parsed: result<json.Value, string> = json.parse("{}")
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(
        !out.has_errors,
        "C backend may emit JSON calls as runtime FFI stubs: {:?}",
        out.diagnostics
    );
    assert!(
        out.c_source.contains("ori_json_parse"),
        "expected C source to reference native JSON runtime symbol"
    );
}

#[test]
fn build_c_backend_reports_unsupported_feature_diagnostic() {
    let dir = TestDir::new("c_backend_unsupported_feature_diagnostic");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.lazy as lz

func main()
    const delayed: lazy<void> = lz.once(do() => io.print("x"))
    lz.force(delayed)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected C backend feature diagnostic");
    assert!(
        out.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "backend.c_unsupported"),
        "{:?}",
        out.diagnostics
    );
    assert!(out.c_source.is_empty());
}

#[test]
fn compile_runs_native_lazy_once_force_once() {
    let dir = TestDir::new("native_lazy_once_force");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.lazy as lz

var calls: int = 0

func compute() -> int
    calls = calls + 1
    return 41
end

func main()
    const delayed: lazy<int> = lz.once(do() => compute())
    const first: int = lz.force(delayed)
    const second: int = lz.force(delayed)
    io.print(string(first + second + calls))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "native_lazy_once_force.exe"
    } else {
        "native_lazy_once_force"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim_end(), "83");
}

#[test]
fn compile_runs_using_dispose_on_native_scope_exit() {
    let dir = TestDir::new("compile_using_dispose_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

var disposed: int = 0

trait Disposable
    mut func dispose(self)
end

struct Resource
    id: int
end

implement Disposable for Resource
    mut func dispose(self)
        disposed = disposed * 10 + self.id
    end
end

func use_normal()
    using first: Resource = Resource(id: 1)
    using second: Resource = Resource(id: 2)
    io.print("inside")
end

func use_return() -> int
    using third: Resource = Resource(id: 3)
    return 7
end

func fail() -> result<int, string>
    return error("fail")
end

func use_propagate() -> result<int, string>
    using fourth: Resource = Resource(id: 4)
    const value: int = fail()?
    return success(value)
end

func use_break()
    loop
        using fifth: Resource = Resource(id: 5)
        break
    end
end

func use_continue()
    var done: bool = false
    loop
        using sixth: Resource = Resource(id: 6)
        if done
            break
        end
        done = true
        continue
    end
end

func main()
    use_normal()
    io.print(string(disposed))

    const value: int = use_return()
    io.print(string(value))
    io.print(string(disposed))

    match use_propagate()
    case success(ok):
        io.print(string(ok))
    case error(message):
        io.print(message)
    end
    io.print(string(disposed))

    use_break()
    io.print(string(disposed))

    use_continue()
    io.print(string(disposed))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "using_dispose.exe"
    } else {
        "using_dispose"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "inside\n21\n7\n213\nfail\n2134\n21345\n2134566\n"
    );
}

#[test]
fn compile_runs_using_dispose_before_native_check_trap() {
    let dir = TestDir::new("compile_using_dispose_native_trap");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

trait Disposable
    mut func dispose(self)
end

struct Resource
    id: int
end

implement Disposable for Resource
    mut func dispose(self)
        io.print("disposed")
    end
end

func main()
    using resource: Resource = Resource(id: 1)
    check false
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "using_dispose_trap.exe"
    } else {
        "using_dispose_trap"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    assert_eq!(stdout, "disposed\n");
}

#[test]
fn compile_runs_using_dispose_before_native_panic() {
    let dir = TestDir::new("compile_using_dispose_native_panic");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

trait Disposable
    mut func dispose(self)
end

struct Resource
    id: int
end

implement Disposable for Resource
    mut func dispose(self)
        io.print("disposed")
    end
end

func main()
    using resource: Resource = Resource(id: 1)
    panic("boom")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "using_dispose_panic.exe"
    } else {
        "using_dispose_panic"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stdout, "disposed\n");
    assert!(stderr.contains("ori panic: boom"), "{stderr}");
}

#[test]
fn compile_runs_result_match_and_propagation() {
    let dir = TestDir::new("compile_result_match_propagation");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func parse(flag: bool) -> result<int, string>
    if flag
        return success(7)
    end
    return error("no value")
end

func add_one(flag: bool) -> result<int, string>
    const value: int = parse(flag)?
    return success(value + 1)
end

func main()
    match add_one(true)
    case success(value):
        io.print(string(value))
    case error(message):
        io.print(message)
    end

    match add_one(false)
    case success(value):
        io.print(string(value))
    case error(message):
        io.print(message)
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "result_match.exe"
    } else {
        "result_match"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "8\nno value\n");
}

#[test]
fn compile_runs_native_composite_values_and_patterns() {
    let dir = TestDir::new("compile_native_composite_values");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct User
    id: int
    name: string
end

enum Status
    Ready
    Done(code: int)
end

func make_user() -> User
    return User(id: 10, name: "Ada")
end

func pair() -> tuple<int, string>
    return (4, "ok")
end

func status() -> Status
    return Status.Done(code: 9)
end

func main()
    const user: User = make_user()
    io.print(string(user.id))
    match user.name
    case "Ada":
        io.print("name")
    case else:
        io.print("bad")
    end

    const item: tuple<int, string> = pair()
    io.print(string(item.0))
    io.print(item.1)
    match item
    case tuple(4, "ok"):
        io.print("tuple")
    case else:
        io.print("bad")
    end

    match status()
    case Done(code):
        io.print(string(code))
    case else:
        io.print("bad")
    end

    io.print(f"literal")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "native_composites.exe"
    } else {
        "native_composites"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "10\nname\n4\nok\ntuple\n9\nliteral\n"
    );
}

#[test]
fn compile_runs_deep_match_with_managed_enum_payload_native() {
    let dir = TestDir::new("compile_deep_match_managed_payload_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct User
    name: string
end

enum Event
    Empty
    Text(value: string)
    Record(user: User)
    Pair(data: tuple<int, string>)
end

func event(kind: int) -> Event
    if kind == 0
        return Event.Text(value: "ready")
    end
    if kind == 1
        return Event.Record(user: User(name: "Ada"))
    end
    if kind == 2
        return Event.Pair(data: tuple(7, "seven"))
    end
    return Event.Empty
end

func main()
    match event(0)
    case Text(value):
        io.print(value)
    case else:
        io.print("bad")
    end

    match event(1)
    case Record(user):
        io.print(user.name)
    case else:
        io.print("bad")
    end

    match event(2)
    case Pair(data: tuple(id, label)):
        io.print(string(id))
        io.print(label)
    case else:
        io.print("bad")
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "deep_match_managed_payload.exe"
    } else {
        "deep_match_managed_payload"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "ready\nAda\n7\nseven\n");
}

#[test]
fn compile_runs_native_showcase_example() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("ori-driver crate is inside compiler/crates");
    let source = repo_root.join("examples/native_showcase.orl");
    let dir = TestDir::new("compile_native_showcase_example");
    let exe = dir.path(if cfg!(windows) {
        "native_showcase.exe"
    } else {
        "native_showcase"
    });

    let out = run_compile(&source, Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "Grace:admin\nGrace:admin\nboot\nGrace\n7\nseven\ndisposed-1\n"
    );
}

#[test]
fn check_infers_is_expression_as_bool() {
    let dir = TestDir::new("is_expression_bool");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct User
    id: int
end

func main()
    const user: User = User(id: 1)
    const is_user: bool = user is User
    const is_int: bool = 1 is int
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn build_c_backend_compiles_is_check() {
    let dir = TestDir::new("c_backend_is_check");
    dir.write(
        "main.orl",
        r#"namespace app.main

trait Shape
    func area(self) -> int
end

struct User
    id: int
end

struct Circle
    radius: int
end

implement Shape for Circle
    func area(self) -> int
        return self.radius * self.radius
    end
end

func describe(s: any<Shape>) -> bool
    return s is Circle
end

func main()
    const user: User = User(id: 1)
    const is_user: bool = user is User
    const is_circle: bool = user is Circle
    const is_int: bool = 1 is int
    const shape: any<Shape> = Circle(radius: 3)
    const is_shape_circle: bool = describe(shape)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("vtable)[0]"), "{}", out.c_source);
    compile_c_source(&dir, "c_backend_is_check", &out.c_source);
}

#[test]
fn build_c_backend_compiles_propagation() {
    let dir = TestDir::new("c_backend_propagation");
    dir.write(
        "main.orl",
        r#"namespace app.main

func parse(flag: bool) -> result<int, string>
    if flag
        return success(7)
    end
    return error("no value")
end

func maybe(flag: bool) -> optional<int>
    if flag
        return some(3)
    end
    return none
end

func add_one(flag: bool) -> result<int, string>
    const value: int = parse(flag)?
    return success(value + 1)
end

func unwrap_optional(flag: bool) -> optional<int>
    const value: int = maybe(flag)?
    return some(value + 1)
end

func main()
    const a: result<int, string> = add_one(true)
    const b: optional<int> = unwrap_optional(true)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("return ((ori_result_"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("return ((ori_opt_"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_propagation", &out.c_source);
}

#[test]
fn check_enforces_function_where_clause_at_call_site() {
    let dir = TestDir::new("where_constraint_call");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Good
    id: int
end

struct Plain
    id: int
end

trait Marker
    func mark(self) -> int
end

implement Marker for Good
    func mark(self) -> int
        return self.id
    end
end

func require_marker<T>(value: T) -> int where T is Marker
    return 1
end

func main()
    const good: Good = Good(id: 1)
    const plain: Plain = Plain(id: 2)
    const ok: int = require_marker(good)
    const bad: int = require_marker(plain)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"generic.constraint_not_satisfied"));
}

#[test]
fn check_enforces_negative_function_where_clause_at_call_site() {
    let dir = TestDir::new("negative_where_constraint_call");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Plain
    id: int
end

struct Marked
    id: int
end

trait Marker
    func mark(self) -> int
end

implement Marker for Marked
    func mark(self) -> int
        return self.id
    end
end

func reject_marker<T>(value: T) -> int where T is not Marker
    return 1
end

func main()
    const plain: Plain = Plain(id: 1)
    const marked: Marked = Marked(id: 2)
    const ok: int = reject_marker(plain)
    const bad: int = reject_marker(marked)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"generic.negative_constraint_violated"));
}

#[test]
fn check_reports_circular_generic_instantiation() {
    let dir = TestDir::new("generic_circular_instantiation");
    dir.write(
        "main.orl",
        r#"namespace app.main

func recurse<T>(value: T) -> T
    return recurse(value)
end

func main()
    const value: int = recurse(1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"generic.circular_instantiation"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_accepts_grouped_where_clause_with_and() {
    let dir = TestDir::new("grouped_where_and");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Good
    id: int
end

trait MarkerA
    func a(self) -> int
end

trait MarkerB
    func b(self) -> int
end

implement MarkerA for Good
    func a(self) -> int
        return self.id
    end
end

implement MarkerB for Good
    func b(self) -> int
        return self.id
    end
end

func require_both<T>(value: T) -> int
    where (
        T is MarkerA
        and T is MarkerB
    )
    return 1
end

func main()
    const good: Good = Good(id: 1)
    const ok: int = require_both(good)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_reports_chained_comparison() {
    let dir = TestDir::new("chained_comparison");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const bad: bool = 1 < 2 < 3
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected chained comparison error");
    assert!(diagnostic_codes(&out).contains(&"parse.chained_comparison"));
}

#[test]
fn check_reports_invalid_lvalue_assignment() {
    let dir = TestDir::new("invalid_lvalue_assignment");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    1 = 2
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected invalid lvalue error");
    assert!(diagnostic_codes(&out).contains(&"parse.invalid_lvalue"));
}

#[test]
fn check_reports_variadic_parameter_not_last() {
    let dir = TestDir::new("variadic_not_last");
    dir.write(
        "main.orl",
        r#"namespace app.main

func bad(values: int..., suffix: int) -> int
    return suffix
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected variadic parameter order error");
    assert!(diagnostic_codes(&out).contains(&"parse.variadic_not_last"));
}

#[test]
fn check_reports_required_parameter_after_default() {
    let dir = TestDir::new("default_before_required");
    dir.write(
        "main.orl",
        r#"namespace app.main

func bad(left: int = 1, right: int) -> int
    return left + right
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected default parameter order error");
    assert!(diagnostic_codes(&out).contains(&"parse.default_before_required"));
}

#[test]
fn check_reports_duplicate_struct_fields_and_enum_variants() {
    let dir = TestDir::new("duplicate_fields_variants");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Point
    x: int
    x: int
end

enum Status
    Ready
    Ready
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected duplicate declaration errors");
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"bind.duplicate_field"), "{codes:?}");
    assert!(codes.contains(&"bind.duplicate_variant"), "{codes:?}");
}

#[test]
fn check_reports_unknown_names_calls_and_paths() {
    let dir = TestDir::new("unknown_names_calls_paths");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const missing_value: int = missing
    const missing_call: int = missing_func()
    const missing_path: int = unknown.module.value
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected unknown name diagnostics");
    let codes = diagnostic_codes(&out);
    let undefined_count = codes
        .iter()
        .filter(|code| **code == "name.undefined")
        .count();
    assert_eq!(undefined_count, 3, "{codes:?}");
}

#[test]
fn check_reports_self_outside_method() {
    let dir = TestDir::new("self_outside_method");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const value: int = self
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected self usage diagnostic");
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"bind.self_outside_method"), "{codes:?}");
    assert!(
        !codes.contains(&"name.undefined"),
        "`self` should not fall back to name.undefined: {codes:?}"
    );
}

#[test]
fn check_reports_logical_operator_non_bool_operands() {
    let dir = TestDir::new("logical_operator_non_bool_operands");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const bad_and: bool = 1 and true
    const bad_or: bool = false or 2
    const bad_not: bool = not 1
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected bool operand diagnostics");
    let codes = diagnostic_codes(&out);
    let expected_bool_count = codes
        .iter()
        .filter(|code| **code == "type.expected_bool")
        .count();
    assert_eq!(expected_bool_count, 3, "{codes:?}");
}

#[test]
fn check_reports_closure_capture_of_var_binding() {
    let dir = TestDir::new("closure_capture_of_var_binding");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    var counter: int = 0
    const snapshot: func() -> int = do() => counter
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected closure capture diagnostic");
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"mut.closure_captures_var"), "{codes:?}");
}

#[test]
fn check_warns_when_result_expression_is_discarded() {
    let dir = TestDir::new("discarded_result_expression");
    dir.write(
        "main.orl",
        r#"namespace app.main

func fail() -> result<int, string>
    return error("fail")
end

func main()
    fail()
    const handled: result<int, string> = fail()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    let codes = diagnostic_codes(&out);
    let unused_result_count = codes
        .iter()
        .filter(|code| **code == "type.unused_result")
        .count();
    assert_eq!(unused_result_count, 1, "{codes:?}");
}

#[test]
fn check_treats_panic_todo_and_unreachable_as_never() {
    let dir = TestDir::new("panic_todo_unreachable_never");
    dir.write(
        "main.orl",
        r#"namespace app.main

func die(flag: bool) -> int
    if flag
        return 1
    else
        panic("bad")
    end
end

func later() -> int
    todo()
end

func impossible() -> int
    unreachable("impossible")
end

func choose(flag: bool) -> int
    const value: int = if flag then 1 else panic("bad")
    return value
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    let codes = diagnostic_codes(&out);
    assert!(!codes.contains(&"type.missing_return"), "{codes:?}");
    assert!(!codes.contains(&"type.if_branch_mismatch"), "{codes:?}");
}

#[test]
fn check_reports_non_exhaustive_bool_match() {
    let dir = TestDir::new("non_exhaustive_bool_match");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main(flag: bool)
    match flag
    case true:
        return
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"match.non_exhaustive"));
}

#[test]
fn check_reports_non_exhaustive_enum_match() {
    let dir = TestDir::new("non_exhaustive_enum_match");
    dir.write(
        "main.orl",
        r#"namespace app.main

enum Status
    Ready
    Done
end

func main(status: Status)
    match status
    case .Ready:
        return
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"match.non_exhaustive"));
}

#[test]
fn check_reports_payload_enum_variant_matched_as_unit() {
    let dir = TestDir::new("payload_enum_variant_as_unit");
    dir.write(
        "main.orl",
        r#"namespace app.main

enum Status
    Ready
    Done(code: int)
end

func main(status: Status)
    match status
    case Done:
        return
    case Ready:
        return
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"type.pattern_mismatch"));
    assert!(codes.contains(&"match.non_exhaustive"));
}

#[test]
fn check_validates_payload_enum_variant_fields() {
    let dir = TestDir::new("payload_enum_variant_fields");
    dir.write(
        "main.orl",
        r#"namespace app.main

enum Status
    Ready
    Done(code: int)
end

func main(status: Status)
    match status
    case Done(missing):
        return
    case Ready:
        return
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"type.pattern_mismatch"));
    assert!(codes.contains(&"match.non_exhaustive"));
}

#[test]
fn check_accepts_exhaustive_payload_enum_match() {
    let dir = TestDir::new("payload_enum_exhaustive");
    dir.write(
        "main.orl",
        r#"namespace app.main

enum Status
    Ready
    Done(code: int)
end

func main(status: Status)
    match status
    case Done(code):
        const value: int = code
        return
    case Ready:
        return
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_reports_non_bool_function_parameter_contract() {
    let dir = TestDir::new("param_contract_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

func bounded(value: int if it + 1) -> int
    return value
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.expected_bool"));
}

#[test]
fn check_reports_non_bool_struct_field_contract() {
    let dir = TestDir::new("field_contract_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Port
    value: int if it + 1
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.expected_bool"));
}

#[test]
fn check_reports_variadic_argument_type_mismatch() {
    let dir = TestDir::new("variadic_arg_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

func sum(values: int...) -> int
    return 0
end

func main()
    const bad: int = sum("no")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.arg_type_mismatch"));
}

#[test]
fn check_reports_variadic_spread_type_mismatch() {
    let dir = TestDir::new("variadic_spread_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

func sum(values: int...) -> int
    return 0
end

func main()
    const words: list<string> = ["no"]
    const bad: int = sum(..words)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.arg_type_mismatch"));
}

#[test]
fn check_reports_duplicate_parameter_names() {
    let dir = TestDir::new("duplicate_param_names");
    dir.write(
        "main.orl",
        r#"namespace app.main

func add(value: int, value: int) -> int
    return value
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.duplicate_param"));
}

#[test]
fn check_reports_import_after_declaration() {
    let dir = TestDir::new("import_after_declaration");
    dir.write(
        "main.orl",
        r#"namespace app.main

func ready()
end

import ori.io as io
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"parse.import_after_declaration"));
}

#[test]
fn check_reports_missing_namespace() {
    let dir = TestDir::new("missing_namespace");
    dir.write(
        "main.orl",
        r#"func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"parse.namespace_missing"));
}

#[test]
fn check_reports_namespace_not_first() {
    let dir = TestDir::new("namespace_not_first");
    dir.write(
        "main.orl",
        r#"namespace app.main

func ready()
end

namespace app.other
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"parse.namespace_not_first"));
}

#[test]
fn check_reports_type_error_inside_imported_top_level_const() {
    let dir = TestDir::new("imported_const_type_error");
    dir.write(
        "config.orl",
        r#"namespace app.config

const LIMIT: int = "bad"
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.config as config

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.type_mismatch"));
}

#[test]
fn check_uses_imported_top_level_const_type_at_use_site() {
    let dir = TestDir::new("imported_const_use_type");
    dir.write(
        "config.orl",
        r#"namespace app.config

public const LIMIT: int = 21
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.config as config

func main()
    const value: string = config.LIMIT
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.type_mismatch"));
}

#[test]
fn check_reports_missing_local_import() {
    let dir = TestDir::new("missing_import");
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.missing

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.import_not_found"));
}

#[test]
fn check_accepts_implemented_stdlib_import_allowlist() {
    let dir = TestDir::new("implemented_stdlib_imports");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.core as core
import ori.io as io
import ori.fs as fs
import ori.files as files
import ori.string as str
import ori.bytes as bytes_mod
import ori.list as lists
import ori.map as maps
import ori.set as sets
import ori.tree as tree
import ori.hash_table as hash_table
import ori.graph as graph
import ori.math as math
import ori.convert as conv
import ori.iter as iter
import ori.Error as StdError

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "got: {:?}", diagnostic_codes(&out));
}

#[test]
fn check_accepts_core_traits_and_using_core_disposable() {
    let dir = TestDir::new("core_traits_using");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.core as core

struct Resource
    id: int
end

implement core.Disposable for Resource
    mut func dispose(self)
    end
end

implement core.Hashable for Resource
end

func require_hashable<T>(value: T) -> int where T is core.Hashable
    return 1
end

func main()
    using resource: Resource = Resource(id: 1)
    const ok: int = require_hashable(resource)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "got: {:?}", out.diagnostics);
}

#[test]
fn check_accepts_json_stdlib_import() {
    let dir = TestDir::new("json_stdlib_import");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.json as json

func main()
    const parsed: result<json.Value, string> = json.parse("{}")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "got: {:?}", out.diagnostics);
}

#[test]
fn compile_runs_standard_error_type_native() {
    let dir = TestDir::new("compile_standard_error_type_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.Error as StdError
import ori.io as io

func main()
    const err: StdError = StdError(code: "E_TEST", message: "failed", cause: "")
    io.print(err.code)
    io.print(err.message)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "standard_error_type.exe"
    } else {
        "standard_error_type"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "E_TEST\nfailed\n");
}

#[test]
fn build_c_backend_compiles_standard_error_type() {
    let dir = TestDir::new("c_backend_standard_error_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.Error as StdError

func main()
    const err: StdError = StdError(code: "E_C", message: "compiled", cause: "")
    const code: string = err.code
    const message: string = err.message
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.c_source.contains("struct ori_def_"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_string_t code;"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_string_t message;"),
        "{}",
        out.c_source
    );
    // Verifies that the generated struct includes the string cause field.
    assert!(out.c_source.contains("cause;"), "{}", out.c_source);
    compile_c_source(&dir, "c_backend_standard_error_type", &out.c_source);
}

#[test]
fn compile_runs_test_assert_stdlib_native() {
    let dir = TestDir::new("compile_test_assert_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.test as test

func main()
    test.assert(1 + 1 == 2, "math still works")
    test.assert_eq(21 * 2, 42)
    test.assert_ne(21, 42)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "test_assert_stdlib.exe"
    } else {
        "test_assert_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
}

#[test]
fn compile_runs_generic_test_assert_stdlib_native() {
    let dir = TestDir::new("compile_generic_test_assert_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.test as test

func main()
    test.assert_eq("ori", "ori")
    test.assert_ne("ori", "lang")
    test.assert_eq(true, true)
    test.assert_ne(true, false)
    test.assert_eq(1.5, 1.5)
    test.assert_ne(1.5, 2.5)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "generic_test_assert_stdlib.exe"
    } else {
        "generic_test_assert_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
}

#[test]
fn test_runner_reports_test_fail_stdlib_failure() {
    let dir = TestDir::new("test_runner_stdlib_fail");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.test as test

@test
func test_failure()
    test.fail("intentional")
end
"#,
    );

    let out = run_test(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert_eq!(out.results.len(), 1);
    assert!(!out.results[0].passed, "{:#?}", out.results[0].stderr);
    assert!(
        out.results[0].stderr.contains("intentional"),
        "{:#?}",
        out.results[0].stderr
    );
}

#[test]
fn build_c_backend_compiles_test_assert_stdlib() {
    let dir = TestDir::new("c_backend_test_assert_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.test as test

func main()
    test.assert(true, "ok")
    test.assert_eq(2 + 2, 4)
    test.assert_ne(2, 4)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("ori_test_assert"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_test_assert_eq"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_test_assert_ne"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_test_assert_stdlib", &out.c_source);
}

#[test]
fn build_c_backend_compiles_generic_test_assert_stdlib() {
    let dir = TestDir::new("c_backend_generic_test_assert_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.test as test

func main()
    test.assert_eq("ori", "ori")
    test.assert_ne("ori", "lang")
    test.assert_eq(true, true)
    test.assert_ne(true, false)
    test.assert_eq(1.5, 1.5)
    test.assert_ne(1.5, 2.5)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("ori_test_assert_eq_string"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_test_assert_ne_float"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_generic_test_assert_stdlib", &out.c_source);
}

#[test]
fn compile_runs_iter_stdlib_native() {
    let dir = TestDir::new("compile_iter_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.iter as iter
import ori.list as lists
import ori.map as maps

func main()
    const values: list<int> = [1, 2, 3, 4]
    const doubled: list<int> = iter.map(values, do(x: int) => x * 2)
    const filtered: list<int> = iter.filter(doubled, do(x: int) => x > 4)
    const has_large: bool = iter.any(values, do(x: int) => x > 3)
    const all_positive: bool = iter.all(values, do(x: int) => x > 0)
    const even_count: int = iter.count_where(values, do(x: int) => x % 2 == 0)
    const first_two: list<int> = iter.take(values, 2)
    const after_two: list<int> = iter.skip(values, 2)
    const reversed: list<int> = iter.reverse(values)
    const sum: int = iter.reduce(values, 0, do(acc: int, x: int) => acc + x)
    const first_even: optional<int> = iter.find(values, do(x: int) => x % 2 == 0)
    const sorted: list<int> = iter.sort([4, 1, 3, 2])
    const sorted_desc: list<int> = iter.sort_by([4, 1, 3, 2], do(a: int, b: int) => b - a)
    const unique: list<int> = iter.unique([1, 2, 1, 3, 2])
    const expanded: list<int> = iter.flat_map([1, 2, 3], do(x: int) => [x, x * 10])
    const zipped: list<tuple<int, int>> = iter.zip([1, 2, 3], [10, 20])
    const first_pair: tuple<int, int> = lists.get(zipped, 0)
    const second_pair: tuple<int, int> = lists.get(zipped, 1)
    const parts: tuple<list<int>, list<int>> = iter.partition(values, do(x: int) => x % 2 == 0)
    const evens: list<int> = parts.0
    const odds: list<int> = parts.1
    const grouped: map<int, list<int>> = iter.group_by(values, do(x: int) => x % 2)
    const grouped_even: list<int> = maps.get(grouped, 0)
    const grouped_odd: list<int> = maps.get(grouped, 1)
    const nested: list<list<int>> = [[1, 2], [3], [], [4, 5]]
    const flat: list<int> = iter.flatten(nested)
    io.print(string(lists.len(filtered)))
    io.print(string(lists.get(filtered, 0)))
    io.print(string(lists.get(filtered, 1)))
    io.print(string(has_large))
    io.print(string(all_positive))
    io.print(string(even_count))
    io.print(string(lists.get(first_two, 1)))
    io.print(string(lists.get(after_two, 0)))
    io.print(string(lists.get(reversed, 0)))
    io.print(string(sum))
    if some(found) = first_even
    io.print(string(found))
    end
    io.print(string(lists.get(sorted, 0)))
    io.print(string(lists.get(sorted, 3)))
    io.print(string(lists.get(sorted_desc, 0)))
    io.print(string(lists.get(sorted_desc, 3)))
    io.print(string(lists.len(unique)))
    io.print(string(lists.get(unique, 2)))
    io.print(string(lists.len(flat)))
    io.print(string(lists.get(flat, 0)))
    io.print(string(lists.get(flat, 4)))
    io.print(string(lists.len(expanded)))
    io.print(string(lists.get(expanded, 0)))
    io.print(string(lists.get(expanded, 5)))
    io.print(string(lists.len(zipped)))
    io.print(string(first_pair.0))
    io.print(string(first_pair.1))
    io.print(string(second_pair.0))
    io.print(string(second_pair.1))
    io.print(string(lists.len(evens)))
    io.print(string(lists.get(evens, 0)))
    io.print(string(lists.get(evens, 1)))
    io.print(string(lists.len(odds)))
    io.print(string(lists.get(odds, 0)))
    io.print(string(lists.get(odds, 1)))
    io.print(string(lists.len(grouped_even)))
    io.print(string(lists.get(grouped_even, 0)))
    io.print(string(lists.get(grouped_even, 1)))
    io.print(string(lists.len(grouped_odd)))
    io.print(string(lists.get(grouped_odd, 0)))
    io.print(string(lists.get(grouped_odd, 1)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "iter_stdlib.exe"
    } else {
        "iter_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n").trim_end(),
        "2\n6\n8\ntrue\ntrue\n2\n2\n3\n4\n10\n2\n1\n4\n4\n1\n3\n3\n5\n1\n5\n6\n1\n30\n2\n1\n10\n2\n20\n2\n2\n4\n2\n1\n3\n2\n2\n4\n2\n1\n3"
    );
}

#[test]
fn compile_runs_generic_iter_stdlib_native() {
    let dir = TestDir::new("compile_generic_iter_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.iter as iter
import ori.list as lists
import ori.map as maps
import ori.string as strings

func main()
    const words: list<string> = ["pear", "fig", "apple", "fig"]
    const lengths: list<int> = iter.map(words, do(word: string) => strings.len(word))
    const short: list<string> = iter.filter(words, do(word: string) => strings.len(word) < 5)
    const has_apple: bool = iter.any(words, do(word: string) => word == "apple")
    const all_named: bool = iter.all(words, do(word: string) => strings.len(word) > 0)
    const fig_count: int = iter.count_where(words, do(word: string) => word == "fig")
    const first_two: list<string> = iter.take(words, 2)
    const after_two: list<string> = iter.skip(words, 2)
    const reversed: list<string> = iter.reverse(words)
    const total_len: int = iter.reduce(words, 0, do(acc: int, word: string) => acc + strings.len(word))
    const found: optional<string> = iter.find(words, do(word: string) => word == "apple")
    const expanded: list<string> = iter.flat_map(["x", "y"], do(word: string) => [word, word])
    const sorted: list<string> = iter.sort(["pear", "apple", "fig"])
    const sorted_by_len: list<string> = iter.sort_by(["pear", "apple", "fig"], do(a: string, b: string) => strings.len(a) - strings.len(b))
    const unique: list<string> = iter.unique(["fig", "fig", "pear"])
    const zipped: list<tuple<string, int>> = iter.zip(["a", "b"], [1, 2])
    const first_pair: tuple<string, int> = lists.get(zipped, 0)
    const parts: tuple<list<string>, list<string>> = iter.partition(words, do(word: string) => word == "fig")
    const figs: list<string> = parts.0
    const other: list<string> = parts.1
    const grouped: map<string, list<string>> = iter.group_by(words, do(word: string) => word)
    const grouped_figs: list<string> = maps.get(grouped, "fig")
    const nested: list<list<string>> = [["a"], ["b", "c"]]
    const flat: list<string> = iter.flatten(nested)
    io.print(string(lists.get(lengths, 0)))
    io.print(string(lists.len(short)))
    io.print(string(has_apple))
    io.print(string(all_named))
    io.print(string(fig_count))
    io.print(lists.get(first_two, 1))
    io.print(lists.get(after_two, 0))
    io.print(lists.get(reversed, 0))
    io.print(string(total_len))
    if some(value) = found
        io.print(value)
    end
    io.print(lists.get(expanded, 3))
    io.print(lists.get(sorted, 0))
    io.print(lists.get(sorted_by_len, 0))
    io.print(string(lists.len(unique)))
    io.print(first_pair.0)
    io.print(string(first_pair.1))
    io.print(string(lists.len(figs)))
    io.print(string(lists.len(other)))
    io.print(string(lists.len(grouped_figs)))
    io.print(lists.get(flat, 2))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "generic_iter_stdlib.exe"
    } else {
        "generic_iter_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n").trim_end(),
        "4\n3\ntrue\ntrue\n2\nfig\napple\nfig\n15\napple\ny\napple\nfig\n2\na\n1\n2\n2\n2\nc"
    );
}

#[test]
fn build_c_backend_compiles_iter_stdlib() {
    let dir = TestDir::new("c_backend_iter_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.iter as iter

func main()
    const values: list<int> = [1, 2, 3, 4]
    const doubled: list<int> = iter.map(values, do(x: int) => x * 2)
    const filtered: list<int> = iter.filter(doubled, do(x: int) => x > 4)
    const has_large: bool = iter.any(values, do(x: int) => x > 3)
    const all_positive: bool = iter.all(values, do(x: int) => x > 0)
    const even_count: int = iter.count_where(values, do(x: int) => x % 2 == 0)
    const first_two: list<int> = iter.take(values, 2)
    const after_two: list<int> = iter.skip(values, 2)
    const reversed: list<int> = iter.reverse(values)
    const sum: int = iter.reduce(values, 0, do(acc: int, x: int) => acc + x)
    const first_even: optional<int> = iter.find(values, do(x: int) => x % 2 == 0)
    const sorted: list<int> = iter.sort([4, 1, 3, 2])
    const sorted_desc: list<int> = iter.sort_by([4, 1, 3, 2], do(a: int, b: int) => b - a)
    const unique: list<int> = iter.unique([1, 2, 1, 3, 2])
    const expanded: list<int> = iter.flat_map([1, 2, 3], do(x: int) => [x, x * 10])
    const zipped: list<tuple<int, int>> = iter.zip([1, 2, 3], [10, 20])
    const first_pair: tuple<int, int> = zipped[0]
    const second_pair: tuple<int, int> = zipped[1]
    const first_sum: int = first_pair.0 + first_pair.1
    const second_sum: int = second_pair.0 + second_pair.1
    const parts: tuple<list<int>, list<int>> = iter.partition(values, do(x: int) => x % 2 == 0)
    const evens: list<int> = parts.0
    const odds: list<int> = parts.1
    const partition_first_even: int = evens[0]
    const partition_first_odd: int = odds[0]
    const grouped: map<int, list<int>> = iter.group_by(values, do(x: int) => x % 2)
    const nested: list<list<int>> = [[1, 2], [3], [], [4, 5]]
    const flat: list<int> = iter.flatten(nested)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("ori_list_map"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_list_filter"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_iter_any"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_iter_all"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_iter_count_where"),
        "{}",
        out.c_source
    );
    assert!(out.c_source.contains("ori_iter_take"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_iter_skip"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_iter_reverse"),
        "{}",
        out.c_source
    );
    assert!(out.c_source.contains("ori_iter_reduce"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_iter_find"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_iter_sort"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_iter_sort_by"),
        "{}",
        out.c_source
    );
    assert!(out.c_source.contains("ori_iter_unique"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_iter_flat_map"),
        "{}",
        out.c_source
    );
    assert!(out.c_source.contains("ori_iter_zip"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_iter_partition"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_iter_group_by"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_iter_flatten"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_iter_stdlib", &out.c_source);
}

#[test]
fn compile_runs_format_stdlib_native() {
    let dir = TestDir::new("compile_format_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.format as fmt
import ori.io as io

func main()
    io.print(fmt.number(12.345, 2))
    io.print(fmt.percent(0.125, 1))
    io.print(fmt.hex(255))
    io.print(fmt.binary(5))
    io.print(fmt.bytes_size(1536, "binary"))
    io.print(fmt.date(0, "iso"))
    io.print(fmt.datetime(0, "iso", ""))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "format_stdlib.exe"
    } else {
        "format_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n").trim_end(),
        "12.35\n12.5%\nff\n101\n1.5 KiB\n1970-01-01\n1970-01-01T00:00:00Z"
    );
}

#[test]
fn build_c_backend_compiles_format_stdlib() {
    let dir = TestDir::new("c_backend_format_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.format as fmt

func main()
    const number: string = fmt.number(12.345, 2)
    const percent: string = fmt.percent(0.125, 1)
    const hexed: string = fmt.hex(255)
    const binary: string = fmt.binary(5)
    const size: string = fmt.bytes_size(1536, "binary")
    const date: string = fmt.date(0, "iso")
    const datetime: string = fmt.datetime(0, "iso", "")
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(
        out.c_source.contains("ori_format_number"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_format_datetime"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_format_stdlib", &out.c_source);
}

#[test]
fn compile_runs_os_stdlib_native() {
    let dir = TestDir::new("compile_os_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lst
import ori.os as os

func main()
    const env_value: optional<string> = os.env("ORI_TEST_OS_VALUE")
    if some(value) = env_value
        io.print(value)
    else
        io.print("missing")
    end

    const args: list<string> = os.args()
    io.print(string(lst.len(args)))
    io.print(os.platform())
    io.print(os.arch())
    const pid: int = os.pid()
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "os_stdlib.exe"
    } else {
        "os_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe)
        .arg("alpha")
        .arg("beta")
        .env("ORI_TEST_OS_VALUE", "works")
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = stdout
        .replace("\r\n", "\n")
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(lines[0], "works");
    assert_eq!(lines[1], "3");
    assert!(["windows", "linux", "macos", "unknown"].contains(&lines[2].as_str()));
    assert!(!lines[3].is_empty());
}

#[test]
fn build_c_backend_compiles_os_stdlib() {
    let dir = TestDir::new("c_backend_os_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.os as os

func stop()
    os.exit(0)
end

func main()
    const args: list<string> = os.args()
    const env_value: optional<string> = os.env("PATH")
    const pid: int = os.pid()
    const platform: string = os.platform()
    const arch: string = os.arch()
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.c_source.contains("ori_os_args"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_os_env"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_opt_str_t"), "{}", out.c_source);
    assert!(
        out.c_source.contains("int main(int argc, char** argv)"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_os_stdlib", &out.c_source);
}

#[test]
fn compile_runs_random_stdlib_native() {
    let dir = TestDir::new("compile_random_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.random as rng

func main()
    const number: int = rng.int(1, 3)
    const ratio: float = rng.float(0.0, 1.0)
    const flag: bool = rng.bool()
    const items: list<int> = [10, 20, 30]
    const picked: optional<int> = rng.choice(items)
    const shuffled: list<int> = rng.shuffle(items)
    io.print(string(number))
    io.print(string(ratio >= 0.0 and ratio <= 1.0))
    io.print(string(flag or not flag))
    if some(value) = picked
        io.print(string(value == 10 or value == 20 or value == 30))
    end
    io.print(string(lists.len(shuffled)))
    io.print(string(lists.contains(shuffled, 10) and lists.contains(shuffled, 20) and lists.contains(shuffled, 30)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "random_stdlib.exe"
    } else {
        "random_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = stdout
        .replace("\r\n", "\n")
        .lines()
        .map(str::to_owned)
        .collect();
    let number = lines[0].parse::<i64>().unwrap();
    assert!((1..=3).contains(&number));
    assert_eq!(lines[1], "true");
    assert_eq!(lines[2], "true");
    assert_eq!(lines[3], "true");
    assert_eq!(lines[4], "3");
    assert_eq!(lines[5], "true");
}

#[test]
fn compile_runs_generic_random_choice_and_shuffle_native() {
    let dir = TestDir::new("compile_generic_random_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.random as rng

func main()
    const words: list<string> = ["alpha", "beta", "gamma"]
    const picked: optional<string> = rng.choice(words)
    if some(value) = picked
        io.print(string(value == "alpha" or value == "beta" or value == "gamma"))
    end
    const shuffled: list<string> = rng.shuffle(words)
    io.print(string(lists.len(shuffled)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "generic_random_stdlib.exe"
    } else {
        "generic_random_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "true\n3\n");
}

#[test]
fn compile_runs_json_stdlib_native() {
    let dir = TestDir::new("compile_json_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.json as json

func main()
    const parsed: result<json.Value, string> = json.parse("{\"name\":\"ori\",\"ok\":true}")
    match parsed
    case success(value):
        io.print(json.stringify(value))
        io.print(json.stringify_pretty(value))
    case error(message):
        io.print(message)
    end

    const invalid: result<json.Value, string> = json.parse("{")
    match invalid
    case success(value):
        io.print(json.stringify(value))
    case error(message):
        io.print("invalid")
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "json_stdlib.exe"
    } else {
        "json_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "{\"name\":\"ori\",\"ok\":true}\n{\n  \"name\": \"ori\",\n  \"ok\": true\n}\ninvalid\n"
    );
}

#[test]
fn build_c_backend_compiles_random_stdlib() {
    let dir = TestDir::new("c_backend_random_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.random as rng

func main()
    const number: int = rng.int(1, 3)
    const ratio: float = rng.float(0.0, 1.0)
    const flag: bool = rng.bool()
    const items: list<int> = [10, 20, 30]
    const picked: optional<int> = rng.choice(items)
    const shuffled: list<int> = rng.shuffle(items)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.c_source.contains("ori_random_int"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_random_float"),
        "{}",
        out.c_source
    );
    assert!(out.c_source.contains("ori_random_bool"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_random_choice"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("ori_random_shuffle"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_random_stdlib", &out.c_source);
}

#[test]
fn compile_runs_time_stdlib_native() {
    let dir = TestDir::new("compile_time_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.time as time

func main()
    time.sleep(0)
    io.print(string(time.duration_ms(10, 42)))
    io.print(string(time.now() > 0))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "time_stdlib.exe"
    } else {
        "time_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n").trim_end(), "32\ntrue");
}

#[test]
fn build_c_backend_compiles_time_stdlib() {
    let dir = TestDir::new("c_backend_time_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.time as time

func main()
    time.sleep(0)
    const now: int = time.now()
    const delta: int = time.duration_ms(10, 42)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.c_source.contains("ori_time_now"), "{}", out.c_source);
    assert!(out.c_source.contains("ori_time_sleep"), "{}", out.c_source);
    assert!(
        out.c_source.contains("ori_time_duration_ms"),
        "{}",
        out.c_source
    );
    compile_c_source(&dir, "c_backend_time_stdlib", &out.c_source);
}

#[test]
fn compile_runs_mem_stdlib_intrinsics_native() {
    let dir = TestDir::new("compile_mem_stdlib_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.mem as mem

func main()
    const value: int = 41
    const flag: bool = true
    io.print(string(mem.size_of(value)))
    io.print(":")
    io.print(string(mem.align_of(value)))
    io.print(":")
    io.print(string(mem.size_of(flag)))
    io.print(":")
    io.print(string(mem.align_of(flag)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "mem_stdlib.exe"
    } else {
        "mem_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n").trim_end(),
        "8\n:\n8\n:\n1\n:\n1"
    );
}

#[test]
fn build_c_backend_compiles_mem_stdlib_intrinsics() {
    let dir = TestDir::new("c_backend_mem_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.mem as mem

func main()
    const value: int = 41
    const size: int = mem.size_of(value)
    const align: int = mem.align_of(value)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.c_source.contains("ori_mem_"), "{}", out.c_source);
    compile_c_source(&dir, "c_backend_mem_stdlib", &out.c_source);
}

#[test]
fn check_reports_unknown_stdlib_import() {
    let dir = TestDir::new("unknown_stdlib_import");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.nope as nope

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"bind.stdlib_module_unknown"),
        "got: {:?}",
        codes
    );
}

#[test]
fn check_reports_ambiguous_local_import_path() {
    let dir = TestDir::new("ambiguous_import");
    std::fs::create_dir_all(dir.path("app")).unwrap();
    dir.write(
        "util.orl",
        r#"namespace app.util

public func answer() -> int
    return 1
end
"#,
    );
    dir.write(
        "app/util.orl",
        r#"namespace app.util

public func answer() -> int
    return 2
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.import_ambiguous"));
}

#[test]
fn check_reports_import_namespace_mismatch() {
    let dir = TestDir::new("namespace_mismatch");
    dir.write(
        "util.orl",
        r#"namespace app.other

func answer() -> int
    return 1
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"project.namespace_file_mismatch"));
}

#[test]
fn check_reports_local_import_cycle() {
    let dir = TestDir::new("import_cycle");
    dir.write(
        "a.orl",
        r#"namespace app.a

import app.b

func main()
end
"#,
    );
    dir.write(
        "b.orl",
        r#"namespace app.b

import app.a

func value() -> int
    return 1
end
"#,
    );

    let out = run_check(&dir.path("a.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"project.circular_import"));
}

#[test]
fn check_reports_duplicate_import_alias() {
    let dir = TestDir::new("dup_alias");
    dir.write(
        "a.orl",
        r#"namespace app.a

func value() -> int
    return 1
end
"#,
    );
    dir.write(
        "b.orl",
        r#"namespace app.b

func value() -> int
    return 2
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.a as m
import app.b as m

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.duplicate_alias"));
}

#[test]
fn check_reports_alias_shadowing_local_definition() {
    let dir = TestDir::new("alias_shadows_local");
    dir.write(
        "util.orl",
        r#"namespace app.util

func helper() -> int
    return 3
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as helper

func helper()
end

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.alias_shadows_local"));
}

#[test]
fn check_allows_distinct_aliases_for_same_short_name() {
    let dir = TestDir::new("distinct_aliases");
    dir.write(
        "a.orl",
        r#"namespace app.a

public func value() -> int
    return 1
end
"#,
    );
    dir.write(
        "b.orl",
        r#"namespace app.b

public func value() -> int
    return 2
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.a as a
import app.b as b

func main()
    const x: int = a.value()
    const y: int = b.value()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_reports_private_item_import() {
    let dir = TestDir::new("private_import");
    dir.write(
        "util.orl",
        r#"namespace app.util

func secret() -> int
    return 42
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as util

func main()
    const value: int = util.secret()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"name.private"));
}

#[test]
fn check_warns_unused_import() {
    let dir = TestDir::new("unused_import");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func helper() -> int
    return 1
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as util

func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.unused_import"));
}

#[test]
fn compile_runs_entry_namespace_main_with_imported_call() {
    let dir = TestDir::new("compile_import");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func answer() -> int
    return 13
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as util
import ori.io as io

func main()
    io.print(string(util.answer()))
end
"#,
    );

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
    dir.write(
        "c.orl",
        r#"namespace app.c

public func value() -> int
    return 8
end
"#,
    );
    dir.write(
        "b.orl",
        r#"namespace app.b

import app.c as c

public func value() -> int
    return c.value()
end
"#,
    );
    dir.write(
        "a.orl",
        r#"namespace app.a

import app.b as b

public func value() -> int
    return b.value()
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.a as a
import ori.io as io

func main()
    io.print(string(a.value()))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "transitive.exe"
    } else {
        "transitive"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "8");
}

// if some / while some / check

#[test]
fn check_if_some_type_checks() {
    let dir = TestDir::new("ifsome_check");
    dir.write(
        "main.orl",
        r#"namespace app.main

func get_name(flag: bool) -> optional<int>
    if flag
        return some(42)
    end
    return none
end

func main()
    const maybe: optional<int> = get_name(true)
    if some(n) = maybe
        const doubled: int = n + n
    end
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_if_some_wrong_type_reports_error() {
    let dir = TestDir::new("ifsome_wrong_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const value: int = 5
    if some(n) = value
        const x: int = n
    end
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(
        out.has_errors,
        "expected type error for if some on non-optional"
    );
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"type.ifsome_not_optional"),
        "got: {:?}",
        codes
    );
}

#[test]
fn check_while_some_type_checks() {
    let dir = TestDir::new("whilesome_check");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    var source: optional<int> = some(3)
    while some(n) = source
        source = none
    end
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_while_some_wrong_type_reports_error() {
    let dir = TestDir::new("whilesome_wrong_type");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    var count: int = 0
    while some(n) = count
        count = 0
    end
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(
        out.has_errors,
        "expected type error for while some on non-optional"
    );
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"type.whilesome_not_optional"),
        "got: {:?}",
        codes
    );
}

#[test]
fn check_check_stmt_type_checks() {
    let dir = TestDir::new("check_stmt");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const x: int = 5
    check x > 0
    check x > 0, "x must be positive"
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_check_stmt_non_bool_reports_error() {
    let dir = TestDir::new("check_stmt_non_bool");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const x: int = 5
    check x
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected type error for check on non-bool");
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"type.expected_bool"), "got: {:?}", codes);
}

#[test]
fn check_using_stmt_type_checks() {
    let dir = TestDir::new("using_stmt");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    using resource: int = 42
    const doubled: int = resource + resource
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected using to require Disposable");
    assert!(diagnostic_codes(&out).contains(&"using.not_disposable"));
}

#[test]
fn check_reports_const_reassignment_and_same_scope_shadowing() {
    let dir = TestDir::new("const_reassignment_shadowing");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const value: int = 1
    value = 2
    const value: int = 3
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"bind.const_reassignment"), "{codes:?}");
    assert!(codes.contains(&"bind.shadowing"), "{codes:?}");
}

#[test]
fn check_reports_missing_return_on_non_void_function() {
    let dir = TestDir::new("missing_return_non_void");
    dir.write(
        "main.orl",
        r#"namespace app.main

func maybe(flag: bool) -> int
    if flag
        return 1
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.missing_return"));
}

#[test]
fn check_reports_loop_control_outside_loop() {
    let dir = TestDir::new("loop_control_outside_loop");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    if true
        break
    end
    match true
    case true:
        continue
    case false:
        return
    end
    continue
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert_eq!(
        codes
            .iter()
            .filter(|code| **code == "control.loop_required")
            .count(),
        3
    );
}

#[test]
fn check_accepts_loop_control_inside_loops() {
    let dir = TestDir::new("loop_control_inside_loops");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    var i: int = 0
    while i < 3
        i = i + 1
        if i == 1
            continue
        end
        break
    end

    repeat 2 times
        continue
    end

    loop
        break
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_reports_loop_control_inside_closure() {
    let dir = TestDir::new("loop_control_inside_closure");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    loop
        const stop: func() -> void = do() -> void
            break
        end
        break
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"control.loop_required"));
}

#[test]
fn check_reports_numeric_literal_overflow() {
    let dir = TestDir::new("numeric_literal_overflow");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const too_big_default: int = 9223372036854775808
    const too_big_u8: u8 = 256u8
    const too_big_i8: int8 = 128i8
    const too_big_f32: float32 = 1.0e999f32
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert_eq!(
        codes
            .iter()
            .filter(|code| **code == "type.numeric_literal_out_of_range")
            .count(),
        4
    );
}

#[test]
fn check_reports_numeric_literal_invalid_suffix() {
    let dir = TestDir::new("numeric_literal_invalid_suffix");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const bad_float: float = 3.5f128
    const bad_int: int = 42u128
    const bad_hex: int = 0xFFg
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    let codes = diagnostic_codes(&out);
    assert_eq!(
        codes
            .iter()
            .filter(|code| **code == "type.numeric_literal_invalid")
            .count(),
        3
    );
}

#[test]
fn check_accepts_numeric_literal_suffixes() {
    let dir = TestDir::new("numeric_literal_suffixes");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const a: u8 = 42u8
    const b: int16 = 42i16
    const c: int32 = 0x2Ai32
    const d: u16 = 0b101010u16
    const e: int64 = 0o52i64
    const f: float32 = 3.5f32
    const g: float = 3.5f64
    const h: u64 = 18446744073709551615u64
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn build_preserves_numeric_literal_suffix_values() {
    let dir = TestDir::new("numeric_literal_suffix_values");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const a: float = 3.5f64
    const b: int = 42i64
    const c: u8 = 0x2Au8
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("double a = 3.5"), "{}", out.c_source);
    assert!(
        out.c_source.contains("int64_t b = INT64_C(42)"),
        "{}",
        out.c_source
    );
    assert!(
        out.c_source.contains("uint8_t c = INT64_C(42)"),
        "{}",
        out.c_source
    );

    let exe = dir.path(if cfg!(windows) {
        "numeric_literal_suffix_values.exe"
    } else {
        "numeric_literal_suffix_values"
    });
    let compile = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!compile.has_errors, "{:?}", compile.diagnostics);
}

#[test]
fn check_reports_using_binding_reassignment() {
    let dir = TestDir::new("using_binding_reassignment");
    dir.write(
        "main.orl",
        r#"namespace app.main

trait Disposable
    mut func dispose(self)
end

struct Resource
    id: int
end

implement Disposable for Resource
    mut func dispose(self)
    end
end

func main()
    using resource: Resource = Resource(id: 1)
    resource = Resource(id: 2)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"mut.using_binding_mutated"));
}

#[test]
fn check_reports_mut_method_call_on_const_binding() {
    let dir = TestDir::new("mut_method_on_const");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Counter
    value: int

    mut func increment(self)
        self.value = self.value + 1
    end
end

func main()
    const locked: Counter = Counter(value: 0)
    locked.increment()

    var open: Counter = Counter(value: 0)
    open.increment()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"mut.const_method_call"));
}

#[test]
fn build_if_some_generates_c() {
    let dir = TestDir::new("ifsome_build");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func get_value(flag: bool) -> optional<int>
    if flag
        return some(7)
    end
    return none
end

func main()
    const maybe: optional<int> = get_value(true)
    if some(n) = maybe
        io.print(string(n))
    end
end
"#,
    );
    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.c_source.contains("has_value"),
        "expected has_value in generated C: {}",
        out.c_source
    );
}

#[test]
fn build_c_backend_compiles_runtime_abi_values() {
    let dir = TestDir::new("c_backend_runtime_abi");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.string as str

func maybe(flag: bool) -> optional<int>
    if flag
        return some(7)
    end
    return none
end

func parse(flag: bool) -> result<int, string>
    if flag
        return success(11)
    end
    return error("no value")
end

func main()
    const bool_text: string = string(true)
    const float_text: string = string(2.5)
    const numbers: list<int> = [1, 2, 3]
    const first: int = numbers[0]
    const parsed_int: result<int, string> = str.parse_int("12")
    const parsed_float: result<float, string> = str.parse_float("1.5")
    const maybe_value: optional<int> = maybe(true)
    if some(value) = maybe_value
        const copied: int = value
    end

    match parse(false)
    case success(value):
        const ok: int = value
    case error(message):
        const err: string = message
    end
end
"#,
    );
    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out
        .c_source
        .contains("typedef struct { bool has_value; int64_t value; } ori_opt_i64_t;"));
    assert!(out.c_source.contains("typedef struct ori_result_i64_str_t"));
    assert!(out.c_source.contains("typedef struct ori_result_f64_str_t"));
    assert!(out.c_source.contains("ori_list_at"));
    assert!(out.c_source.contains("ori_bool_to_string"));
    assert!(out.c_source.contains("ori_float_to_string"));
    assert!(out.c_source.contains("strtoll"));
    assert!(out.c_source.contains("strtod"));
    compile_c_source(&dir, "runtime_abi", &out.c_source);
}

#[test]
fn check_project_manifest_directory_uses_declared_entry() {
    let dir = TestDir::new("project_manifest_entry");
    dir.write(
        "ori.proj",
        r#"name = "demo"
version = "0.1.0"
entry = "src/main.orl"
"#,
    );
    dir.write(
        "src/app/util/mod.orl",
        r#"namespace app.util

public func answer() -> int
    return 42
end
"#,
    );
    dir.write(
        "src/main.orl",
        r#"namespace app.main

import app.util as util

func main()
    const value: int = util.answer()
end
"#,
    );

    let out = run_check(&dir.path).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_project_manifest_file_uses_declared_entry() {
    let dir = TestDir::new("project_manifest_file_entry");
    dir.write(
        "ori.proj",
        r#"name = "demo"
version = "0.1.0"
entry = "src/main.orl"
"#,
    );
    dir.write(
        "src/app/model/index.orl",
        r#"namespace app.model

public struct User
    id: int
end
"#,
    );
    dir.write(
        "src/main.orl",
        r#"namespace app.main

import app.model as model

func id(user: model.User) -> int
    return user.id
end

func main()
end
"#,
    );

    let out = run_check(&dir.path("ori.proj")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_project_manifest_missing_entry_reports_error() {
    let dir = TestDir::new("project_manifest_missing_entry");
    dir.write(
        "ori.proj",
        r#"name = "demo"
version = "0.1.0"
"#,
    );

    let err = match run_check(&dir.path) {
        Ok(_) => panic!("expected missing entry error"),
        Err(err) => err,
    };
    assert!(err.contains("missing `entry`"), "{err}");
}

#[test]
fn compile_project_tree_with_imported_structs_and_enums() {
    let dir = TestDir::new("project_tree_struct_enum_run");
    dir.write(
        "ori.proj",
        r#"name = "demo"
version = "0.1.0"
entry = "src/main.orl"
"#,
    );
    dir.write(
        "src/app/model/mod.orl",
        r#"namespace app.model

public struct User
    id: int
    name: string
end

public enum Status
    Ready
    Done(code: int)
end

public func stable_code(status: Status) -> int
    return 8
end
"#,
    );
    dir.write(
        "src/main.orl",
        r#"namespace app.main

import app.model as model
import ori.io as io

func main()
    const user: model.User = model.User(id: 34, name: "Ada")
    const status: model.Status = model.Status.Done(code: 8)
    io.print(string(user.id + model.stable_code(status)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "project_tree.exe"
    } else {
        "project_tree"
    });
    let out = run_compile(&dir.path("ori.proj"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "42");
}

#[test]
fn compile_runs_native_closure_capture_and_higher_order_call() {
    let dir = TestDir::new("native_closure_capture");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func apply_twice(value: int, f: func(int) -> int) -> int
    return f(f(value))
end

func double(n: int) -> int
    return n * 2
end

func main()
    const offset: int = 3
    const add_offset: func(int) -> int = do(x: int) => x + offset
    io.print(string(add_offset(4)))
    io.print(string(apply_twice(5, do(x: int) => x * 2)))
    io.print(string(apply_twice(2, double)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "native_closure.exe"
    } else {
        "native_closure"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.replace("\r\n", "\n");
    assert_eq!(stdout, "7\n20\n8\n");
}

#[test]
fn compile_runs_native_block_closure_with_arc_hooks() {
    let dir = TestDir::new("native_block_closure_arc");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const prefix: string = "value"
    const format: func(int) -> string = do(x: int) -> string
        const next: int = x + 1
        return prefix
    end
    io.print(format(9))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "native_block_closure_arc.exe"
    } else {
        "native_block_closure_arc"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.replace("\r\n", "\n");
    assert_eq!(stdout, "value\n");
}

#[test]
fn build_c_backend_compiles_block_closure_with_arc_edges() {
    let dir = TestDir::new("c_backend_block_closure_arc");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const prefix: string = "value"
    const format: func(int) -> string = do(x: int) -> string
        const next: int = x + 1
        return prefix
    end
    const rendered: string = format(9)
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.c_source.contains("ori_arc_register_edge"));
    assert!(out.c_source.contains("ori_arc_collect_cycles();"));

    compile_c_source(&dir, "c_backend_block_closure_arc", &out.c_source);
}

#[test]
fn check_type_alias_expands_in_hir_lowering() {
    // A type alias should expand transparently so that the aliased type's
    // codegen properties (e.g. int arithmetic, struct field access) work.
    let dir = TestDir::new("type_alias_expand");
    dir.write(
        "main.orl",
        r#"namespace app.main

alias Score = int
alias Name = string

struct Player
    name: Name
    score: Score
end

func total(a: Score, b: Score) -> Score
    return a + b
end

func main()
    const p: Player = Player(name: "Alice", score: 10)
    const t: Score = total(p.score, 5)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn compile_type_alias_works_end_to_end_native() {
    let dir = TestDir::new("type_alias_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

alias Count = int

func increment(n: Count) -> Count
    return n + 1
end

func main()
    const value: Count = increment(41)
    io.print(string(value))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "type_alias.exe"
    } else {
        "type_alias"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "42\n");
}

#[test]
fn compile_runs_map_set_literals_native() {
    let dir = TestDir::new("map_set_literals");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.map as maps
import ori.set as sets

func main()
    const my_map: map<int, int> = { 10: 100, 20: 200 }
    const my_set: set<int> = set { 10, 20, 30 }
    io.print(string(maps.get(my_map, 20)))
    io.print(if sets.contains(my_set, 30) then "1" else "0")
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "map_set_literals.exe"
    } else {
        "map_set_literals"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "200\n1\n");
}

#[test]
fn compile_runs_index_slicing_native() {
    let dir = TestDir::new("index_slicing");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const text: string = "hello world"
    const part: string = text[1..5]
    io.print(part)

    const arr: list<int> = [10, 20, 30, 40, 50]
    const sub: list<int> = arr[2..4]
    io.print(string(sub[0]))
    io.print(string(sub[1]))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "index_slicing.exe"
    } else {
        "index_slicing"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "ello\n30\n40\n");
}

#[test]
fn compile_runs_pipe_operator_native() {
    let dir = TestDir::new("pipe_operator");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Point
    x: int
    y: int
    z: int
end

func double(p: Point) -> Point
    return Point(x: p.x * 2, y: p.y * 2, z: p.z * 2)
end

func extract_x(p: Point) -> int
    return p.x
end

func main()
    const base: Point = Point(x: 1, y: 2, z: 3)

    -- pipe operator `|>` allows calling functions like methods
    const answer: int = base |> double |> extract_x
    io.print(string(answer))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "pipe_operator.exe"
    } else {
        "pipe_operator"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "2\n");
}

#[test]
fn compile_runs_field_assignment_and_implicit_self_method_native() {
    let dir = TestDir::new("field_assignment_implicit_self_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Counter
    value: int

    mut func increment()
        self.value = self.value + 1
    end
end

func main()
    var counter: Counter = Counter(value: 1)
    counter.value = 2
    counter.increment()
    io.print(string(counter.value))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "field_assignment_implicit_self_native.exe"
    } else {
        "field_assignment_implicit_self_native"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "3\n");
}

#[test]
fn check_reports_anonymous_struct_field_mismatch() {
    let dir = TestDir::new("anon_struct_field_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Vec2
    x: float
    y: float
end

func main()
    const bad: Vec2 = .{ x: 1.0 }
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"type.anon_struct_field_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_anonymous_struct_without_expected_type() {
    let dir = TestDir::new("anon_struct_no_context");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    .{ x: 1.0, y: 2.0 }
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"type.anon_struct_type_unknown"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_struct_update_without_braces() {
    let dir = TestDir::new("struct_update_without_braces");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Config
    verbose: bool
end

func main()
    const a: Config = Config(verbose: false)
    const b: Config = a with
        verbose: true
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"parse.unexpected_token"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_float_range_as_type_error() {
    let dir = TestDir::new("float_range_type_error");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const r: range<int> = 0.0..1.0
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"parse.invalid_range"), "{codes:?}");
    assert!(
        !codes.contains(&"type.type_mismatch"),
        "range endpoint errors should use the dedicated diagnostic: {codes:?}"
    );
}



#[test]
fn check_reports_success_void_mismatch() {
    let dir = TestDir::new("success_void_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main

func make() -> result<int, string>
    return success()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"contract.success_void_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_byte_unicode_escape() {
    let dir = TestDir::new("byte_unicode_escape");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const data: bytes = b"\u{0041}"
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"parse.byte_unicode_escape"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn compile_runs_struct_update_expression_native() {
    let dir = TestDir::new("struct_update_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

struct Point
    x: int
    y: int
    z: int
end

func main()
    const base: Point = Point(x: 1, y: 2, z: 3)
    const moved: Point = base with { y: 20 } end
    const shifted: Point = moved with { x: 7, z: moved.z + 4 } end

    io.print(string(base.x + base.y + base.z))
    io.print(string(moved.x + moved.y + moved.z))
    io.print(string(shifted.x + shifted.y + shifted.z))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "struct_update_native.exe"
    } else {
        "struct_update_native"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "6\n24\n34\n");
}

#[test]
fn build_lowers_struct_update_expression_to_c() {
    let dir = TestDir::new("struct_update_c");
    dir.write(
        "main.orl",
        r#"namespace app.main

struct Point
    x: int
    y: int
    z: int
end

func main()
    const base: Point = Point(x: 1, y: 2, z: 3)
    const moved: Point = base with { y: 20 } end
    const shifted: Point = moved with { x: 7, z: moved.z + 4 } end
    const total: int = base.x + moved.y + shifted.z
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    compile_c_source(&dir, "struct_update_c", &out.c_source);
}

#[test]
fn compile_is_check_on_any_trait_native() {
    let dir = TestDir::new("is_check_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

trait Shape
    func area(self) -> int
end

struct Circle
    radius: int
end

implement Shape for Circle
    func area(self) -> int
        return self.radius * self.radius
    end
end

struct Square
    side: int
end

implement Shape for Square
    func area(self) -> int
        return self.side * self.side
    end
end

func describe(s: any<Shape>)
    if s is Circle
        io.print("circle")
    else
        io.print("other")
    end
end

func main()
    const c: any<Shape> = Circle(radius: 3)
    const sq: any<Shape> = Square(side: 4)
    describe(c)
    describe(sq)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "is_check.exe"
    } else {
        "is_check"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "circle\nother\n");
}

#[test]
fn compile_runs_fs_stdlib_canonical_and_compat_aliases() {
    let dir = TestDir::new("compile_fs_stdlib");
    let input_path = dir.path("input.txt");
    let output_path = dir.path("output.txt");
    let bytes_output_path = dir.path("output.bin");
    std::fs::write(&input_path, "hello fs").unwrap();

    let input = ori_path_literal(&input_path);
    let output = ori_path_literal(&output_path);
    let bytes_output = ori_path_literal(&bytes_output_path);

    dir.write(
        "main.orl",
        &format!(
            r#"namespace app.main

import ori.bytes as bytes_mod
import ori.fs as fs
import ori.files as files
import ori.io as io

func main()
    const input_path: string = "{input}"
    const output_path: string = "{output}"
    const bytes_output_path: string = "{bytes_output}"

    match fs.read_text(input_path)
        case success(text):
            io.print(text)
        case error(e):
            io.print("read failed: " + e)
    end

    io.print(if fs.exists(input_path) then "exists" else "missing")
    io.print(if files.exists(input_path) then "compat" else "no compat")

    match fs.read_text(output_path)
        case success(_):
            io.print("unexpected")
        case error(_):
            io.print("missing ok")
    end

    match fs.write_text(output_path, "new fs")
        case success(_):
            io.print("wrote")
        case error(e):
            io.print("write failed: " + e)
    end

    match fs.read_all(input_path)
        case success(text):
            io.print(text + " all")
        case error(e):
            io.print("read_all failed: " + e)
    end

    match fs.read_bytes(input_path)
        case success(raw):
            io.print(string(bytes_mod.len(raw)))
            match fs.write_bytes(bytes_output_path, raw)
                case success(_):
                    io.print("bytes wrote")
                case error(e):
                    io.print("bytes write failed: " + e)
            end
        case error(e):
            io.print("bytes read failed: " + e)
    end
end
"#
        ),
    );

    let exe = dir.path(if cfg!(windows) {
        "fs_stdlib.exe"
    } else {
        "fs_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output_run = Command::new(&exe).output().unwrap();
    assert!(output_run.status.success(), "{:?}", output_run);
    let stdout = String::from_utf8(output_run.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "hello fs\nexists\ncompat\nmissing ok\nwrote\nhello fs all\n8\nbytes wrote\n"
    );
    assert_eq!(std::fs::read_to_string(output_path).unwrap(), "new fs");
    assert_eq!(std::fs::read(bytes_output_path).unwrap(), b"hello fs");
}

#[test]
fn compile_runs_fs_bytes_preserve_nul_native() {
    let dir = TestDir::new("compile_fs_bytes_preserve_nul");
    let input_path = dir.path("binary-input.bin");
    let output_path = dir.path("binary-output.bin");
    std::fs::write(&input_path, b"A\0B").unwrap();

    let input = ori_path_literal(&input_path);
    let output = ori_path_literal(&output_path);

    dir.write(
        "main.orl",
        &format!(
            r#"namespace app.main

import ori.fs as fs
import ori.io as io

func main()
    match fs.read_bytes("{input}")
        case success(raw):
            io.print("len=" + string(raw.len()))
            io.print(raw.to_hex())
            match fs.write_bytes("{output}", raw)
                case success(_):
                    io.print("wrote")
                case error(e):
                    io.print("write_error=" + e)
            end
        case error(e):
            io.print("read_error=" + e)
    end
end
"#
        ),
    );

    let exe = dir.path(if cfg!(windows) {
        "fs_bytes_preserve_nul.exe"
    } else {
        "fs_bytes_preserve_nul"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output_run = Command::new(&exe).output().unwrap();
    assert!(output_run.status.success(), "{:?}", output_run);
    let stdout = String::from_utf8(output_run.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "len=3\n410042\nwrote\n");
    assert_eq!(std::fs::read(output_path).unwrap(), b"A\0B");
}

#[test]
fn compile_runs_escaped_literals_and_fstrings() {
    let dir = TestDir::new("escaped_literals_fstrings");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    io.print("line\nnext")

    const raw: bytes = b"\x68\x69\x21"
    match raw.decode_utf8()
        case success(text):
            io.print(text)
        case error(e):
            io.print(e)
    end

    const name: string = "Ori"
    const n: int = 3
    io.print(f"hello {name} {n + 2}")
    io.print(f"brace {{ {name} }}")
    io.print("""
        alpha
        beta
        """)
    io.print(f"""
        multi {name}
        """)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "escaped_literals_fstrings.exe"
    } else {
        "escaped_literals_fstrings"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "line\nnext\nhi!\nhello Ori 5\nbrace { Ori }\nalpha\nbeta\nmulti Ori\n"
    );
}

#[test]
fn compile_runs_triple_string_baseline_and_f_triple_string() {
    let dir = TestDir::new("triple_string_baseline");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const name: string = "Ada"
    io.print("""
        line one
          line two
        """)
    io.print(f"""
        hello {name}
          score {1 + 2}
        """)
end
"#,
    );

    let exe = exe_path(&dir, "triple_string_baseline");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "line one\n  line two\nhello Ada\n  score 3\n"
    );
}

#[test]
fn compile_runs_short_circuit_without_rhs_side_effects() {
    let dir = TestDir::new("short_circuit_side_effect");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func explode() -> bool
    panic("short-circuit failed")
    return true
end

func main()
    if false and explode()
        io.print("bad-and")
    end
    if true or explode()
        io.print("ok")
    end
end
"#,
    );

    let exe = exe_path(&dir, "short_circuit_side_effect");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "ok\n");
}

#[test]
fn compile_runtime_panics_on_integer_division_by_zero() {
    let dir = TestDir::new("runtime_int_div_zero");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    var zero: int = 0
    io.print(string(10 / zero))
end
"#,
    );

    let exe = exe_path(&dir, "runtime_int_div_zero");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
}

#[test]
fn compile_runs_float_division_by_zero_as_infinity() {
    let dir = TestDir::new("runtime_float_div_zero");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    var zero: float = 0.0
    io.print(string(10.0 / zero))
end
"#,
    );

    let exe = exe_path(&dir, "runtime_float_div_zero");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.to_ascii_lowercase().contains("inf"), "{stdout}");
}

#[test]
fn check_reports_exception_words_as_plain_missing_names() {
    let dir = TestDir::new("exception_words_missing_names");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    throw("bad")
    catch("bad")
    try("bad")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(
        !codes.iter().any(|code| code.starts_with("parse.")),
        "{:?}",
        out.diagnostics
    );
    assert!(
        codes
            .iter()
            .filter(|code| **code == "name.undefined")
            .count()
            >= 3,
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn compile_runs_prompt_full_pipeline_program() {
    let dir = TestDir::new("prompt_full_pipeline_program");
    dir.write(
        "util.orl",
        r#"namespace app.util

public func seed() -> int
    return 2
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.util as util
import ori.io as io
import ori.iter as iter

var disposed: int = 0

struct Resource
    id: int if it > 0
    name: string
end

enum AppError
    Validation(message: string)
end

trait Disposable
    mut func dispose(self)
end

trait Named
    func name(self) -> string

    func kind(self) -> string
        return "resource"
    end
end

implement Disposable for Resource
    mut func dispose(self)
        disposed = disposed + self.id
    end
end

implement Named for Resource
    func name(self) -> string
        return self.name
    end
end

func make_resource(id: int) -> result<Resource, AppError>
    if id > 0
        return success(Resource(id: id, name: "item-" + string(id)))
    end
    return error(AppError.Validation(message: "bad id"))
end

func load() -> result<Resource, AppError>
    const resource: Resource = make_resource(util.seed())?
    return success(resource)
end

func describe<T>(item: T) -> string where T is Named
    return item.name()
end

func main()
    match load()
    case success(resource):
        using cleanup: Resource = resource
        io.print(describe(resource))
        io.print(resource.kind())
        const maybe_name: optional<string> = some(resource.name)
        if some(name) = maybe_name
            io.print(name)
        end

        const doubled: list<int> = iter.map([1, 2, 3], do(x: int) => x * util.seed())
        var total: int = 0
        for value, index in doubled
            total = total + value + index
        end
        check total == 15
        io.print(string(total))

        match total
        case n if n >= 15:
            io.print("high")
        case else:
            io.print("low")
        end
    case error(err):
        match err
        case .Validation(message):
            io.print(message)
        end
    end

    io.print(string(disposed))
end
"#,
    );

    let exe = exe_path(&dir, "prompt_full_pipeline_program");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "item-2\nresource\nitem-2\n15\nhigh\n2\n"
    );
}

#[test]
fn check_official_examples() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let examples = [
        "examples/hello_world.orl",
        "examples/calculator.orl",
        "examples/bytes_usage.orl",
        "examples/collections_demo.orl",
        "examples/logic_and_matching.orl",
    ];

    for example in examples {
        let path = root.join(example);
        assert!(path.exists(), "missing official example: {example}");
        let out = run_check(&path).unwrap();
        assert!(
            !out.has_errors,
            "{example} should type-check: {:?}",
            out.diagnostics
        );
    }
}

#[test]
fn check_readme_quick_example() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let readme = std::fs::read_to_string(root.join("README.md")).unwrap();
    assert!(
        !readme.contains("(\namespace)") && !readme.contains("optional, \nesult)"),
        "README has broken line splits inside important language terms"
    );
    assert!(readme.contains("(namespace)"));
    assert!(readme.contains("(optional, result)"));

    let examples = extract_ori_code_fences(&readme);
    assert!(!examples.is_empty(), "README should include an Ori example");

    for (index, source) in examples.iter().enumerate() {
        let dir = TestDir::new(&format!("readme_quick_example_{index}"));
        dir.write("main.orl", source);

        let check = run_check(&dir.path("main.orl")).unwrap();
        assert!(
            !check.has_errors,
            "README Ori example {index} should type-check: {:?}",
            check.diagnostics
        );

        let build = run_build(&dir.path("main.orl")).unwrap();
        assert!(
            !build.has_errors,
            "README Ori example {index} should build with the C backend: {:?}",
            build.diagnostics
        );
    }
}

fn extract_ori_code_fences(markdown: &str) -> Vec<String> {
    let mut examples = Vec::new();
    let mut current = String::new();
    let mut in_ori = false;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_ori {
                examples.push(current.trim().to_string());
                current.clear();
                in_ori = false;
            } else {
                in_ori = matches!(trimmed, "```ori" | "```orl");
            }
            continue;
        }

        if in_ori {
            current.push_str(line);
            current.push('\n');
        }
    }

    examples
}

#[test]
fn compile_runs_unicode_identifier_and_contextual_times() {
    let dir = TestDir::new("unicode_identifier_contextual_times");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const times: int = 2
    const café: string = "ok"

    repeat times times
        io.print(café)
    end

    io.print(string(times))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "unicode_identifier_contextual_times.exe"
    } else {
        "unicode_identifier_contextual_times"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "ok\nok\n2\n");
}

#[test]
fn check_reports_invalid_escapes_and_fstring_diagnostics() {
    let dir = TestDir::new("invalid_escapes_fstrings");
    let source = r#"namespace app.main

func main()
    const bad_string: string = "\q"
    const bad_bytes: bytes = b"\xG0"
    const unclosed: string = f"hello {name"
    const empty: string = f"{}"
    const unmatched: string = f"hello }"
    const trailing: string = f"{1 2}"
end
"#;
    dir.write("main.orl", source);

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "expected parser diagnostics");
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"parse.invalid_escape"), "{codes:?}");
    assert!(codes.contains(&"parse.fstring_unclosed_expr"), "{codes:?}");
    assert!(codes.contains(&"parse.fstring_empty_expr"), "{codes:?}");
    assert!(
        codes.contains(&"parse.fstring_unmatched_brace"),
        "{codes:?}"
    );
    assert!(
        codes.contains(&"parse.fstring_expr_trailing_tokens"),
        "{codes:?}"
    );
    let trailing = out
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "parse.fstring_expr_trailing_tokens")
        .expect("trailing-token f-string diagnostic should exist");
    let expected_start = source.find("1 2").expect("fixture should contain `1 2`") + 2;
    assert_eq!(
        trailing
            .labels
            .first()
            .map(|label| label.span.start as usize),
        Some(expected_start),
        "{trailing:?}"
    );
}

#[test]
fn compile_runs_bytes_stdlib() {
    let dir = TestDir::new("compile_bytes_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.string as str

func main()
    const b1: bytes = "hello".to_bytes()
    const b2: bytes = " world".to_bytes()
    const combined: bytes = b1.concat(b2)

    io.print("Length 1: " + string(b1.len()))
    io.print("Combined: " + string(combined.len()))

    match combined.decode_utf8()
        case success(s):
            io.print("Decoded: " + s)
        case error(e):
            io.print("Failed: " + e)
    end

    const hex: string = b1.to_hex()
    io.print("Hex: " + hex)

    match hex.from_hex()
        case success(b):
            match b.decode_utf8()
                case success(s):
                    io.print("FromHex: " + s)
                case error(_):
                    io.print("Err1")
            end
        case error(e):
            io.print("Err2: " + e)
    end

    match str.from_bytes(b1)
        case success(s):
            io.print("FromBytes: " + s)
        case error(_):
            io.print("ErrBytes")
    end

    match "abc".from_hex()
        case success(_):
            io.print("BadHex")
        case error(_):
            io.print("HexErr")
    end

    const sliced: bytes = combined.slice(0, 5)
    match sliced.decode_utf8()
        case success(s):
            io.print("Sliced: " + s)
        case error(_):
            io.print("Err3")
    end

    const first: u8 = b1.get(0)
    io.print("First: " + string(int(first)))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "bytes_stdlib.exe"
    } else {
        "bytes_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let expected = "Length 1: 5\nCombined: 11\nDecoded: hello world\nHex: 68656c6c6f\nFromHex: hello\nFromBytes: hello\nHexErr\nSliced: hello\nFirst: 104\n";
    assert_eq!(stdout.replace("\r\n", "\n"), expected);
}

#[test]
fn compile_runs_bytes_preserve_nul_native() {
    let dir = TestDir::new("compile_bytes_preserve_nul");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.string as str

func main()
    const raw: bytes = b"\x41\x00\x42"
    io.print("len=" + string(raw.len()))
    io.print(raw.to_hex())

    match raw.decode_utf8()
        case success(_):
            io.print("decode_unexpected")
        case error(_):
            io.print("decode_nul_error")
    end

    match str.from_bytes(raw)
        case success(_):
            io.print("from_bytes_unexpected")
        case error(_):
            io.print("from_bytes_nul_error")
    end

    match "410042".from_hex()
        case success(decoded):
            io.print("decoded_len=" + string(decoded.len()))
            io.print(decoded.to_hex())
        case error(e):
            io.print("error=" + e)
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "bytes_preserve_nul.exe"
    } else {
        "bytes_preserve_nul"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let expected = "len=3\n410042\ndecode_nul_error\nfrom_bytes_nul_error\ndecoded_len=3\n410042\n";
    assert_eq!(stdout.replace("\r\n", "\n"), expected);
}

#[test]
fn compile_runs_unicode_string_len_and_slice_native() {
    let dir = TestDir::new("compile_unicode_string_len_slice");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

func main()
    const text: string = "\u{00e1}\u{00e9}"
    io.print("len=" + string(text.len()))
    io.print(text.slice(0, 1))
    io.print("index=" + string(text.index_of("\u{00e9}")))
    io.print("emoji_index=" + string("\u{1f642}x".index_of("x")))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "unicode_string_len_slice.exe"
    } else {
        "unicode_string_len_slice"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let expected = "len=2\n\u{00e1}\nindex=1\nemoji_index=1\n";
    assert_eq!(stdout.replace("\r\n", "\n"), expected);
}

#[test]
fn compile_runtime_panics_on_list_index_out_of_bounds() {
    let dir = TestDir::new("runtime_list_index_oob");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const values: list<int> = [1]
    values[1]
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "runtime_list_oob.exe"
    } else {
        "runtime_list_oob"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ori list index out of bounds"), "{stderr}");
}

#[test]
fn compile_runtime_panics_on_bytes_index_out_of_bounds() {
    let dir = TestDir::new("runtime_bytes_index_oob");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const raw: bytes = "a".to_bytes()
    raw.get(1)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "runtime_bytes_oob.exe"
    } else {
        "runtime_bytes_oob"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ori bytes index out of bounds"), "{stderr}");
}

#[test]
fn compile_runtime_panics_on_negative_repeat_count() {
    let dir = TestDir::new("runtime_negative_repeat");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    repeat -1
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "runtime_negative_repeat.exe"
    } else {
        "runtime_negative_repeat"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
}

#[test]
fn compile_runtime_panics_on_invalid_string_slice_bounds() {
    let dir = TestDir::new("runtime_string_slice_bounds");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.string as str

func main()
    str.slice("abc", -1, 1)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "runtime_string_slice_bounds.exe"
    } else {
        "runtime_string_slice_bounds"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ori string slice bounds out of range"),
        "{stderr}"
    );
}

#[test]
fn compile_runtime_panics_on_invalid_list_slice_bounds() {
    let dir = TestDir::new("runtime_list_slice_bounds");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.list as lists

func main()
    const values: list<int> = [1]
    lists.slice(values, 0, 2)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "runtime_list_slice_bounds.exe"
    } else {
        "runtime_list_slice_bounds"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ori list slice bounds out of range"),
        "{stderr}"
    );
}

#[test]
fn compile_runtime_panics_on_invalid_bytes_slice_bounds() {
    let dir = TestDir::new("runtime_bytes_slice_bounds");
    dir.write(
        "main.orl",
        r#"namespace app.main

func main()
    const raw: bytes = "a".to_bytes()
    raw.slice(0, 2)
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "runtime_bytes_slice_bounds.exe"
    } else {
        "runtime_bytes_slice_bounds"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ori bytes slice bounds out of range"),
        "{stderr}"
    );
}

#[test]
fn check_accepts_inert_test_attribute_until_test_runner_lands() {
    let dir = TestDir::new("inert_test_attr");
    dir.write(
        "main.orl",
        r#"namespace app.main

@test
func test_addition()
    check 1 + 1 == 2
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert_eq!(diagnostic_codes(&out), Vec::<&'static str>::new());
}

#[test]
fn test_runner_executes_test_attribute_functions() {
    let dir = TestDir::new("test_runner_executes");
    dir.write(
        "main.orl",
        r#"namespace app.main

func add(left: int, right: int) -> int
    return left + right
end

@test
func test_addition()
    check add(1, 2) == 3
end

@test
func test_second_case()
    check add(2, 2) == 4
end
"#,
    );

    let out = run_test(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert_eq!(out.results.len(), 2, "{:?}", out.diagnostics);
    assert!(
        out.results.iter().all(|result| result.passed),
        "{:#?}",
        out.results
            .iter()
            .map(|result| (&result.name, result.passed, &result.stderr))
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_runner_reports_failed_check() {
    let dir = TestDir::new("test_runner_failed_check");
    dir.write(
        "main.orl",
        r#"namespace app.main

@test
func test_failure()
    check 1 == 2
end
"#,
    );

    let out = run_test(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert_eq!(out.results.len(), 1);
    assert!(!out.results[0].passed, "{:#?}", out.results[0].stderr);
}

#[test]
fn test_runner_rejects_non_concrete_test_signature() {
    let dir = TestDir::new("test_runner_invalid_signature");
    dir.write(
        "main.orl",
        r#"namespace app.main

@test
func test_with_param(value: int)
    check value == 1
end
"#,
    );

    let out = run_test(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes: Vec<_> = out.diagnostics.iter().map(|diag| diag.code).collect();
    assert!(
        codes.contains(&"attr.invalid_test_signature"),
        "{:?}",
        out.diagnostics
    );
    assert!(out.results.is_empty());
}

#[test]
fn fmt_normalizes_new_block_syntax_indentation() {
    let dir = TestDir::new("fmt_new_syntax");
    dir.write(
        "main.orl",
        r#"namespace app.main

@test
func test_formatting()
check 1 == 1
if 1 == 1
check 2 == 2
else if 2 == 3
check false
else
check true
end
match 1
case 1:
check true
case else:
check false
end
end
"#,
    );

    let out = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert_eq!(
        out.formatted,
        r#"namespace app.main

@test
func test_formatting()
    check 1 == 1
    if 1 == 1
        check 2 == 2
    else if 2 == 3
        check false
    else
        check true
    end
    match 1
    case 1:
        check true
    case else:
        check false
    end
end
"#
    );
}

#[test]
fn fmt_preserves_collection_syntax_semantics() {
    let dir = TestDir::new("fmt_collection_syntax");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.map as maps
import ori.queue as queue
import ori.set as sets

func main()
const values: map<int, int> = {1: 10, 2: 20}
const seen: set<int> = set {1, 2}
const todo: queue.Queue<string> = queue.new()
queue.enqueue(todo, "ready")
if sets.contains(seen, 2)
io.print(string(maps.get(values, 1)))
end
match queue.dequeue(todo)
case some(item):
io.print(item)
case none:
io.print("empty")
end
end
"#,
    );

    let out = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out
        .formatted
        .contains("    const values: map<int, int> = {1: 10, 2: 20}"));
    assert!(out
        .formatted
        .contains("    const seen: set<int> = set {1, 2}"));
    assert!(out.formatted.contains("    match queue.dequeue(todo)"));
    assert!(out.formatted.contains("    case some(item):"));
    assert!(out.formatted.contains("        io.print(item)"));

    dir.write("formatted.orl", &out.formatted);
    let checked = run_check(&dir.path("formatted.orl")).unwrap();
    assert!(!checked.has_errors, "{:?}", checked.diagnostics);
}

#[test]
fn check_reports_deprecated_attribute_use_site_warning() {
    let dir = TestDir::new("deprecated_attr_warning");
    dir.write(
        "main.orl",
        r#"namespace app.main

@deprecated("use new_api() instead")
func old_api() -> int
    return 1
end

func main()
    const value: int = old_api()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"attr.deprecated"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_imported_deprecated_attribute_use_site_warning() {
    let dir = TestDir::new("imported_deprecated_attr_warning");
    dir.write(
        "legacy.orl",
        r#"namespace app.legacy

@deprecated("use app.newer.value instead")
public const value: int = 1
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main

import app.legacy as legacy

func main()
    const current: int = legacy.value
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"attr.deprecated"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_warns_on_doc_param_tag_name_mismatch() {
    let dir = TestDir::new("doc_param_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main

--|
Adds two numbers.
@param wrong Missing real parameter name.
|--
public func add(left: int, right: int) -> int
    return left + right
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"doc.param_name_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_unknown_attribute() {
    let dir = TestDir::new("unknown_attr");
    dir.write(
        "main.orl",
        r#"namespace app.main

@custom_marker
func main()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"attr.unknown"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_invalid_attribute_target_and_args() {
    let dir = TestDir::new("invalid_attr_target_args");
    dir.write(
        "main.orl",
        r#"namespace app.main

@test
struct Suite
    value: int
end

@deprecated(reason: old)
func old_api()
end

@inline("always")
func hot_path()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"attr.invalid_target"),
        "{:?}",
        out.diagnostics
    );
    assert_eq!(
        codes
            .iter()
            .filter(|code| **code == "attr.invalid_arg")
            .count(),
        2,
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn check_reports_duplicate_attribute_as_warning() {
    let dir = TestDir::new("duplicate_attr_warning");
    dir.write(
        "main.orl",
        r#"namespace app.main

@inline
@inline
func hot_path()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"attr.duplicate"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn compile_runs_stdlib_source_module_string_utils() {
    let dir = TestDir::new("stdlib_source_string_utils");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.string.utils as su

func main()
    io.print(string(su.is_empty("")))
    io.print(string(su.is_empty("hi")))
    io.print(string(su.blank("   ")))
    io.print(string(su.blank("x")))
    io.print(su.replicate("ab", 3))
end
"#,
    );

    let stdout = compile_and_run(&dir, "stdlib_source_string_utils");
    assert_eq!(stdout, "true\nfalse\ntrue\nfalse\nababab\n");
}

#[test]
fn compile_runs_stdlib_source_module_string_utils_layer2() {
    let dir = TestDir::new("stdlib_source_string_utils_layer2");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.string.utils as su

func main()
    io.print(su.default("", "fb"))
    io.print(su.default("x", "fb"))
    io.print(string(su.equals_ignore_case("Hello", "hello")))
    io.print(string(su.equals_ignore_case("ABC", "abd")))
    io.print(su.center("hi", 6))
    io.print(su.center("hello", 3))
    io.print(string(su.count("ababab", "ab")))
    io.print(string(su.count("aaa", "aa")))
    io.print(string(su.count("hello", "x")))
    io.print(string(su.count("hello", "")))
end
"#,
    );

    let stdout = compile_and_run(&dir, "stdlib_source_string_utils_layer2");
    assert_eq!(stdout, "fb\nx\ntrue\nfalse\n  hi  \nhello\n3\n1\n0\n0\n");
}

#[test]
fn check_stdlib_source_module_unknown_still_reports_error() {
    let dir = TestDir::new("stdlib_source_unknown");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.string.nonexistent as sn
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"bind.stdlib_module_unknown"),
        "{:?}",
        out.diagnostics
    );
}
