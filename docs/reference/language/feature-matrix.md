# Language Documentation Feature Matrix

> Audience: maintainer
> Status: current release matrix
> Surface: reference
> Source of truth: no

## Objetivo

Rastrear se a documentacao publica e de referencia cobre o que a RC promete.

Esta matriz nao mede ambicao futura. Ela responde a uma pergunta simples:

**um usuario da RC consegue entender o contrato atual sem ler relatorios
historicos?**

## Fonte normativa

Quando houver conflito, use esta ordem:

1. `docs/spec/language/final-language-contract.md`
2. `docs/spec/language/zenith-language-spec.md`
3. specs de topico em `docs/spec/language/`
4. testes em `tests/behavior/MATRIX.md`
5. esta matriz

## Status permitido

- `release-covered`: coberto o suficiente para a RC.
- `reference-covered`: coberto na referencia; pode ganhar guia melhor depois.
- `contract-limited`: implementado/testado, mas com limites que precisam ficar explicitos.
- `post-RC`: fora da promessa da RC.

## Matriz atual

| Feature | Evidencia atual | Docs atuais | Status | Follow-up |
| --- | --- | --- | --- | --- |
| Forma de arquivo | `syntax_coherence_core`, parser/check root | `docs/public/language/language-reference.md`, `docs/reference/grammar/syntax.md` | `release-covered` | manter exemplos pequenos |
| Namespace/import | fixtures multifile/import | `docs/reference/language/modules-and-visibility.md` | `release-covered` | nenhum bloqueio |
| `const`/`var` | fixtures de mutabilidade e `public var` | `docs/public/language/language-reference.md`, `docs/reference/language/modules-and-visibility.md` | `release-covered` | nenhum bloqueio |
| `optional<T>` | `optional_result_basic`, `optional_question_basic`, helpers | `docs/reference/language/errors-and-results.md`, `docs/reference/language/types.md` | `release-covered` | manter `or_error` fora da promessa atual |
| `result<T,E>` | `result_question_basic`, `result_or_wrap_basic` | `docs/reference/language/errors-and-results.md`, `docs/reference/language/types.md` | `release-covered` | manter `or_panic` fora da promessa atual |
| `enum` com payload | `enum_match` e fixtures negativas | `docs/reference/language/types.md`, specs de linguagem | `reference-covered` | adicionar guia publico dedicado pos-RC |
| `trait`/`apply` | `methods_trait_apply`, apply/inherent methods | `docs/public/learn/learn-zenith-in-30-minutes.md`, `docs/reference/language/types.md` | `release-covered` | nenhum bloqueio |
| `any<Trait>` | `list<any<...>>`, dyn trait fixtures | `docs/public/language/language-reference.md`, `docs/reference/language/types.md` | `contract-limited` | reforcar limites de objeto seguro em guia pos-RC |
| funcoes genericas | `generic_arg_inference_basic`, `generic_monomorphization_nested_call`, `generic_monomorphization_text_basic`, `monomorphization_*` | specs de linguagem e matrix de testes | `release-covered` | manter exemplos curtos para inferencia e limite de monomorfizacao |
| callables/delegates | closures, HOFs, `list_reduce_value_hof_basic`, `list_map_cross_type_deferred_error`, `list_reduce_cross_type_deferred_error` e callable diagnostics | specs e `docs/reference/language/expression-readability.md` | `reference-covered` | criar pagina curta de callables pos-RC |
| closures/lambdas | `closure_capture_basic`, `lambda_hof_basic`, nested funcs | specs e matrix de testes | `reference-covered` | criar exemplos publicos pos-RC |
| collections/generic keys | `map_*`, `set_*`, `map_struct_key_basic`, `set_struct_key_basic`, `generic_helper_name_collision_safe`, `std_collections_managed_arc`, `std_collections_unsupported_generic_shape_error`, `std_collections_nested_managed_payload_error` | `docs/reference/stdlib/collections.md`, specs de linguagem | `contract-limited` | manter limites de `keys/values/merge`, payloads aninhados e colecoes avancadas explicitos |
| FFI callbacks | `extern_c_callback_basic`, `extern_c_callback_user_data_basic`, `extern_c_callback_closure_error`, `extern_c_callback_signature_error` | `docs/spec/language/ffi.md` | `contract-limited` | manter claro que so refs top-level imediatas cruzam; capturas continuam fora |
| FFI C-repr structs | `extern_c_struct_arg_basic`, `extern_c_struct_return_basic`, `extern_c_struct_unannotated_error`, `extern_c_struct_managed_field_error` | `docs/spec/language/ffi.md` | `contract-limited` | manter explicito que so structs nao genericas com campos FFI-safe cruzam por valor |
| FFI extern const | `extern_c_const_basic`, `extern_c_const_struct_basic`, `extern_c_const_managed_error` | `docs/spec/language/ffi.md` | `contract-limited` | manter explicito que so globals C de leitura com tipos FFI-safe entram nesta versao |
| FFI target attrs | `extern_c_target_const_basic`, `extern_c_target_unsupported_error` | `docs/spec/language/ffi.md` | `contract-limited` | deixar claro que seleciona itens, mas nao resolve bibliotecas nem linker flags |
| Runtime ownership / ORC | `orc_*`, `std_collections_managed_arc`, `using_basic`, `using_disposable_auto`, `using_panic_cleanup` | `docs/spec/language/runtime-model.md`, `docs/spec/language/post-v1-runtime-abi-ownership-audit.md` | `contract-limited` | manter explicito que ARC/ORC e detalhe de runtime; ciclo amplo fica fora ate existir API publica formadora de ciclo |
| `std.mem` ownership facade | `std_mem_generic_facade_basic`, `std_mem_appendix_b_values`, `std_mem_appendix_b_*_deferred_error` | `docs/spec/language/runtime-model.md`, `docs/spec/language/stdlib-reference-by-topic.md`, `docs/public/stdlib/stdlib-reference.md` | `contract-limited` | manter lista de formatos aceitos e rejeicoes Appendix B visivel |
| `lazy<T>` | `lazy_explicit_order_basic`, `lazy_primitive_text_basic`, `lazy_generic_deferred_error`, `lazy_reuse_error` | `docs/reference/stdlib/concurrency-lazy-test-net.md` | `contract-limited` | helpers executaveis sao especializados; `lazy<T>` generico fica pos-RC |
| `where` contracts | `where_contracts_ok` e fixtures negativas | specs, diagnostics, tests | `reference-covered` | adicionar exemplo publico pequeno pos-RC |
| formatter | driver tests e `zt fmt --check` | `docs/public/packages/tooling-guide.md`, `docs/reference/cli/zt.md` | `release-covered` | nenhum bloqueio |
| diagnostics | strict/profile fixtures e catalogo CLI | `docs/reference/diagnostics/cli-diagnostics.md`, `docs/public/packages/tooling-guide.md` | `release-covered` | manter exemplos de erro curtos |

