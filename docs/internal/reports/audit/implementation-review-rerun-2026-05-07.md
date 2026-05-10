# Implementation Review Rerun - 2026-05-07

> Fonte executada: `docs/spec/language/implementation-review.md`
> Escopo: auditoria completa por evidencias locais, sem marcar o checklist original como concluido sem prova.
> Status original recomendado: **Beta tecnico**. Veja a atualizacao final de 2026-05-08 para o estado atual.

## Atualizacao final de RC publica - 2026-05-08

Status recomendado atual: **aprovado para RC publica local**.

Esta decisao cobre o estado do repositorio local apos as Etapas 1-8 de
`docs/internal/reports/audit/implementation-plan-rc-public.md`.

Addendum de lacunas de release: `docs/internal/reports/audit/rc-public-release-gap-closure-2026-05-08.md`.

Ela nao afirma que tag, GitHub Release, anuncio publico ou matriz remota de CI
ja foram executados. Esses passos pertencem a execucao de release.

### Criterios de aprovacao

| Criterio | Resultado | Evidencia |
|---|---:|---|
| Contrato publico de `std.collections` claro | PASS | `docs/reference/stdlib/collections.md`, `docs/spec/language/stdlib-model.md`, `docs/internal/reports/audit/implementation-plan-rc-public.md` |
| Documentacao publica consolidada | PASS | `docs/internal/reports/audit/rc-public-docs-consolidation-2026-05-07.md`, `python tools/check_docs_paths.py` |
| Sanitizer em ambiente compativel | PASS | `docs/internal/reports/audit/evidence/runtime-sanitizers-2026-05-07.md` |
| Valgrind | PASS | `docs/internal/reports/audit/evidence/valgrind-2026-05-08.md` |
| Dr. Memory | EXECUTADO, nao aceito como gate | `docs/internal/reports/audit/evidence/drmemory-2026-05-08.md` |
| Performance completa | PASS com `warn` justificado | `docs/internal/reports/audit/evidence/performance-2026-05-08.md` |
| Comandos publicos de release | PASS | `docs/internal/reports/audit/evidence/public-release-commands-2026-05-08.md` |
| Diff e baselines revisados | PASS | `docs/internal/reports/audit/evidence/diff-cleanup-2026-05-08.md` |
| Suite oficial de release | PASS | `reports/suites/release__20260508T112750Z.json`, 365/365 |

### Bloqueios fechados

- ZDoc: `.\zt.exe doc check zenith.ztproj` passa.
- Literal inteiro fora de range: fixture negativa permanente criada.
- `zt test zenith.ztproj --ci`: projeto raiz agora tem teste positivo direto.
- Sanitizer: passou em WSL/GCC compativel.
- Valgrind: corpus minimo passou no WSL depois de corrigir vazamento real em `list_set` copy-on-write.
- Dr. Memory: instalado e executado, mas a instrumentacao falha no WSL atual ate com `/bin/true`; nao e bloqueio porque Valgrind e sanitizer ja passaram.
- Performance completa: `nightly --release-gate` saiu com codigo `0`.
- `std.collections`: contrato publico especializado foi documentado; shapes genericas nao suportadas falham de forma explicita.
- ZPM publico: `install`, `install --locked`, `update`, `publish`, `zt zpm` e `zt pkg` passaram em sandbox.
- Ajuda publica: `zt help zpm` nao anuncia comandos inexistentes.

### Dividas tecnicas aceitas para pos-RC

- `grid2d<T>`, `pqueue<T>`, `circbuf<T>` e `btreemap<K, V>` genericos reais continuam pos-RC.
- Registry remoto do ZPM fica fora do contrato publico da RC.
- Instalador nativo fica fora do contrato publico da RC.
- Public announcement e GitHub Release tag ainda precisam ser executados no fluxo de release.
- Matriz remota Windows/Linux/macOS deve rodar antes da publicacao final da tag.

