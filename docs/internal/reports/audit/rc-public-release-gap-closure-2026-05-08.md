# RC Public Release Gap Closure - 2026-05-08

> Audience: maintainer
> Status: historical RC closure checklist; superseded for installable packages by `0.4.1-alpha.1`
> Surface: internal release report
> Source of truth: yes, for the 2026-05-08 gap triage in this file

Current packaging note: the active installable package line is
`0.4.1-alpha.1` (2026-05-09). This report remains evidence for the earlier
RC-readiness discussion.

## Decisao curta

Nao encontrei P0/P1 local novo.

A linguagem esta aprovada para **RC publica local** se o contrato da RC continuar
claro: C backend como oracle atual, stdlib com subconjuntos explicitos, e
ecossistema ainda em maturacao.

O que ainda bloqueia a **publicacao real da tag/release** nao e uma correcao de
linguagem local. Sao passos de execucao de release:

- [x] rodar matriz remota de CI no commit final;
- [x] revisar arvore limpa no commit de RC;
- [x] revisar escopo local do diff de RC;
- [x] gerar release notes com limites aceitos;
- [x] definir politica de artefatos de distribuicao para a RC;
- [x] criar tag e GitHub Release somente depois dos itens acima.

Estado atual deste registro: a RC publica era o alvo de corte em
`v1.0.0-rc.1`, com execucao de release tratada como etapa separada.

## Topicos urgentes reavaliados

| Item | Importancia | Estado para RC | Decisao |
| --- | --- | --- | --- |
| Docs publicas atras da implementacao | P2 | Parcialmente fechado neste passe | Nao bloqueia se release notes citarem limites e docs canonicas |
| `grid2d<T>`, `pqueue<T>`, `circbuf<T>`, `btreemap<K,V>` genericos | P2 | Divida tecnica explicita | Nao prometer na RC |
| Concorrencia: facade tipada, runtime estreito | P2 | Contrato limitado documentado | Nao bloquear RC; manter payload executavel especializado |
| FFI util, mas estreita | P2 | Subconjunto documentado | Nao bloquear RC; nao vender como FFI geral |
| Relatorios historicos parecem atuais | P2 | Corrigido neste passe para arquivos principais | Manter relatorios antigos marcados como historicos |
| Backend conformance | P3 | Futuro | Exigir antes de stable multi-backend, nao da RC C-backend |
| Ciclo RC | P3 | Falta execucao final | Bloqueia publicacao da tag, nao implementacao local |
| LSP/registry/installers/playground/ecossistema | P3 | Parcial/maduro por partes | Nao prometer maturidade de ecossistema na RC |

## O que foi fechado neste passe

- [x] `docs/reference/language/errors-and-results.md` nao usa mais
  a frase antiga de helper fora do primeiro slice do compilador.
- [x] `docs/reference/language/types.md` separa helpers atuais de helpers fora
  do subconjunto publico.
- [x] `docs/reference/language/functions-and-control-flow.md` usa `case else`
  em vez de `default`.
- [x] `docs/reference/language/expression-readability.md` usa o contrato atual
  de `.or_wrap(text)` e `.or_return(value)`.
- [x] `docs/reference/language/feature-matrix.md` foi trocado por matriz de
  release com status `release-covered`, `reference-covered`,
  `contract-limited` e `post-RC`.
- [x] `reports/pending-language-issues-current.md` foi marcado como historico.
- [x] `reports/deep-analysis-report.md` foi marcado como historico.
- [x] `docs/internal/reports/audit/final-language-implementation-audit-2026-05-03.md`
  foi marcado como historico/superseded.
- [x] `docs/internal/reports/audit/R2.M7-spec-vs-implementation-audit.md`
  foi marcado como historico/superseded.
- [x] `docs/internal/reports/audit/language-implementation-audit-2026-04-29.md`
  foi marcado como historico/superseded.
