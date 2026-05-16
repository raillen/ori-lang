# Native async state machine design

Status: implemented subset contract for the native backend.

Date: 2026-05-14.

Update: 2026-05-16.

The native backend now generates a native async state-machine subset:

- the public `async func` returns `future<T>` immediately;
- the wrapper allocates a frame with `state`, result `future`, awaited future
  slots, parameters, simple pre-await locals and await bindings;
- the generated internal `step` function dispatches by state;
- each supported `await` uses `ori_future_poll`, `ori_future_value_*` and
  `ori_future_on_ready`;
- pending futures register a continuation instead of blocking;
- failed and cancelled future states propagate to the result future;
- managed params, locals and bindings stored in the frame are retained through
  ARC frame edges, released by per-`await` liveness when dead, and cleaned on
  normal return, `?`, failed future and cancelled future paths.

Supported source shapes are sequential `await value`, `const x: T = await
value`, `return await value`, a final void expression, tail `if`/`while`/`for`/
`match` without nested `await` or inner `return`, and the narrow `const x =
(await value)?` form when the awaited `result<T, E>` exactly matches the async
function's returned `result<T, E>`.

Shapes outside this subset now fail with `backend.native_unsupported` before
Cranelift. The backend no longer falls back to legacy async body spawn for
source-level `await`, and `emit_await` must not call `ori_task_block_on*`.
`task.block_on` remains only an explicit sync bridge used by user code, async
`main`, and the test harness entry point.

## Goal

`async func` must compile to a native state machine. Calling an async function
must create a `future<T>` and return quickly. The body must advance only when
the executor polls or schedules its continuation.

The runtime foundation already provides:

- pollable futures;
- a FIFO executor queue;
- `future.on_ready` style continuation scheduling;
- a non-blocking timer for `task.sleep`;
- private failed/cancelled future states.

The implemented compiler path generates the async frame and continuation
function for the supported v1 subset. Broader nested-control-flow lowering can
extend this contract later without reintroducing synchronous `await` fallback.

## Async frame

Each `async func f(a: A, b: B) -> T` gets an internal frame:

```text
AsyncFrame_f
    state: int
    result: future<T>
    param_a: A
    param_b: B
    live locals that cross await
    live temporaries that cross await
```

Rules:

- Parameters are copied into the frame at call time.
- Locals that do not cross an `await` stay normal native locals.
- Managed values stored in the frame are retained when written, released when
  liveness shows they are dead after an `await`, and also released when the
  frame completes, fails or is cancelled.
- Temporaries are promoted to frame fields only when they are live after an
  `await`.

## States

The generated state enum is numeric and private to the backend:

```text
0 = start
1..N = resume after await N
9000 = completed
9001 = failed
9002 = cancelled
```

Each `await` becomes:

```text
poll awaited future
if ready:
    read value and continue
if failed:
    fail result future and cleanup frame
if cancelled:
    cancel result future and cleanup frame
if pending:
    store next state
    register continuation with awaited future
    return
```

## Return type

Source syntax stays readable:

```ori
async func load() -> string
```

Call type remains:

```text
future<string>
```

Inside the async body, `return value` completes the frame's result future with
`value`. The generated outer function returns the future handle immediately.

## `result<T, E>` and `?`

`await` does not hide domain errors.

```ori
const value: int = (await read())?
```

Lowering order:

1. Await `future<result<int, E>>`.
2. If the future failed/cancelled at runtime, fail/cancel the frame future.
3. If the future is ready with `error(e)`, propagate `error(e)` through the
   async function's declared `result`.
4. If ready with `success(v)`, continue with `v`.

## Diagnostics

Already enforced:

- `async.await_outside_async`
- `async.await_non_future`
- `async.capture_not_transferable`
- `async.using_unsupported`
- C backend async/concurrency rejection

Current implementation rules:

- reject async frames whose live locals cannot be represented safely;
- reject unsupported `using` if cleanup across suspension is not implemented;
- keep every unsupported backend path under `backend.native_unsupported`.

## Runtime hooks used by codegen

The state machine uses these internal runtime hooks:

- `ori_future_poll`
- `ori_future_value_i64`
- `ori_future_value_f64`
- `ori_future_value_ptr`
- `ori_future_pending`
- `ori_future_on_ready`
- `ori_future_complete_i64`
- `ori_future_complete_f64`
- `ori_future_complete_ptr`
- `ori_future_complete_void`
- `ori_future_fail`
- `ori_future_cancel`
- `ori_executor_schedule`
- `ori_executor_run_one`
- `ori_executor_drain`

These are internal ABI helpers, not public stdlib functions.

`ori_future_pending` is the constructor used by the public async wrapper before
the first `step` is scheduled. Pointer futures retain registered ARC payloads
while the future is alive, so generated state machines can complete `future<T>`
with managed values without losing ownership during frame cleanup.