### Evolucoes futuras

- Implementar colecoes avancadas genericas reais quando o backend suportar esse contrato com seguranca.
- Promover registry remoto do ZPM quando existir endpoint estavel.
- Criar instaladores nativos versionados.
- Ampliar monitoramento de performance para comparar runs locais e CI.

### Riscos residuais

- `macro_medium_check` teve `warn` de p95 isolado; a mediana ficou estavel e o budget absoluto passou.
- Validacao remota de CI ainda nao foi executada neste turno.
- `update-registry` existe como comando interno, mas nao faz parte da ajuda publica da RC enquanto o registry remoto nao estiver estavel.

### Decisao

Nao ha P1 aberto.

Nao ha P2 sem decisao explicita.

Decisao local: **aprovado para RC publica**.

Nota: as secoes abaixo preservam auditorias anteriores. Quando houver conflito,
a atualizacao final de 2026-05-08 acima prevalece.

## Atualizacao pos-correcao - 2026-05-07

Status das correcoes deste relatorio: **concluido localmente, com sanitizer delegado ao CI Linux**.

Correcoes aplicadas:

- P1 ZDoc: `zt doc check zenith.ztproj` agora passa sem warnings.
- P1 literal inteiro fora do range: `9223372036854775808` agora emite `error[type.invalid_conversion]`.
- P1 teste permanente: adicionado fixture `int_literal_out_of_range_error`.
- P2 gate direto de testes: `zt test zenith.ztproj --ci` agora passa no projeto raiz.
- P2 sanitizer: o check foi adicionado ao CI Linux; nesta maquina continua `SKIP` porque a toolchain local nao aceita ASAN/UBSAN.
- P2 performance: criada suite local `release`; `python run_suite.py release` passa e inclui `perf/quick`.
- P2 ZDoc publica: os 95 `warning[doc.missing_public_doc]` foram cobertos por blocos ZDoc.
- P3 descoberta: `zt help zpm` agora existe e aponta para `zpm.exe --help`.
- P3 temporarios locais: `.gitignore` cobre os artefatos citados (`emit_debug.txt`, `emit_stdout.txt`, `out.txt`, `test_output.txt`, `tmp/*_raw.txt`).

Evidencias novas:

| Comando | Resultado | Observacao |
|---|---:|---|
| `python build.py` | PASS | `zt.exe` e `zpm.exe` reconstruidos. |
| `.\zt.exe doc check zenith.ztproj` | PASS | `doc check ok`, sem warnings. |
| `.\zt.exe test zenith.ztproj --ci` | PASS | `test ok (pass=1 skip=0)`. |
| `.\zt.exe check tests\behavior\int_literal_out_of_range_error\zenith.ztproj --ci` | PASS como fixture negativa | Emite `error[type.invalid_conversion]` para literal fora do range. |
| `python run_suite.py pr_gate --no-perf` | PASS | 363/363. Relatorio: `reports/suites/pr_gate__20260507T191009Z.json`. |
| `python run_suite.py release` | PASS | 364/364, inclui `perf/quick`. Relatorio: `reports/suites/release__20260507T191946Z.json`. |
| `python tests\heavy\run_heavy_tests.py --suite all` | PASS | `heavy/semantic_curated` e `heavy/fuzz_semantic` passaram. |
| `python tests\hardening\test_runtime_sanitizers.py` | SKIP local | Toolchain local sem suporte a sanitizer; CI Linux cobre o gate. |
| `.\zt.exe check zenith.ztproj --all --ci` | PASS | Projeto raiz valida em modo CI. |
| `.\zt.exe fmt zenith.ztproj --check` | PASS | Formatter sem drift. |
| `.\zt.exe help zpm` | PASS | Topico integrado ao help do `zt`. |
| `git diff --check` | PASS | Apenas avisos normais de CRLF no Windows. |

