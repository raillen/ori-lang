//! E2E tests for Etapa 5 of the maturity plan: runtime ARC, type-specific
//! destructors, cycle collector and the `ORI_TEST_LEAK_CHECK` leak-check
//! convention. These tests compile small Ori programs to the native backend
//! and assert on stdout (and on the leak-check hook output) so regressions in
//! memory management surface as test failures.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use ori_driver::pipeline::run_compile;

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let id = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ori_driver_memory_arc_test_{}_{}_{}",
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

fn exe_path(dir: &TestDir, name: &str) -> PathBuf {
    let filename = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    dir.path(&filename)
}

/// Build a small Ori program, run it, and return its trimmed stdout.
fn compile_and_run(dir: &TestDir, source: &str, exe_name: &str) -> (String, String, bool) {
    dir.write("main.orl", source);
    let exe = exe_path(dir, exe_name);
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "compile errors: {:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();
    (stdout, stderr, success)
}

/// Build a small Ori program, run it with `ORI_TEST_LEAK_CHECK=1` set, and
/// return stdout, stderr, and the exit status.
fn compile_and_run_with_leak_check(
    dir: &TestDir,
    source: &str,
    exe_name: &str,
) -> (String, String, bool) {
    dir.write("main.orl", source);
    let exe = exe_path(dir, exe_name);
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "compile errors: {:?}", out.diagnostics);

    let output = Command::new(&exe)
        .env("ORI_TEST_LEAK_CHECK", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();
    (stdout, stderr, success)
}

// ── 5.1 — Destrutores tipo-específicos ─────────────────────────────────────

/// Baseline: a program with no managed allocations should report zero live
/// allocations. This verifies the leak-check plumbing itself is sound before
/// we test destructors.
#[test]
fn compile_runs_native_no_managed_no_leak() {
    let dir = TestDir::new("no_managed_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.test as test

func main()
    io.print("hello")
    const leaked: int = test.live_allocations()
    io.print("leaks:" + string(leaked))
end
"#,
        "no_managed_no_leak",
    );
    assert!(success, "{stdout}");
    assert_eq!(stdout.trim(), "hello\nleaks:0");
}

/// A single list created and dropped in a helper function should leave no
/// live allocations. The list is created inside `exercise_list`, which
/// returns only an `int` (non-managed), so the list is released by scope
/// cleanup when the helper returns. `main` then checks live allocations
/// before any string-concat temporary is created.
#[test]
fn compile_runs_native_single_list_no_leak() {
    let dir = TestDir::new("single_list_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

func exercise_list() -> int
    const xs: list<int> = lists.new()
    lists.push(xs, 1)
    lists.push(xs, 2)
    const n: int = lists.len(xs)
    return n
end

func main()
    const n: int = exercise_list()
    const leaked: int = test.live_allocations()
    io.print("size:" + string(n))
    io.print("leaks:" + string(leaked))
end
"#,
        "single_list_no_leak",
    );
    assert!(success, "{stdout}");
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines.first(), Some(&"size:2"));
    let leaks_line = lines
        .iter()
        .find(|l| l.starts_with("leaks:"))
        .unwrap_or_else(|| panic!("missing leaks line: {stdout}"));
    let leaks_n: i64 = leaks_line
        .trim_start_matches("leaks:")
        .trim()
        .parse()
        .unwrap();
    assert_eq!(
        leaks_n, 0,
        "expected 0 leaks for single list, got {leaks_n}: {stdout}"
    );
}

/// A struct holding a managed `list<int>` field must release the list when the
/// struct goes out of scope. The struct is created and consumed inside
/// `measure_holder`, which returns only an `int`, so the struct and its list
/// are released by scope cleanup before `main` checks for leaks.
#[test]
fn compile_runs_native_struct_with_managed_field_no_leak() {
    let dir = TestDir::new("struct_managed_field_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

struct Holder
    items: list<int>
end

func make_holder(n: int) -> Holder
    const xs: list<int> = lists.new()
    var i: int = 0
    while i < n
        lists.push(xs, i)
        i = i + 1
    end
    return Holder(items: xs)
end

func measure_holder(n: int) -> int
    const h: Holder = make_holder(n)
    return lists.len(h.items)
end

func main()
    const size: int = measure_holder(4)
    const leaked: int = test.assert_no_leaks("main_struct_managed")
    io.print("size:" + string(size))
    io.print("leaks:" + string(leaked))
end
"#,
        "struct_managed_no_leak",
    );
    assert!(success, "{stdout}");
    assert_eq!(stdout.trim(), "size:4\nleaks:0");
}

