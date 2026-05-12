use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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
    assert!(build.c_source.contains("ORI__app_model_make_user"));
    assert!(build.c_source.contains("ORI__app_main_make_user"));
    assert!(!build.c_source.contains("ORI__app_facade_model_make_user"));
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
    assert!(build.c_source.contains("ORI__app_main_pass"));
    assert!(build.c_source.contains("ORI__app_model_same"));
    assert!(build.c_source.contains("int main(void)"));
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
    assert!(build.c_source.contains("ORI__app_main_make_user"));
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
    assert_eq!(build.c_source.matches("\nstruct ori_def_").count(), 2);
    assert_eq!(build.c_source.matches("typedef struct ori_def_").count(), 2);
    assert!(build.c_source.contains("ORI__left_user_same"));
    assert!(build.c_source.contains("ORI__right_user_same"));
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
        .contains("static const int64_t app_config_LIMIT = INT64_C(21);"));
    assert!(build.c_source.contains("int64_t value = app_config_LIMIT;"));
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
    const stored: string = string(55)
    io.print(stored)
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
    assert_eq!(stdout.replace("\r\n", "\n"), "-120\n0\n55\n");
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
    dir.write("main.orl", r#"namespace app.main

import ori.convert as conv
import ori.io as io
import ori.string as str

func main()
    const parts: list<string> = str.split("a,b,c", ",")
    io.print(str.join(parts, "|"))
    io.print(str.repeat("ha", 3))
    io.print(str.pad_left("7", 3, "0"))
    io.print(str.pad_right("x", 3, "."))
    io.print(string(str.index_of("abcdef", "cd")))
    io.print(conv.float_to_string(2.5))
    io.print(conv.bool_to_string(false))
    if some(n) = conv.string_to_int("41")
        io.print(string(n + 1))
    end
    if some(f) = conv.string_to_float("3.5")
        io.print(conv.float_to_string(f))
    end
end
"#);

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
        "a|b|c\nhahaha\n007\nx..\n2\n2.5\nfalse\n42\n3.5\n"
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
    io.print(string(math.floor(3.9) + math.ceil(3.1) + math.round(3.5)))
    if math.pow(2.0, 3.0) == 8.0 and math.log(1.0) == 0.0
        io.print("powlog")
    else
        io.print("bad")
    end
    if math.sin(0.0) == 0.0 and math.cos(0.0) == 1.0 and math.tan(0.0) == 0.0
        io.print("trig")
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
    assert_eq!(stdout.replace("\r\n", "\n"), "11\npowlog\ntrig\n");
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

    let exe = dir.path(if cfg!(windows) {
        "string_split_chars.exe"
    } else {
        "string_split_chars"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "red\nblue\nc\n5\n");
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
fn compile_runs_more_collection_stdlib() {
    let dir = TestDir::new("compile_more_collection_stdlib");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.map as maps
import ori.set as sets

func main()
    var values: list<int> = [3, 1, 2]
    lists.insert(values, 1, 7)
    lists.remove(values, 2)
    io.print(string(lists.index_of(values, 7)))
    if lists.contains(values, 2)
        io.print("contains")
    end
    lists.sort(values)
    lists.reverse(values)
    const chunk: list<int> = lists.slice(values, 1, 3)
    io.print(string(lists.pop(chunk)))
    io.print(string(chunk[0] + lists.len(values)))

    const seen: set<int> = sets.new()
    sets.add(seen, 1)
    sets.add(seen, 2)
    sets.remove(seen, 1)
    io.print(string(sets.len(seen)))

    const scores: map<int, int> = maps.new()
    maps.set(scores, 1, 10)
    maps.set(scores, 2, 20)
    maps.set(scores, 3, 30)
    maps.remove(scores, 2)
    const keys: list<int> = maps.keys(scores)
    const vals: list<int> = maps.values(scores)
    io.print(string(lists.len(keys) + lists.len(vals)))
    io.print(string(keys[0] + keys[1] + vals[0] + vals[1]))
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "more_collection_stdlib.exe"
    } else {
        "more_collection_stdlib"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "1\ncontains\n2\n6\n1\n4\n44\n");
}

#[test]
fn compile_runs_for_loop_over_map() {
    let dir = TestDir::new("compile_for_map");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.map as maps

func main()
    const labels: map<int, string> = maps.new()
    maps.set(labels, 1, "alpha")
    maps.set(labels, 2, "beta")

    var key_total: int = 0
    for key in labels
        key_total = key_total + key
    end
    io.print(string(key_total))

    for key, label in labels
        io.print(string(key))
        io.print(label)
    end
end
"#,
    );

    let exe = dir.path(if cfg!(windows) {
        "for_map.exe"
    } else {
        "for_map"
    });
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "3\n1\nalpha\n2\nbeta\n");
}

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
            .contains("ORI__app_main_add(INT64_C(7), INT64_C(5))"),
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
            .contains("ORI__app_main_combine(INT64_C(4), INT64_C(2))"),
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

func sum(seed: int, values: int..) -> int
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
        out.c_source.contains("ORI__app_main_sum(INT64_C(1),"),
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

func sum(seed: int, values: int..) -> int
    var total: int = seed
    for value in values
        total = total + value
    end
    return total
end

func count(values: int..) -> int
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
fn compile_runs_using_dispose_on_native_scope_exit() {
    let dir = TestDir::new("compile_using_dispose_native");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io

var disposed: int = 0

struct Resource
    id: int

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

func sum(values: int..) -> int
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

func sum(values: int..) -> int
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
    assert!(diagnostic_codes(&out).contains(&"bind.import_namespace_mismatch"));
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
    assert!(diagnostic_codes(&out).contains(&"bind.import_cycle"));
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

// ── if some / while some / check ─────────────────────────────────────────────

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
    assert!(!out.has_errors, "{:?}", out.diagnostics);
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
    const numbers: list<int> = [1, 2, 3]
    const first: int = numbers[0]
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
    assert!(out.c_source.contains("ori_list_at"));
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
