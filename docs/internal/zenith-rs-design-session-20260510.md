# Zenith Rust — Sessão de Design e Análise Técnica
> Data: 10 de maio de 2026
> Contexto: análise do compilador C + decisões de design para o compilador Rust

---

## Parte I — Diagnóstico do Compilador C

### Pipeline de Compilação
```
Source (.zt) → Lexer → AST → Binder + Type Checker → HIR → ZIR → Verifier → C Emitter → gcc/clang → native
```
Três IRs: AST → HIR → ZIR. Bem intencionado, limites tênues na prática.

### Arquivos Críticos por Tamanho
| Arquivo | Linhas | Risco |
|---|---|---|
| `targets/c/emitter.c` | **14.020** | Global state, não paralelizável |
| `hir/lowering/from_ast.c` | **5.740** | Type checking + lowering misturados |
| `driver/lsp.c` | **5.977** | Acoplamento total com compilador |
| `runtime/c/zenith_rt_outcome.c` | **4.192** | Macro explosion |
| `runtime/c/zenith_rt_templates.h` | **2.638** | 218KB de macros C |

### Problemas Estruturais Graves
- 4 globals estáticos no `emitter.c` tornam compilação paralela impossível
- `zt_type` usa `char *name` livre para distinção de tipos — comparações por `strcmp`
- ARC manual sem detecção de ciclos
- Runtime de generics via macros C de 218KB — unmaintainable
- `zenith_rt_outcome.c` com 291 símbolos — código template expandido manualmente

### Sistema de Diagnósticos (ponto forte)
- Codes estruturados, severity, effort levels (`quick_fix`, `moderate`, `requires_thinking`)
- `zt_cog_profile` para ajuste de verbosidade por perfil cognitivo — inovador
- Fraqueza: `char message[512]` pode truncar; único `span` por diagnóstico

---

## Parte II — Decisão: Dois Repositórios Separados

```
github.com/zenith-lang/zenith       # C compiler (estável, referência)
github.com/zenith-lang/zenith-rs    # Rust compiler (evolução ativa)
```

- `stdlib/` e `tests/conformance/` compartilhados (submodule ou symlink)
- C compiler congela em v1.x, recebe só bugfixes
- Rust compiler valida contra a conformance suite do C como oracle

---

## Parte III — Clean Slate em Rust (Decisão)

**Recomendação aceita: reescrita limpa em Rust**, sem migração incremental do C.

Razões:
- Mudanças de linguagem planejadas invalidam o C como base de porte
- Sem FFI complexity
- Design para a linguagem NOVA, não para a velha

### Estrutura de Crates
```
zenith-rs/crates/
├── zenith-lexer/
├── zenith-ast/
├── zenith-parser/
├── zenith-types/          # TypeId interning, sem char* comparisons
├── zenith-hir/
├── zenith-codegen-c/
├── zenith-runtime/        # Arc<T>, coleções Rust reais, sem macros template
├── zenith-diagnostics/    # miette/ariadne
├── zenith-lsp/            # tower-lsp
└── zenith-driver/
```

---

## Parte IV — Mudanças de Linguagem para a Versão Rust

### `text` → `string` ✅ Aceito
- `string` é o termo universal (Python, Go, JavaScript, Java, Swift, Kotlin, Rust)
- Para neurodivergentes com qualquer exposição prévia, reduz fricção cognitiva
- Migration path: `zt fmt --migrate-string` no C compiler

### `apply Trait to Type` → `implement Trait for Type` ✅ Aceito
- Subject-verb-object linear: "Type implementa Trait"
- Familiar de Rust, Java, Swift — reduz carga cognitiva de aprendizado
- `apply` permanece válido no C compiler com deprecation warning

### `where` multi-linha com `and` ✅ Aceito
```zt
func process<T>(value: T) -> T
    where
        T is Comparable
        and T is Addable
    -- body
end
```

### `where ( ... )` grouping ✅ Aceito
```zt
struct Cache<Key, Value>
    where (
        Key is Hashable
        and Key is Equatable
        and Value is Disposable
    )
    items: map<Key, Value>
end
```

### `alias` para type aliases ✅ Aceito (novo)
```zt
alias UserId = int
alias UserMap = map<string, User>
```
Mais semântico que `type` — fica claro que é um alias, não uma nova declaração.

