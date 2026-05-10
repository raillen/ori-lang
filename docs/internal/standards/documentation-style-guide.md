# Documentation Style Guide

Este guia define o padrao obrigatorio para docs do projeto.

## Objetivo

- Deixar a documentacao consistente e facil de escanear.
- Melhorar legibilidade para pessoas com TDAH e dislexia.
- Evitar ambiguidades entre planejamento, implementacao e auditoria.

## Estrutura minima obrigatoria

Todo documento novo deve ter, nesta ordem:

1. `# Titulo`
2. `## Objetivo`
3. `## Escopo`
4. `## Dependencias`
5. `## Conteudo principal`
6. `## Validacao`
7. `## Historico de atualizacao`

## Regras de escrita

- Use paragrafos curtos (1 a 4 linhas).
- Use listas curtas e orientadas a acao.
- Evite jargao sem definicao.
- Evite blocos longos de texto sem subtitulo.
- Prefira verbos no imperativo para passos de execucao.
- Em paginas de usuario, comece pela resposta pratica antes da teoria longa.
- Separe claramente `current`, `partial`, `planned`, `historical` e `rejected`.
- Inclua exemplos validos e erros comuns quando documentar feature de linguagem.
- Nao obrigue o usuario a ler `docs/internal/decisions/language/` para aprender uma feature.

## Regras de emoji

- Emoji e opcional.
- Use apenas para sinalizacao importante.
- Limite: no maximo 1 emoji por secao.
- Nao use emoji em titulo principal.

## Regra de dependencias explicitas

A secao `Dependencias` deve sempre listar:

- Documento(s) upstream usados como fonte.
- Documento(s) downstream impactados.
- Arquivos de codigo e testes que validam o documento.

Template:

```md
## Dependencias

- Upstream:
  - `path/doc-upstream.md`
- Downstream:
  - `path/doc-downstream.md`
- Codigo/Testes relacionados:
  - `path/arquivo.c`
  - `path/teste.c`
```

## Template recomendado

```md
# Nome do Documento

## Objetivo

## Escopo

## Dependencias

## Conteudo principal

## Validacao

## Historico de atualizacao
- YYYY-MM-DD: descricao curta da alteracao.
```