Status historico apos esta rodada: **estado intermediario antes da validacao final**. Naquele momento, a confirmacao de sanitizer em ambiente compativel ainda nao tinha sido feita. Este estado foi superado pela atualizacao final de 2026-05-08.

## Resumo executivo

O nucleo do compilador, runtime e runner oficial esta forte nesta rodada:

- `build`, `check`, `fmt`, `smoke` e `pr_gate` passaram.
- O `pr_gate` passou 360/360, incluindo behavior, formatter, hardening e fuzz replay.
- O smoke passou 69/69.
- Fuzz leve de lexer e parser passou sem crashes e sem timeouts.
- O gate novo de stack overflow esta coberto por `panic_stack_overflow`.

Na execucao original, a revisao completa encontrou bloqueios reais para release publico estavel:

- `zt doc check zenith.ztproj` falha com 41 erros ZDoc.
- O fuzz semantico pesado aceita `9223372036854775808` como `int`, mesmo sendo maior que `i64::MAX`.
- `zt test zenith.ztproj --ci` nao e um gate verde direto, porque pega fixtures negativas do repositorio.
- Sanitizers foram pulados nesta maquina, pois o compilador local nao aceita ASAN/UBSAN.

Estado atual apos correcao: os bloqueios P1 estao fechados. `zt doc check zenith.ztproj` passa sem warnings, o fuzz semantico pesado passa, e o literal `9223372036854775808` virou diagnostico estavel.

## Priorizacao por importancia

### P0 - Nada critico encontrado nesta rodada

Nao houve falha de build, crash de runner oficial, regressao massiva ou corrupcao evidente nos gates principais.

### P1 - Corrigido

Falhas originais:

- `zt doc check zenith.ztproj` falha com 41 erros ZDoc.
- O fuzz semantico pesado aceita `9223372036854775808` como `int`, mesmo estando fora de `i64::MAX`.

Correcoes aplicadas:

- ZDoc da stdlib foi corrigido ate `zt doc check zenith.ztproj` retornar `doc check ok`.
- Alvos ZDoc quebrados foram ajustados, incluindo consts publicas e BOM UTF-8 no leitor ZDoc.
- Os 95 avisos `doc.missing_public_doc` foram cobertos por blocos ZDoc.
- O checker rejeita literal inteiro fora do range com `error[type.invalid_conversion]`.
- Foi adicionado fixture permanente `int_literal_out_of_range_error`.

Evidencia atual:

- `.\zt.exe doc check zenith.ztproj`: PASS.
- `.\zt.exe check tests\behavior\int_literal_out_of_range_error\zenith.ztproj --ci`: falha esperada com `error[type.invalid_conversion]`.
- `python tests\heavy\run_heavy_tests.py --suite all`: PASS.

Conclusao: P1 nao bloqueia mais RC/Estavel.

### P2 - Confiabilidade de release (historico fechado)

Registro historico desta rodada:

- `zt test zenith.ztproj --ci` foi registrado como problema de gate direto no projeto raiz.
- Sanitizer foi registrado como limitado pelo compilador Windows local.
- Performance completa foi registrada como evidencia ausente naquela rodada.
- ZDoc publico foi registrado com warnings de documentacao ausente.

Estado atual:

- `.\zt.exe test zenith.ztproj --ci`: PASS.
- Sanitizer em WSL2/GCC: PASS, evidencia em `docs/internal/reports/audit/evidence/runtime-sanitizers-2026-05-07.md`.
- Valgrind em WSL2: PASS, evidencia em `docs/internal/reports/audit/evidence/valgrind-2026-05-08.md`.
- Performance completa: PASS com aviso justificado, evidencia em `docs/internal/reports/audit/evidence/performance-2026-05-08.md`.
- ZDoc publico: PASS sem warnings em `.\zt.exe doc check zenith.ztproj`.

Conclusao: nao ha P2 aberto sem decisao explicita para a RC publica local.