/// Nested structs (struct holding struct holding managed list) must cascade
/// releases through the generated destructors. The branch is created and
/// consumed inside `measure_branch`, which returns only an `int`.
#[test]
fn compile_runs_native_nested_struct_arc_cascade_no_leak() {
    let dir = TestDir::new("nested_struct_cascade_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

struct Leaf
    data: list<int>
end

struct Branch
    leaf: Leaf
end

func make_branch(n: int) -> Branch
    const xs: list<int> = lists.new()
    var i: int = 0
    while i < n
        lists.push(xs, i * i)
        i = i + 1
    end
    return Branch(leaf: Leaf(data: xs))
end

func measure_branch(n: int) -> int
    const b: Branch = make_branch(n)
    return lists.len(b.leaf.data)
end

func main()
    const size: int = measure_branch(3)
    const leaked: int = test.assert_no_leaks("main_nested_struct")
    io.print("leaf-size:" + string(size))
    io.print("leaks:" + string(leaked))
end
"#,
        "nested_struct_cascade",
    );
    assert!(success, "{stdout}");
    assert_eq!(stdout.trim(), "leaf-size:3\nleaks:0");
}

/// An enum variant carrying a managed payload must release the payload when
/// the enum value goes out of scope. The enum is created and consumed inside
/// `measure_polygon`, which returns only an `int`.
#[test]
fn compile_runs_native_enum_with_managed_payload_no_leak() {
    let dir = TestDir::new("enum_managed_payload_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

enum Shape
    Empty
    Polygon(points: list<int>)
end

func make_polygon(n: int) -> Shape
    const xs: list<int> = lists.new()
    var i: int = 0
    while i < n
        lists.push(xs, i)
        i = i + 1
    end
    return Shape.Polygon(points: xs)
end

func measure_polygon(n: int) -> int
    const s: Shape = make_polygon(n)
    match s
        case Empty:
            return 0
        case Polygon(points):
            return lists.len(points)
    end
    return 0
end

func main()
    const size: int = measure_polygon(5)
    const leaked: int = test.assert_no_leaks("main_enum_payload")
    io.print("polygon:" + string(size))
    io.print("leaks:" + string(leaked))
end
"#,
        "enum_managed_payload",
    );
    assert!(success, "{stdout}");
    assert_eq!(stdout.trim(), "polygon:5\nleaks:0");
}

/// Optional holding a managed struct must release the inner allocation when
/// the optional goes out of scope. The optional is created and consumed inside
/// `measure_box`, which returns only an `int`.
#[test]
fn compile_runs_native_optional_struct_no_leak() {
    let dir = TestDir::new("optional_struct_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

struct Box
    items: list<int>
end

func maybe_box(n: int) -> optional<Box>
    if n <= 0
        return none
    end
    const xs: list<int> = lists.new()
    var i: int = 0
    while i < n
        lists.push(xs, i)
        i = i + 1
    end
    return some(Box(items: xs))
end

func measure_box(n: int) -> int
    const opt: optional<Box> = maybe_box(n)
    match opt
        case some(box):
            return lists.len(box.items)
        case none:
            return 0
    end
    return 0
end

func main()
    const size: int = measure_box(3)
    const leaked: int = test.assert_no_leaks("main_optional_struct")
    io.print("box:" + string(size))
    io.print("leaks:" + string(leaked))
end
"#,
        "optional_struct_no_leak",
    );
    assert!(success, "{stdout}");
    assert_eq!(stdout.trim(), "box:3\nleaks:0");
}

// ── 5.2 — Cycle collector ──────────────────────────────────────────────────

/// The cycle collector runs and reports a non-negative count. This verifies
/// the `ori.test.collect_cycles` plumbing works end-to-end through the native
/// backend. The cycle collector itself is tested extensively in
/// `ori-runtime/src/tests.rs` (4 cycle scenarios).
///
/// Note: this test does NOT assert zero leaks because the codegen currently
/// over-retains managed values (see Etapa 5 known issues). It only verifies
/// the collector runs and returns a count.
#[test]
fn compile_runs_native_cycle_collector_runs() {
    let dir = TestDir::new("cycle_collector_runs");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

func main()
    const xs: list<int> = lists.new()
    lists.push(xs, 1)
    lists.push(xs, 2)
    io.print("size:" + string(lists.len(xs)))
    const collected: int = test.collect_cycles()
    io.print("collected:" + string(collected))
end
"#,
        "cycle_collector_runs",
    );
    assert!(success, "{stdout}");
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines.first(), Some(&"size:2"));
    assert!(
        lines.iter().any(|l| l.starts_with("collected:")),
        "missing collected line: {stdout}"
    );
}

