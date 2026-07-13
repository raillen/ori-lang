use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use ori_driver::pipeline::{run_build, run_check, run_compile, run_fmt, run_test, CheckOutput};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let id = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ori_driver_concurrency_test_{}_{}_{}",
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

fn exe_path(dir: &TestDir, name: &str) -> PathBuf {
    let filename = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    dir.path(&filename)
}

#[test]
fn check_accepts_concurrency_stdlib_types() {
    let dir = TestDir::new("check_concurrency_types");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task
import ori.channel as channel
import ori.atomic as atomic

main()
    const job: task.Job[int] = task.spawn(do() => 41)
    const joined: result[int, task.JoinError] = task.join(job)
    const ch: channel.Channel[int] = channel.create()
    const sent: result[void, channel.SendError] = channel.send(ch, 7)
    const received: result[int, channel.ReceiveError] = channel.receive(ch)
    channel.close(ch)
    const counter: atomic.AtomicInt = atomic.new(1)
    atomic.store(counter, atomic.add(counter, 2))
    const current: int = atomic.load(counter)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_accepts_async_func_and_await_types() {
    let dir = TestDir::new("check_async_func_await_types");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

async compute() -> int
    return 41
end

async main()
    const future: future[int] = compute()
    const value: int = await future
    await task.sleep(1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn fmt_preserves_async_state_machine_surface() {
    let dir = TestDir::new("fmt_async_surface");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

async main()
await task.sleep(1)
if true
await task.sleep(1)
end
end
"#,
    );

    let out = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.formatted.contains(
            r#"async main()
    await task.sleep(1)
    if true
        await task.sleep(1)
    end
end"#
        ),
        "{}",
        out.formatted
    );

    dir.write("formatted.orl", &out.formatted);
    let checked = run_check(&dir.path("formatted.orl")).unwrap();
    assert!(!checked.has_errors, "{:?}", checked.diagnostics);
}

#[test]
fn check_rejects_await_outside_async_func() {
    let dir = TestDir::new("await_outside_async_func");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

main()
    await task.sleep(1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"async.await_outside_async"), "{codes:?}");
}

#[test]
fn check_rejects_await_on_non_future_value() {
    let dir = TestDir::new("await_non_future_value");
    dir.write(
        "main.orl",
        r#"module app.main

async main()
    const value: int = await 1
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"async.await_non_future"), "{codes:?}");
}

#[test]
fn check_rejects_non_transferable_spawn_capture() {
    let dir = TestDir::new("spawn_non_transferable_capture");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

main()
    const callback: func() -> int = do() => 1
    const job: task.Job[int] = task.spawn(do() => callback())
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(
        codes.contains(&"async.capture_not_transferable"),
        "{codes:?}"
    );
}

#[test]
fn check_rejects_non_transferable_channel_value() {
    let dir = TestDir::new("channel_non_transferable_value");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.channel as channel

main()
    const ch: channel.Channel[func() -> int] = channel.create()
    const sent: result[void, channel.SendError] = channel.send(ch, do() => 1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"concurrency.not_transferable"), "{codes:?}");
}

#[test]
fn check_allows_using_inside_async_func() {
    let dir = TestDir::new("async_using_allowed");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

trait Disposable
    mut dispose(self)
end

struct Resource
    id: int
end

implement Disposable for Resource
    mut dispose(self)
    end
end

async main()
    using resource: Resource = Resource {id: 1}
    await task.sleep(1)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
}

#[test]
fn check_rejects_keyword_import_aliases_without_hanging() {
    let dir = TestDir::new("map_set_alias_conflict");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.map as map
import ori.set as set

main()
    const lookup: map[int, int] = { 1: 40, 2: 41 }
    const seen: set[int] = set { 3, 4 }
    const value: int = map.get(lookup, 2)
    const ok: bool = set.contains(seen, 4)
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let codes = diagnostic_codes(&out);
    assert!(codes.contains(&"parse.expected_identifier"), "{codes:?}");
}

