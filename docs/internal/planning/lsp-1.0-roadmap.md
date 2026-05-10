# Compass LSP 1.0 Roadmap

Status: active
Created: 2026-04-28
Scope: LSP server, VSCode extension, tests, docs, and release packaging.

Este roadmap define o que ainda falta para chamar o Compass LSP de 1.0.
O objetivo e sair de "beta local funcional" para "ferramenta confiavel para uso diario".

## Estado atual

O LSP beta ja cobre o fluxo principal de edicao:

- diagnostics em tempo real;
- semantic tokens usando o lexer oficial;
- completion global, local, por import, por membro e por builtin;
- hover e go-to-definition para simbolos top-level e locais;
- references e rename para simbolos top-level e locais;
- signature help para chamadas diretas, `alias.func(...)`, builtins e metodos `apply`;
- workspace index em memoria;
- integracao local com a extensao VSCode.

## Criterio de 1.0

O LSP 1.0 so deve fechar quando estes pontos estiverem verdes:

- nenhum P0/P1 conhecido em edicao normal;
- smoke suite do LSP passa em Windows e Linux;
- VSCode Extension Development Host validado manualmente;
- pacote `.vsix` gerado e instalado localmente;
- docs publicas explicam instalacao, uso e limites reais;
- recursos marcados como 1.0 tem testes automatizados.

## Fase 1 - Escopo local real

Objetivo: fazer hover, definition, references e rename concordarem sobre escopos locais.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.01 | Criar modelo simples de escopo por funcao, bloco, closure, `using`, `for` e `match` | done |
| LSP1.02 | Garantir shadowing correto: simbolo interno vence simbolo externo | done |
| LSP1.03 | References para parametros locais | done |
| LSP1.04 | References para `var`, `const`, `using` e bindings de `match` | done |
| LSP1.05 | Rename para parametros locais | done |
| LSP1.06 | Rename para `var`, `const`, `using` e bindings de `match` | done |
| LSP1.07 | Testes de nao-renomear simbolos fora do escopo | done |
| LSP1.08 | Testes de shadowing com nomes repetidos | done |

Definition of done:

- rename local nao altera strings, comentarios ou simbolos homonimos fora do escopo;
- references locais e top-level usam a mesma regra de resolucao;
- falhas retornam resposta vazia segura, nao edits perigosos.

## Fase 2 - Semantica compartilhada com o compilador

Objetivo: reduzir logica paralela no LSP e reutilizar o conhecimento do compilador.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.09 | Expor uma API interna de analise para o LSP consultar simbolos e tipos | done |
| LSP1.10 | Trocar inferencia ad hoc por resultados do binder/checker quando disponiveis | done |
| LSP1.11 | Manter fallback leve quando o arquivo esta incompleto ou com erro parcial | done |
| LSP1.12 | Usar diagnosticos do checker sem duplicar mensagens confusas | done |
| LSP1.13 | Resolver tipos genericos simples em completion de membro | done |
| LSP1.14 | Resolver aliases de tipo em completion, hover e signature help | done |
| LSP1.15 | Testes para arquivo incompleto durante digitacao | done |

Definition of done:

- `zt check` e LSP nao discordam em casos basicos de tipo;
- completion continua funcionando enquanto o usuario digita codigo incompleto;
- erros aparecem com mensagem curta, acionavel e sem jargao interno.

## Fase 3 - Completion 1.0

Objetivo: deixar completion previsivel, sem ruido e cobrindo a linguagem atual.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.16 | Completion de membro para structs genericas simples | done |
| LSP1.17 | Completion de membro para aliases de tipos estruturais | done |
| LSP1.18 | Completion para variantes de enum em contexto de tipo conhecido | done |
| LSP1.19 | Completion para patterns de `match` com enum/result/optional | done |
| LSP1.20 | Completion para `import` com ranking por proximidade de namespace | done |
| LSP1.21 | Completion snippets revisados para sintaxe atual completa | done |
| LSP1.22 | Reduzir sugestoes globais em contextos de membro e import | done |

Definition of done:

- completion sugere menos itens, mas mais corretos;
- cada keyword atual tem snippet ou sugestao quando fizer sentido;
- itens experimentais nao aparecem como se fossem estaveis.

## Fase 4 - Signature Help e Hover

