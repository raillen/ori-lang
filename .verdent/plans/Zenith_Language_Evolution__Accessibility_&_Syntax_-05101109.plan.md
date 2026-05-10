# Evolução da Linguagem Zenith: Acessibilidade e Legibilidade

## Arquitetura de Repositórios

```
zenith-lang/                    # Repositório C (atual)
├── compiler/                   # C compiler
├── runtime/c/                  # C runtime
├── stdlib/                     # Standard library em .zt
└── tooling/                    # Formatter, LSP, etc.

zenith-lang-rust/               # NOVO Repositório Rust
├── compiler/                   # Rust compiler (novo)
├── runtime/rust/               # Rust runtime (novo)
├── stdlib/                     # Standard library (compartilhado)
└── tooling/                    # Rust-based tooling
```

**Nota**: A stdlib `.zt` é compartilhada entre ambos. A linguagem fonte é única.

---

## 1. Melhorias em Traits + Apply

### 1.1 Problemas Atuais

O sistema atual funciona, mas tem limitações de legibilidade:

```zt
trait OrderedBox<T> where T is Addable and T is Comparable
    func get() -> T
end

apply OrderedBox<int> to IntBox
    func get() -> int
        return self.value
    end
end
```

### 1.2 Propostas de Melhoria

**A) Trazer o tipo receiver para a linha do trait (acessibilidade visual)**

```zt
trait Box<T> for T
    func get() -> T
end

apply Box<int> to IntBox
    func get() -> int
        return self.value
    end
end
```

- `for T` explicita o tipo receivers sem precisar do parâmetro genérico no nome
- Elimina ambiguidade: `OrderedBox<T>` era tipo ou trait?

**B) Renomear `apply Trait to Type` → `implement Trait for Type`**

```zt
-- Forma atual
apply Healable to Player

-- Proposta mais explícita
implement Healable for Player
```

- `implement` é mais auto-explicativo que `apply`
- Segue vocabulário mais comum (Rust, Java, C#)

**C) Trait com default implementation visível**

```zt
trait Drawable
    func draw(canvas: Canvas)
    
    func draw_with_border(canvas: Canvas, width: int)
        self.draw(canvas)
        canvas.draw_rect_outline(width)
    end
end
```

- Default methods visíveis no trait declaration
- Elimina necessidade de arquivo separado para default

**D) Associated types com nome explícito**

```zt
trait Container<Item>
    type Container has items: list<Item>
    
    func add(item: Item)
    func get(index: int) -> Item
end
```

- `type Container has items` define constraint estrutural
- Visualmente mais claro que `<Item>` apenas

---

## 2. Melhorias em Where Clauses

### 2.1 Problemas Atuais

```zt
func render_report<Item,Error>(
    title: text where validate.not_empty(title),
    subtitle: text where validate.not_empty(subtitle),
    items: list<Item>,
    render: func(Item) -> text
) -> text
where Item is TextRenderable<Item> and Error is TextRenderable<Error>
```

- Onde está o constraint? No parameter ou no return type? Precisa ler o código todo
- `: text where` parece decoration, não constraint

### 2.2 Propostas de Melhoria

**A) Separar visualmente parameter constraints com `requires`**

```zt
func render_report<Item, Error>(
    title: text
        requires validate.not_empty(title),
    subtitle: text
        requires validate.not_empty(subtitle),
    items: list<Item>,
    render: func(Item) -> text
) -> text
    requires Item is TextRenderable<Item>
    requires Error is TextRenderable<Error>
end
```

- `requires` é semanticamente mais claro que `where` para constraints de parâmetro
- Constraints em linhas próprias facilita scan

**B) Group constraints no início da função (pattern mais legível)**

```zt
func process<User, Result>(
    requires User is Fetchable and User is Serializable,
    requires Result is TextRenderable<Result>,
    user_id: int,
    config: Config
) -> Result
    -- body
end
```

