# Embed Ori as a shared library (`ori compile --lib`)

Hosts (Godot GDExtension, Python, C engines) can `dlopen` an Ori **cdylib**
and call functions marked `@c_export`.

## Language

```orl
module app.embed_add

@c_export
public add_scores(a: int, b: int) -> int
    return a + b
end

@c_export("mul_scores")
public mul(a: int, b: int) -> int
    return a * b
end
```

- Only `public` free functions.
- Phase 1 types: `int`, `float`, `bool`, void return.
- Optional rename: `@c_export("symbol_name")`.

## Compile

```bash
# Needs a staged runtime with cdylib (libori_runtime.so):
#   sh tools/stage_native_runtime.sh --profile release
export ORI_USE_SYSTEM_LINKER=1   # recommended for --lib on Linux
ori compile --lib examples/embed/add_scores.orl -o libadd_scores.so
```

The library **dynamically** depends on `libori_runtime.so` (same triple under
`runtime/<triple>/`). Keep that directory on `LD_LIBRARY_PATH` / `rpath` /
next to the host binary.

## Host contract

```c
void *h = dlopen("libadd_scores.so", RTLD_NOW);
int  (*ori_rt_init)(void) = dlsym(h, "ori_rt_init");
void (*ori_rt_shutdown)(void) = dlsym(h, "ori_rt_shutdown");
void (*ori_module_init)(void) = dlsym(h, "__ori_module_init"); // optional globals
int64_t (*add_scores)(int64_t, int64_t) = dlsym(h, "add_scores");

ori_rt_init();
if (ori_module_init) ori_module_init();
printf("%lld\n", (long long)add_scores(2, 3)); // 5
ori_rt_shutdown();
```

## Smoke test

```bash
sh tools/qa/embed_smoke.sh
```

## Phases (see `docs/planning/PLANO-CDYLIB-EMBED.md`)

| Phase | Status |
|-------|--------|
| P1 `--lib` + `@c_export` scalars + `ori_rt_*` | **done** |
| P2 strings (ptr+len) | planned |
| P3 host→Ori callbacks | planned |
| P4 Godot GDExtension example | planned |
| P5 Windows / macOS | planned |