Objetivo: transformar informacao de simbolo em ajuda legivel dentro do editor.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.23 | Signature Help para chamadas com named arguments | done |
| LSP1.24 | Signature Help para metodos `apply` com receiver resolvido | done |
| LSP1.25 | Definir politica para overload futura: mostrar todas ou escolher melhor candidato | done |
| LSP1.26 | Hover com origem do simbolo: local, importado, stdlib ou builtin | done |
| LSP1.27 | Hover com tipo inferido quando o checker souber informar | done |
| LSP1.28 | Markdown de hover curto, com exemplo apenas quando util | done |

Definition of done:

- hover responde em texto curto e escaneavel;
- signature help mostra parametro ativo corretamente;
- mensagens continuam acessiveis para leitura rapida.

Politica para overload futura:

- O LSP 1.0 nao inventa candidatos de overload.
- Enquanto o checker resolver para uma unica declaracao, Signature Help mostra essa declaracao.
- Quando o checker expuser uma lista real de candidatos viaveis, o LSP deve mostrar todas as assinaturas viaveis e destacar a melhor candidata.
- Se a chamada for ambigua, o LSP deve preferir uma resposta curta e honesta em vez de sugerir uma assinatura possivelmente errada.

Politica para hover:

- Hover deve abrir com um bloco curto de codigo: assinatura, tipo ou declaracao.
- A segunda linha deve explicar a origem: local, importado, stdlib, builtin ou workspace.
- Exemplos nao entram por padrao. Eles so devem aparecer quando reduzem ambiguidade real.

## Fase 5 - Semantic Tokens e TextMate

Objetivo: alinhar destaque visual com a sintaxe real, sem manter duas linguagens paralelas.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.29 | Garantir cobertura de tokens para todas as keywords atuais | done |
| LSP1.30 | Marcar namespaces, tipos, funcoes, propriedades e variaveis com consistencia | done |
| LSP1.31 | Adicionar testes de semantic tokens para novas keywords | done |
| LSP1.32 | Revisar grammar TextMate como fallback visual | done |
| LSP1.33 | Validar tema claro e escuro no VSCode | done |

Definition of done:

- lexer oficial e grammar TextMate nao divergem em keywords estaveis;
- tokens semanticamente errados sao tratados como bug, nao como detalhe visual.

Notas de validacao visual:

- A grammar TextMate foi revisada contra as keywords atuais do lexer.
- O fallback visual nao define cores proprias. Ele usa scopes padrao do VSCode, entao funciona com temas claro e escuro sem acoplar paleta.
- O fallback visual cobre `with`, `f"..."`, builtins atuais como `print()` e `read()`, e `lazy<T>` como tipo reconhecido pelo checker.

## Fase 6 - Robustez do servidor

Objetivo: o LSP nao pode travar, vazar estado ruim ou degradar muito em workspace real.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.34 | Testar arquivos grandes e workspace com muitos `.zt` | done |
| LSP1.35 | Medir tempo de resposta de completion, hover e diagnostics | done |
| LSP1.36 | Melhorar reindexacao incremental de create/change/delete | done |
| LSP1.37 | Garantir cancelamento/ignorar respostas obsoletas quando o documento muda rapido | done |
| LSP1.38 | Harden JSON-RPC: mensagens invalidas nao derrubam o processo | done |
| LSP1.39 | Testes de shutdown/exit e ciclo de vida do processo | done |

Definition of done:

- o servidor aguenta edicao rapida sem respostas antigas sobrescreverem estado novo;
- falhas de parse parcial nao derrubam features independentes;
- logs ajudam a debugar sem poluir o usuario final.

Evidencia automatizada:

- Smoke de robustez abre 30 arquivos `.zt` com varias declaracoes cada.
- Completion e hover sao medidos com limite de 5s por request no teste.
- Diagnostics carregam `version`; a extensao ignora diagnostics antigos.
- `didClose` remove simbolos do workspace indexado.
- Payload JSON-RPC invalido nao derruba o servidor.
- Shutdown/exit e encerramento do processo sao validados.

## Fase 7 - VSCode Extension 1.0

Objetivo: transformar a extensao local em pacote instalavel e validado.

Decisao de ordem:

- Nao empacotar antes de fechar as funcoes faltantes da extensao.
- O `.vsix` deve ser artefato de validacao, nao uma forma de esconder pendencias.
- Primeiro a extensao precisa ter comandos, configuracoes, mensagens de erro e fluxos basicos coerentes.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.40 | Auditar e fechar funcoes faltantes da extensao antes do pacote | done |
| LSP1.41 | Harden ativacao: erro claro quando `zt-lsp` ou `zt` nao existem | done |
| LSP1.42 | Validar comandos `check`, `build` e `run` em terminal integrado | done |
| LSP1.43 | Validar workspace com `zenith.ztproj` e workspace sem projeto | done |
| LSP1.44 | Revisar configuracoes expostas ao usuario | done |
| LSP1.45 | Documentar troubleshooting curto | done |
| LSP1.46 | Gerar `.vsix` local com versao e metadados corretos | done |

