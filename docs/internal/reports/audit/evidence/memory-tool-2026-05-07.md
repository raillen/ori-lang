# Memory Tool Evidence - 2026-05-07

Etapa: 4 - Rodar Valgrind ou ferramenta equivalente.

Nota de atualizacao: em 2026-05-08, Valgrind ficou disponivel no WSL e foi
executado como evidencia primaria. Ver
`docs/internal/reports/audit/evidence/valgrind-2026-05-08.md`.

## Resultado

- Status final: passou.
- Ferramenta escolhida: LeakSanitizer + UBSan via GCC/ASAN em WSL.
- Justificativa: `valgrind` nao estava disponivel no WSL usado para a validacao; LeakSanitizer cobre vazamentos de heap e UBSan cobre comportamento indefinido relevante para esta rodada.
- Comando final:

```bash
python3 tests/hardening/test_runtime_memory_tool.py
```

- Stdout final:

```text
[OK] runtime_core
[OK] collections_runtime
[OK] arc_shared_runtime
[OK] thread_boundary_copy
[OK] stdlib_bytes
[OK] std_collections_basic
[OK] std_collections_managed_arc
[OK] arc_value_semantics
[OK] orc_last_use_move
[OK] fuzz_replay
{"driver": "replay", "seeds": 0, "failures": 0, "failed_names": []}
memory tool corpus ok
```

## Ambiente usado

- Sistema: WSL2 Ubuntu em Linux `6.6.87.2-microsoft-standard-WSL2`.
- Arquitetura: `x86_64`.
- Python: `Python 3.12.3`.
- Compilador: `gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0`.
- Valgrind: indisponivel no ambiente usado.

## Corpus minimo

- Runtime core: `runtime/c/zenith_rt.c` + `tests/runtime/c/test_runtime.c`.
- Stdlib core: `tests/behavior/std_bytes_ops`.
- `std.collections`: `tests/runtime/c/test_collections_generic.c`, `tests/behavior/std_collections_basic`, `tests/behavior/std_collections_managed_arc`.
- ARC/ORC: `tests/runtime/c/test_shared_text.c`, `tests/behavior/value_semantics_arc_isolation`, `tests/behavior/orc_last_use_move_basic`.
- Boundary/concurrency ownership: `tests/runtime/c/test_thread_boundary_copy.c`.
- Fuzz replay: `tests/fuzz/replay.py --timeout 8`.

## Flags usadas

```text
-D_POSIX_C_SOURCE=200809L
-D_DEFAULT_SOURCE
-DZT_MAX_STACK_SIZE=16777216
-std=gnu11
-g
-fno-omit-frame-pointer
-fsanitize=address
-fsanitize=undefined
-lm
ASAN_OPTIONS=detect_leaks=1:halt_on_error=1:abort_on_error=1
LSAN_OPTIONS=exitcode=23:report_objects=1
UBSAN_OPTIONS=halt_on_error=1:print_stacktrace=1
```

## Achados e classificacao

1. Aliases retidos no harness C nao eram liberados.
   - Tipo: vazamento real no teste/harness.
   - Correcao: `tests/runtime/c/test_runtime.c` agora libera aliases extras depois de operacoes `set_owned`.

2. Clones de `grid2d<text>` e `grid3d<text>` vazavam valores padrao criados por `_new`.
   - Tipo: vazamento real no runtime.
   - Correcao: `runtime/c/zenith_rt_templates.h` agora libera o valor padrao antes de sobrescrever slots clonados em `set_owned` e `fill_owned`.

3. Retornos gerenciados usados dentro de `zt_text_len(...)` nao eram liberados no C emitido.
   - Tipo: vazamento real no emissor C.
   - Correcao: `compiler/targets/c/emitter.c` agora materializa argumentos gerenciados temporarios tambem para chamadas externas `c.*` em expressoes, chama a funcao e libera o temporario depois.

4. Atualizacao copy-on-write de mapas podia perder a referencia antiga em atribuicoes diretas.
   - Tipo: vazamento real no emissor C.
   - Correcao: `compiler/targets/c/emitter.c` agora guarda o mapa antigo, chama `set_owned`, libera o antigo quando a operacao retorna nova instancia e so entao atualiza o destino.

## Validacao complementar

- `python build.py`: passou.
- `python -m py_compile tests/hardening/test_runtime_memory_tool.py tests/hardening/test_runtime_sanitizers.py`: passou.
- `python3 tests/hardening/test_runtime_sanitizers.py` no WSL: passou.
- `python tools/check_docs_paths.py`: passou.

## Decisao

Etapa 4 desbloqueada para prosseguir. A ferramenta equivalente passou sem vazamentos relevantes e sem UB relevante no corpus minimo definido para RC publica.