### `.Variant` shorthand em match ✅ Aceito
```zt
match result
case .Success(content):
    return content
case .NotFound:
    return none
end
```

### `mutating func` em vez de `func increment(mut self)` ✅ Aceito
```zt
struct Counter
    value: int

    mutating func increment()
        self.value = self.value + 1
    end

    func get_value() -> int
        return self.value
    end
end
```
Mutation intent no nível da declaração, não enterrado nos parâmetros.

### `is` como operador de tipo em expressões ✅ Aceito (novo)
```zt
if shape is Circle:
    draw_with_radius(shape)
end
```

### Error handling com `struct` + `implement Error for` ✅ Aceito
Sem nova keyword `error` — usar struct + trait:
```zt
struct NetworkError
    code: int
    message: string
end

implement Error for NetworkError
    func message() -> string
        return f"Network error {self.code}: {self.message}"
    end
end
```

### Named args obrigatórios quando 3+ params do mesmo tipo ✅ Aceito (linter rule)
```zt
copy_rect(x: 10, y: 10, width: 200, height: 100)
```

---

## Parte V — O que NÃO Mudar ou Adiar

| Decisão | Status | Razão |
|---|---|---|
| `func(T) -> R` para callable types | Manter | Visualmente distinto, melhor para dislexia |
| `where` para type constraints | Manter | Conciso, natural |
| `public func` placement | Manter | Consistente |
| `section` como keyword | Rejeitar | Comentários convencionais suficientes |
| `requires` vs `where` | Rejeitar | Custo > benefício |
| `function takes ... returns ...` | Rejeitar | Mais verboso, não mais acessível |
| Associated types no trait | Adiar (pós-v2) | HKT territory |
| Trait inheritance | Rejeitar | Contradiz composição |
| `error` keyword de declaração | Rejeitar | Redundante com struct + implement |

---

## Parte VI — Plano de Migração Recomendado

### Estratégia: Híbrida Incremental (para o C), Clean Slate (para o Rust)

**Milestone 0 — Estabilização C (1-2 meses)**
- Eliminar globals estáticos do emitter
- Quebrar `from_ast.c` em type checking + lowering
- Substituir `char message[512]` por alocação dinâmica
- Snapshot tests do output do emitter
- Fuzzing do parser na CI

**Milestone 1 — Runtime Rust (2-3 meses)**
- `Arc<T>` nativo, cycle detection via `Weak<T>`
- Coleções Rust reais sem macros template
- Expor via `cbindgen` com mesma interface C
- Substitui `zenith_rt_templates.h` (218KB) e `zenith_rt_outcome.c` (212KB)

**Milestone 2 — Emitter Rust (3-4 meses)**
- Deserializar ZIR via FFI, reescrever emitter em Rust modular
- Validar: `zt build --emit-c-legacy` == `zt build --emit-c-rust`

**Milestone 3-4 — HIR/ZIR + Frontend Rust (4-6 meses)**
- TypeId interning em vez de `char *type_name`
- Lexer, parser, binder, checker em Rust

**Duração estimada total**: 14-20 meses solo para paridade completa

---

## Parte VII — Spec Formal (Prioridades Antes do Compilador Rust)

Deve existir antes de escrever o parser:
1. Gramática EBNF (extraível do parser C)
2. Tabela de precedência de operadores (explícita)
3. Regras de exhaustiveness para match (algoritmo)
4. Semântica de `?` (tipos aceitos, return type compatibility)
5. Invariants do sistema de tipos

Durante o desenvolvimento:
6. Semântica de `using` cleanup order
7. Algoritmo de trait resolution
8. Regras de monomorphization
9. ABI do runtime

---

## Comparação C vs Rust (síntese)

| Dimensão | C | Rust |
|---|---|---|
| Segurança de memória | Fraca (UB, ARC manual) | Forte (ownership, zero UB por construção) |
| Manutenibilidade | Baixa (14k-line files, globals) | Alta (tipos algébricos, sem globals) |
| Testabilidade | Baixa (globals impedem paralelos) | Alta (cada módulo isolável) |
| Tooling | Fraco | Excelente (Clippy, rustfmt, cargo fuzz, miri) |
| Facilidade de evolução | Baixa (novo backend = crise) | Alta (traits permitem backends plugáveis) |
| Performance | Excelente | Igual ou melhor |