### P3 - Acabamento e descoberta (historico fechado para RC)

Registro historico desta rodada:

- `zt help zpm` foi registrado como lacuna de descoberta.
- Artefatos temporarios e lacunas de descoberta foram registrados no fluxo local.

Estado atual:

- `.\zt.exe help zpm`: PASS.
- Artefatos temporarios conhecidos estao ignorados ou fora do diff de release.
- Os arquivos novos nao rastreados sao docs, testes, evidencias e fixtures intencionais; devem entrar no commit da RC.
- O comando local de release esta representado por `python run_suite.py release`.

Conclusao: nao ha P3 que bloqueie a RC publica local.

## Evidencias executadas na auditoria original

| Comando | Resultado | Observacao |
|---|---:|---|
| `python build.py` | PASS | `zt.exe` e `zpm.exe` reconstruidos. |
| `.\zt.exe check zenith.ztproj --all --ci` | PASS | Projeto raiz valida em modo CI. |
| `.\zt.exe fmt zenith.ztproj --check` | PASS | Formatter nao encontrou drift. |
| `python tools/check_docs_current_syntax.py` | PASS | Docs de sintaxe atualizadas pelo checker existente. |
| `.\zpm.exe --help` | PASS | CLI do ZPM responde. |
| `.\zpm.exe install --locked` | N/A | Nao ha `zpm.lock` no projeto raiz. |
| `python run_suite.py smoke --no-perf` | PASS | 69/69. Relatorio: `reports/suites/smoke__20260507T174608Z.json`. |
| `python run_suite.py pr_gate --no-perf` | PASS | 360/360. Relatorio: `reports/suites/pr_gate__20260507T174245Z.json`. |
| `python tests\fuzz\fuzz_lexer.py --iters 200 --seed 20260421` | PASS | 0 crashes, 0 timeouts. |
| `python tests\fuzz\fuzz_parser.py --iters 200 --seed 20260421` | PASS | 0 crashes, 0 timeouts. |
| `python tests\heavy\run_heavy_tests.py --suite all` | FAIL | Replay curado passou; fuzz semantico falhou em literal inteiro fora do range. |
| `python tests\hardening\test_runtime_sanitizers.py` | SKIP | Compilador local nao suporta flags de sanitizer. |
| `.\zt.exe doc check zenith.ztproj` | FAIL | 41 erros ZDoc, alem de muitos avisos de doc publica ausente. |
| `.\zt.exe test zenith.ztproj --ci` | FAIL | O comando direto pega fixtures negativas intencionais. |
| `git diff --check` | PASS | Sem whitespace errors; apenas avisos normais de CRLF. |
| `.\zt.exe help fmt` / `.\zt.exe help doc` | PASS | Ajuda existe para esses topicos. |
| `.\zt.exe help zpm` | FAIL | Topico `zpm` nao existe no help do `zt`. |

## Estado por fase

