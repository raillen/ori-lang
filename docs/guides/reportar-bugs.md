# Como reportar bugs

Status: politica pratica para Ori **S3 / `0.3.0`** (+ `0.3.1`/B).

Um bom bug report deve permitir reproduzir o problema com poucos comandos.

## Linguagem ou type checker

Inclua:

- versao do Ori: `ori --version`;
- sistema: Windows, Linux ou macOS;
- arquivo `.orl` minimo;
- comando usado, por exemplo `ori check main.orl`;
- saida completa do diagnostico.

Use esta categoria para parser, type checker, imports, generics, traits,
matching, `try`, ARC ou regras de ownership.

## Stdlib ou runtime

Inclua tambem:

- modulo usado, por exemplo `ori.fs`, `ori.json` ou `ori.process`;
- se o problema ocorre em `ori run`, `ori compile` ou ambos;
- se possivel, rode com `ORI_TEST_LEAK_CHECK=1` quando o bug envolver memoria.

## Tooling

Use esta categoria para `ori fmt`, `ori doc`, `ori new`, `ori repl`, LSP,
extensao VS Code, release package e scripts.

Inclua:

- comando exato;
- arquivo ou projeto minimo;
- se o problema ocorre fora do checkout do repo;
- logs do Output Channel da extensao, quando for VS Code.

## Formato sugerido

```text
Titulo: descricao curta

Ambiente:
- Ori:
- OS:
- Comando:

Reproducao:
1. ...
2. ...

Esperado:

Obtido:

Arquivo minimo:
module app.main

main()
end
```

Nao inclua projetos grandes no primeiro relato. Comece pelo menor arquivo que
mostra o problema; anexos maiores podem vir depois.