#[test]
fn compile_runs_task_spawn_join_native() {
    let dir = TestDir::new("compile_task_spawn_join_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

main()
    const job: task.Job[int] = task.spawn(do() => 41)
    match task.join(job)
        case success(value):
            io.print(string(value))
        case error(_):
            io.print("join-error")
    end
end
"#,
    );

    let exe = exe_path(&dir, "task_spawn_join");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_main_and_await_ready_future_native() {
    let dir = TestDir::new("compile_async_main_ready_future_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io

async compute() -> int
    return 41
end

async main()
    const value: int = await compute()
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "async_main_ready_future");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_async_function_call_returns_before_first_await_native() {
    let dir = TestDir::new("compile_async_call_returns_before_await_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task
import ori.time as time

async delayed() -> int
    await task.sleep(250)
    return 41
end

main()
    const start: int = time.now()
    const future: future[int] = delayed()
    const elapsed: int = time.duration_ms(start, time.now())
    if elapsed > 120
        io.print("blocked")
        return
    end
    const value: int = task.block_on(future)
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "async_call_returns_before_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_simple_async_state_machine_reads_await_value_native() {
    let dir = TestDir::new("compile_simple_async_state_machine_await_value_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async delayed() -> int
    await task.sleep(1)
    return 40
end

async compute() -> int
    const value: int = await delayed()
    return value + 1
end

main()
    const value: int = task.block_on(compute())
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "simple_async_state_machine_await_value");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_simple_async_state_machine_two_await_states_native() {
    let dir = TestDir::new("compile_simple_async_state_machine_two_await_states_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async left() -> int
    await task.sleep(1)
    return 20
end

async right() -> int
    await task.sleep(1)
    return 21
end

async compute() -> int
    const a: int = await left()
    const b: int = await right()
    return a + b
end

main()
    const value: int = task.block_on(compute())
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "simple_async_state_machine_two_await_states");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_simple_async_state_machine_scalar_param_native() {
    let dir = TestDir::new("compile_simple_async_state_machine_scalar_param_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async add_later(base: int) -> int
    await task.sleep(1)
    return base + 1
end

main()
    const value: int = task.block_on(add_later(40))
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "simple_async_state_machine_scalar_param");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_simple_async_state_machine_return_await_native() {
    let dir = TestDir::new("compile_simple_async_state_machine_return_await_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async delayed() -> int
    await task.sleep(1)
    return 41
end

async forward() -> int
    return await delayed()
end

main()
    const value: int = task.block_on(forward())
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "simple_async_state_machine_return_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_await_in_call_argument_native() {
    let dir = TestDir::new("compile_async_await_in_call_argument_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async delayed() -> int
    await task.sleep(1)
    return 40
end

add_one(value: int) -> int
    return value + 1
end

async compute() -> int
    return add_one(await delayed())
end

main()
    const value: int = task.block_on(compute())
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "async_await_in_call_argument");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_await_inside_operator_native() {
    let dir = TestDir::new("compile_async_await_inside_operator_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async left() -> int
    await task.sleep(1)
    return 20
end

async right() -> int
    await task.sleep(1)
    return 21
end

async compute() -> int
    return (await left()) + (await right())
end

main()
    const value: int = task.block_on(compute())
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "async_await_inside_operator");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_await_in_condition_native() {
    let dir = TestDir::new("compile_async_await_in_condition_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async flag() -> bool
    await task.sleep(1)
    return true
end

async main()
    if await flag()
        io.print("yes")
    end
end
"#,
    );

    let exe = exe_path(&dir, "async_await_in_condition");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "yes");
}

#[test]
fn compile_runs_simple_async_state_machine_managed_param_native() {
    let dir = TestDir::new("compile_simple_async_state_machine_managed_param_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async echo_later(text: string) -> string
    await task.sleep(1)
    return text
end

main()
    const text: string = task.block_on(echo_later("ori"))
    io.print(text)
end
"#,
    );

    let exe = exe_path(&dir, "simple_async_state_machine_managed_param");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "ori");
}

#[test]
fn compile_runs_simple_async_state_machine_managed_await_binding_native() {
    let dir = TestDir::new("compile_simple_async_state_machine_managed_await_binding_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async load_text() -> string
    await task.sleep(1)
    return "ori"
end

async compute() -> string
    const text: string = await load_text()
    return text
end

main()
    const text: string = task.block_on(compute())
    io.print(text)
end
"#,
    );

    let exe = exe_path(&dir, "simple_async_state_machine_managed_await_binding");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "ori");
}

#[test]
fn compile_runs_async_main_with_two_awaits_native() {
    let dir = TestDir::new("compile_async_main_two_awaits_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async left() -> int
    await task.sleep(1)
    return 20
end

async right() -> int
    await task.sleep(1)
    return 21
end

async main()
    const a: int = await left()
    const b: int = await right()
    io.print(string(a + b))
end
"#,
    );

    let exe = exe_path(&dir, "async_main_two_awaits");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_result_question_mark_native() {
    let dir = TestDir::new("compile_async_result_question_mark_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io

async compute() -> result[int, string]
    return success(41)
end

async use_value() -> result[int, string]
    const value: int = (await compute())?
    return success(value)
end

async main()
    const outcome: result[int, string] = await use_value()
    match outcome
        case success(value):
            io.print(string(value))
        case error(err):
            io.print(err)
    end
end
"#,
    );

    let exe = exe_path(&dir, "async_result_question_mark");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_result_question_mark_error_state_machine_native() {
    let dir = TestDir::new("compile_async_result_question_mark_error_state_machine_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io

async fail() -> result[int, string]
    return error("bad")
end

async use_value() -> result[int, string]
    const value: int = (await fail())?
    return success(value)
end

async main()
    const outcome: result[int, string] = await use_value()
    match outcome
        case success(value):
            io.print(string(value))
        case error(err):
            io.print(err)
    end
end
"#,
    );

    let exe = exe_path(&dir, "async_result_question_mark_error_state_machine");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "bad");
}

#[test]
fn compile_runs_async_state_machine_tail_control_flow_native() {
    let dir = TestDir::new("compile_async_state_machine_tail_control_flow_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async main()
    const values: list[int] = [1, 2, 3]
    var total: int = 0

    await task.sleep(1)

    var i: int = 0
    while i < 2
        io.print(string(values[i]))
        i = i + 1
    end

    for value in values
        total = total + value
    end

    match total
        case 6:
            io.print("six")
        case _:
            io.print("other")
    end
end
"#,
    );

    let exe = exe_path(&dir, "async_state_machine_tail_control_flow");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines, ["1", "2", "six"]);
}

#[test]
fn compile_runs_managed_collections_across_await_native() {
    let dir = TestDir::new("compile_managed_collections_across_await_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.map as maps
import ori.set as sets
import ori.task as task

async main()
    const label: string = "answer"
    const values: list[string] = ["first", "second"]
    const lookup: map[int, int] = { 1: 40, 2: 41 }
    const seen: set[int] = set { 3, 4 }

    await task.sleep(1)

    io.print(label)
    io.print(values[1])
    io.print(string(maps.get(lookup, 2)))
    if sets.contains(seen, 4)
        io.print("seen")
    end
end
"#,
    );

    let exe = exe_path(&dir, "managed_collections_across_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines, ["answer", "second", "41", "seen"]);
}

#[test]
fn compile_runs_managed_struct_across_await_native() {
    let dir = TestDir::new("compile_managed_struct_across_await_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

struct User
    name: string
end

async main()
    const user: User = User { name: "Ada" }

    await task.sleep(1)

    io.print(user.name)
end
"#,
    );

    let exe = exe_path(&dir, "managed_struct_across_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "Ada");
}

#[test]
fn compile_runs_managed_enum_payload_across_await_native() {
    let dir = TestDir::new("compile_managed_enum_payload_across_await_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

struct User
    name: string
end

enum Event
    Ready(user: User)
    Empty
end

async main()
    const event: Event = Event.Ready(user: User { name: "Ada" })

    await task.sleep(1)

    match event
        case Ready(user):
            io.print(user.name)
        case Empty:
            io.print("empty")
    end
end
"#,
    );

    let exe = exe_path(&dir, "managed_enum_payload_across_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "Ada");
}

#[test]
fn compile_runs_managed_closure_capture_across_await_native() {
    let dir = TestDir::new("compile_managed_closure_capture_across_await_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async main()
    const prefix: string = "value"
    const format: func(int) -> string = do(x: int) -> string
        const next: int = x + 1
        return prefix
    end

    await task.sleep(1)

    io.print(format(9))
end
"#,
    );

    let exe = exe_path(&dir, "managed_closure_capture_across_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "value");
}

#[test]
fn compile_runs_await_task_sleep_native() {
    let dir = TestDir::new("compile_await_task_sleep_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async compute() -> int
    await task.sleep(1)
    return 41
end

async main()
    const value: int = await compute()
    io.print(string(value))
end
"#,
    );

    let exe = exe_path(&dir, "await_task_sleep");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_async_fs_read_and_write_native() {
    let dir = TestDir::new("compile_async_fs_read_write_native");
    let input_path = dir.path("input.txt");
    let output_path = dir.path("output.txt");
    std::fs::write(&input_path, "from async fs").unwrap();
    let input = input_path.to_string_lossy().replace('\\', "/");
    let output = output_path.to_string_lossy().replace('\\', "/");
    dir.write(
        "main.orl",
        &format!(
            r#"module app.main

import ori.fs as fs
import ori.io as io

async main()
    const read_result: result[string, string] = await fs.read_text_async("{input}")
    const write_result: result[string, string] = await fs.write_text_async("{output}", "written async")
    match read_result
        case success(text):
            io.print(text)
        case error(err):
            io.print(err)
    end
    match write_result
        case success(_):
            io.print("write-ok")
        case error(err):
            io.print(err)
    end
end
"#
        ),
    );

    let exe = exe_path(&dir, "async_fs_read_write");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let process = Command::new(&exe).output().unwrap();
    assert!(process.status.success(), "{:?}", process);
    let stdout = String::from_utf8(process.stdout).unwrap();
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines, ["from async fs", "write-ok"]);
    assert_eq!(
        std::fs::read_to_string(output_path).unwrap(),
        "written async"
    );
}

#[test]
fn test_runner_accepts_async_test_functions() {
    let dir = TestDir::new("test_runner_async_test");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task
import ori.test as test

@test
async async_check()
    await task.sleep(1)
    test.assert(true, "async test should run")
end
"#,
    );

    let out = run_test(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert_eq!(out.results.len(), 1);
    assert!(out.results[0].passed, "{:?}", out.results[0].stderr);
}

#[test]
fn fmt_preserves_async_func_and_await_indentation() {
    let dir = TestDir::new("fmt_async_func_await");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

async main()
await task.sleep(1)
end
"#,
    );

    let out = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);
    assert!(
        out.formatted
            .contains("async main()\n    await task.sleep(1)\nend\n"),
        "{}",
        out.formatted
    );
}

#[test]
fn compile_runs_channel_atomic_and_block_on_native() {
    let dir = TestDir::new("compile_channel_atomic_block_on_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.atomic as atomic
import ori.channel as channel
import ori.io as io
import ori.task as task

main()
    const ch: channel.Channel[int] = channel.create()
    channel.send(ch, 29)
    match channel.receive(ch)
        case success(value):
            const counter: atomic.AtomicInt = atomic.new(value)
            io.print(string(atomic.add(counter, 12)))
        case error(_):
            io.print("receive-error")
    end
    channel.close(ch)
    task.block_on(task.sleep(1))
end
"#,
    );

    let exe = exe_path(&dir, "channel_atomic_block_on");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "41");
}

#[test]
fn compile_runs_transferable_collections_through_channel_native() {
    let dir = TestDir::new("collection_handles_through_channel_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.channel as channel
import ori.deque as deque
import ori.io as io
import ori.queue as queue

main()
    const q: queue.Queue[int] = queue.new()
    queue.enqueue(q, 5)
    const qch: channel.Channel[queue.Queue[int]] = channel.create()
    channel.send(qch, q)
    match channel.receive(qch)
        case success(received):
            match queue.dequeue(received)
                case some(value):
                    io.print(string(value))
                case none:
                    io.print("queue-empty")
            end
        case error(_):
            io.print("queue-error")
    end

    const d: deque.Deque[string] = deque.new()
    deque.push_back(d, "ok")
    const dch: channel.Channel[deque.Deque[string]] = channel.create()
    channel.send(dch, d)
    match channel.receive(dch)
        case success(received):
            match deque.front(received)
                case some(value):
                    io.print(value)
                case none:
                    io.print("deque-empty")
            end
        case error(_):
            io.print("deque-error")
    end
end
"#,
    );

    let exe = exe_path(&dir, "collection_handles_channel");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.replace("\r\n", "\n"), "5\nok\n");
}

#[test]
fn native_backend_accepts_async_await_shape_nested_await() {
    let dir = TestDir::new("native_async_nested_await");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io

async compute() -> int
    return 1
end

async main()
    if true
        const value: int = await compute()
        io.print(string(value))
    end
end
"#,
    );

    let exe = exe_path(&dir, "native_async_nested_await");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "1");
}

#[test]
fn c_backend_rejects_async_functions() {
    let dir = TestDir::new("c_backend_rejects_async_functions");
    dir.write(
        "main.orl",
        r#"module app.main

async main()
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let text = out
        .diagnostics
        .iter()
        .flat_map(|diagnostic| {
            std::iter::once(diagnostic.message.as_str())
                .chain(diagnostic.notes.iter().map(|note| note.as_str()))
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        text.contains("C backend does not support async functions yet; use the native backend"),
        "{text}"
    );
}

#[test]
fn c_backend_rejects_concurrency_runtime_calls() {
    let dir = TestDir::new("c_backend_rejects_concurrency");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.task as task

main()
    task.block_on(task.sleep(1))
end
"#,
    );

    let out = run_build(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    let text = out
        .diagnostics
        .iter()
        .flat_map(|diagnostic| {
            std::iter::once(diagnostic.message.as_str())
                .chain(diagnostic.notes.iter().map(|note| note.as_str()))
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        text.contains("C backend does not support concurrency/async runtime calls yet"),
        "{text}"
    );
}

#[test]
fn compile_runs_async_await_in_loop_and_branch_native() {
    let dir = TestDir::new("compile_async_await_in_loop_and_branch_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async compute(x: int) -> int
    await task.sleep(1)
    return x + 10
end

async test_branching_await(flag: bool) -> int
    if flag
        const a: int = await compute(5)
        return a
    else
        const b: int = await compute(15)
        return b
    end
end

async test_loop_await() -> int
    var sum: int = 0
    var i: int = 0
    while i < 3
        const val: int = await compute(i)
        sum = sum + val
        i = i + 1
    end
    return sum
end

main()
    const r1: int = task.block_on(test_branching_await(true))
    const r2: int = task.block_on(test_branching_await(false))
    const r3: int = task.block_on(test_loop_await())
    io.print(string(r1) + " " + string(r2) + " " + string(r3))
end
"#,
    );

    let exe = exe_path(&dir, "async_await_loop_branch");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "15 25 33"); // 5+10=15, 15+10=25, sum over i=0,1,2 of (i+10) = 10+11+12 = 33
}

#[test]
fn compile_runs_async_await_with_managed_variables_in_branch_native() {
    let dir = TestDir::new("compile_async_await_managed_in_branch_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async get_prefix(flag: bool) -> string
    await task.sleep(1)
    if flag
        return "yes"
    else
        return "no"
    end
end

async format_msg(flag: bool) -> string
    const prefix: string = await get_prefix(flag)
    if flag
        const msg: string = prefix + "-ok"
        await task.sleep(1)
        return msg
    else
        const msg2: string = prefix + "-fail"
        await task.sleep(1)
        return msg2
    end
end

main()
    const r1: string = task.block_on(format_msg(true))
    const r2: string = task.block_on(format_msg(false))
    io.print(r1 + " " + r2)
end
"#,
    );

    let exe = exe_path(&dir, "async_await_managed_branch");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    println!("STATUS: {:?}", output.status);
    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "yes-ok no-fail");
}

#[test]
fn compile_runs_async_using_dispose_native() {
    let dir = TestDir::new("compile_async_using_dispose_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

trait Disposable
    mut dispose(self)
end

var dispose_count: int = 0

struct Resource
    id: int
end

implement Disposable for Resource
    mut dispose(self)
        dispose_count = dispose_count + self.id
    end
end

async get_resource(id: int) -> Resource
    await task.sleep(1)
    return Resource {id: id}
end

async test_using()
    using r1: Resource = await get_resource(10)
    await task.sleep(1)
    if true
        using r2: Resource = await get_resource(20)
        await task.sleep(1)
    end
end

async test_using_early_return(flag: bool)
    using r1: Resource = await get_resource(100)
    if flag
        using r2: Resource = await get_resource(200)
        return
    end
    using r3: Resource = await get_resource(400)
end

main()
    task.block_on(test_using())
    task.block_on(test_using_early_return(true))
    io.print("disposed: " + string(dispose_count))
end
"#,
    );

    let exe = exe_path(&dir, "async_using_dispose");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "disposed: 330");
}

#[test]
fn compile_runs_async_using_dispose_on_cancel() {
    let dir = TestDir::new("compile_async_using_dispose_on_cancel");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

trait Disposable
    mut dispose(self)
end

var dispose_count: int = 0

struct Resource
    id: int
end

implement Disposable for Resource
    mut dispose(self)
        dispose_count = dispose_count + self.id
    end
end

async get_resource(id: int) -> Resource
    await task.sleep(1)
    return Resource {id: id}
end

async worker(token: task.CancelToken)
    using r: Resource = await get_resource(77)
    const fut: future[void] = task.sleep(5000)
    task.associate(token, fut)
    await fut
end

main()
    const token: task.CancelToken = task.create_token()
    const job: task.Job[void] = task.spawn(do() -> void
        task.block_on(worker(token))
    end)
    task.block_on(task.sleep(50))
    task.cancel(token)
    task.join(job)
    io.print("disposed: " + string(dispose_count))
end
"#,
    );

    let exe = exe_path(&dir, "async_using_dispose_cancel");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "disposed: 77");
}

#[test]
fn compile_runs_async_using_dispose_on_break() {
    let dir = TestDir::new("compile_async_using_dispose_on_break");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

trait Disposable
    mut dispose(self)
end

var dispose_count: int = 0

struct Resource
    id: int
end

implement Disposable for Resource
    mut dispose(self)
        dispose_count = dispose_count + self.id
    end
end

async get_resource(id: int) -> Resource
    await task.sleep(1)
    return Resource {id: id}
end

async worker()
    var total: int = 0
    while total < 100
        using r: Resource = await get_resource(10)
        total = total + 10
        if total >= 20
            break
        end
        await task.sleep(1)
    end
end

main()
    task.block_on(worker())
    io.print("disposed: " + string(dispose_count))
end
"#,
    );

    let exe = exe_path(&dir, "async_using_dispose_break");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "disposed: 20");
}