- [x] `docs/internal/reports/audit/implementation-plan-rc-public.md` nao trata
  mais a refatoracao documental como pendencia aberta.

## Dividas tecnicas aceitas para pos-RC

Estas dividas nao bloqueiam a RC se aparecerem nas release notes como limites.

- [ ] Implementar `grid2d<T>` e `grid3d<T>` genericos reais.
- [ ] Implementar `pqueue<T>` com contrato de ordenacao:
  - constraint de tipo ordenavel; ou
  - comparador explicito.
- [ ] Implementar `circbuf<T>` generico real.
- [ ] Implementar `btreemap<K,V>` e `btreeset<T>` com contrato de chave/valor
  ordenavel.
- [ ] Ampliar runtime de `std.jobs`, `std.channels`, `std.shared` e
  `std.atomic` para payloads alem do subconjunto `int` executavel atual.
- [ ] Ampliar FFI para capturas, valores gerenciados, varargs, campos callable
  e retornos callable quando o runtime puder garantir ownership correto.
- [ ] Evoluir ciclo de memoria para casos ciclicos/ownership avancado.

## Evolucao futura

Nao vender como promessa da RC:

- [ ] conformance de backends alem do C;
- [ ] registry remoto do ZPM;
- [ ] instaladores publicados e assinados;
- [ ] LSP de producao;
- [ ] playground publico;
- [ ] documentacao publica expandida para callables, closures/lambdas,
  `where`, `any<Trait>` e enum payloads.

## Evidencias usadas

| Evidencia | Resultado |
| --- | --- |
| `docs/internal/reports/audit/implementation-review-rerun-2026-05-07.md` | aprovado para RC publica local |
| `docs/internal/reports/audit/implementation-plan-rc-public.md` | Etapas 1-8 marcadas como concluidas |
| `docs/internal/reports/audit/language-complete-analysis-2026-05-08.md` | P0/P1 local ausente; P2/P3 classificados |
| `python tools/build_installers.py --help` | orquestrador de instaladores existe |
| `python tools/build_linux_packages.py --help` | fluxo Linux existe via `fpm` |
| `python tools/build_installers.py --version 0.0.0-rc-check --target auto --dry-run --skip-build` | dry-run do orquestrador passou |
| `python -m py_compile tools/build_installers.py tools/build_linux_packages.py tools/build_lsp.py tools/package_vscode_extension.py tools/release.py` | scripts de release/empacotamento compilam |
| `python tools/check_docs_paths.py` | passou |
| `python tools/check_docs_current_syntax.py` | passou |
| `.\zt.exe doc check zenith.ztproj` | passou |
| `git diff --check` | passou depois de remover trailing whitespace em `reports/deep-analysis-report.md` |
| `docs/internal/reports/audit/evidence/release-diff-review-2026-05-08.md` | diff/release artifact review registrado |
| `docs/internal/reports/release/1.0.0-rc.1-draft-github-release-notes.md` | release notes draft criadas |
| `python run_suite.py release` em 2026-05-09 | passou, 365/365, `reports/suites/release__20260509T005740Z.json` |
| GitHub Actions `CI` no commit `3db2a30` | passou em Ubuntu, macOS e Windows |
| GitHub Actions `Examples Smoke` no commit `3db2a30` | passou depois do ajuste dos exemplos publicos |
| `v1.0.0-rc.1` | tag planejada para a linha RC |
| GitHub Release `Zenith 1.0.0-rc.1` | pre-release planejado, sem binarios anexos |

## Proxima ordem de execucao

1. [x] Rodar checks locais depois deste ajuste documental.
2. [x] Revisar diff final e confirmar que os arquivos historicos nao parecem
   a lista de bloqueios atual.
3. [x] Rodar CI remoto no commit final.
4. [x] Gerar release notes com limites aceitos.
5. [x] Gerar/anexar artefatos prometidos ou declarar que instaladores sao
   fora do contrato da RC.
6. [x] Publicar tag/release.