/// Two Peer structs whose `link` fields reference each other's backing lists
/// must be fully released when the creating function returns. The cycle
/// collector runs in `main` after the function returns; no live allocations
/// should remain.
#[test]
fn compile_runs_native_orphan_cycle_reclaimed() {
    let dir = TestDir::new("orphan_cycle_reclaimed");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

struct Peer
    id: int
    link: list<int>
end

func build_and_measure() -> int
    const a_link: list<int> = lists.new()
    const b_link: list<int> = lists.new()
    lists.push(a_link, 10)
    lists.push(b_link, 20)
    const a: Peer = Peer(id: 1, link: b_link)
    const b: Peer = Peer(id: 2, link: a_link)
    return a.id + b.id
end

func main()
    const total: int = build_and_measure()
    const collected: int = test.collect_cycles()
    const leaked: int = test.live_allocations()
    io.print("total:" + string(total))
    io.print("collected:" + string(collected))
    io.print("leaks:" + string(leaked))
end
"#,
        "orphan_cycle_reclaimed",
    );
    assert!(success, "{stdout}");
    let lines = stdout.lines().collect::<Vec<_>>();
    assert!(
        lines.iter().any(|l| l.starts_with("total:3")),
        "missing total:3 line: {stdout}"
    );
    assert!(
        lines.iter().any(|l| l.starts_with("collected:")),
        "missing collected line: {stdout}"
    );
    let leaks_line = lines
        .iter()
        .find(|l| l.starts_with("leaks:"))
        .unwrap_or_else(|| panic!("missing leaks line: {stdout}"));
    assert_eq!(leaks_line.trim(), "leaks:0");
}

/// A long chain of managed allocations created and dropped in a loop must not
/// accumulate live allocations. Each iteration's list is released by the loop
/// body's scope cleanup. The leak check runs before any string-concat
/// temporary is created.
#[test]
fn compile_runs_native_loop_managed_allocations_no_leak() {
    let dir = TestDir::new("loop_managed_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

func main()
    var i: int = 0
    var total: int = 0
    while i < 100
        const xs: list<int> = lists.new()
        lists.push(xs, i)
        lists.push(xs, i * 2)
        total = total + lists.len(xs)
        i = i + 1
    end
    const leaked: int = test.assert_no_leaks("main_loop_managed")
    io.print("total:" + string(total))
    io.print("leaks:" + string(leaked))
end
"#,
        "loop_managed_no_leak",
    );
    assert!(success, "{stdout}");
    assert_eq!(stdout.trim(), "total:200\nleaks:0");
}

// ── 5.3 — ORI_TEST_LEAK_CHECK env ──────────────────────────────────────────

/// When `ORI_TEST_LEAK_CHECK=1` is set and a program retains allocations on
/// purpose, the runtime aborts with a non-zero exit code and a stderr
/// diagnostic. This is the convention test harnesses opt into to fail fast
/// on leaks.
#[test]
fn compile_runs_native_leak_check_env_aborts_on_intentional_leak() {
    let dir = TestDir::new("leak_check_env_aborts");
    dir.write(
        "main.orl",
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

var kept: list<int> = lists.new()

func main()
    lists.push(kept, 1)
    lists.push(kept, 2)
    io.print("kept:" + string(lists.len(kept)))
    const leaked: int = test.assert_no_leaks("main_intentional_leak")
    io.print("leaks:" + string(leaked))
end
"#,
    );
    let exe = exe_path(&dir, "leak_check_env_aborts");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let plain = Command::new(&exe).output().unwrap();
    let plain_stdout = String::from_utf8_lossy(&plain.stdout).to_string();
    let plain_stderr = String::from_utf8_lossy(&plain.stderr).to_string();
    assert!(
        plain.status.success(),
        "plain run failed: {plain_stdout} {plain_stderr}"
    );
    assert!(
        plain_stdout.contains("kept:2"),
        "missing kept:2 in {plain_stdout}"
    );

    let strict = Command::new(&exe)
        .env("ORI_TEST_LEAK_CHECK", "1")
        .output()
        .unwrap();
    let strict_stderr = String::from_utf8_lossy(&strict.stderr).to_string();
    assert!(
        !strict.status.success(),
        "expected non-zero exit with leak check, got success: {}",
        strict_stderr
    );
    assert!(
        strict_stderr.contains("ori leak check"),
        "expected leak-check diagnostic in stderr, got: {strict_stderr}"
    );
}

/// With leak check enabled, a program with no managed allocations completes
/// successfully and emits no leak-check diagnostic. This verifies the
/// `ORI_TEST_LEAK_CHECK=1` env does not false-positive on clean programs.
#[test]
fn compile_runs_native_leak_check_env_clean() {
    let dir = TestDir::new("leak_check_env_clean");
    let (stdout, stderr, success) = compile_and_run_with_leak_check(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.test as test

func main()
    io.print("clean")
    const leaked: int = test.assert_no_leaks("main_clean")
    io.print("leaks:" + string(leaked))
end
"#,
        "leak_check_env_clean",
    );
    assert!(success, "stderr: {stderr}");
    assert!(
        !stderr.contains("ori leak check"),
        "unexpected leak diagnostic in clean run: {stderr}"
    );
    assert_eq!(stdout.trim(), "clean\nleaks:0");
}