---

## Parte VIII — Decisões de Linguagem (Sessão Estendida)

### Renomeações Confirmadas
- `text` → `string` (tipo primitivo)
- `TextRepresentable` → `Displayable` (trait)
- `to_text()` → `to_string()` (função contextual builtin)
- `std.text` / `text.zt` → `std.string` / `string.zt`
- `apply Trait to Type` → `implement Trait for Type`
- `type Alias = T` → `alias Alias = T`

### Sem `_` Placeholder
Tipos sempre explícitos, sem exceção. Sem inferência exposta ao usuário.

### Mutabilidade: `mut func`
```zt
struct Counter
    value: int

    mut func increment()
        self.value = self.value + 1
    end

    func get_value() -> int
        return self.value
    end
end
```
- `mut func` = mutation intent no nível da declaração (não nos params)
- Chamar `mut func` em binding `const` → erro de compilação
- `var` continua sendo o marcador de mutabilidade para bindings

### Igualdade Estrutural (`==` por default)
- Todos os tipos têm `==` e `!=` automaticamente (igualdade estrutural)
- `list<T>`: elemento a elemento em ordem
- `map<K,V>`: mesmas chaves e valores (sem ordem)
- `set<T>`: mesmos elementos (sem ordem)
- `optional<T>`: `none == none`; `some(a) == some(b)` iff `a == b`
- `any<Trait>`: `==` é erro de compilação (sem igualdade em dynamic dispatch)
- Override: `implement Equatable for T` com `func equals(other: T) -> bool`

### Closures: keyword `do`
`func` = declaração nomeada. `do` = função anônima / closure.

```zt
-- Tipo (não muda)
var handler: func(string) -> bool

-- Criação inline
handler = do(input: string) => len(input) > 0

-- Criação em bloco
handler = do(input: string)
    const trimmed: string = input.trim()
    return len(trimmed) > 0
end

-- Como argumento (tipo inferido)
const valid = iter.filter(names, do(n: string) => len(n) > 0)
```
- Return type omitido quando inferível
- Captura por valor imutável (default)
- Capturar `var` mutável → erro de compilação

### Range Sempre Inclusivo (`a..b`)
- `a..b` sempre inclui `a` e `b` (sem `..=` separado)
- Consistente com o comportamento atual de slices (`value[0..finish]` inclui `finish`)
- `for i in 0..9` → 10 iterações: 0,1,2,...,9
- `items.indices()` → shorthand para `0..len(items)-1` (evita off-by-one)
- `items[..]` → shorthand para slice completo
- `range<T>` como tipo de primeiro grau com `.length()`, `.start`, `.end`, `.contains(v)`

### Loop Infinito: `loop ... end`
```zt
loop
    const input: string = console.read_line()?
    if input == "quit"
        break
    end
    process(input)
end
```
- `while true ... end` continua válido no C compiler; `loop` é canônico no Rust compiler
- `zt fmt` migra `while true` para `loop` automaticamente

### Keyword `func` Mantida
- `func` é a keyword correta para declaração de função
- `fn` (muito curto), `def` (genérico), `function` (muito longo) rejeitados

### Triple-quote Strings
```zt
const sql: string = """
    SELECT *
    FROM users
    WHERE active = true
    """
-- Indentação stripped baseada no nível do """ de fechamento
```

### Functional Patterns em `std.iter` (Rust compiler)
Genéricos reais substituem as versões monomórficas `map_int`, `filter_int`, `reduce_int`:
```zt
-- Generic para qualquer T e R
iter.map<T, R>(values: list<T>, mapper: func(T) -> R) -> list<R>
iter.filter<T>(values: list<T>, predicate: func(T) -> bool) -> list<T>
iter.reduce<T, R>(values: list<T>, initial: R, reducer: func(R, T) -> R) -> R
iter.flat_map / find / any / all / zip / flatten / take / skip / partition / count_where
```
- Avaliação EAGER por padrão (retorna `list<T>`, não lazy iterator)
- `lazy<T>` existente para casos que precisam de lazy explícito

