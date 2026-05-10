# Zenith Examples

Colecao de exemplos curtos para demonstrar a linguagem no GitHub.

## Como rodar

Use `check`, `build` e `run` em qualquer exemplo via projeto:

```bash
./zt.exe check examples/hello-world/zenith.ztproj
./zt.exe build examples/hello-world/zenith.ztproj
./zt.exe run examples/hello-world/zenith.ztproj
```

Ou rode um arquivo `.zt` avulso diretamente (single-file mode):

```bash
./zt.exe run examples/hello-world/src/app/main.zt
```

## Exemplos

1. `hello-world`
   - `print()` + `func main()` sem tipo de retorno
2. `structs-and-match`
   - `struct`, `enum`, `match`
3. `optional-and-result`
   - `optional<T>`, `result<T, E>`, operador `?`
4. `multifile-imports`
   - `namespace`, `import`, `public func`
5. `std-json`
   - `std.json.parse`, `std.json.stringify`, `std.json.pretty`
6. `extern-c-puts`
   - bloco `extern c` com chamada nativa
7. `c-bindings-sqlite3`
   - exemplo real de binding C via shim seguro para SQLite
8. `cli-calculator`
   - exemplo de ferramenta CLI com `result` para erro recuperavel
9. `file-processor`
   - leitura, limpeza e escrita de arquivo com `std.fs` e `std.text`
10. `todo-app`
   - `struct`, lista e contagem simples
11. `data-parser`
   - `std.json.parse`, `map.get`, `optional` e `result`

## Dica para README do GitHub

Para vitrine rapida, use:

1. `hello-world`
2. `cli-calculator`
3. `todo-app`
4. `data-parser`
