# Errors, Null, Void — mapa mental Ori

> Guia pedagógico (superfície **S3 / `0.3.0`** + inferência local `0.3.1`/B).  
> Normativo: [09-errors](../spec/09-errors.md), [04-types](../spec/04-types.md), [01-overview](../spec/01-overview.md).

## Quatro conceitos, quatro papéis

| Conceito | Papel | Quando usar |
|----------|--------|-------------|
| **`void`** | “Nada útil retornado” | Tipo de retorno de `main() -> void`, side effects only |
| **`optional[T]`** | “Pode não haver valor” | Parsing, lookup, EOF — ausência **não** é erro |
| **`result[T, E]`** | “Sucesso ou falha explícita” | I/O, validação, APIs que podem falhar com motivo |
| **`check`** | Pré-condição / contrato | Invariantes em runtime (`check cond, "msg"`) |

Ori **não tem null**. Use `none` dentro de `optional[T]` ou `err(...)` dentro de `result[T, E]`.

## `void`

```ori
greet() -> void
    io.println("hello")
end
```

- Não confunda com `optional`: `void` não é um valor que você armazena.
- Descartar `result` sem tratar emite `type.unused_result` (warning).

## `optional[T]`

```ori
find_user(id: int) -> optional[string]
    if id == 0
        return none
    end
    return some("alice")
end
```

- `none` = ausência esperada.
- `if some(x) = expr` / `match` desempacota com segurança.
- **`try expr`** em `optional` propaga `none` (ver cap. 09). Postfix `expr?` foi
  **removido** no S3 (`parse.question_propagate_removed`).

## `result[T, E]`

```ori
read_config(path: string) -> result[string, string]
    return fs.read_text(path)
end
```

- `ok(value)` ou `err(reason)`.
- Trate com `match`, **`try expr`**, ou helpers Layer 2 (`parse_int_or`, `get_or`).
- Preferir `result` a `bool` para operações que falham.

## `check`

```ori
divide(a: int, b: int) -> int
    check b != 0, "division by zero"
    return a / b
end
```

- Falha de contrato aborta com diagnóstico runtime (não é `result`).
- Diferente de `if`: `check` documenta invariantes.

## Comparativo rápido

| Situação | Tipo / construção |
|----------|-------------------|
| Função só imprime | `-> void` |
| “Não achei” sem erro | `optional[T]` + `none` |
| Falha com mensagem | `result[T, string]` |
| Pré-condição que deve ser verdade | `check` |

## Ferramentas DX

```bash
ori explain name.undefined
ori explain type.type_mismatch
ori doctor
ori summary path/to/main.orl
```

Ver catálogo completo: [`13-error-catalog.md`](../spec/13-error-catalog.md).
