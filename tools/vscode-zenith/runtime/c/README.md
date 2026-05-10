# Zenith Next Runtime C

Esta pasta concentra o runtime C do Zenith Next.

Arquivos ativos agora:

- `zenith_rt_manifest.h`: lista canonica da unity source e dos componentes que invalidam o cache do runtime
- `zenith_rt.h`: contrato inicial de ABI do runtime
- `zenith_rt.c`: unity source do runtime C; inclui os componentes `.c` internos em ordem fixa
- `zenith_rt_core.c`, `zenith_rt_outcome.c`, `zenith_collections_generic.c`, `zenith_collections_rt.c`,
  `zenith_rt_memory.c`, `zenith_rt_host.c`, `zenith_rt_format.c`, `zenith_rt_path.c`,
  `zenith_rt_math.c`, `zenith_rt_scalar.c`, `zenith_rt_random.c`, `zenith_rt_encoding.c`,
  `zenith_rt_json.c`, `zenith_rt_net.c`, `zenith_rt_http.c`, `zenith_rt_borealis.c`,
  `zenith_rt_dyn.c`:
  componentes internos compilados via `zenith_rt.c`

Subset implementado neste corte:

- tipos base `zt_int`, `zt_float`, `zt_bool`
- header comum de heap com RC
- `zt_retain` e `zt_release`
- `zt_runtime_error`, `zt_runtime_error_ex`, `zt_runtime_error_with_span`, `zt_runtime_report_error`, `zt_check`, `zt_panic`
- texto: `zt_text_from_utf8`, `zt_text_concat`, `zt_text_index`, `zt_text_slice`, `zt_text_eq`, `zt_text_len`
- lista especializada: `zt_list_i64_new`, `zt_list_i64_from_array`, `zt_list_i64_push`, `zt_list_i64_get`, `zt_list_i64_set`, `zt_list_i64_len`, `zt_list_i64_slice`
- lista especializada: `zt_list_text_new`, `zt_list_text_from_array`, `zt_list_text_push`, `zt_list_text_get`, `zt_list_text_set`, `zt_list_text_len`, `zt_list_text_slice`
- mapa especializado: `zt_map_text_text_new`, `zt_map_text_text_from_arrays`, `zt_map_text_text_set`, `zt_map_text_text_get`, `zt_map_text_text_len`
- `Optional<int>`: `zt_optional_i64_present`, `zt_optional_i64_empty`, `zt_optional_i64_is_present`, `zt_optional_i64_coalesce`
- `Optional<text>`: `zt_optional_text_present`, `zt_optional_text_empty`, `zt_optional_text_is_present`, `zt_optional_text_coalesce`
- `Optional<list<int>>`: `zt_optional_list_i64_present`, `zt_optional_list_i64_empty`, `zt_optional_list_i64_is_present`, `zt_optional_list_i64_coalesce`
- `Optional<list<text>>`: `zt_optional_list_text_present`, `zt_optional_list_text_empty`, `zt_optional_list_text_is_present`, `zt_optional_list_text_coalesce`
- `Optional<map<text,text>>`: `zt_optional_map_text_text_present`, `zt_optional_map_text_text_empty`, `zt_optional_map_text_text_is_present`, `zt_optional_map_text_text_coalesce`
- `Outcome<int,text>`: `zt_outcome_i64_text_success`, `zt_outcome_i64_text_failure`, `zt_outcome_i64_text_is_success`, `zt_outcome_i64_text_value`, `zt_outcome_i64_text_propagate`
- `Outcome<void,text>`: `zt_outcome_void_text_success`, `zt_outcome_void_text_failure`, `zt_outcome_void_text_is_success`, `zt_outcome_void_text_propagate`
- `Outcome<text,text>`: `zt_outcome_text_text_success`, `zt_outcome_text_text_failure`, `zt_outcome_text_text_is_success`, `zt_outcome_text_text_value`, `zt_outcome_text_text_propagate`
- `Outcome<list<int>,text>`: `zt_outcome_list_i64_text_success`, `zt_outcome_list_i64_text_failure`, `zt_outcome_list_i64_text_is_success`, `zt_outcome_list_i64_text_value`, `zt_outcome_list_i64_text_propagate`
- `Outcome<list<text>,text>`: `zt_outcome_list_text_text_success`, `zt_outcome_list_text_text_failure`, `zt_outcome_list_text_text_is_success`, `zt_outcome_list_text_text_value`, `zt_outcome_list_text_text_propagate`
- `Outcome<map<text,text>,text>`: `zt_outcome_map_text_text_success`, `zt_outcome_map_text_text_failure`, `zt_outcome_map_text_text_failure_message`, `zt_outcome_map_text_text_is_success`, `zt_outcome_map_text_text_value`, `zt_outcome_map_text_text_propagate`

Compatibilidade semantica atual:

- texto segue indices publicos 0-based
- `texto[i]` materializa novo `text` owned
- `slice` usa fim inclusivo
- `slice(..., -1)` significa ate o fim
- `list<int>` usa container owned com indices publicos 0-based
- `list<text>` retem elementos internamente e devolve `text` owned em `index_seq`
- `map<text,text>` usa container owned com busca linear no MVP, `map[key]` devolve `text` owned e `map_set` retem chave e valor internamente
- `Optional<text>` e `Optional<list<int>>` sao heap-managed para simplificar ownership no backend C
- `Outcome<int,text>`, `Outcome<void,text>`, `Outcome<text,text>`, `Outcome<list<int>,text>`, `Outcome<list<text>,text>` e `Outcome<map<text,text>,text>` sao heap-managed para simplificar ownership no backend C
- `zt_runtime_error_info` guarda kind, mensagem, codigo opcional e span opcional para diagnostico estruturado
- a boundary host minima do runtime C expoe `zt_host_read_file`, `zt_host_write_stdout` e `zt_host_write_stderr`, com override por `zt_host_set_api`
- Borealis desktop hook: `zt_borealis_desktop_api`, `zt_borealis_set_desktop_api` e `zt_borealis_get_desktop_api` com adapter Raylib inicial por carga dinamica (fallback para stub quando ausente)

Estado atual do runtime:

- `compiler/driver/pipeline.c` usa `zenith_rt_manifest.h` para achar a fonte principal
  e para invalidar `.ztc-tmp/runtime/zenith_rt.o` quando qualquer componente muda
- `zenith_rt_core.c` isola glue base: overflow, erro estruturado e registro de heap dinamico
- `zenith_rt_memory.c` isola o scaffolding de pool/validacao de memoria que antes ficava
  inline dentro de `zenith_rt.c`
- `zenith_rt_path.c` isola helpers publicos `zt_path_*` sem mudar ABI
- `zenith_rt_math.c` isola helpers publicos de `std.math`, aritmetica segura
  `zt_*_i64` e `zt_validate_between_i64` sem mudar ABI
- `zenith_rt_host.c` isola host API, fs, console, tempo, OS e processo
- `zenith_rt_format.c`, `zenith_rt_scalar.c`, `zenith_rt_random.c` e
  `zenith_rt_encoding.c` isolam helpers de stdlib sem mudar ABI
- `zenith_rt_dyn.c` isola dispatch dinamico e `list<dyn>`
- a ABI C cobre toda a superficie heap-managed que o compilador/target C expoe hoje
- erros runtime agora podem carregar span e codigo opcionais
- a boundary host minima ja esta presente para arquivo e streams
- novas specializations futuras passam a ser expansao de superficie da linguagem, nao gap do runtime atual