| Fase | Estado | Evidencia curta |
|---|---:|---|
| 1. Preparacao e mapeamento | OK para RC local | Worktree revisado; relatorio final e plano de RC criados. |
| 2. Contrato final da linguagem | OK para RC local | `check --all --ci` passa e `doc check` esta verde. |
| 3. Lexer, parser e sintaxe | OK | Fuzz lexer/parser 200 iteracoes cada sem crash/timeout; `pr_gate` verde. |
| 4. Semantica, checker e tipagem | OK para RC local | Suite oficial verde; fuzz pesado passou apos rejeicao de literal inteiro fora do range. |
| 5. Pattern matching e controle de fluxo | OK | Coberto por behavior tests no `pr_gate`. |
| 6. Runtime, ARC, memoria e valor | OK para RC local | Stack overflow coberto; sanitizer WSL e Valgrind passaram; ciclos ARC seguem como politica futura. |
| 7. Error model, panic e cleanup | OK | `panic_basic`, `panic_stack_overflow`, `using_panic_cleanup` e runtime failures esperados passam. |
| 8. Standard library | OK para RC local | Casos de stdlib passam no runner; ZDoc da stdlib passa sem warnings; `std.collections` tem contrato publico claro. |
| 9. Concurrency, jobs, channels e transferable | OK | Fixtures de jobs, boundary copy e unsupported errors passam no `pr_gate`. |
| 10. FFI e fronteiras nativas | OK | Casos extern C, callbacks e ABI passam no `pr_gate`. |
| 11. Backend, ZIR, runtime ABI e conformance | OK | Hardening de determinismo, roundtrip emit C e compilacao concorrente passam. |
| 12. Tooling, formatter, LSP, runner e docs | OK para RC local | Formatter, runner oficial, help e `doc check` passam. |
| 13. Seguranca, robustez e fuzzing | OK para RC local | Fuzz leve, replay, fuzz semantico pesado, sanitizer WSL e Valgrind passam; Dr. Memory foi rejeitado como gate por incompatibilidade do ambiente. |
| 14. Qualidade e divida tecnica | OK para RC local | Gates principais verdes; arquivos nao rastreados atuais sao artefatos intencionais de docs, testes e evidencias para o commit da RC. |
| 15. Performance e escalabilidade | OK para RC local | Gate completo reexecutado em 2026-05-08; evidencia em `docs/internal/reports/audit/evidence/performance-2026-05-08.md` e `reports/suites/release__20260508T112750Z.json`. |
| 16. Validacao final | OK para RC local | Nao ha P1 aberto nem P2 sem decisao explicita; pendencias restantes sao execucao de release e evolucao pos-RC. |

## Problemas encontrados

### P1 - ZDoc falhava no projeto raiz (corrigido)

Estado atual: `.\zt.exe doc check zenith.ztproj` passa com `doc check ok`.

Na auditoria original, `.\zt.exe doc check zenith.ztproj` falhava com 41 erros.

Exemplos confirmados:

- `stdlib/zdoc/std/bool.zdoc`: `doc.malformed_block`, texto fora de bloco ZDoc.
- `stdlib/zdoc/std/debug.zdoc`: `doc.malformed_block`, texto fora de bloco ZDoc.
- `stdlib/zdoc/std/float.zdoc`: `doc.malformed_block`, texto fora de bloco ZDoc.
- `stdlib/zdoc/std/int.zdoc`: `doc.malformed_block`, texto fora de bloco ZDoc.
- `stdlib/zdoc/std/math.zdoc`: `doc.malformed_block`, incluindo BOM no inicio do arquivo.
- `stdlib/zdoc/std/collections.zdoc`: alvos nao resolvidos `cols`, `size`, `queue_values`, `stack_values`.
- `stdlib/zdoc/std/io.zdoc`: alvos nao resolvidos `input`, `output`, `stderr`.
- `stdlib/zdoc/std/math.zdoc`: alvos nao resolvidos `e`, `tau`.
- `stdlib/zdoc/std/text.zdoc`: alvo nao resolvido `v1_new_apis`.

Impacto original: a documentacao publica da stdlib nao passava no proprio checker. Isso bloqueava release publico estavel.

Correcao: blocos malformados foram reparados, alvos inexistentes foram ajustados, consts publicas passaram a ser resolvidas pelo ZDoc, o leitor tolera BOM UTF-8 inicial e os avisos de documentacao publica foram cobertos.

### P1 - Fuzz semantico aceitava literal inteiro fora do range (corrigido)

Estado atual: `python tests\heavy\run_heavy_tests.py --suite all` passa.

Na auditoria original, `python tests\heavy\run_heavy_tests.py --suite all` falhou no bloco `heavy/fuzz_semantic`.

Caso reproduzido pelo fuzzer:

```zt
namespace fuzz.large_int

public func main() -> int
    -- Literal maior que i64 max
    const x: int = 9223372036854775808
    return x
end
```

