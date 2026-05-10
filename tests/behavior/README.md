# Behavior Tests

Projetos Zenith usados para validar comportamento observavel do MVP novo.

## Projetos atuais

- `simple_app/`: projeto valido. O entrypoint retorna `40 + 2`, logo o executavel deve sair com codigo `42`.
- `error_type_mismatch/`: projeto invalido. Deve falhar em semantica com span no arquivo `.zt`.
- `error_syntax/`: projeto invalido. Deve falhar no parser com span no arquivo `.zt`.

## Golden C

`simple_app/golden/simple-app.c` e o golden do C gerado pelo driver para o projeto `simple_app`.

O teste de conformance normaliza `CRLF` e `LF` antes de comparar, para manter o golden legivel e evitar ruido de plataforma.

## Build artifacts

Pastas `build/` dentro dos projetos de behavior sao saida de teste e ficam ignoradas por `.gitignore`.

- `control_flow_while/`: projeto valido. Exercita `while` no pipeline real ate o executavel.
- `control_flow_repeat/`: projeto valido. Exercita `repeat N times` no pipeline real ate o executavel.
- `control_flow_match/`: projeto valido. Exercita `match case/default` no pipeline real ate o executavel.
- `control_flow_break_continue/`: projeto valido. Exercita `break` e `continue` dentro de `while`.

- `functions_calls/`: projeto valido. Exercita chamadas diretas, recursao simples e retornos `bool`/`float`/`text`/`void`/`int`.

- `functions_named_args/`: projeto valido. Exercita parametros nomeados em ordem declarada.

- `functions_defaults/`: projeto valido. Exercita valores padrao de parametros.

- `functions_main_signature_error/`: projeto invalido. Deve falhar na validacao do entrypoint `main`.

- `functions_invalid_call_error/`: projeto invalido. Deve falhar em semantica por argumento ausente com span.
- `float_arithmetic_nested/`: projeto valido. Garante que aritmetica `float` aninhada no backend C preserva fracoes decimais em runtime.

- `structs_constructor/`: projeto valido. Exercita construcao de `struct` no pipeline real.

- `structs_field_defaults/`: projeto valido. Exercita defaults de campo em construtor de `struct`.

- `structs_field_read/`: projeto valido. Exercita leitura de campo de `struct`.

- `structs_field_update/`: projeto valido. Exercita atribuicao em campo de `struct`.

- `structs_with_expression/`: projeto valido. Exercita `with` expression para criar `struct` derivada com overrides parciais (`source with field: value`), preservando campos nao listados via copia de campo, sem mutar a fonte.

- `methods_inherent/`: projeto valido. Exercita metodo inerente via `apply Type`.

- `methods_mutating/`: projeto valido. Exercita metodo mutante com `mut func`.

- `methods_trait_apply/`: projeto valido. Exercita metodo de trait via `apply Trait to Type`.

- `list_basic/`: projeto valido. Exercita literal, indexacao 0-based e atualizacao de `list<int>`.
- `list_text_basic/`: projeto valido. Exercita literal, indexacao 0-based e atualizacao de `list<text>`.
- `list_struct_generic/`: projeto valido. Exercita `list<Struct>` pelo caminho generico do runtime para structs simples.
- `generic_helper_name_collision_safe/`: projeto valido. Exercita helpers genericos de list/set/map para structs com o mesmo nome simples em namespaces diferentes.
- `tuple_generated_struct_callbacks/`: projeto valido. Exercita `tuple<text, int>` como struct C gerada e como elemento de `list<tuple<...>>` com callbacks genericos.
- `extern_c_struct_arg_basic/`: projeto valido. Exercita `attr repr("c")` em struct passada por valor para um shim C linkado por `build.linker_flags`.
- `extern_c_struct_return_basic/`: projeto valido. Exercita retorno por valor de struct `attr repr("c")` vindo de um shim C linkado.
- `extern_c_const_basic/`: projeto valido. Exercita leitura de `extern c const` escalar definido por um shim C linkado.
- `extern_c_const_struct_basic/`: projeto valido. Exercita leitura de `extern c const` com struct `attr repr("c")`.
- `extern_c_const_managed_error/`: projeto invalido. Garante que `extern const` rejeita tipo gerenciado como `text`.
- `extern_c_callback_user_data_basic/`: projeto valido. Exercita callback C com `user_data` explicito e cleanup `using` dentro da funcao chamada via trampoline.
- `extern_c_target_const_basic/`: projeto valido. Exercita `attr target(...)` escolhendo uma constante externa por plataforma.
- `extern_c_target_unsupported_error/`: projeto invalido. Garante diagnostico para seletor de target desconhecido.
- `extern_c_struct_unannotated_error/`: projeto invalido. Garante que struct sem `attr repr("c")` nao cruza `extern c` por valor.
- `extern_c_struct_managed_field_error/`: projeto invalido. Garante que campo gerenciado, como `text`, nao entra em struct C-repr por valor.
- `group_removed_error/`: projeto invalido. Garante que `group<...>` foi removido e `tuple<...>` e o nome canonico.