Definition of done:

- usuario consegue instalar e abrir `.zt` sem passos escondidos;
- erro de LSP ausente ou build ausente tem mensagem clara;
- docs publicas batem com o comportamento real.

Checklist minima antes de gerar `.vsix`:

- todos os comandos declarados em `package.json` estao registrados no `extension.js`;
- `zenith.newFile` funciona em workspace com projeto e sem projeto;
- providers principais continuam ativos: diagnostics, completion, hover, definition, references, rename, signature help, document symbols, workspace symbols, formatting e semantic tokens;
- mudancas de arquivo atualizam o indice sem exigir reiniciar o VSCode;
- erro de binario ausente orienta o usuario em texto curto;
- extensao procura binarios na pasta da extensao, em `bin/`, na raiz do repo e no `PATH`;
- README da extensao e guia publico descrevem o comportamento real.

Evidencia automatizada:

- `extension.js` passa em `node --check`.
- `package.json` e `language-configuration.json` passam em `python -m json.tool`.
- `tests/lsp/test_lsp_smoke.py` continua passando com a extensao e o protocolo atuais.
- `python tools/package_vscode_extension.py` gera `dist/zenith-vscode-1.0.0.vsix`.
- O `.vsix` inclui `bin/zt.exe`, `bin/zt-lsp.exe`, `stdlib/` e `runtime/`.
- `code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --install-extension dist\zenith-vscode-1.0.0.vsix --force` instala a extensao em perfil isolado.
- `code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --list-extensions --show-versions` lista `zenith-lang.zenith-vscode@1.0.0`.
- Com `ZENITH_HOME` apontando para a extensao instalada, `zt check`, `zt build` e `zt run` passam em `tests\behavior\check_intrinsic_basic`.

Observacao sobre validacao manual:

- A instalacao local do `.vsix` foi validada por CLI em perfil isolado.
- A validacao visual no Extension Development Host continua na Fase 8, porque exige janela interativa do VSCode.

## Fase 8 - Matriz de validacao

Objetivo: fechar 1.0 com evidencia repetivel.

| ID | Item | Status |
| --- | --- | --- |
| LSP1.47 | Rodar smoke LSP em Windows | done |
| LSP1.48 | Rodar smoke LSP em Linux | done |
| LSP1.49 | Rodar teste manual no VSCode Extension Development Host | pending |
| LSP1.50 | Rodar teste de `.vsix` instalado localmente | done |
| LSP1.51 | Registrar resultado em relatorio de release LSP 1.0 | done |
| LSP1.52 | Atualizar docs publicas finais | done |

Definition of done:

- cada validacao tem comando, data e resultado;
- qualquer falha residual vira item explicito de release notes;
- se algo ficar fora de 1.0, fica marcado como pos-1.0.

Evidencia em Windows, 2026-04-28:

- `python tests/lsp/test_lsp_smoke.py`: passou com `lsp smoke ok`.
- `node --check tools/vscode-zenith/extension.js`: passou.
- `python -m json.tool tools\vscode-zenith\package.json`: passou.
- `python -m json.tool tools\vscode-zenith\language-configuration.json`: passou.
- `python -m json.tool tools\vscode-zenith\syntaxes\zenith.tmLanguage.json`: passou.
- `python -m py_compile tools\package_vscode_extension.py`: passou.
- `python tools\package_vscode_extension.py`: gerou `dist\zenith-vscode-1.0.0.vsix`.
- `code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --install-extension dist\zenith-vscode-1.0.0.vsix --force`: instalou com sucesso.
- `code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --list-extensions --show-versions`: listou `zenith-lang.zenith-vscode@1.0.0`.
- `zt check`, `zt build` e `zt run` pelo binario instalado no `.vsix`: passaram em `tests\behavior\check_intrinsic_basic`.
- Relatorio criado em `docs/internal/reports/release/LSP1.0-validation-report.md`.
- Guia publico `docs/public/guides/editor-vscode.md` atualizado com instalacao via `.vsix`.

Evidencia adicional em Windows, 2026-04-29:

- `zt create --app` agora gera o hello world curto da linguagem atual: `func main()` com `print("Hello from Zenith")`.
- `python tests\driver\test_create_scaffold.py`: passou.
- `python tests\driver\test_single_file_run.py`: passou.
- `python tests\driver\test_cli_option_values.py`: passou.
- `python tests\driver\test_project_only_inputs.py`: passou.
- `python tests\lsp\test_lsp_smoke.py`: passou com `lsp smoke ok`, incluindo `print()` como builtin e sem atalho `io.print(...)` nao qualificado.
- `python tools\package_vscode_extension.py`: recriou `dist\zenith-vscode-1.0.0.vsix` com `zt.exe` e `zt-lsp.exe` atualizados.
- `code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --install-extension dist\zenith-vscode-1.0.0.vsix --force`: instalou o VSIX atualizado com sucesso.

Bloqueios restantes da Fase 8:

- Extension Development Host ainda precisa de validacao visual/interativa no VSCode.

Evidencia adicional em Windows/WSL, 2026-05-09:

- `python tools/build_lsp.py`: passou e recriou `zt-lsp.exe`.
- `python tests/lsp/test_lsp_smoke.py`: passou com `lsp smoke ok`.
- `wsl -e sh -lc 'python3 tests/lsp/test_lsp_smoke.py'`: passou com `lsp smoke ok`.
- `node --check tools/vscode-zenith/extension.js`: passou.
- `python -m json.tool` em `tools\vscode-zenith\package.json`, `language-configuration.json`, `syntaxes\zenith.tmLanguage.json` e `snippets\zenith.code-snippets`: passou.
- `python tools/package_vscode_extension.py`: passou e recriou `dist\zenith-vscode-1.0.0.vsix` com `snippets/zenith.code-snippets`.
- `zt.exe check` passou em `list_value_api_basic`, `map_value_api_basic`, `set_core_api_basic`, `std_regex_basic`, `std_console_basic` e `lazy_primitive_text_basic`.
- O LSP foi realinhado com a superficie atual: `f"..."` substitui a sugestao antiga de `fmt`; `with` e `lazy<T>` aparecem em completion/grammar; `std.list`, `std.map` e `std.set` expoem API intrinseca no autocomplete, hover e Signature Help.
- A extensao agora sincroniza documentos com `didOpen` apenas na primeira abertura e `didChange` nas alteracoes seguintes; `didClose` limpa qualquer documento Zenith fechado.

Evidencia adicional de cobertura total da stdlib, 2026-05-09:

- `tests/lsp/test_lsp_smoke.py` importa cada modulo em `stdlib/std` isoladamente e exige completion minimo para a API publica de todos eles.
- Fachadas sem funcoes publicas escritas em `.zt`, mas reconhecidas pelo checker, agora entram no catalogo intrinseco do LSP: `std.jobs.spawn/join`, `std.channels.create/send/receive/close`, `std.shared.create/get/set` e `std.atomic.create/load/store/add`.
- O catalogo intrinseco agora e mesclado com declaracoes reais do modulo. Assim `jobs.Job<T>` e `jobs.spawn(...)` aparecem juntos, sem esconder structs publicas.
- Hover e Signature Help foram validados para `std.list.append(...)` e `std.jobs.spawn(...)`, incluindo labels de parametros.
- O timeout do harness LSP foi ajustado para 10s porque `std.collections` no WSL sobre `/mnt/c` pode passar de 5s mesmo respondendo corretamente.
- Validacao final passou em Windows e WSL com `lsp smoke ok`; `node --check`, JSON da extensao/snippets e `python tools/package_vscode_extension.py` tambem passaram.
- Checks de concorrencia passaram para `wave4_concurrency_surface`, `wave4_concurrency_generic_surface` e `std_jobs_int_basic`; os fixtures negativos `std_atomic_bool_type_error` e `std_shared_text_type_error` falharam como esperado.

## Fora do 1.0

Estes itens sao uteis, mas nao devem bloquear o 1.0:

- publicacao no Marketplace se o `.vsix` local ja estiver validado;
- decisao final de Marketplace antes da release interna;
- refatorar todo o servidor LSP para multiplos arquivos;
- suporte completo a sobrecarga futura antes de a linguagem estabilizar essa regra;
- index persistente em disco;
- code actions complexas;
- formatter completo via LSP.

## Ordem recomendada

1. Fechar escopo local: references e rename.
2. Integrar melhor binder/checker ao LSP.
3. Endurecer completion e signature help.
4. Validar semantic tokens e grammar.
5. Medir robustez em workspace real.
6. Fechar funcoes faltantes da extensao.
7. Empacotar `.vsix`.
8. Fazer validacao manual e relatorio final.