#[test]
fn compile_runs_async_file_using_dispose_on_cancel() {
    let dir = TestDir::new("compile_async_file_using_dispose_on_cancel");
    let test_file = dir
        .path("async_file.txt")
        .to_string_lossy()
        .replace('\\', "/");
    dir.write(
        "main.orl",
        &format!(
            r#"module app.main

import ori.fs as fs
import ori.io as io
import ori.bytes as bytes_mod
import ori.task as task

async worker(token: task.CancelToken, path: string) -> result[int, string]
    using file: fs.File = fs.open_write(path)?
    fs.write(file, b"ok")?
    const fut: future[void] = task.sleep(5000)
    task.associate(token, fut)
    await fut
    return success(0)
end

main()
    const path: string = "{test_file}"
    const token: task.CancelToken = task.create_token()
    const job: task.Job[void] = task.spawn(do() -> void
        task.block_on(worker(token, path))
    end)
    task.block_on(task.sleep(50))
    task.cancel(token)
    task.join(job)
    match fs.open_read(path)
        case success(file):
            match fs.read(file, 8)
                case success(data):
                    fs.close(file)
                    io.print(string(bytes_mod.len(data)))
                case error(err):
                    fs.close(file)
                    io.print("err:" + err)
            end
        case error(err):
            io.print("open-err:" + err)
    end
end
"#
        ),
    );

    let exe = exe_path(&dir, "async_file_using_dispose_cancel");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "2");
}