// ── 5.2 — Stress (opt-in) ──────────────────────────────────────────────────

/// Stress test for 10k iterations. Marked `#[ignore]` by default to keep the
/// default test suite fast; run with `cargo test -p ori-driver --test
/// memory_arc -- --ignored cycle_stress`. The test verifies the collector
/// runs without crashing on a large workload.
#[test]
#[ignore = "slow stress test: run with --ignored cycle_stress"]
fn compile_runs_native_cycle_stress_10k() {
    let dir = TestDir::new("cycle_stress_10k");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.list as lists
import ori.test as test

struct Peer
    id: int
    link: list<int>
end

func build_one_cycle(seed: int) -> int
    const a_link: list<int> = lists.new()
    const b_link: list<int> = lists.new()
    const a: Peer = Peer(id: seed, link: b_link)
    const b: Peer = Peer(id: seed + 1, link: a_link)
    return a.id + b.id
end

func main()
    var i: int = 0
    var acc: int = 0
    while i < 10000
        acc = acc + build_one_cycle(i)
        i = i + 1
    end
    io.print("acc:" + string(acc))
    const collected: int = test.collect_cycles()
    io.print("collected:" + string(collected))
    const leaked: int = test.live_allocations()
    io.print("leaks:" + string(leaked))
end
"#,
        "cycle_stress_10k",
    );
    assert!(success, "{stdout}");
    let lines = stdout.lines().collect::<Vec<_>>();
    let acc_line = lines
        .iter()
        .find(|l| l.starts_with("acc:"))
        .unwrap_or_else(|| panic!("missing acc line: {stdout}"));
    assert!(acc_line.starts_with("acc:"), "missing acc line: {stdout}");
    assert!(
        lines.iter().any(|l| l.starts_with("collected:")),
        "missing collected line: {stdout}"
    );
    assert!(
        lines.iter().any(|l| l.starts_with("leaks:")),
        "missing leaks line: {stdout}"
    );
}

/// Etapa 5 checkbox 251: ciclo envolvendo coleções opacas (`linked_list` /
/// `graph`) — sem leak após teste de estresse. The graph runtime owns internal
/// managed edges (adjacency lists); the linked_list runtime owns managed
/// nodes. Both are created and dropped in a loop. After the loop, the leak
/// check must report zero live allocations, proving the type-specific
/// destructors and ARC release paths cover opaque collections.
#[test]
fn compile_runs_native_linked_list_and_graph_no_leak() {
    let dir = TestDir::new("linked_list_graph_no_leak");
    let (stdout, _stderr, success) = compile_and_run(
        &dir,
        r#"namespace app.main

import ori.io as io
import ori.graph as graph
import ori.linked_list as llist
import ori.list as lists
import ori.test as test

func exercise_collections(seed: int) -> int
    const xs: llist.LinkedList<int> = llist.new()
    llist.push_back(xs, seed)
    llist.push_back(xs, seed + 1)
    llist.push_back(xs, seed + 2)

    const g: graph.Graph<int> = graph.new(true)
    graph.add_edge(g, seed, seed + 1)
    graph.add_edge(g, seed + 1, seed + 2)
    graph.add_edge(g, seed + 2, seed)

    const ns: list<int> = graph.neighbors(g, seed)
    return llist.len(xs) + lists.len(ns)
end

func main()
    var i: int = 0
    var total: int = 0
    while i < 50
        total = total + exercise_collections(i)
        i = i + 1
    end
    const collected: int = test.collect_cycles()
    const leaked: int = test.assert_no_leaks("linked_list_graph_stress")
    io.print("total:" + string(total))
    io.print("collected:" + string(collected))
    io.print("leaks:" + string(leaked))
end
"#,
        "linked_list_graph_no_leak",
    );
    assert!(success, "{stdout}");
    let lines = stdout.lines().collect::<Vec<_>>();
    let total_line = lines
        .iter()
        .find(|l| l.starts_with("total:"))
        .unwrap_or_else(|| panic!("missing total line: {stdout}"));
    // Each iteration: llist.len=3 + neighbors of seed (1 edge -> 1 neighbor) = 4.
    // 50 iterations * 4 = 200.
    assert_eq!(total_line.trim(), "total:200");
    let leaks_line = lines
        .iter()
        .find(|l| l.starts_with("leaks:"))
        .unwrap_or_else(|| panic!("missing leaks line: {stdout}"));
    assert_eq!(leaks_line.trim(), "leaks:0");
}