Correcao: o checker agora rejeita o literal com `error[type.invalid_conversion]`, e o caso foi fixado em `tests/behavior/int_literal_out_of_range_error`.

## Estado final consolidado

As secoes acima mantem o historico da auditoria original e das correcoes
intermediarias. Esta secao consolida o estado atual depois das Etapas 1-8.

### Bloqueios atuais

Nao ha bloqueio local de repositorio para RC publica.

O que ainda precisa acontecer antes da tag publica nao e correcao de codigo
local, e sim execucao de release:

- commit e push das mudancas de readiness;
- matriz remota de CI em Windows, Linux e macOS;
- tag a partir de uma arvore limpa;
- release notes e anuncio publico.

### Seguranca e robustez

| Area | Estado final | Evidencia |
|---|---:|---|
| Fuzz lexer/parser/replay | PASS | `python run_suite.py release` |
| Sanitizer | PASS | `docs/internal/reports/audit/evidence/runtime-sanitizers-2026-05-07.md` |
| Valgrind | PASS | `docs/internal/reports/audit/evidence/valgrind-2026-05-08.md` |
| Dr. Memory | Executado, nao aceito como gate | `docs/internal/reports/audit/evidence/drmemory-2026-05-08.md` |
| Performance completa | PASS com `warn` justificado | `docs/internal/reports/audit/evidence/performance-2026-05-08.md` |
| Release suite | PASS | `reports/suites/release__20260508T112750Z.json`, 365/365 |

Dr. Memory nao bloqueia a RC local porque a falha acontece antes do codigo do
Zenith: a ferramenta cai com `Floating point exception` ate ao instrumentar
`/bin/true` neste WSL. Valgrind e sanitizer ja cobrem o gate de memoria aceito.

### Lacunas contra o contrato final

| Area | Estado atual |
|---|---|
| Documentacao | Corrigido: ZDoc do projeto raiz passa sem warnings. |
| Numericos | Corrigido: literal `9223372036854775808` emite `error[type.invalid_conversion]`. |
| Tooling | Corrigido para RC: `zt test zenith.ztproj --ci` passa com teste raiz positivo. |
| ZPM/help publico | Corrigido para RC: `zt help zpm` existe e nao anuncia comandos fora do contrato publico. |
| Runtime/memoria | Corrigido para RC: sanitizer e Valgrind passaram; Dr. Memory foi executado e rejeitado por incompatibilidade do ambiente. |
| Release/performance | Corrigido para RC: release suite e performance nightly foram executadas e arquivadas. |
| CI remoto | Execucao de release: matriz Windows/Linux/macOS deve rodar fora deste passo local. |

### Divida tecnica aceita para pos-RC

Esta divida nao bloqueia a RC publica local:

- implementar `grid2d<T>`, `pqueue<T>`, `circbuf<T>` e `btreemap<K, V>` genericos reais;
- promover registry remoto do ZPM quando houver endpoint estavel;
- criar instaladores nativos versionados;
- manter Dr. Memory como ferramenta opcional apenas em ambiente compativel;
- ampliar comparacao de performance entre runs locais e CI;
- revisar politica futura de ciclos ARC sem misturar com os bloqueios P1 ja fechados.

### Evolucao futura

1. Rodar matriz remota Windows/Linux/macOS antes da tag publica.
2. Publicar tag e release notes a partir de arvore limpa.
3. Preparar anuncio publico.
4. Planejar a onda pos-RC de colecoes avancadas genericas reais.
5. Reavaliar Dr. Memory em ambiente Linux nativo ou WSL/kernel compativel.

## Recomendacao final

Status recomendado atual: **aprovado para RC publica local**.

Nao ha P1 aberto. Nao ha P2 sem decisao explicita. As pendencias restantes sao
de execucao de release e evolucao pos-RC, nao bloqueios locais da implementacao.