#[test]
fn compile_runs_async_await_in_match_native() {
    let dir = TestDir::new("compile_async_await_in_match_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async compute(x: int) -> int
    await task.sleep(1)
    return x + 1
end

async pick(flag: bool) -> int
    var out: int = 0
    match flag
        case true:
            out = await compute(10)
        case false:
            out = await compute(20)
    end
    return out
end

main()
    const left: int = task.block_on(pick(true))
    const right: int = task.block_on(pick(false))
    io.print(string(left) + " " + string(right))
end
"#,
    );

    let exe = exe_path(&dir, "async_await_in_match");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "11 21");
}

/// Matrix coverage: `for` loop with `await` inside the body. The async state
/// machine must lift the for-loop iterator state across the await point so
/// the loop resumes correctly after each await. This complements the existing
/// if/else, while, and match coverage to complete the
/// if/else/match/while/for async matrix.
#[test]
fn compile_runs_async_await_in_for_loop_native() {
    let dir = TestDir::new("compile_async_await_in_for_loop_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async compute(x: int) -> int
    await task.sleep(1)
    return x * x
end

async sum_squares(values: list[int]) -> int
    var total: int = 0
    for value in values
        const sq: int = await compute(value)
        total = total + sq
    end
    return total
end

main()
    const xs: list[int] = [1, 2, 3, 4]
    const outcome: int = task.block_on(sum_squares(xs))
    io.print(string(outcome))
end
"#,
    );

    let exe = exe_path(&dir, "async_await_in_for_loop");
    let out = run_compile(&dir.path("main.orl"), Path::new(&exe)).unwrap();
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    // 1*1 + 2*2 + 3*3 + 4*4 = 1 + 4 + 9 + 16 = 30
    assert_eq!(stdout.trim(), "30");
}