## Lacunas que nao bloqueiam a RC

Estas melhorias sao desejaveis, mas nao impedem a RC se o contrato acima ficar
visivel nas release notes:

- guia publico dedicado para closures/lambdas/callables;
- guia publico dedicado para `where` contracts;
- exemplos extras de `any<Trait>` com limites de objeto seguro;
- exemplos extras de `enum` com payload.

## Historico de atualizacao

- 2026-04-25: matriz inicial criada com cobertura aproximada.
- 2026-05-08: matriz atualizada para refletir a RC local e separar docs
  suficientes para release de melhorias pos-RC.
- 2026-05-10: adicionada cobertura explicita para chaves estruturais seguras
  em `set` e `map`.
- 2026-05-10: adicionada cobertura explicita para `std.list.reduce<T,T>` no
  subconjunto primitivo/text.
- 2026-05-10: adicionada cobertura explicita para structs `attr repr("c")`
  cruzando `extern c` por valor.
- 2026-05-10: adicionada cobertura explicita para `extern const` com tipos
  FFI-safe.
- 2026-05-10: adicionada cobertura explicita para `attr target(...)` em
  itens `extern`.
- 2026-05-10: adicionada cobertura explicita para callbacks FFI com
  `user_data` explicito.
- 2026-05-10: adicionada cobertura explicita para ownership runtime,
  cleanup deterministico e fachada `std.mem` Appendix B.