- Todas as constraints no topo = primeiro scanning = reduz carga cognitiva
- `requires` explícito = constraint de valor
- `where` fica para generic type constraints

**C) Constraint blocks mais visuais**

```zt
func validate_and_process<Data>(
    data: Data,
    rules: ValidationRules
) -> result<Data, ValidationError>
    where Data is Validatable and Data is Serializable
    -- body
end
```

- Manter `where` para type constraints (decisão atual é boa)
- Usar `requires` para value-level constraints

**D) Contract syntax mais explícita (futuro)**

```zt
struct Player
    hp: int
        invariant hp >= 0 and hp <= 999
    name: text
        invariant validate.not_empty(name)
end
```

- `invariant` é semanticamente mais claro que `where` inline
- Visível durante scan do struct

---

## 3. Formalização de Generic Constraints

### 3.1 Problema Atual

Constraints são strings em implementation, sem verificação formal:

```zt
where T is Addable and T is Comparable
```

- `Addable` e `Comparable` são nomes literais
- Não há conexão explícita com trait definitions
- Parser não pode verificar sem semantic analysis

### 3.2 Propostas

**A) Explicit trait reference syntax**

```zt
func double<T>(value: T) -> T
    requires T implements core.Addable
    return value + value
end
```

- `implements` é mais explícito que `is`
- `core.Addable` é fully qualified = sem ambiguidade
- Pode coexistir com forma curta `is`

**B) Constraint inheritance syntax**

```zt
trait Readable
    func read() -> bytes

trait Parsable<Output> for Readable
    func parse(input: bytes) -> Output

-- Herda constraints automaticamente
trait JsonParsable<Output> for Parsable<Output>
    func parse_json(input: text) -> Output
```

**C) Visual group para multi-constraints**

```zt
struct Cache<Key, Value>
    where (
        Key is Hashable and
        Key is Equatable and
        Value is Disposable
    )
    items: map<Key, Value>
end
```

- Parênteses explicitam grouping
- Melhora scan em constraints complexos

---

## 4. Error Hierarchy Formalization

### 4.1 Problema Atual

```zt
result<int, text>
```

- `text` como error type não tem estrutura
- Não há hierarquia de erros nativa
- Propagation com `?` não dá contexto adicional

### 4.2 Propostas

**A) Error types básicos**

```zt
-- Erro básico primitivo
error NetworkError(code: int, message: text)
error ParseError(location: int, expected: text, got: text)
error ValidationError(field: text, reason: text)

-- Uso
func fetch(url: text) -> result<bytes, NetworkError>
func parse(input: text) -> result<Data, ParseError>
```

**B) Error composition com `and`**

```zt
func process(input: text) -> result<Output, ParseError and ValidationError>
```

- Permite que função retorne múltiplos tipos de erro
- Chamador pode pattern match em cada

**C) Error context propagation**

```zt
-- Com contexto adicional na propagação
const data: bytes = fetch(url)?
    .map_error(|e| NetworkError.with_context(e, url))
```

**D) Builtin error traits**

```zt
trait IsRecoverable
    func is_retryable() -> bool

trait HasDetails
    func details() -> map<text, text>
```

---

## 5. Specification Formal (SPEC.md)

### 5.1 Estrutura Proposta

```markdown
# Zenith Language Specification

## 1. Overview
- Philosophy: Reading-first, explicit, accessible
- Design goals for neurodivergent users

## 2. Lexical Structure
- Tokens, keywords, identifiers
- Operators and punctuation
- Whitespace and formatting rules

## 3. Syntax
- Grammar in EBNF
- Expression precedence table
- Reserved words and symbols

## 4. Types System
- Primitive types
- Generic types and constraints
- Trait system
- Type checking rules

## 5. Values and Variables
- Mutability model
- Initialization rules
- Scoping

## 6. Functions
- Parameter passing
- Return values
- Closures

## 7. Control Flow
- Match expressions
- Conditionals
- Loops

## 8. Traits and Apply
- Trait declarations
- Implementations
- Default implementations

## 9. Error Handling
- Optional<T>
- Result<T, E>
- Propagation

## 10. Memory Model
- Value semantics
- Ownership (future)
- Lifetime (future)

## 11. Standard Library
- Core prelude
- Common types
- Functions

## 12. Appendices
- BNF grammar
- Keyword list
- Operator precedence
```

