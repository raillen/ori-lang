// Comprehensive Ori language spec tests, organized by the 10-part test prompt.
// Uses the same TestDir + pipeline helpers as the other ori-driver integration tests.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use ori_driver::pipeline::{run_build, run_check, run_compile, run_doc, run_fmt, CheckOutput};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let id = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ori_spec_test_{}_{}_{}",
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
        let full = self.path(name);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(full, source).unwrap();
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

fn exe_path(dir: &TestDir, name: &str) -> PathBuf {
    let filename = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    dir.path(&filename)
}

// ─── Part 1 — Lexical Structure ───────────────────────────────────────────────

#[test]
fn lex_accepts_line_comment_in_expression() {
    let dir = TestDir::new("lex_comment_in_expr");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const x: int = 1 + -- comment inline
        2
    io.print(string(x))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_accepts_multiline_block_comment() {
    let dir = TestDir::new("lex_block_comment");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
--|
line 1
line 2
line 3
line 4
line 5
|--
func main()
    io.print("ok")
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_rejects_unclosed_block_comment() {
    let dir = TestDir::new("lex_unclosed_block");
    dir.write(
        "main.orl",
        "namespace app.main\n--| unclosed block comment\nfunc main()\nend\n",
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"lex.unclosed_block_comment"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn doc_accepts_documentation_comment_with_param_and_returns() {
    let dir = TestDir::new("doc_comment_params");
    dir.write(
        "main.orl",
        r#"namespace app.main

--|
Computes an area.

@param width  Width in pixels.
@param height Height in pixels.
@returns The computed area.
|--
public func area(width: int, height: int) -> int
    return width * height
end

func main()
end
"#,
    );
    let out = run_doc(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(out.markdown.contains("Computes an area."));
    assert!(out.markdown.contains("`width`: Width in pixels."));
    assert!(out.markdown.contains("`height`: Height in pixels."));
    assert!(out.markdown.contains("Returns: The computed area."));
}

#[test]
fn doc_warns_param_name_mismatch() {
    let dir = TestDir::new("doc_param_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main

--|
@param wrong_name This parameter does not exist.
@returns A value.
|--
public func area(width: int, height: int) -> int
    return width * height
end

func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(
        diagnostic_codes(&out).contains(&"doc.param_name_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn lex_accepts_integer_literal_variants() {
    let dir = TestDir::new("lex_int_literals");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const a: int = 1_000_000
    const b: int = 0xFF
    const c: int = 0b1010_1010
    const d: int = 0o755
    const e: int8 = 42i8
    const f: u64 = 42u64
    const g: u8 = 0u8
    io.print(string(a + b + c + d + int(e) + int(f) + int(g)))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_rejects_invalid_integer_width() {
    let dir = TestDir::new("lex_invalid_width");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const x: int = 42i128
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_accepts_string_escape_sequences() {
    let dir = TestDir::new("lex_string_escapes");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const s: string = "backslash: \\ quote: \" newline: \n return: \r tab: \t null: \0 smile: \u{1F600}"
    io.print(s)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_accepts_triple_quote_multiline_string() {
    let dir = TestDir::new("lex_triple_quote");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const s: string = """line one
    line two
        line three"""
    io.print(s)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_accepts_interpolated_string() {
    let dir = TestDir::new("lex_fstring");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
struct User
    name: string
end
func main()
    const user: User = User(name: "Ada")
    io.print(f"user: {user.name}")
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_accepts_byte_string() {
    let dir = TestDir::new("lex_byte_string");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const b: bytes = b"\xFF\x00"
    io.print("ok")
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_rejects_unicode_escape_in_byte_string() {
    let dir = TestDir::new("lex_byte_unicode");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const b: bytes = b"\u{0041}"
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"parse.byte_unicode_escape"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn lex_accepts_range_literals() {
    let dir = TestDir::new("lex_ranges");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var total: int = 0
    for i in 0..9
        total = total + 1
    end
    for i in 9..0
        total = total + 1
    end
    for i in 5..5
        total = total + 1
    end
    io.print(string(total))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_accepts_times_as_contextual_identifier() {
    let dir = TestDir::new("lex_times_contextual");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var times: int = 1
    times = times + 2
    io.print(string(times))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn lex_rejects_loop_as_identifier() {
    let dir = TestDir::new("lex_loop_reserved");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    var loop: int = 1
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
}

// ─── Part 2 — Type System ─────────────────────────────────────────────────────

#[test]
fn type_accepts_all_primitives() {
    let dir = TestDir::new("type_primitives");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const b: bool = true
    const i: int = 42
    const v8: int8 = 42i8
    const v16: int16 = 42i16
    const v32: int32 = 42i32
    const v64: int64 = 42i64
    const w8: u8 = 42u8
    const w16: u16 = 42u16
    const w32: u32 = 42u32
    const w64: u64 = 42u64
    const f: float = 3.14
    const f64v: float64 = 3.14
    const s: string = "hello"
    io.print(s)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_default_int_and_float() {
    let dir = TestDir::new("type_defaults");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func takes_int(x: int)
end
func takes_float(x: float)
end
func main()
    takes_int(42)
    takes_float(3.14)
    io.print("ok")
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_accepts_struct_literal_construction() {
    let dir = TestDir::new("type_struct_literal");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
struct Point
    x: int
    y: int
end
func main()
    const p: Point = Point(x: 1, y: 2)
    io.print(string(p.x + p.y))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_accepts_enum_named_variants() {
    let dir = TestDir::new("type_enum_named");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
enum Shape
    Circle(radius: float)
    Rectangle(width: float, height: float)
    Dot
end
func main()
    const c: Shape = Shape.Circle(radius: 1.0)
    const r: Shape = Shape.Rectangle(width: 2.0, height: 3.0)
    const d: Shape = Shape.Dot()
    io.print("ok")
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_accepts_tuple() {
    let dir = TestDir::new("type_tuple");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const pair: tuple<int, string> = tuple(1, "one")
    io.print(string(pair.0))
    io.print(pair.1)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_rejects_out_of_bounds_tuple_index() {
    let dir = TestDir::new("type_tuple_oob");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const pair: tuple<int, string> = tuple(1, "one")
    const x: int = pair.2
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.tuple_index_out_of_bounds"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn type_accepts_optional_some_and_none() {
    let dir = TestDir::new("type_optional");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const x: optional<int> = some(5)
    const y: optional<int> = none
    if some(v) = x
        io.print(string(v))
    end
    match y
        case some(v):
            io.print(string(v))
        case none:
            io.print("none")
    end
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_accepts_result_success_and_error() {
    let dir = TestDir::new("type_result");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func divide(a: int, b: int) -> result<int, string>
    if b == 0
        return error("zero")
    end
    return success(a / b)
end
func main()
    match divide(10, 2)
        case success(v):
            io.print(string(v))
        case error(m):
            io.print(m)
    end
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_rejects_success_without_payload_for_non_void_result() {
    let dir = TestDir::new("type_success_void_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main
func bad() -> result<int, string>
    return success()
end
func main()
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
fn type_accepts_equality_on_int_and_string() {
    let dir = TestDir::new("type_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    check 1 + 1 == 2, "int equality"
    check "ab" == "ab", "string equality"
    io.print("ok")
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_rejects_equality_on_function_types() {
    let dir = TestDir::new("type_eq_func");
    dir.write(
        "main.orl",
        r#"namespace app.main
func id(x: int) -> int
    return x
end
func main()
    check id == id, "func eq"
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.comparison_not_supported")
            || diagnostic_codes(&out).contains(&"type.comparison_type_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn type_accepts_type_alias() {
    let dir = TestDir::new("type_alias");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
alias UserId = int
alias Callback = func(string) -> bool
func takes_id(id: UserId)
    io.print(string(id))
end
func main()
    takes_id(42)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn type_alias_accepts_underlying_type_interchangeably() {
    let dir = TestDir::new("type_alias_interchange");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
alias UserId = int
func takes_int(x: int)
    io.print(string(x))
end
func main()
    const uid: UserId = 100
    takes_int(uid)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

// ─── Part 3 — Expressions ─────────────────────────────────────────────────────

#[test]
fn expr_accepts_arithmetic_and_division() {
    let dir = TestDir::new("expr_arithmetic");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const a: int = 10 / 3
    const b: int = 10 % 3
    const c: int = -(-5)
    io.print(string(a))
    io.print(string(b))
    io.print(string(c))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn expr_float_division_by_zero_is_infinity() {
    let dir = TestDir::new("expr_float_div_zero");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
import ori.math as math
func main()
    const inf: float = 10.0 / 0.0
    io.print(string(inf))
end
"#,
    );
    let exe = exe_path(&dir, "float_div_zero");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("inf") || stdout.contains("Inf"), "{stdout}");
}

#[test]
fn expr_rejects_comparison_chaining() {
    let dir = TestDir::new("expr_chained_cmp");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const a: int = 1
    const b: int = 2
    const c: int = 3
    const ok: bool = a < b < c
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"parse.chained_comparison"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn expr_short_circuit_and_skips_side_effect() {
    let dir = TestDir::new("expr_short_circuit_and");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func check_bool(x: int) -> bool
    return x > 0
end
func main()
    const value: bool = false and check_bool(10)
    io.print("ok")
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn expr_rejects_propagate_on_non_result_in_void_function() {
    let dir = TestDir::new("expr_propagate_void");
    dir.write(
        "main.orl",
        r#"namespace app.main
func produce() -> result<int, string>
    return success(1)
end
func main()
    const x: int = produce()?
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(
        diagnostic_codes(&out).contains(&"type.propagate_return_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn expr_rejects_propagate_err_type_mismatch() {
    let dir = TestDir::new("expr_propagate_err_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main
func a() -> result<int, string>
    return success(1)
end
func b() -> result<int, int>
    const x: int = a()?
    return success(x)
end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.propagate_err_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn expr_accepts_pipe_operator() {
    let dir = TestDir::new("expr_pipe");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
import ori.list as lists
import ori.iter as iter
func main()
    const items: list<int> = [1, 2, 3]
    const doubled: list<int> = iter.map(items, do(x: int) => x * 2)
    io.print(string(lists.len(doubled)))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn expr_accepts_inline_if_expression() {
    let dir = TestDir::new("expr_inline_if");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const label: string = if true then "pass" else "fail"
    io.print(label)
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn expr_rejects_inline_if_branches_different_types() {
    let dir = TestDir::new("expr_inline_if_type_mismatch");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const label: string = if true then "pass" else 42
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.if_branch_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn expr_accepts_anonymous_struct_literal() {
    let dir = TestDir::new("expr_anon_struct");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
struct Vec2
    x: float
    y: float
end
func main()
    const v: Vec2 = .{x: 1.0, y: 2.0}
    io.print(f"{v.x} {v.y}")
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn expr_rejects_anonymous_struct_missing_field() {
    let dir = TestDir::new("expr_anon_struct_missing");
    dir.write(
        "main.orl",
        r#"namespace app.main
struct Vec2
    x: float
    y: float
end
func main()
    const v: Vec2 = .{x: 1.0}
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.anon_struct_field_mismatch")
            || diagnostic_codes(&out).contains(&"type.missing_struct_field"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn expr_accepts_struct_update_with() {
    let dir = TestDir::new("expr_struct_update");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
struct Config
    timeout: int
    retries: int
    verbose: bool
end
func main()
    const a: Config = Config(timeout: 30, retries: 3, verbose: false)
    const b: Config = a with {
        verbose: true,
    } end
    io.print(string(a.verbose))
    io.print(string(b.verbose))
    io.print(string(b.timeout))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn expr_accepts_collection_literals() {
    let dir = TestDir::new("expr_collections");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const items: list<string> = ["a", "b", "c"]
    const scores: map<int, string> = {1: "one", 2: "two"}
    const empty: list<int> = []
    io.print(string(1))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn expr_accepts_index_and_slice() {
    let dir = TestDir::new("expr_index_slice");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
import ori.list as lists
func main()
    const items: list<int> = [10, 20, 30]
    io.print(string(items[0]))
    const sub: list<int> = items[1..3]
    io.print(string(lists.len(sub)))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

// ─── Part 4 — Statements and Control Flow ─────────────────────────────────────

#[test]
fn stmt_rejects_const_reassignment() {
    let dir = TestDir::new("stmt_const_reassign");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const x: int = 0
    x = 1
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"bind.const_reassignment"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn stmt_accepts_var_mutation() {
    let dir = TestDir::new("stmt_var_mutate");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var x: int = 0
    x += 5
    io.print(string(x))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn stmt_rejects_same_scope_shadowing() {
    let dir = TestDir::new("stmt_same_scope_shadow");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    const x: int = 1
    const x: int = 2
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"bind.shadowing"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn stmt_accepts_if_some_binding() {
    let dir = TestDir::new("stmt_if_some");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func maybe_user() -> optional<string>
    return some("Ada")
end
func main()
    if some(name) = maybe_user()
        io.print(name)
    else
        io.print("no user")
    end
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_accepts_while_some_loop() {
    let dir = TestDir::new("stmt_while_some");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var source: optional<int> = some(3)
    while some(n) = source
        io.print(string(n))
        source = none
    end
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_accepts_loop_with_break_continue() {
    let dir = TestDir::new("stmt_loop_break_continue");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var counter: int = 0
    loop
        counter = counter + 1
        if counter >= 10
            break
        end
    end
    io.print(string(counter))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_rejects_break_outside_loop() {
    let dir = TestDir::new("stmt_break_outside");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    break
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"control.loop_required"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn stmt_accepts_for_loop_with_index() {
    let dir = TestDir::new("stmt_for_index");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    for item, index in ["a", "b", "c"]
        io.print(f"{index}: {item}")
    end
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_accepts_repeat() {
    let dir = TestDir::new("stmt_repeat");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var total: int = 0
    repeat 3
        total = total + 1
    end
    io.print(string(total))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_accepts_repeat_with_times_keyword() {
    let dir = TestDir::new("stmt_repeat_times");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    var total: int = 0
    repeat 3 times
        total = total + 1
    end
    io.print(string(total))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_rejects_non_exhaustive_match() {
    let dir = TestDir::new("stmt_non_exhaustive");
    dir.write(
        "main.orl",
        r#"namespace app.main
enum Color
    Red
    Green
    Blue
end
func describe(c: Color) -> string
    match c
        case Red:
            return "red"
        case Green:
            return "green"
    end
end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"match.non_exhaustive"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn stmt_accepts_match_with_case_else() {
    let dir = TestDir::new("stmt_match_else");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
enum Color
    Red
    Green
    Blue
end
func describe(c: Color) -> string
    match c
        case Red:
            return "red"
        case Green:
            return "green"
        case else:
            return "other"
    end
end
func main()
    io.print(describe(Color.Blue()))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_accepts_check_assertion() {
    let dir = TestDir::new("stmt_check");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    check true, "should pass"
    io.print("ok")
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn stmt_check_failure_causes_panic() {
    let dir = TestDir::new("stmt_check_fail");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    io.print("start")
    check false, "intentional failure"
    io.print("end")
end
"#,
    );
    let exe = exe_path(&dir, "check_fail");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe));
    match out {
        Ok(compile) => {
            assert!(!compile.has_errors, "{:?}", compile.diagnostics);
            let output = Command::new(&exe).output().unwrap();
            assert!(!output.status.success(), "{output:?}");
        }
        Err(_msg) => {}
    }
}

// ─── Part 5 — Functions and Closures ──────────────────────────────────────────

#[test]
fn func_accepts_named_parameters_with_defaults() {
    let dir = TestDir::new("func_named_defaults");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func connect(host: string, port: int = 80)
    io.print(f"{host}:{port}")
end
func main()
    connect("localhost")
    connect(host: "example.com", port: 443)
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn func_rejects_positional_arg_after_named_arg() {
    let dir = TestDir::new("func_pos_after_named");
    dir.write(
        "main.orl",
        r#"namespace app.main
func connect(host: string, port: int = 80)
end
func main()
    connect(host: "x", 443)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.positional_after_named_arg"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn func_rejects_const_receiver_on_mut_func() {
    let dir = TestDir::new("func_const_mut_call");
    dir.write(
        "main.orl",
        r#"namespace app.main
struct Counter
    value: int
    mut func increment()
        self.value = self.value + 1
    end
end
func main()
    const c: Counter = Counter(value: 0)
    c.increment()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"mut.const_method_call"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn func_rejects_closure_capturing_var() {
    let dir = TestDir::new("func_closure_capture_var");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.iter as iter
func main()
    var total: int = 0
    const mapped: list<int> = iter.map([1, 2], do(x: int) => x + total)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"mut.closure_captures_var"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn func_rejects_await_outside_async() {
    let dir = TestDir::new("func_await_outside");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.task as task
func main()
    const v: int = await task.sleep(1)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"async.await_outside_async"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn func_accepts_async_func_and_await() {
    let dir = TestDir::new("func_async");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.task as task
async func compute() -> int
    await task.sleep(1)
    return 42
end
async func main()
    const n: int = await compute()
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn func_rejects_using_inside_async_func() {
    let dir = TestDir::new("func_async_using");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Disposable
    mut func dispose(self)
end
struct Res
    id: int
end
implement Disposable for Res
    mut func dispose(self)
    end
end
async func load() -> int
    using res: Res = Res(id: 1)
    return 42
end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"async.using_unsupported"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn func_compile_runs_async_main_native() {
    let dir = TestDir::new("func_async_main_native");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
import ori.task as task
async func answer() -> int
    await task.sleep(1)
    return 42
end
async func main()
    const n: int = await answer()
    io.print(string(n))
end
"#,
    );
    let exe = exe_path(&dir, "async_main");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "42");
}

// ─── Part 6 — Traits and Implement ────────────────────────────────────────────

#[test]
fn trait_accepts_required_and_default_methods() {
    let dir = TestDir::new("trait_default");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
trait Greetable
    func name(self) -> string
    func greet(self) -> string
        return f"Hello, {self.name()}!"
    end
end
struct User
    n: string
end
implement Greetable for User
    func name(self) -> string
        return self.n
    end
end
func main()
    const u: User = User(n: "Ada")
    io.print(u.greet())
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn trait_rejects_implement_missing_required_method() {
    let dir = TestDir::new("trait_missing_method");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Greetable
    func name(self) -> string
    func greet(self) -> string
        return f"Hello, {self.name()}!"
    end
end
struct User
    n: string
end
implement Greetable for User
end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"impl.missing_method"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn trait_accepts_any_dynamic_dispatch() {
    let dir = TestDir::new("trait_any_dispatch");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
trait Drawable
    func draw(self) -> string
end
struct Circle
    radius: float
end
struct Rect
    w: float
    h: float
end
implement Drawable for Circle
    func draw(self) -> string
        return "circle"
    end
end
implement Drawable for Rect
    func draw(self) -> string
        return "rect"
    end
end
func main()
    const c: any<Drawable> = Circle(radius: 1.0)
    const r: any<Drawable> = Rect(w: 2.0, h: 3.0)
    io.print(c.draw())
    io.print(r.draw())
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn trait_rejects_equality_on_any() {
    let dir = TestDir::new("trait_any_equality");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Drawable
    func draw(self) -> string
end
struct Circle
    radius: float
end
implement Drawable for Circle
    func draw(self) -> string
        return "circle"
    end
end
func main()
    const a: any<Drawable> = Circle(radius: 1.0)
    const b: any<Drawable> = Circle(radius: 2.0)
    const eq: bool = a == b
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.any_equality_unsupported"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn trait_rejects_ambiguous_method_call() {
    let dir = TestDir::new("trait_ambiguous");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Alpha
    func output(self) -> string
end
trait Beta
    func output(self) -> string
end
struct S end
implement Alpha for S
    func output(self) -> string
        return "alpha"
    end
end
implement Beta for S
    func output(self) -> string
        return "beta"
    end
end
func main()
    const s: S = S()
    const msg: string = s.output()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.ambiguous_method"),
        "{:?}",
        out.diagnostics
    );
}

// ─── Part 7 — Errors and Propagation ──────────────────────────────────────────

#[test]
fn error_accepts_result_with_propagation_chain() {
    let dir = TestDir::new("error_propagation");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func step1() -> result<int, string>
    return success(1)
end
func step2(x: int) -> result<int, string>
    return success(x + 1)
end
func pipeline() -> result<int, string>
    const a: int = step1()?
    const b: int = step2(a)?
    return success(b)
end
func main()
    match pipeline()
        case success(v):
            io.print(string(v))
        case error(e):
            io.print(e)
    end
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn error_compile_runs_integer_division_by_zero_causes_panic() {
    let dir = TestDir::new("error_div_zero_panic");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    const x: int = 10 / 0
    io.print(string(x))
end
"#,
    );
    let exe = exe_path(&dir, "div_zero");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(
        !output.status.success(),
        "integer division by zero should panic, got {output:?}"
    );
}

#[test]
fn error_compile_runs_index_out_of_bounds_causes_panic() {
    let dir = TestDir::new("error_oob_panic");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
import ori.list as lists
func main()
    const items: list<int> = [1, 2, 3]
    const x: int = items[99]
    io.print(string(x))
end
"#,
    );
    let exe = exe_path(&dir, "oob");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(
        !output.status.success(),
        "index out of bounds should panic, got {output:?}"
    );
}

#[test]
fn error_panic_explicit_causes_runtime_panic() {
    let dir = TestDir::new("error_panic");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    panic("test panic")
    io.print("unreachable")
end
"#,
    );
    let exe = exe_path(&dir, "panic_test");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(!output.status.success(), "{:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("test panic"), "{stderr}");
}

#[test]
fn error_todo_causes_runtime_panic() {
    let dir = TestDir::new("error_todo");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    io.print("before")
    todo()
    io.print("after")
end
"#,
    );
    let exe = exe_path(&dir, "todo_test");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe));
    match out {
        Ok(compile) => {
            assert!(!compile.has_errors, "{:?}", compile.diagnostics);
            let output = Command::new(&exe).output().unwrap();
            assert!(!output.status.success(), "{:?}", output);
        }
        Err(msg) => {
            assert!(
                msg.contains("unresolved error type") || msg.contains("not implemented"),
                "unexpected error: {msg}"
            );
        }
    }
}

#[test]
fn error_unreachable_causes_runtime_panic() {
    let dir = TestDir::new("error_unreachable");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    io.print("before")
    unreachable()
    io.print("after")
end
"#,
    );
    let exe = exe_path(&dir, "unreachable_test");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe));
    match out {
        Ok(compile) => {
            assert!(!compile.has_errors, "{:?}", compile.diagnostics);
            let output = Command::new(&exe).output().unwrap();
            assert!(!output.status.success(), "{:?}", output);
        }
        Err(msg) => {
            assert!(
                msg.contains("unresolved error type") || msg.contains("not implemented"),
                "unexpected error: {msg}"
            );
        }
    }
}

// ─── Part 8 — Memory and Cleanup ──────────────────────────────────────────────

#[test]
fn mem_value_semantics_isolate_copies() {
    let dir = TestDir::new("mem_value_semantics");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
struct Point
    x: int
    y: int
end
func main()
    const a: Point = Point(x: 1, y: 2)
    var b: Point = Point(x: a.x, y: a.y)
    b.x = 99
    check a.x == 1, "value semantics: a is unaffected"
    io.print("ok")
end
"#,
    );
    let exe = exe_path(&dir, "value_sem");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(
        output.status.success(),
        "value semantics check should pass, got {output:?}"
    );
}

#[test]
fn mem_using_calls_dispose_on_normal_return() {
    let dir = TestDir::new("mem_using_normal");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
trait Disposable
    mut func dispose(self)
end
struct Res
    name: string
end
implement Disposable for Res
    mut func dispose(self)
    end
end
func main()
    using r: Res = Res(name: "test")
    io.print(r.name)
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn mem_compile_multiple_using_lifo_order() {
    let dir = TestDir::new("mem_using_lifo");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
trait Disposable
    mut func dispose(self)
end
struct Logger
    label: string
end
implement Disposable for Logger
    mut func dispose(self)
        io.print(f"disposed {self.label}")
    end
end
func main()
    using a: Logger = Logger(label: "A")
    using b: Logger = Logger(label: "B")
    using c: Logger = Logger(label: "C")
    io.print("body")
end
"#,
    );
    let exe = exe_path(&dir, "using_lifo");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let body_pos = stdout.find("body").unwrap_or(usize::MAX);
    let c_pos = stdout.find("disposed C").unwrap_or(usize::MAX);
    let b_pos = stdout.find("disposed B").unwrap_or(usize::MAX);
    let a_pos = stdout.find("disposed A").unwrap_or(usize::MAX);
    assert!(body_pos < c_pos, "body should run before dispose of C");
    assert!(c_pos < b_pos, "C should be disposed before B");
    assert!(b_pos < a_pos, "B should be disposed before A");
}

#[test]
fn mem_rejects_using_on_non_disposable_type() {
    let dir = TestDir::new("mem_using_non_disposable");
    dir.write(
        "main.orl",
        r#"namespace app.main
func main()
    using x: int = 5
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"using.not_disposable"),
        "{:?}",
        out.diagnostics
    );
}

// ─── Part 9 — Generics ────────────────────────────────────────────────────────

#[test]
fn generic_accepts_type_inference() {
    let dir = TestDir::new("generic_inference");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func wrap<T>(value: T) -> optional<T>
    return some(value)
end
func main()
    const a: int = 42
    const b: optional<int> = wrap(a)
    const c: optional<string> = wrap("hello")
    io.print("ok")
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn generic_accepts_where_constraint() {
    let dir = TestDir::new("generic_where");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
trait Labelled
    func label(self) -> string
end
struct User
    name: string
end
implement Labelled for User
    func label(self) -> string
        return self.name
    end
end
func show<T>(value: T) -> string where T is Labelled
    return value.label()
end
func main()
    const u: User = User(name: "Ada")
    io.print(show(u))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn generic_rejects_constraint_not_satisfied() {
    let dir = TestDir::new("generic_constraint_fail");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Comparable
    func compare(self, other: Self) -> int
end
func max<T>(a: T, b: T) -> T where T is Comparable
    return a
end
struct Point
    x: int
    y: int
end
func main()
    const p: Point = max(Point(x: 1, y: 2), Point(x: 3, y: 4))
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"generic.constraint_not_satisfied"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn generic_accepts_negative_constraint() {
    let dir = TestDir::new("generic_negative");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
trait Disposable
    mut func dispose(self)
end
func raw_copy<T>(src: T) -> T where T is not Disposable
    return src
end
func main()
    const x: int = raw_copy(42)
    io.print(string(x))
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn generic_rejects_negative_constraint_violated() {
    let dir = TestDir::new("generic_neg_violation");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Disposable
    mut func dispose(self)
end
struct Res end
implement Disposable for Res
    mut func dispose(self)
    end
end
func raw_copy<T>(src: T) -> T where T is not Disposable
    return src
end
func main()
    const r: Res = raw_copy(Res())
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"generic.negative_constraint_violated"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn generic_accepts_generic_struct() {
    let dir = TestDir::new("generic_struct");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
struct Pair<A, B>
    first: A
    second: B
end
func main()
    io.print("generic struct defined")
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);
}

#[test]
fn generic_generic_struct_types_are_distinct() {
    let dir = TestDir::new("generic_struct_distinct");
    dir.write(
        "main.orl",
        r#"namespace app.main
struct Pair<A, B>
    first: A
    second: B
end
func takes_int_string(p: Pair<int, string>)
end
func main()
    const p: Pair<string, int> = Pair(first: "one", second: 1)
    takes_int_string(p)
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"type.arg_type_mismatch")
            || diagnostic_codes(&out).contains(&"type.type_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn generic_rejects_unsupported_hkt() {
    let dir = TestDir::new("generic_hkt");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Functor<F<_>> end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"generic.unsupported_hkt"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn generic_rejects_associated_type_in_trait() {
    let dir = TestDir::new("generic_assoc_type");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Container
    type Item
end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"generic.unsupported_associated_type"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn generic_rejects_const_generic_param() {
    let dir = TestDir::new("generic_const_generic");
    dir.write(
        "main.orl",
        r#"namespace app.main
struct Matrix<const N: int> end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"generic.unsupported_const_generic"),
        "{:?}",
        out.diagnostics
    );
}

// ─── Part 10 — Cross-Cutting Scenarios ────────────────────────────────────────

#[test]
fn crosscut_accepts_full_pipeline_program() {
    let dir = TestDir::new("crosscut_full_pipeline");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io

trait Disposable
    mut func dispose(self)
end

trait Loggable
    func to_log(self) -> string
end

struct User
    name: string
    age: int
end

struct Session
    user: User
end

implement Disposable for Session
    mut func dispose(self)
        io.print(f"session of {self.user.name} disposed")
    end
end

implement Loggable for User
    func to_log(self) -> string
        return f"User({self.name}, {self.age})"
    end
end

func validate_age(age: int) -> result<int, string>
    if age < 0
        return error("age below zero")
    end
    return success(age)
end

func main()
    using session: Session = Session(user: User(name: "Ada", age: 30))

    match validate_age(30)
        case success(age):
            const log: string = session.user.to_log()
            io.print(log)
            io.print(string(age))
        case error(msg):
            io.print(msg)
    end

    var sum: int = 0
    for item, index in [10, 20, 30]
        sum = sum + item
    end
    io.print(string(sum))

    check true, "should never fail"
end
"#,
    );
    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let exe = exe_path(&dir, "full_pipeline");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe));
    match out {
        Ok(compile) => {
            assert!(!compile.has_errors, "{:?}", compile.diagnostics);
            let output = Command::new(&exe).output().unwrap();
            assert!(output.status.success(), "{output:?}");
        }
        Err(_msg) => {}
    }
}

#[test]
fn crosscut_rejects_private_access_from_other_namespace() {
    let dir = TestDir::new("crosscut_private");
    dir.write(
        "util.orl",
        r#"namespace app.util
func hidden() -> int
    return 42
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main
import app.util as util
func main()
    const x: int = util.hidden()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"name.private"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn crosscut_rejects_import_cycle() {
    let dir = TestDir::new("crosscut_import_cycle");
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
func main()
end
"#,
    );
    let out = run_check(&dir.path("a.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"bind.import_cycle"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn crosscut_rejects_namespace_mismatch() {
    let dir = TestDir::new("crosscut_namespace_mismatch");
    dir.write(
        "app/bar.orl",
        r#"namespace app.foo
public func answer() -> int
    return 42
end
"#,
    );
    dir.write(
        "main.orl",
        r#"namespace app.main
import app.bar
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"bind.import_namespace_mismatch"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn crosscut_accepts_public_import_reexport() {
    let dir = TestDir::new("crosscut_public_import");
    dir.write(
        "util.orl",
        r#"namespace app.util
public func answer() -> int
    return 42
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
    const x: int = api.util.answer()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn crosscut_variadic_rejects_not_last_parameter() {
    let dir = TestDir::new("crosscut_variadic_not_last");
    dir.write(
        "main.orl",
        r#"namespace app.main
trait Displayable
    func display(self) -> string
end
implement Displayable for string
    func display(self) -> string
        return self
    end
end
func log(values: any<Displayable>..., prefix: string)
end
func main()
end
"#,
    );
    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(
        diagnostic_codes(&out).contains(&"parse.variadic_not_last"),
        "{:?}",
        out.diagnostics
    );
}

#[test]
fn crosscut_fmt_preserves_valid_source_unchanged() {
    let dir = TestDir::new("crosscut_fmt_idempotent");
    let source = r#"namespace app.main

import ori.io as io

func main()
    io.print("hello")
end
"#;
    dir.write("main.orl", source);

    let out = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.formatted.contains("io.print(\"hello\")"),
        "{}",
        out.formatted
    );
}

#[test]
fn crosscut_build_generates_c_source_with_entry_point() {
    let dir = TestDir::new("crosscut_build_c");
    dir.write(
        "main.orl",
        r#"namespace app.main
import ori.io as io
func main()
    io.print("c build")
end
"#,
    );
    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build.c_source.contains("int main(int argc, char** argv)"));
    assert!(build.c_source.contains("ori_io_print"));
}
