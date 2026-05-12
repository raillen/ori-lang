# ARC Implementation Plan — Ori Language

## Current State

The ARC (Automatic Reference Counting) infrastructure is **scaffolded but not implemented**. All hooks are no-ops. The compiler already emits retain/release calls at the correct points, but the runtime stubs do nothing.

### What already works

| Feature | Status |
|---|---|
| `using`/dispose cleanup | ✅ Implemented (both backends) |
| `is_managed_ty()` classification | ✅ All heap types identified |
| `managed_stack` tracking in native backend | ✅ Pushes/pops at scope boundaries |
| `emit_arc_retain_if_managed` calls | ✅ Emitted for: `let` bindings, function args, closure captures, `break`/`continue`/`return` cleanup |
| `emit_arc_release_if_managed` calls | ✅ Emitted at scope exit (reverse order) |
| `emit_arc_collect_cycles` calls | ✅ Emitted at function scope exit |
| `ori_arc_retain` / `ori_arc_release` | ✅ Implemented with proper atomic refcounting |
| `ori_closure_t` struct | ✅ Handles closure captures properly in C backend |
| C backend `managed_stack` tracking | ✅ Implemented |

### What is missing

| Gap | Impact |
|---|---|
| No cycle detection in `ori_arc_collect_cycles` | Cyclic references would leak even with ARC |

---

## Managed Types

Defined in `native_backend.rs:253-269` (`is_managed_ty`):

```
String, Bytes, List<T>, Map<K,V>, Set<T>, Range<T>,
Optional<T>, Result<T,E>, Tuple(...), Named(DefId),
Any<Trait>, Func{...}
```

All of these are **pointer-sized** in the Cranelift backend (`ptr_ty`). The actual data lives on the heap.

---

## Phase 1: Runtime — Refcounted Allocation Header (✅ Completed)

**File:** `compiler/crates/ori-runtime/src/lib.rs`

### 1.1 Define the allocation header

Every managed heap object gets a common header prepended:

```rust
#[repr(C)]
struct OriHeapHeader {
    refcount: i64,       // atomic reference count
    destructor: unsafe extern "C" fn(*mut u8), // type-specific cleanup
}
```

### 1.2 Allocation helper

```rust
unsafe fn ori_alloc(size: usize, destructor: unsafe extern "C" fn(*mut u8)) -> *mut u8 {
    let total = size + std::mem::size_of::<OriHeapHeader>();
    let ptr = libc::malloc(total) as *mut u8;
    if !ptr.is_null() {
        let header = ptr as *mut OriHeapHeader;
        (*header).refcount = 1;
        (*header).destructor = destructor;
        ptr.add(std::mem::size_of::<OriHeapHeader>())
    } else {
        ptr
    }
}
```

### 1.3 Implement `ori_arc_retain`

```rust
#[no_mangle]
pub unsafe extern "C" fn ori_arc_retain(ptr: *mut u8) {
    if ptr.is_null() { return; }
    let header = ptr.sub(std::mem::size_of::<OriHeapHeader>()) as *mut OriHeapHeader;
    // Atomic increment — use core::sync::atomic::AtomicI64 or libc atomics
    (*header).refcount += 1; // TODO: make atomic
}
```

### 1.4 Implement `ori_arc_release`

```rust
#[no_mangle]
pub unsafe extern "C" fn ori_arc_release(ptr: *mut u8) {
    if ptr.is_null() { return; }
    let header = ptr.sub(std::mem::size_of::<OriHeapHeader>()) as *mut OriHeapHeader;
    (*header).refcount -= 1; // TODO: make atomic
    if (*header).refcount <= 0 {
        let destructor = (*header).destructor;
        if let Some(dtor) = destructor {
            dtor(ptr); // type-specific cleanup (free nested managed values)
        }
        libc::free(header as *mut libc::c_void);
    }
}
```

### 1.5 Type-specific destructors

Each managed type needs a destructor that releases nested managed values:

| Type | Destructor behavior |
|---|---|
| `String` | `libc::free(ptr)` — no nested refs |
| `List<T>` | For each element: if T is managed, `ori_arc_release(elem)`; then `libc::free(data)` + `free(list)` |
| `Map<K,V>` | For each key/value: if managed, release; then `free(keys)` + `free(values)` + `free(map)` |
| `Set<T>` | Same as List |
| `Optional<T>` | If `has_value`: release inner T; then `free(optional)` |
| `Result<T,E>` | Release the active variant's payload; then `free(result)` |
| `Tuple(...)` | Release each managed element; then `free(tuple)` |
| `Named(DefId)` | Call `ORI__{Type}_dispose` if exists; then `free(struct)` |
| `Any<Trait>` | Release `data_ptr`; `free(vtable)`; `free(any)` |
| `Func{...}` (closure) | Release `env_ptr` if non-null; `free(closure)` |
| `Range<T>` | No nested refs; just `free(range)` |

**Important:** Destructors are type-erased function pointers stored in the header. The compiler backend must pass the correct destructor at allocation time.

---

## Phase 2: Native Backend (Cranelift) — Allocation Changes (✅ Completed)

**File:** `compiler/crates/ori-codegen/src/native_backend.rs`

### 2.1 Replace `malloc_bytes` with `ori_alloc`-equivalent

Currently `malloc_bytes(size)` calls `malloc(size)`. Change to call a new runtime function:

```rust
fn alloc_managed(&mut self, size: u32, destructor_name: &str) -> Result<ir::Value, String> {
    // Call ori_alloc(size, destructor_fn_ptr)
    // Returns pointer to data (past the header)
}
```

Or simpler: keep `malloc_bytes` but add a second step that initializes the header. The key is that the returned pointer must point **past** the header so existing code that stores fields at known offsets still works.

**Alternative (simpler):** Store the header at a **negative offset** from the data pointer. All existing field offset calculations remain unchanged. Only the allocator and retain/release need to know about the header.

### 2.2 Update all allocation sites

Every `self.malloc_bytes(...)` call that allocates a managed type must be updated:

| Expression | Location (approx line) |
|---|---|
| `None_` | ~3087 |
| `Some_(inner)` | ~3100 |
| `Ok_(inner)` | ~3116 |
| `Err_(inner)` | ~3132 |
| `StructLit` | ~3325 |
| `EnumVariant` | ~3446 |
| `TupleLit` | ~3484 |
| `Range` | ~3434 |
| `Closure` (closure object) | ~1321 |
| `Any` (any box) | ~1351, ~1379 |

Each needs to pass the correct destructor function reference.

### 2.3 Declare destructor functions in `declare_stdlib`

Add declarations for each type-specific destructor as imported functions. These are implemented in the runtime (Phase 1.5).

### 2.4 Verify retain/release call sites

The compiler already emits retain/release at the right places. Verify:

- `let x = <managed-expr>` → retain (line ~1984)
- Function args (managed) → retain (line ~3250, ~3254)
- Closure captures → retain (line ~1219 area — currently only pushes to managed_stack, no retain call)
- `break`/`continue` → scope cleanup releases (line ~2032, ~2042)
- `return` → scope cleanup + retain return value (line ~1961-1963)
- `?` propagation → retain error + scope cleanup (line ~3293-3294)
- Assignment `x = y` → retain new, release old (line ~1997-1998)

**Bug to fix:** Closure capture prologue (`emit_closure_capture_prologue`, line ~1195-1227) pushes to `managed_stack` but does **not** call `emit_arc_retain_if_managed`. The capture values come from the environment struct — they should be retained since the closure now holds a reference.

---

## Phase 3: C Backend — ARC Support (✅ Completed)

**File:** `compiler/crates/ori-codegen/src/c_backend.rs`

### 3.1 Add `managed_stack` to `CCodegen`

```rust
struct CCodegen {
    // ... existing fields ...
    managed_stack: Vec<(String, Ty)>,
}
```

### 3.2 Add `ori_arc_retain` / `ori_arc_release` to embedded runtime

**File:** `compiler/crates/ori-driver/src/pipeline.rs` (`ORI_RUNTIME_C` constant)

Replace the no-op stubs with real implementations (mirroring Phase 1).

### 3.3 Emit retain/release in C codegen

Mirror the native backend logic:

- `HirStmt::Let` with managed type → `ori_arc_retain({var});` + push to `managed_stack`
- `HirStmt::Assign` with managed type → `ori_arc_retain(new); ori_arc_release(old);`
- Function args (managed) → retain before call
- Scope exit → release in reverse order from `managed_stack`
- `return` → retain return value + scope cleanup
- `break`/`continue` → scope cleanup

### 3.4 Closure captures — retain in C backend

Currently line ~1126-1128, closure captures are shallow-copied:

```c
env_tmp->cap_name = cap_name;
```

Add retain for managed captures:

```c
env_tmp->cap_name = cap_name;
ori_arc_retain(cap_name);  // if managed
```

And add a destructor for the env struct that releases all managed captures.

### 3.5 Add `ori_closure_t` destructor

When freeing a closure, release `env_ptr` if non-null (calling the env struct's destructor).

---

## Phase 4: Cycle Detection (✅ Completed)

**File:** `compiler/crates/ori-runtime/src/lib.rs`

### 4.1 Implement `ori_arc_collect_cycles`

For now, a simple mark-and-sweep or deferred cycle detection is acceptable. Options:

**Option A (simple):** Keep a global registry of all managed allocations. Periodically scan for cycles using a mark-and-sweep pass. This requires a stop-the-world pause.

**Option B (deferred):** Do nothing for now. Cycle detection can be added later. Most Ori programs will not create reference cycles (no mutable shared state by default).

**Recommendation:** Start with Option B. The `ori_arc_collect_cycles` stub returning 0 is fine for initial implementation. Add cycle detection in a follow-up.

---

## Phase 5: Testing (✅ Completed)

### 5.1 Unit tests for runtime

Add tests in `ori-runtime` that verify:

- `ori_arc_retain` increments refcount
- `ori_arc_release` decrements and frees at 0
- Destructor is called on free
- Nested managed values are released by destructors

### 5.2 Integration tests

Add `.ori` test files:

```
// test_basic_arc.ori
let s = "hello"
let t = s  // retain
// both s and t point to same string, refcount = 2
// at scope exit, both released, string freed once
```

```
// test_list_arc.ori
let a = [1, 2, 3]
let b = a  // retain list
// scope exit releases both
```

```
// test_closure_arc.ori
let msg = "hello"
let f = fn() { ori.io.print(msg) }  // closure captures msg
// msg refcount: 1 (original) + 1 (capture) = 2
// f holds a reference to msg
```

### 5.3 Valgrind / leak sanitizer

Run compiled programs under Valgrind or with `-fsanitize=leak` to verify zero leaks.

---

## Implementation Order

| Phase | Effort | Dependencies |
|---|---|---|
| Phase 1: Runtime header + retain/release | Medium | None |
| Phase 1.5: Type-specific destructors | Large | Phase 1 |
| Phase 2: Native backend allocation changes | Medium | Phase 1, 1.5 |
| Phase 3: C backend ARC support | Large | Phase 1, 1.5 |
| Phase 4: Cycle detection | Deferred | — |
| Phase 5: Testing | Medium | Phase 1-3 |

**Recommended MVP:** Phase 1 + Phase 2 (native backend only) + basic String/List destructors. This covers the most common use cases and eliminates leaks for the primary backend.

---

## Key Design Decisions

1. **Header at negative offset**: The `OriHeapHeader` lives immediately before the data pointer. This means `ptr - sizeof(Header)` gets the header. All existing field offset calculations in the compiler remain valid.

2. **Atomic refcount**: Use `core::sync::atomic::AtomicI64` (or `AtomicI32`) for thread safety. Even though Ori is currently single-threaded, this future-proofs the runtime.

3. **Type-erased destructors**: Each allocation stores a function pointer to its destructor. The compiler backend is responsible for passing the correct destructor at allocation time. This avoids the need for a vtable in every object.

4. **C backend parity**: The C backend must eventually support ARC to avoid leaks in C-compiled programs. However, the native (Cranelift) backend can ship first.

5. **`using`/dispose vs ARC**: These are complementary. `using` ensures a resource is cleaned up at a specific scope exit (like RAII). ARC handles shared ownership. A `using` variable still participates in ARC — `dispose` is called at scope exit, but the underlying memory is only freed when refcount hits 0.
