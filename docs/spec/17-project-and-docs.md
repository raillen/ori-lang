# Projeto e documentação externa

Status: atual (layout **M2.layout** — 2026-07-13).

Este capítulo define:

- `ori.proj`: manifesto do **projeto** (obrigatório na raiz).
- `ori.pkg.toml`: manifesto de **pacote** reutilizável / cache (opcional).
- `.oridoc`: documentação externa de símbolos Ori.

Layout de produto: `docs/planning/repo-and-project-layout.md`.

A ideia é manter o código legível sem obrigar comentários longos dentro do
arquivo `.orl`, e **não** forçar uma pasta mágica (`src/`, `app/`) no projeto.

---

## Layout canônico de projeto

**Obrigatório:** `ori.proj` na raiz.

**Recomendado:** `main.orl` na raiz (`entry = "main.orl"`). O `entry` pode
apontar para outro caminho.

**Opcional:** pastas de domínio com mais `.orl` e árvore de docs espelhada.

```text
meu-projeto/
  ori.proj
  main.orl
  kanban-app/                 -- domínio opcional (nome à escolha)
    board.orl
    cards.orl
  notes-app/
    stickys.orl
  docs/                       -- sidecars opcionais
    kanban-app/
      board.oridoc
      cards.oridoc
```

`ori new <path>` cria:

```text
<path>/
  ori.proj
  main.orl          -- app (lib: lib.orl)
  docs/             -- pasta vazia para sidecars
```

Não cria `src/`, `lib/` ou `bin/` por padrão.

---

## `ori.proj`

`ori.proj` fica na **raiz** do projeto. Formato simples e explícito:

```ini
manifest = 1
name = "demo"
version = "0.1.0"
kind = "app"
entry = "main.orl"

[source]
root_namespace = "app"
-- source.root é opcional; omitido = raiz do projeto (todas as subpastas são domínio)

[dependencies]
demo.math = { path = "../math", version = "0.1.0" }

[docs]
paths = ["docs"]
mode = "sidecar-first"
require_public = "off"
```

Campos atuais:

| Campo | Obrigatorio | Descricao |
|---|---:|---|
| `manifest` | nao | Versao do formato. Hoje aceita `1`. |
| `name` | nao | Nome humano do projeto. |
| `version` | nao | Versao do projeto. |
| `kind` | nao | `app` ou `lib`. Padrao: `app`. |
| `entry` | **sim** | Arquivo `.orl` de entrada (recomendado: `main.orl` na raiz). |
| `source.root` | nao | Pasta raiz de codigo; **omitido = diretorio do `ori.proj`**. |
| `source.root_namespace` | nao | Prefixo de module esperado (ex.: `app`). |
| `dependencies.<name>` | nao | Dependencia local `{ path = "..." }`; versao opcional. |
| `docs.paths` | nao | Pastas/arquivos com `.oridoc`. |
| `docs.mode` | nao | `sidecar-first` ou `inline-first`. Padrao: `sidecar-first`. |
| `docs.require_public` | nao | `off`, `warn` ou `error`. Padrao: `off`. |

Compatibilidade: `entry = "src/main.orl"` e `source.root = "src"` continuam
validos se o usuario preferir esse layout.

Dependencias locais em `[dependencies]` participam da resolucao de imports.

```ori
import demo.math (double)
```

Para `demo.math = { path = "../math" }`, o path deve apontar para um projeto com
`ori.proj` ou um pacote com `ori.pkg.toml`.

---

## `ori.pkg.toml`

`ori.pkg.toml` descreve um pacote instalavel no cache local. **Nao substitui**
`ori.proj` no dia a dia de apps: `ori.proj` organiza o projeto; `ori.pkg.toml`
define o contrato de distribuicao.

```toml
[package]
name = "demo.app"
version = "0.1.0"
entry = "main.orl"
ori_version = "0.3.1"
description = "Demo app"

[dependencies]
demo.math = { path = "../demo-math", version = "0.1.0" }
```

| Campo | Descricao |
|---|---|
| `package.name` | Nome pontilhado alinhado ao module Ori. |
| `package.version` | Versao `major.minor.patch`. |
| `package.entry` | Arquivo `.orl` de entrada do pacote. |
| `package.ori_version` | Versao minima esperada do compilador Ori. |

`ori check`, `ori run`, `ori test` e `ori doc` aceitam `ori.pkg.toml` como
entrada quando usado como pacote.

---

## `.oridoc`

Um arquivo `.oridoc` documenta simbolos de um module. Preferencia de layout:

```text
kanban-app/board.orl
docs/kanban-app/board.oridoc
```

Tambem e valido lado a lado:

```text
board.orl
board.oridoc
```

Ou qualquer pasta listada em `[docs].paths`.

### Formato (resumo)

```text
oridoc 1

module app.kanban.board

doc load_board
  summary:
    Carrega o board.
  returns:
    `result[Board, string]`
end

doc module self
  summary:
    Dominio de board do kanban.
end
```

Prioridade inline vs sidecar: `[docs].mode` (`sidecar-first` default).

---

## Comandos

```bash
ori new meu-projeto
ori new meu-lib --lib
ori check .                 # sobe ate achar ori.proj
ori check ori.proj
ori run .
ori doc
ori doc check
```

---

## Monorepo da linguagem

O repositorio `ori-lang` nao e um app de usuario. O workspace Cargo vive em
`compiler/`. Ver `docs/planning/repo-and-project-layout.md`.