/// Regression: deeply nested await bodies must emit valid Cranelift SSA.
///
/// This covers `for { while { await ... } }`, where values used across the
/// suspension point must be reloaded from the async frame instead of reused from
/// a non-dominating pre-await block.
#[test]
fn compile_runs_async_await_in_deeply_nested_bodies_native() {
    let dir = TestDir::new("compile_async_await_deeply_nested_native");
    dir.write(
        "main.orl",
        r#"module app.main

import ori.io as io
import ori.task as task

async compute(x: int) -> int
    await task.sleep(1)
    return x + 1
end

async deeply_nested(limit: int) -> int
    var total: int = 0
    const xs: list[int] = [1, 2, 3]
    for value in xs
        var guard: int = 0
        while guard < limit
            total = total + await compute(value)
            guard = guard + 1
        end
    end
    return total
end

main()
    const r: int = task.block_on(deeply_nested(2))
    io.print(string(r))
end
"#,
    );

    let exe = exe_path(&dir, "async_await_deeply_nested");
    let out = match run_compile(&dir.path("main.orl"), Path::new(&exe)) {
        Ok(o) => o,
        Err(e) => panic!("compile error: {e}"),
    };
    assert!(!out.has_errors, "{:?}", out.diagnostics);

    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    // For value=1,2,3 with limit=2 (guard=0,1):
    //   value=1: compute(1)=2 twice -> 4; value=2: 3 twice -> 6; value=3: 4 twice -> 8
    // total = 4 + 6 + 8 = 18
    assert_eq!(stdout.trim(), "18");
}