- `list_dyn_trait_basic/`: projeto valido. Exercita `list<any<TextRepresentable>>` heterogenea com iteracao e `item.to_text()`.
- `dyn_trait_heterogeneous_collection/`: projeto valido. Exercita `list<any<Drawable>>` com literal heterogeneo, iteracao, slice, indexacao, `std.list.append`, `set` e dispatch por trait.
- `dyn_generic_trait_error/`: projeto invalido. Garante que `any<GenericTrait<T>>` explica por que a trait nao e dinamica e sugere generics com `where`.
- `map_basic/`: projeto valido. Exercita literal, indexacao por chave e atualizacao de `map<text,text>`.
- `map_empty_expected_type/`: projeto valido. Exercita mapa vazio `{}` com tipo esperado `map<int,bool>` em atribuicao e chamada de funcao, cobrindo o caminho `make_map<...>` do emitter C.
- `map_int_text_basic/`: projeto valido. Exercita literal, indexacao por chave `int`, `map_len` e atualizacao de `map<int,text>`.
- `map_struct_expected_type/`: projeto valido. Exercita mapa vazio `{}` com tipo esperado `map<int,Flag>` e garante emissao do `optional<Flag>` auxiliar exigido pelo helper gerado de `map`.
- `optional_result_basic/`: projeto valido. Exercita 
one`, `success` e `error`.

- `multifile_import_alias/`: projeto valido. Exercita varredura de `source.root` e chamada qualificada via import alias.
- `public_const_module/`: projeto valido. Exercita `public const` em nivel de modulo com import alias (`mod.CONST`).
- `public_var_module/`: projeto valido. Exercita leitura de `public var` em nivel de modulo via import alias (`mod.VAR`).
- `public_var_module_state/`: projeto valido. Exercita persistencia de estado de `public var` entre funcoes do modulo.
- `readability_block_depth_pass/`: projeto valido. Garante que `warning[style.block_too_deep]` nao bloqueia build em modo normal.
- `readability_block_depth_strict_error/`: projeto invalido em check. Garante que `[diagnostics] profile = "strict"` promove warning de profundidade de bloco para erro.
- `readability_enum_default_pass/`: projeto valido. Garante que `warning[control_flow.enum_default_case]` nao bloqueia build em modo normal.
- `readability_enum_default_strict_error/`: projeto invalido em check. Garante que `[diagnostics] profile = "strict"` promove warning de `case default` em enum conhecido para erro.
- `readability_function_length_pass/`: projeto valido. Garante que `warning[style.function_too_long]` nao bloqueia build em modo normal.
- `readability_function_length_strict_error/`: projeto invalido em check. Garante que `[diagnostics] profile = "strict"` promove warning de funcao longa para erro.
- `readability_warnings_pass/`: projeto valido. Garante que `warning[name.similar]` e `warning[name.confusing]` nao bloqueiam build em modo normal.
- `readability_warnings_strict_error/`: projeto invalido em check. Garante que `[diagnostics] profile = "strict"` promove warnings de legibilidade de nomes para erro.
- `optional_struct_qualified_managed/`: projeto valido. Exercita `optional<mod.Struct>` com nome qualificado entre modulos, retorno direto de `struct`, atribuicao/call-site com wrap implicito, campo opcional dentro de outra `struct` e isolamento de `list<text>` no payload.
- `public_var_cross_namespace_write_error/`: projeto invalido. Garante que `public var` nao pode ser mutado fora do namespace de origem.
- `closure_capture_basic/`: projeto valido. Exercita closure anonima com captura imutavel por valor.
- `closure_mut_capture_error/`: projeto invalido. Garante que closure v1 nao permite mutar variavel capturada.
- `lambda_hof_basic/`: projeto valido. Exercita lambda de expressao (`func(...) => expr`) com `map_int`, `filter_int`, `reduce_int` e captura imutavel.
- `nested_function_basic/`: projeto valido. Exercita `func` local dentro de outra funcao, chamada por nome e captura imutavel do escopo pai.
- `nested_function_mut_capture_error/`: projeto invalido. Garante que `func` local rejeita mutacao de variavel capturada do escopo pai.
- `list_hof_int_basic/`: projeto valido. Exercita HOFs de `std.list` para `list<int>`: map, filter, reduce, find, any, all e count.
- `list_hof_text_basic/`: projeto valido. Exercita HOFs de `std.list` para `list<text>`: map, filter, find, any, all, count e sort_by.
- `list_hof_bool_basic/`: projeto valido. Exercita HOFs de `std.list` para `list<bool>`: map, filter, find, any, all, count e sort_by.
- `list_map_cross_type_deferred_error/`: projeto invalido. Garante que `std.list.map<T,U>` fora do subset same-type falha com mensagem pos-RC.
- `list_reduce_value_hof_basic/`: projeto valido. Exercita `std.list.reduce<T,T>` para listas de `text`, `bool` e `float`.
- `list_reduce_cross_type_deferred_error/`: projeto invalido. Garante que `std.list.reduce<T,U>` fora do subset same-type falha com mensagem pos-RC.
- `std_collections_nested_managed_payload_error/`: projeto invalido. Garante que payloads gerenciados aninhados em `grid2d<T>` e `circbuf<T>` seguem rejeitados no subset runtime atual.
- `syntax_coherence_core/`: projeto valido. Exercita sintaxe 1G para `case some(name):`, `case else:`, `type` alias, `any<Trait>`, `func main()` sem retorno e closure de expressao unica.
- `syntax_coherence_inline_constraints/`: projeto valido. Exercita constraints inline `<T: Trait>` e trailing contextual `given T is Trait`.
- `lambda_return_mismatch_error/`: projeto invalido. Garante que lambda infere retorno pelo tipo `func(...) -> ...` esperado e rejeita retorno incompatível.
- `lazy_explicit_order_basic/`: projeto valido. Exercita `lazy<int>` explicito, garantindo que o thunk nao roda na criacao e roda no `force_int`.
- `lazy_generic_deferred_error/`: projeto invalido. Garante que `lazy<T>` fora de `int`, `float`, `bool` e `text` falha no `check` com mensagem pos-RC.
- `lazy_reuse_error/`: projeto invalido em runtime. Garante que `lazy<int>` e one-shot e rejeita segundo consumo.
- `std_math_nonfinite_policy/`: projeto valido. Exercita `std.math.nan()` e `std.math.infinity()` como funcoes, cobrindo construcao, igualdade IEEE, ordenacao de infinitos e formatacao.
- `std_math_nan_order_error/`: projeto invalido em runtime. Garante que comparacao ordenada envolvendo `NaN` falha com `runtime.float_nan_compare`.
- `std_random_basic/`: projeto valido. Exercita `std.random` baseline (`seed`, `next`, `between`) e valida o estado publico (`seeded`, `last_seed`, `draw_count`).
- `std_random_state_observability/`: projeto valido. Exercita leitura de estado publico de `std.random` e API `stats()`.
- `std_random_between_branches/`: projeto valido. Exercita ramos de `std.random.between` (`min == max` e `max < min`) sem consumo indevido de draw.
- `std_random_format_tier5/`: projeto valido. Exercita `random.float_between`, `random.choice/shuffle` para `list<int>` e `list<text>`, auto-derive `TextRepresentable` e expansao de `std.format`.
- `std_small_helpers/`: projeto valido. Exercita helpers pequenos de `std.validate`, `std.text`, `std.list` e `std.map`.
- `std_validate_broader/`: projeto valido. Exercita a expansao de `std.validate` para `float`, `bool`, estado de `optional`/`result`, listas e mapas suportados pelo backend atual.
- `list_value_api_basic/`: projeto valido. Exercita API value-style de `std.list` para `list<int>` e `list<text>`: append, prepend, contains, reverse, set, remove_*, slice, concat e index_of.
- `list_value_api_primitives/`: projeto valido. Exercita a mesma API value-style de `std.list` para listas primitivas especializadas (`float`, `bool`, `int8` e `u8`), incluindo `get`, `first`, `last`, `rest`, `skip`, `set`, `remove_*`, `slice`, `contains` e `index_of`.
- `std_mem_generic_facade_basic/`: projeto valido. Exercita `std.mem.own/view/edit` como fachada generica para `text`, `list<int>`, `list<float>`, `list<bool>`, `list<int8>`, `list<u8>` e `list<text>`.
- `std_mem_generic_facade_unsupported_type_error/`: projeto invalido. Garante erro claro quando `std.mem.own/view/edit` recebe tipos ainda fora do conjunto estabilizado.
- `std_mem_appendix_b_values/`: projeto valido. Exercita o recorte Appendix B de `std.mem.own/view/edit` para escalares primitivos, tuplas/structs seguras, `list<tuple>`, `list<struct>`, `set<int>`, `set<text>` e mapas primitivo/texto.
- `std_mem_appendix_b_deferred_type_error/`: projeto invalido. Garante que formatos com isolamento gerenciado ainda nao estabilizado continuam rejeitados com mensagem clara.
- `std_mem_appendix_b_nested_list_deferred_error/`: projeto invalido. Garante que `list<list<int>>` continua bloqueado ate haver isolamento recursivo de listas.
- `std_mem_appendix_b_set_list_key_deferred_error/`: projeto invalido. Garante que `set<list<int>>` continua bloqueado ate existir hash/equality estavel para chaves de colecao.
- `std_mem_appendix_b_set_tuple_key_deferred_error/`: projeto invalido. Garante que `set<tuple<int,text>>` continua bloqueado ate existir hash/equality estrutural para tuplas.
- `std_mem_appendix_b_map_key_deferred_error/`: projeto invalido. Garante que chaves de mapa por tupla continuam bloqueadas ate hash/equality estrutural.
- `std_mem_appendix_b_map_nested_value_deferred_error/`: projeto invalido. Garante que valores de mapa com listas recursivas continuam bloqueados ate isolamento profundo.
- `std_mem_appendix_b_managed_struct_deferred_error/`: projeto invalido. Garante que structs gerenciadas seguem bloqueadas ate existir isolamento profundo gerado.
- `std_mem_appendix_b_enum_payload_deferred_error/`: projeto invalido. Garante que enums com payload seguem bloqueados ate clone/edit gerado por variante.
- `std_mem_appendix_b_optional_payload_deferred_error/`: projeto invalido. Garante que optional/result payloads seguem bloqueados ate clone/edit gerado por payload.
- `map_value_api_basic/`: projeto valido. Exercita API value-style de `std.map` para `map<text,text>`: get, contains, set, remove, keys, values e merge.
- `map_value_api_generic/`: projeto valido. Exercita API value-style de `std.map` para `map<int,text>` e `map<text,int>`, cobrindo set, remove, keys, values, merge, get e has_key em mapas monomorfizados.
- `map_value_api_unsupported_key_error/`: projeto invalido. Garante que helpers value-style de `std.map` rejeitam chaves fora de `int/text` com mensagem clara.
- `map_struct_key_basic/`: projeto valido. Exercita `std.map` com `map<Struct,int>` seguro, cobrindo set, has_key/contains, remove e len com hash/equality gerados.
- `map_struct_unsupported_key_error/`: projeto invalido. Garante que `map<Struct,V>` rejeita campos de chave sem hash/equality materializados no backend.
- `set_struct_key_basic/`: projeto valido. Exercita `std.set` com `set<Struct>` seguro, cobrindo add, has, remove e len com hash/equality gerados.
- `set_struct_unsupported_key_error/`: projeto invalido. Garante que `set<Struct>` rejeita campos sem hash/equality materializados no backend.
- `orc_last_use_move_basic/`: projeto valido. Exercita o primeiro corte de ORC: atribuicao de ultimo uso de local gerenciado vira move no C gerado, sem `zt_retain`.
- `orc_last_use_no_move_after_alias/`: projeto valido. Garante que ORC preserva retain/copy quando o local gerenciado ainda e usado depois do alias.
- `orc_last_use_loop_backedge_no_move/`: projeto valido. Garante que ORC bloqueia move quando um loop volta para uma condicao que ainda usa o local gerenciado.
- `orc_last_use_branch_sibling_move/`: projeto valido. Garante que ORC permite move em um ramo quando os usos posteriores do local existem apenas em caminhos irmaos.
- `orc_field_last_use_move/`: projeto valido. Garante que ORC move campo gerenciado de struct quando nao ha uso posterior relevante, zerando o campo de origem.
- `orc_field_no_move_after_object_use/`: projeto valido. Garante que ORC preserva retain/copy quando o mesmo campo ainda e usado depois pelo objeto.
- `orc_field_move_other_field_later_use/`: projeto valido. Garante que ORC move um campo gerenciado mesmo quando outro campo do mesmo objeto e lido depois.
- `orc_sink_param_owned_transfer_basic/`: projeto valido. Garante que parametro sink recebe um owner retido quando o caller ainda usa o argumento.
- `orc_sink_param_last_use_arg_move/`: projeto valido. Garante que parametro sink recebe um argumento de ultimo uso sem retain e que o caller zera a origem depois da chamada.
- `orc_sink_param_return_move/`: projeto valido. Garante que parametro sink recebe argumento de ultimo uso em `return call(...)`, com a origem zerada antes do cleanup.
- `orc_sink_param_effect_move/`: projeto valido. Garante que parametro sink recebe argumento de ultimo uso em chamada standalone, com a origem zerada depois da chamada.
- `orc_sink_param_duplicate_source/`: projeto valido. Garante que ORC nao move duas vezes a mesma origem quando ela alimenta dois parametros sink; a segunda passagem recebe retain.
- `std_console_basic/`: projeto valido. Exercita `std.console` sem bloquear: linhas, deteccao, tamanho e leitura de tecla opcional.
- `std_debug_basic/`: projeto valido. Exercita `std.debug.size_of` e `std.debug.type_name` para tipos basicos, tupla, struct, enum, list, map e `any<Trait>`.

- `size_of_builtin_removed_error/`: projeto invalido. Garante que `size_of(...)` nao existe mais como builtin global.
- `to_text_builtin_basic/`: projeto valido. Exercita `to_text(value)` como builtin core para valores `TextRepresentable`.
- `type_conversions_basic/`: projeto valido. Exercita `std.int`, `std.float` e `std.bool` para conversoes explicitas e parse com `optional<T>`.
- `todo_builtin_fail/`: projeto invalido em runtime. Garante que `todo(message)` falha com mensagem clara.
- `unreachable_builtin_fail/`: projeto invalido em runtime. Garante que `unreachable(message)` falha com mensagem clara.
- `panic_stack_overflow/`: projeto invalido em runtime. Garante que recursao sem limite vira `runtime.panic` antes de estourar a pilha nativa.
- `check_intrinsic_message_fail/`: projeto invalido em runtime. Garante que `check(condition, message)` preserva a mensagem recebida.
- `noncanonical_*_error/`: projetos invalidos. Exercitam sugestoes action-first para sintaxe comum de outras linguagens (`string`, `let`, `&&`, `||`, `!`, `null`, `throw`, `abstract`, `virtual`, `union`, `partial`).
- `std_random_cross_namespace_write_error/`: projeto invalido. Garante que `std.random.draw_count` nao pode ser mutado fora do namespace `std.random`.
- `borealis_backend_fallback_stub/`: projeto valido em `run-pass`. Solicita backend desktop (`backend_id=1`) e valida fallback seguro para stub (janela + draw + leitura de input) quando adapter nao esta disponivel no ambiente.
- `borealis_raylib_binding_stub/`: projeto valido em `run-pass`. Exercita o modulo `borealis.raylib` no caminho real do compilador, cobrindo shapes, texto, input, `measure_text`, helpers de `raymath`/`reasings`, `require_available()`, validacao clara para caminho vazio de textura/som e comportamento stub-safe para draw de textura sem DLL nativa.
- `borealis_raylib_assets_real/`: projeto valido em `run-pass`. Quando Raylib nativa estiver disponivel, carrega uma textura `.png` e um som `.wav` reais, valida dimensoes/handle, exercita draw de textura, inicializacao de audio e `load_sound/play/stop/unload` ponta a ponta. Quando a DLL nao estiver presente, o probe fecha com sucesso sem forcar ambiente.
- `borealis_foundations_stub/`: projeto valido em `run-pass`. Exercita assets, events tipados, save, storage, services, database, UI/HUD, editor metadata e settings persistente, incluindo loaders tipados de assets, metadata logica, ids estaveis, conflito claro de `kind` por chave, perfis de settings em `storage`, widgets de interface e persistencia de string vazia sem confundir com item removido.
- `borealis_ecs_hybrid_stub/`: projeto valido em `run-pass`. Exercita o subset inicial de componentes do ECS (`borealis.engine.ecs`) com stub autocontido para runtime atual.
- `borealis_runtime_gameplay_stub/`: projeto valido em `run-pass`. Exercita os modulos de runtime/jogabilidade do Borealis (`contracts`, `entities`, `movement`, `controllers`, `vehicles`, `animation`, `audio`, `ai`, `camera`, `input`, `world` e `procedural`) em um fluxo integrado.
- `multifile_missing_import/`: projeto invalido. Deve falhar quando um import nao existe em `source.root`.
- `multifile_namespace_mismatch/`: projeto invalido. Deve falhar quando 
amespace` nao corresponde ao caminho do arquivo.
- `multifile_duplicate_symbol/`: projeto invalido. Deve falhar quando dois arquivos geram o mesmo simbolo efetivo no programa agregado.
- `multifile_private_access/`: projeto invalido. Deve falhar quando um simbolo sem `public` e acessado via import alias.
- `monomorphization_limit_error/`: projeto invalido. Deve falhar no gate de monomorfizacao quando `build.monomorphization_limit` ficar abaixo das instancias genericas reais.
- `monomorphization_function_limit_error/`: projeto invalido. Deve falhar quando especializacoes de funcoes genericas excedem `build.monomorphization_limit`.
- `generic_arg_inference_basic/`: projeto valido em `check` e `build`. Exercita inferencia direta de `T` a partir do argumento posicional.
- `generic_monomorphization_nested_call/`: projeto valido em `check` e `build`. Exercita inferencia por tipo composto (`list<T>`), especializacao transitiva (`generic` chamando `generic`) e passagem entre funcoes genericas com nomes de parametro diferentes (`T` -> `U`).
- `generic_monomorphization_text_basic/`: projeto valido. Exercita retorno de `T` especializado para `text`, garantindo retain/cleanup correto para valores gerenciados.
- `monomorphization_many_instances_basic/`: projeto valido. Exercita varias especializacoes concretas de funcoes genericas e listas abaixo de um limite explicito.
- `generic_arg_inference_missing_error/`: projeto invalido. Garante que `T` nao pode ser inferido apenas pelo retorno e pede tipo explicito.
- `generic_arg_inference_conflict_error/`: projeto invalido. Garante erro claro quando dois argumentos tentam inferir tipos diferentes para o mesmo `T`.
- `match_guard_basic/`: projeto valido. Exercita `case ... given ...` em `match`, incluindo build nativo e execucao do caminho guardado.
- `match_guard_non_bool_error/`: projeto invalido. Garante que a guarda de `match` precisa ser `bool`.
- `const_destructuring_basic/`: projeto valido. Exercita `const (a, b) = expr` com tupla, formatter, build nativo e execucao.
- `const_destructuring_non_tuple_error/`: projeto invalido. Garante que destruturacao em `const` exige inicializador de tupla.
- `const_destructuring_arity_error/`: projeto invalido. Garante que a quantidade de nomes precisa bater com a aridade da tupla.
- `multivalue_match_basic/`: projeto valido. Exercita `match (a, b)` com padroes de tupla e comparacao por elemento no backend C.
- `multivalue_match_type_error/`: projeto invalido. Garante erro claro quando um padrao de tupla usa tipo incompativel.
- `operator_overloading_level2_basic/`: projeto valido. Exercita `Addable`, `Subtractable` e `Comparable` como unico nivel aceito de overload para `+`, `-`, `<`, `<=`, `>` e `>=`.
- `operator_overloading_missing_trait_error/`: projeto invalido. Garante que operador em tipo de usuario exige o trait central correspondente.
- `pipe_operator_basic/`: projeto valido. Exercita `value |> f |> g(extra)` como composicao esquerda-para-direita por chamadas normais.
- `pipe_operator_non_callable_error/`: projeto invalido. Garante erro claro quando o lado direito de `|>` nao e chamavel.
- `where_contracts_ok/`: projeto valido. Exercita contratos `where` em parametro, construcao de `struct` e atribuicao de campo.
- `where_contract_param_error/`: projeto invalido em runtime. Deve falhar com `error[runtime.contract]` por violacao de contrato em parametro.
- `where_contract_construct_error/`: projeto invalido em runtime. Deve falhar com `error[runtime.contract]` por violacao de contrato em construcao de `struct`.
- `where_contract_field_assign_error/`: projeto invalido em runtime. Deve falhar com `error[runtime.contract]` por violacao de contrato em atribuicao de campo.
- `std_net_basic/`: projeto valido. Exercita `std.net` no baseline atual via loopback TCP local. O script `run-loopback.ps1` sobe um servidor local em `127.0.0.1:41234`, executa o binario e fecha o listener automaticamente.
- `std_http_basic/`: projeto valido. Exercita `std.http` no baseline HTTP v1 com GET e POST via servidor local em `127.0.0.1:41235`, sem depender de rede externa.
- `std_jobs_text_basic/`: projeto valido. Exercita `Job<text>` com `jobs.spawn`/`jobs.join`, incluindo chamada sem argumento e chamada com payload `text`.
- `std_channels_text_basic/`: projeto valido. Exercita `Channel<text>` com `channels.create`, `send`, `receive`, `close` e retorno `optional<text>`.
- `std_shared_text_type_error/`: projeto invalido. Garante diagnostico claro para `Shared<text>` enquanto `std.shared` executavel suporta somente `Shared<int>`.
- `std_atomic_bool_type_error/`: projeto invalido. Garante diagnostico claro para `Atomic<bool>` enquanto `std.atomic` executavel suporta somente `Atomic<int>`.
- `std_os_args_basic/`: projeto valido. Exercita `std.os.args()` com `argv[0]` e com argumentos encaminhados pelo driver depois de `zt run ... -- <args>`.
- `std_collections_managed_arc/`: projeto valido. Exercita copy/mutate isolation em `grid2d<text>`, `pqueue<text>`, `circbuf<text>`, `btreemap<text,text>`, `btreeset<text>` e `grid3d<text>`.
- `std_collections_queue_stack_cow/`: projeto valido. Exercita `queue/stack` com retorno estruturado em `dequeue/pop`, preservando isolamento apos copia compartilhada.
- `std_collections_unsupported_generic_shape_error/`: projeto invalido. Garante que `grid2d`, `pqueue`, `circbuf` e `btreemap` rejeitam shapes genericas fora do subconjunto v1 com mensagem clara.
- `std_collections_values_iteration/`: projeto valido. Exercita `queue_values<T>`/`stack_values<T>` para listas genericas e snapshots `values`/`keys` com ordem definida para grid, pqueue, circbuf, btreemap e btreeset.
- `edge_boundaries_empty/`: projeto valido. Exercita valores-limite (`u8/u16/u32/u64`, `int` proximo ao limite) e estruturas vazias (`text/list/map/bytes`).
- `std_test_basic/`: projeto valido. Exercita `std.test` diretamente via `main`, validando os desfechos de `skip(...)` e `fail(...)` no comando `zt test`.
- `std_test_attr_pass_skip/`: projeto valido. Exercita o harness real de `zt test` com funcoes marcadas por `attr test`, cobrindo 1 caso pass e 1 caso skip.
- `std_test_attr_fail/`: projeto invalido para o runner. Exercita o harness real de `zt test` com funcoes marcadas por `attr test`, cobrindo 1 caso pass, 1 caso skip e 1 caso fail.
- `attributes_v1/`: projeto valido. Exercita `attr deprecated("...")`, `attr todo("...")` e `attr skip("...")` no check e no runner de testes.
- `std_test_helpers_pass/`: projeto valido. Exercita `is_true`, `is_false`, `equal_int`, `equal_text`, `not_equal_int` e `not_equal_text` no caminho feliz.
- `std_test_helpers_bool_fail/`: projeto invalido em runtime. Congela a mensagem de falha de `is_true(false)`.
- `std_test_helpers_equal_fail/`: projeto invalido em runtime. Congela a mensagem esperado/recebido de `equal_int(actual, expected)`.
- `std_test_helpers_not_equal_fail/`: projeto invalido em runtime. Congela a mensagem de `not_equal_text` quando os textos sao iguais.
- `std_test_throws_pass/`: projeto valido. Exercita `test.throws(...)` com funcao nomeada e closure inline que chamam `panic(...)`.
- `std_test_throws_fail/`: projeto invalido em runtime. Congela a mensagem de `test.throws(...)` quando o corpo nao falha.

- `enum_match/`: fixture de comportamento para enum com payload + match com binding de payload (check semantico OK; build E2E bloqueado pelo stub de lowering HIR->ZIR no source atual).
- `enum_match_non_exhaustive_error/`: fixture invalida para diagnostico de match nao exaustivo em enum conhecido.