### Features do Rust Incorporadas ✅
```zt
-- if some binding
if some(user) = get_user(id)
    greet(user)
end

-- while some binding
while some(line) = reader.next_line()
    process(line)
end

-- Struct update syntax
const debug: Config = original with
    verbose: true
end

-- Iterable<T> trait (for ... in ... para user types)
trait Iterable<Item>
    mut func next() -> optional<Item>
end

-- From<T> conversion trait
trait From<Other>
    func from(value: Other) -> Self
end

-- Default trait
trait Default
    func default() -> Self
end
```

### Features Rejeitadas / Adiadas
- Sem `_` placeholder para inferência
- Sem `unsafe` blocks (`extern c` é o mecanismo de escape)
- Sem macros de qualquer tipo
- Sem lifetime explícito (ARC + value semantics gerencia isso)
- `const` generics: adiar pós-v2
- Destructuring em `const`/`var`: adiar

### Operator Precedence (tabela formal)
| Nível | Operadores | Associatividade |
|---|---|---|
| 1 (mais alto) | `.field` `call()` `[index]` | esquerda |
| 2 | `?` propagation | postfix |
| 3 | `-` unário, `not` | prefixo |
| 4 | `*` `/` `%` | esquerda |
| 5 | `+` `-` | esquerda |
| 6 | `==` `!=` `<` `<=` `>` `>=` | sem encadeamento* |
| 7 | `and` | esquerda |
| 8 | `or` | esquerda |
| 9 (mais baixo) | `\|>` pipe | esquerda |

*`a < b < c` é ERRO de compilação (não `(a < b) < c` silencioso)

### Namespace Obrigatório — Mantido
- `namespace` deve ser a primeira declaração do arquivo
- Para scripts avulsos: `namespace script` implícito com nota informativa (não silencioso)
- O `zt new` e o LSP inserem automaticamente

### Error Hierarchy
- Sem keyword `error` para declarações — usar `struct` + `implement Error for`
- `trait Error` na stdlib com `func message() -> string`
- Enum de errors para union de tipos de erro (exhaustive match garantido)

### Variadic Parameters
```zt
-- Sufixo ... no último parâmetro
public func log(message: string, values: any<Displayable>...)

-- Spread de lista para variadic
const parts: list<string> = ["a", "b", "c"]
concat(..parts)
```

### Decisões Pendentes (próximas sessões)
- Organização de projeto e hierarquia de namespaces vs filesystem
- Sistema `attr` (anotações/decoradores)
- Modelo de concorrência (excluindo Borealis)
- Closures: captura de `var` — algum mecanismo futuro?
- Módulo `std.string` completo (migração de `std.text`)

---

## Parte IX — Identidade da Nova Linguagem

### Nome: **Ori**

**Origem:** Hebraico — אוֹרִי = "minha luz"
**Filosofia:** Clareza como guia. Iluminar o caminho de quem acha programação inacessível.

### Identidade Técnica

| Item | Valor |
|---|---|
| Nome | Ori |
| Extensão de arquivo | `.ori` |
| Binário CLI | `ori` |
| Repositório GitHub | `ori-lang/ori` (sugestão) |
| LSP | `ori-lsp` |
| Formatter | `ori fmt` |

### Comandos CLI canônicos
```
ori new <project>     -- criar projeto
ori build             -- compilar
ori run               -- compilar e executar
ori check             -- type check sem compilar
ori test              -- executar testes
ori fmt               -- formatar código
ori doc               -- gerar documentação
```

### Relação com Zenith
- Ori nasce como linguagem independente, sem contrato de compatibilidade com Zenith
- O código Zenith existente serve como ponto de partida e referência, não como contrato
- A stdlib `.zt` é adaptada para `.ori` com todas as mudanças de linguagem aplicadas
- Zenith (compilador C) permanece estável em repositório separado

### Identidade — Decisões Confirmadas
- Stdlib: `ori.*` (ex: `ori.string`, `ori.io`, `ori.iter`, `ori.math`)
- Arquivo de projeto: `ori.proj`
- Package manager: integrado no `ori` CLI (`ori add`, `ori remove`, `ori publish`)
- Extensão de arquivo: `.ori`

### Decisões de identidade pendentes
- Package registry: formato e localização
- Site e documentação pública