### 5.2 Exemplos Canônicos

Cada decisão de linguagem deve ter:
1. Descrição do problema
2. Solução chosen com rationale
3. Exemplos canônicos (válido e inválido)
4. Edge cases documentados

---

## 6. Melhorias de Sintaxe Adicionais

### 6.1 Precedência de Operadores Explícita

**Problema**: Precedência não documentada formalmente

**Solução**:

```zt
-- Tabela de precedência explícita
-- highest
* / %      -- multiplicative
+ -        -- additive
< > <= >=  -- comparison
== !=      -- equality
and        -- logical and
or         -- logical or
=          -- assignment (lowest)

-- Para clareza, usar parênteses quando ambíguo
const result: int = (a + b) * c
```

### 6.2 Pattern Matching Explícito

```zt
-- Forma atual
match result
case ReadResult.Success(content)
    return content
case default
    return ""
end

-- Proposta com labels mais claros
match result
case .Success(content)
    return content
case .NotFound
    return ""
case .InvalidEncoding(message)
    return message
case .Unknown
    return ""
end
```

- `.Variant` é mais conciso e legível que `EnumName.Variant`
- Reduz repetição de nome do enum

### 6.3 Visibility Keyword Explícito

```zt
-- Forma atual
public func foo()
private func bar()

-- Proposta mais consistente
func public foo()
func private bar()

-- Ou manter current syntax mas com lista explícita
visibility public func foo()
visibility private func bar()
```

### 6.4 Function Type Annotation Explícita

```zt
-- Forma atual
render: func(Item) -> text

-- Proposta mais legível
render: function takes(Item) returns(text)
```

- `function takes ... returns ...` é mais explícito que `func(...) -> ...`
- Melhora para leitores com dyslexia

### 6.5 Named Parameters Visíveis

```zt
-- Com labels visuais
func create_user(
    name: text,
    age: int,
    email: text
)

-- Chamada atual
create_user("Julia", 25, "julia@example.com")

-- Proposta com labels explícitos
create_user(
    name: "Julia",
    age: 25,
    email: "julia@example.com"
)

-- Ou prefixo visual
create_user(name: "Julia", age: 25, email: "julia@example.com")
```

### 6.6 Block Labels para organização visual

```zt
-- Labels visuais para blocos longos
section Initialization
    setup_variables()
    load_config()
end section

section Processing
    process_data()
    validate_results()
end section

section Cleanup
    free_resources()
end section
```

- Auxilia navegação em arquivos longos
- Facilita identificação de seções para TDAH

---

## 7. Resumo das Alterações Prioritárias

### Legibilidade Imediata (MVP++)
1. `implement Trait for Type` (renomear apply)
2. `requires` para parameter constraints
3. Constraint blocks com parênteses `where ( ... )`
4. `.Variant` shorthand em match
5. `function takes ... returns ...` para function types

### Estrutural (Próxima versão)
6. Trait `for Type` syntax (receiver explicito)
7. Default implementations visíveis
8. Associated types estruturados
9. Error type definitions
10. Specification formal (SPEC.md)

### Avançado (Future)
11. `invariant` para field constraints
12. Visibility como prefixo consistente
13. Block labels para organização
14. Named parameters explícitos

---

## 8. Compatibilidade

Todas as mudanças devem ser backward-compatible ou ter migration path claro:

```zt
-- Old syntax ainda funciona, mas novo syntax é preferred
apply Healable to Player  -- ainda válido
implement Healable for Player  -- novo syntax preferido

-- Migration automático via formatter
```

- Formatter atualiza syntax automaticamente
- Deprecation warnings para old syntax
- Migration guide documentado