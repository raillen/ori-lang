# Zenith VSCode

![Zenith Language](media/brand.png)

Extensao para arquivos `.zt`, integrada ao Compass LSP.

## Recursos

- Diagnostics em tempo real via Compass LSP.
- Completion global, contextual, por import, por membro e por builtin, com assinatura e documentacao em Markdown.
- Completion de membro para `any<Trait>`/`dyn<Trait>` sugere metodos declarados na trait.
- Completion de membro usa inferencia simples em `using`, literais e chamadas resolvidas quando nao ha tipo explicito.
- Hover de simbolos top-level indexados.
- Go-to-definition para simbolos top-level indexados, incluindo outro arquivo.
- References para simbolos top-level indexados e simbolos locais.
- Rename para simbolos top-level indexados e simbolos locais.
- Signature Help para chamadas diretas, `alias.func(...)` e metodos `apply` resolvidos por tipo.
- Semantic Tokens baseados no lexer oficial para highlight semantico leve.
- Indentacao automatica para blocos `func`, `struct`, `trait`, `apply`, `enum`, `if`, `match`, `case`, `default`, `using` e `end`.
- Outline do arquivo via Document Symbols.
- Busca de simbolos do workspace via Workspace Symbols.
- Formatacao pelo formatter oficial do compilador.
- `Zenith: New File` cria `.zt` com `namespace` calculado pelo caminho em `source.root`.
- Arquivos `.zt` vazios criados dentro de `source.root` recebem `namespace` automaticamente.
- Comandos rapidos: `Zenith: Check Project`, `Zenith: Build Project`, `Zenith: Run Project`.

## Instalar pacote local

Na raiz do repositorio:

```powershell
python tools/package_vscode_extension.py
code --install-extension dist\zenith-vscode-1.0.0.vsix --force
```

O pacote inclui:

- `bin/zt.exe`
- `bin/zt-lsp.exe`
- `stdlib/`
- `runtime/`

Ao executar `Zenith: Check Project`, `Zenith: Build Project` ou `Zenith: Run Project`, a extensao configura `ZENITH_HOME` para a pasta instalada da extensao.

## Uso em desenvolvimento

1. Compile o LSP:

```powershell
python tools/build_lsp.py
```

2. Abra `tools/vscode-zenith` no VSCode como pasta de extensao.
3. Pressione `F5` para abrir uma Extension Development Host.
4. Abra um arquivo `.zt`.

## Configuracao

- `zenith.lsp.path`: caminho para `zt-lsp` ou `zt-lsp.exe`.
- `zenith.cli.path`: caminho para `zt` ou `zt.exe`.
- `zenith.files.autoNamespaceOnCreate`: ativa ou desativa o `namespace` automatico em `.zt` vazio.

Sem configuracao manual, a extensao procura binarios nesta ordem:

1. pasta da extensao;
2. `bin/` dentro da extensao;
3. raiz do repositorio local;
4. `PATH` do sistema.

## Troubleshooting curto

### O LSP nao iniciou

Rode:

```powershell
python tools/build_lsp.py
```

Se o binario estiver fora da raiz do repositorio, configure `zenith.lsp.path`.

### Os comandos `check`, `build` ou `run` nao encontram `zt`

Compile o compilador:

```powershell
python build.py
```

Se `zt` estiver em outro lugar, configure `zenith.cli.path`.

### Um arquivo novo nao recebeu `namespace`

Verifique se o arquivo esta dentro de `[source].root` do `zenith.ztproj`.
Se nao houver projeto, a extensao usa `src` como raiz padrao do workspace.

## Limites conhecidos

- O indice atual e em memoria sobre arquivos `.zt` sincronizados.
- Diagnostics usam sincronizacao full-document.
- Builtins sugeridos incluem `print()`, `read()`, `debug()`, `type_name()`, `len()`, `to_text()`, `check()`, `todo()`, `unreachable()`, `panic()`, `f"..."` e conversoes numericas como `int()` e `float()`.
- Completion mostra assinatura, uso basico e descricao para builtins, funcoes da stdlib e funcoes do projeto.
- Comentarios `--` imediatamente acima de uma funcao enriquecem a ajuda exibida pelo autocomplete e pelo Signature Help.
- Imports da stdlib agora expoem membros publicos no autocomplete. Exemplo: depois de `import std.io as io`, `io.` sugere `print`, `read_line`, `read_all`, `input`, `output` e `stderr`.
- Todos os modulos em `stdlib/std` tem cobertura minima no autocomplete. Fachadas especiais como `std.list`, `std.map`, `std.set`, `std.jobs`, `std.channels`, `std.shared` e `std.atomic` tambem expoem a API reconhecida pelo compilador.
- Atalhos de stdlib importada aparecem de forma qualificada quando pertencem ao modulo importado. Builtins reais, como `print()`, continuam sem prefixo.
- Signature Help cobre chamadas diretas, `alias.func(...)` e `objeto.metodo(...)` para metodos `apply` indexados.
- References e Rename cobrem simbolos top-level indexados e simbolos locais do arquivo aberto.
- Hover e Go-to-definition tambem cobrem simbolos locais como parametros, `var`, `const`, bindings de `using`, closures e `match`.
- Completion de membro infere casos simples como `using valores = [1, 2]` para sugerir `valores.get(...)`.
- Completion de membro, Hover e Signature Help tentam resolver tipos explicitos com o checker antes de usar o fallback leve.
- O fallback leve continua ativo em codigo incompleto durante digitacao.
- Diagnostics identicos sao deduplicados antes de aparecer no editor.
- Completion de membro especializa campos simples de structs genericas.
- Completion sugere variantes de enum em `Enum.` e patterns em `case` dentro de `match`.
- Semantic Tokens usa o lexer oficial para a sintaxe atual; a grammar TextMate continua sendo o fallback visual.
- O namespace automatico usa `zenith.ztproj` e `[source].root`; se o arquivo ficar fora de `source.root`, a extensao nao altera o conteudo.
- Comandos `check/build/run` executam no terminal integrado.