/// Etapa 6.4 — formatter audit for async/concurrency constructs + idempotency.
/// Covers: `async func` + `await`, `task.spawn` with a closure, nested `using`
/// (a `using` inside a `match` arm that is itself inside a `using` scope), and
/// a multi-line `match`. Asserts the formatter produces correctly indented
/// output AND is idempotent (formatting the formatted output yields the same
/// text).
///
/// Note: a pre-existing formatter regression with `trait` declarations (the
/// trait's `end` is consumed by the first method signature, leaving subsequent
/// top-level items over-indented) is documented in PLANO Etapa 6.4 Known
/// Issues; this test avoids custom `trait`/`struct` definitions to keep the
/// audit focused on async/concurrency constructs.
#[test]
fn fmt_preserves_async_spawn_nested_using_and_multiline_match_idempotent() {
    let dir = TestDir::new("fmt_async_audit_idempotent");
    let source = r#"module app.main

import ori.fs as fs
import ori.io as io
import ori.task as task

async work(n: int) -> int
await task.sleep(1)
return n * 2
end

process_file(path: string) -> result[int, string]
using file: fs.File = fs.open_read(path)?
match fs.read(file, 100)
case success(data):
using copy: fs.File = fs.open_write(path)?
fs.write(copy, data)?
return success(10)
case error(msg):
return error(msg)
end
end

pick(flag: bool) -> int
match flag
case true:
return 1
case false:
return 2
end
end

main()
const job: task.Job[int] = task.spawn(do() => 41)
const r: int = task.block_on(work(task.join(job)))
io.print(string(r))
end
"#;
    dir.write("main.orl", source);

    let out = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(
        !out.has_errors,
        "fmt must parse without errors: {:?}",
        out.diagnostics
    );
    let once = out.formatted.clone();

    // Audit: async func + await + return at one indent level inside the func.
    assert!(
        once.contains(
            "async work(n: int) -> int\n    await task.sleep(1)\n    return n * 2\nend\n"
        ),
        "async func + await indentation: {once}"
    );
    // task.spawn with closure at 4-space indent inside main.
    assert!(
        once.contains("    const job: task.Job[int] = task.spawn(do() => 41)\n"),
        "task.spawn formatting: {once}"
    );
    // Nested using: `using file` at 4, match at 4, case arm at 4, `using copy`
    // (nested using) at 8 inside the case body. The formatter places `case`
    // labels at the same indent as `match` (switch/case style) with bodies one
    // level deeper.
    assert!(
        once.contains("    using file: fs.File = fs.open_read(path)?\n    match fs.read(file, 100)\n    case success(data):\n        using copy: fs.File = fs.open_write(path)?\n        fs.write(copy, data)?\n        return success(10)\n    case error(msg):\n        return error(msg)\n    end\n"),
        "nested using + match arm indentation: {once}"
    );
    // Multi-line match: `match` and `case` at 4, bodies at 8, `end` at 4.
    assert!(
        once.contains("    match flag\n    case true:\n        return 1\n    case false:\n        return 2\n    end\n"),
        "multi-line match indentation: {once}"
    );

    // Idempotency: formatting the already-formatted output must be a no-op.
    dir.write("main.orl", &once);
    let out2 = run_fmt(&dir.path("main.orl")).unwrap();
    assert!(
        !out2.has_errors,
        "second fmt must parse: {:?}",
        out2.diagnostics
    );
    assert_eq!(
        once, out2.formatted,
        "formatter must be idempotent (format(format(x)) == format(x))"
    );
}
