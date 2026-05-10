# Guia Prático e Teórico - Sintaxe Zenith v2

Data: 18 de Abril de 2026

## Resumo Executivo

Este documento fornece um guia prático integrado para entender e utilizar a sintaxe de Zenith v2. Combina teoria de design com exemplos práticos.

---

## Parte 1: Princípios Fundamentais

### Por que "Reading-First"?

A sintaxe de Zenith foi deliberadamente projetada para otimizar **leitura sobre escrita**:

```zt
-- ✓ ZENITH: Alto contraste visual, fácil de ler
public func process_user(user: User) -> result<text, int>
    if user.is_valid
        return success(user.name)
    else
        return error(404)
    end
end

-- vs. outras linguagens: densa, simbólica
-- pub fn process_user(user: User) -> Result<String, i32> {
--     if user.is_valid() { Ok(user.name) } else { Err(404) }
-- }
```

**Por quê?** Estudos mostram que programadores gastam ~70% do tempo **lendo** código, não escrevendo. Otimizar para leitura beneficia:
- Pessoas com dislexia/ADHD (reduz carga visual)
- Todos (aumenta compreensão rápida)
- Manutenção (código envelhecido é relido frequentemente)

---

## Parte 2: Estrutura Básica do Arquivo

### Template Canônico

```zt
-- Opcional: comentário de implementação
namespace org.project.module

import std.io as io
import org.shared.utils as utils

public func main() -> int
    return 0
end
```

### Regra de Ouro: Declarações em Ordem

1. **Comentário** (opcional, antes de namespace)
2. **Namespace** (obrigatório, primeiro)
3. **Imports** (depois de namespace)
4. **Implementações** (funcs, structs, traits, applies)

---

## Parte 3: Tipagem Explícita

### Filosofia: "Type Clarity"

Nem tipo implícito, nem verbosidade excessiva. Explicitamente, mas sem ruído:

```zt
-- ✓ Claro e conciso
const player_name: text = "Julia"
var hit_points: int = 100

-- Não permite inferência (MVP)
-- const name = "Julia"  -- Erro: tipo ambíguo

-- Não permite abreviações
-- let hp = 100  -- Não canônico
```

### Mapeamento de Tipos

| Zenith | C Equivalente | Uso |
|--------|---------------|-----|
| `text` | `char*` | UTF-8 strings |
| `bytes` | `uint8_t*` | Binary data |
| `int` | `int64_t` | Integer padrão |
| `float` | `double` | Float padrão |
| `bool` | `_Bool` | Valores true/false |
| `list<T>` | "Dynamic array" | Coleções |
| `map<K,V>` | Hash table | Dicionários |
| `optional<T>` | Pointer/null check | Valores opcionais |
| `result<T,E>` | Tagged union | Erro recuperável |

---

## Parte 4: Coleções e Value Semantics

### Lista: Imutável para Leitura, Explícita para Mutação

```zt
-- Literal imutável
const ports: list<int> = [3000, 3001, 3002]
-- ports[0] = 8000  -- Erro! (const)
-- len(ports) OK, indexação OK, slicing OK

-- Literal mutável
var scores: list<text> = ["A", "B", "C"]
scores[0] = "B"           -- OK: atribuição de elemento
scores.append("D")        -- OK: mutação explícita
scores.prepend("Z")       -- OK: outro tipo de mutação

-- Cópia com value semantics
var scores2: list<text> = scores  -- Deep copy via COW
scores2.append("E")               -- scores não é afetado!
```

### Mapa: Lookup Seguro via Optional

```zt
const config: map<text, int> = {"timeout": 5000, "retries": 3}

-- Direto (panics se não existe)
const timeout: int = config["timeout"]      -- OK: 5000

-- Seguro (retorna optional)
const port: optional<int> = config.get("port")
match port
case value p:
    io.print(fmt "Port: {p}")
case none:
    io.print("Port not configured")
end
```

### Copy-on-Write em Ação

```zt
struct Inventory
    items: list<text>
end

var inv1: Inventory = Inventory(items: ["sword", "shield"])
var inv2: Inventory = inv1          -- Shallow copy (shared list)

-- Append a inv2 **triggers deep copy** internamente
inv2.items.append("potion")

-- inv1.items ainda ["sword", "shield"]
-- inv2.items é ["sword", "shield", "potion"]
-- Otimizado: cópia ocorre, não antes
```

---

## Parte 5: Funções e Argumentos Nomeados

### Default + Named Arguments = API Clara

```zt
func create_connection(
    host: text,
    port: int = 5432,
    timeout: int = 30,
    retries: int = 3
) -> result<Connection, text>
    -- Implementação
end

-- Uso com defaults claros
const conn1: Connection = create_connection("localhost")?

-- Uso com overrides específicos
const conn2: Connection = create_connection(
    host: "remote.server",
    timeout: 60,
    retries: 5
)?

-- Ordem não importa quando nomeados
const conn3: Connection = create_connection(
    retries: 10,
    host: "db.local",
    port: 3306
)?
```

**Regra:** Após um nome, todos os seguintes devem ser nomeados:
```zt
-- ✓ OK
create_connection("localhost", port: 5432)

-- ✗ Erro: após named argument, só nomes
-- create_connection("localhost", 5432)
```

---

## Parte 6: Opcionais vs Resultados

### Optional: Ausência

```zt
func find_user(id: int) -> optional<User>
    if user_exists(id)
        return User(...)
    else
        return none
    end
end

-- Uso
match find_user(42)
case value user:
    io.print(user.name)
case none:
    io.print("User not found")
end
```

### Result: Erro com Contexto

```zt
func load_config(path: text) -> result<Config, text>
    if file_exists(path)
        return success(parse_config(path))
    else
        return error("File not found")
    end
end

-- Uso com propagação
func initialize_app() -> result<void, text>
    const config: Config = load_config("app.toml")?
    const db: Database = connect_to_db(config)?
    return success()
end

-- Sem propagação, match manual
match initialize_app()
case success():
    io.print("App ready")
case error(msg):
    io.print(fmt "Init failed: {msg}")
end
```

### Diferença Conceitual

| Aspecto | Optional | Result |
|---------|----------|--------|
| **Semântica** | Valor pode/não pode existir | Operação pode suceder/falhar |
| **Quando usar** | Busca, lookup | Operações I/O, parse, validação |
| **Retorno sucesso** | (implícito) | `success(value)` |
| **Retorno falha** | `none` | `error(reason)` |

---

## Parte 7: Structs e Value Semantics Garantida

### Construção e Cópia

```zt
struct Hero
    name: text
    hp: int where hp > 0
    equipment: list<text> = []
end

const hero1: Hero = Hero(name: "Aragorn", hp: 100)
var hero2: Hero = hero1                  -- Deep copy!
hero2.equipment.append("Sword")

-- hero1.equipment ainda []
// hero2.equipment é ["Sword"]

-- Mutadores modificam cópia
hero2.hp = 80  -- Não afeta hero1
```

### Sem Referências Implícitas

```zt
-- Zenith não permite "passing by reference" implícito
var player: Player = Player(...)
process(player)             -- Passa por cópia

-- Dentro de process:
func process(p: Player)     -- p é cópia
    p.hp = 50               -- Não afeta player original
end
```

---

## Parte 8: Traits e Apply (Composição)

### Trait = Contrato

```zt
trait Combatant
    mut func take_damage(amount: int)
    func get_defense() -> int
end

trait Displayable
    func display() -> text
end
```

### Apply = Implementação

```zt
struct Enemy
    name: text
    health: int
end

-- Método imutante
apply Enemy
    func get_name() -> text
        return self.name
    end
end

-- Método mutante (mut func permite modificar self)
apply Enemy
    mut func reduce_health(amount: int)
        self.health = self.health - amount
    end
end

-- Implementa trait
apply Combatant to Enemy
    mut func take_damage(amount: int)
        self.reduce_health(amount)
    end
    
    func get_defense() -> int
        return 5
    end
end

-- Uso
var enemy: Enemy = Enemy(name: "Goblin", health: 20)
enemy.take_damage(5)        -- Trait method
io.print(enemy.get_name())  -- Simple method
```

---

## Parte 9: Controle de Fluxo

### If-Else: Obrigatório ser Boolean

```zt
var health: int = 80

-- ✓ Correto: condição bool explícita
if health > 0
    io.print("Alive")
else if health == 0
    io.print("Dead")
else
    io.print("Zombie")
end

-- ✗ Erro: no truthy (em outras linguagens 0/null = false)
-- if health then ... end

-- ✗ Erro: não há unless em MVP
-- unless fatigue < 100 then ... end
```

### While: Padrão Loop Clássico

```zt
var turn: int = 0
while turn < 5
    io.print(fmt "Turn {turn}")
    turn = turn + 1
end
```

### For: Orientado a Coleção

```zt
var players: list<text> = ["Alice", "Bob", "Charlie"]

-- Simples
for player in players
    io.print(player)
end

-- Com índice
for player, idx in players
    io.print(fmt "{idx}: {player}")
end

-- Key-value (maps)
var scores: map<text, int> = {"Alice": 100, "Bob": 85}
for name, score in scores
    io.print(fmt "{name}: {score}")
end
```

### Repeat: Contagem Fixa

```zt
repeat 3 times
    io.print("Tick")
end

-- Variável para count
var n: int = 5
repeat n times    -- Avaliado uma vez
    io.print("Tock")
end
```

---

## Parte 10: String Interpolation com fmt

### Lazy Strings (MVP)

```zt
const value: int = 42
const simple: text = "Answer: 42"  -- Literal, sem interpolação

-- fmt para interpolação
const interpolated: text = fmt "Answer: {value}"

-- Expressões dentro
const player: Player = Player(...)
const message: text = fmt "Player {player.name} has {player.hp} HP"

-- Conversão explícita fora de fmt
const text_value: text = to_text(value)
const report: text = "Value=" + text_value
```

---

## Parte 11: Enums e Pattern Matching

### Enum Simples

```zt
enum Direction = North, South, East, West
enum Status = Active, Paused, Stopped

const dir: Direction = Direction.North
const state: Status = Status.Active
```

### Enum com Payloads

```zt
enum Result<T, E>
    success(value: T)
    error(message: E)
end

-- Construção
const ok: Result<int, text> = Result.success(value: 42)
const err: Result<int, text> = Result.error(message: "Failed")
```

### Match

```zt
enum LoadResult = Loaded(text), Failed(int)

const result: LoadResult = Loaded("data")

match result
case Loaded(content):
    io.print(content)
case Failed(code):
    io.print(fmt "Error code: {code}")
end

-- Match com múltiplos casos
match status_code
case 200, 201:
    handle_success()
case 404, 500:
    handle_error()
case default:
    handle_unknown()
end
```

---

## Parte 12: Generics e Constraints

### Declaração Simples

```zt
func find<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable<Item>
    for item in items
        if item == target
            return true
        end
    end
    return false
end

-- Chamada com inferência
const nums: list<int> = [1, 2, 3]
const found: bool = find(nums, 2)  -- Item = int inferido
```

### Struct Genérico

```zt
struct Container<T>
    items: list<T>
end

apply Container<T>
where T is Equatable<T>
    func contains(target: T) -> bool
        for item in self.items
            if item == target
                return true
            end
        end
        return false
    end
end
```

---

## Parte 13: Value-Level Where Contracts

### Definição em Campos

```zt
struct User
    username: text where len(it) > 0 and len(it) <= 20
    age: int where it >= 0 and it < 150
    score: float where it >= 0.0
end

-- Construção direta (não result)
const user: User = User(username: "alice", age: 25, score: 95.5)

-- Violação causa panic
-- const invalid: User = User(username: "", age: 25, score: 95.5)  -- PANICS!
```

### Definição em Parâmetros

```zt
func allocate_memory(bytes: int where bytes > 0)
    -- bytes garantido > 0 antes de usar
end

func set_percentages(p1: int, p2: int where p1 + p2 <= 100)
    -- p1 + p2 validado em runtime
end
```

### Para Validação Recuperável: use result<T, E>

```zt
func try_create_user(
    username: text,
    age: int
) -> result<User, text>
    if len(username) == 0
        return error("Name cannot be empty")
    end
    if age < 0
        return error("Age cannot be negative")
    end
    return success(User(username: username, age: age, score: 0.0))
end
```

---

## Parte 14: Panic vs Recoverable Error

### Panic: Fatal, não recuperável

```zt
func must_parse(s: text) -> int
    match parse_int(s)
    case success(n):
        return n
    case error(msg):
        panic(fmt "Failed to parse: {msg}")
    end
end
```

### Recoverable: Use result<T, E>

```zt
func parse_int(s: text) -> result<int, text>
    if is_valid_integer(s)
        return success(to_int_unsafe(s))
    else
        return error("Not a valid integer")
    end
end
```

---

## Parte 15: Padrões de Acessibilidade

### 1. Alto Contraste Visual

```zt
-- ✓ Blocos delimitados, fácil de scanear
if condition
    statement1
    statement2
else
    statement3
end

-- ✗ Simbólico (menos scanável)
-- if condition { statement1; statement2; } else { statement3; }
```

### 2. Palavras em vez de Símbolos

```zt
-- ✓ READABLE
if ready and not waiting
    process()
end

-- ✗ SYMBOLS
-- if (ready && !waiting) { process(); }
```

### 3. Tipagem Explícita

```zt
-- ✓ CLEAR
const user_count: int = 42
var player_name: text = "Alice"

-- ✗ AMBIGUOUS (requer adivinhação)
-- let count = 42
-- var name = "Alice"
```

---

## Parte 16: Checklist - Escrevendo Código Zenith

- [ ] Arquivo começa com `namespace`?
- [ ] Imports seguem namespace?
- [ ] Todas as funções têm tipos explícitos (`name: Type`)?
- [ ] Funções sem retorno omitem `->`?
- [ ] Collections mutadas via `append()`/`prepend()`?
- [ ] Opcionais usam `match case value / case none`?
- [ ] Resultados usam `try_*()` para função e `?` para propagação?
- [ ] Struct mutations via `apply` methods?
- [ ] Traits implementadas com `apply Trait to Type`?
- [ ] Métodos mutantes marcados com `mut func`?
- [ ] Value contracts em `where` para campos/parâmetros?
- [ ] Parâmetros defaults no final da lista?
- [ ] Argumentos nomeados para clarity?

---

## Parte 17: Resumo Rápido de Sintaxe

```zt
-- NAMESPACE & IMPORTS
namespace app.game
import std.io as io

-- STRUCTS com contratos
struct Hero
    name: text where len(it) > 0
    hp: int = 100 where hp > 0
end

-- TRAITS para comportamento
trait Healable
    mut func heal(amount: int)
end

-- APPLY para métodos e trait impl
apply Hero
    func status() -> text
        return fmt "{self.name}: {self.hp} HP"
    end
end

apply Healable to Hero
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end

-- FUNÇÕES com defaults e result
func create_hero(
    name: text,
    hp: int = 100
) -> result<Hero, text>
    if len(name) == 0
        return error("Name required")
    else
        return success(Hero(name: name, hp: hp))
    end
end

-- OPTIONAL vs RESULT
func find_hero(roster: list<Hero>, name: text) -> optional<Hero>
    for hero in roster
        if hero.name == name
            return hero
        end
    end
    return none
end

-- CONTROLE DE FLUXO
func main() -> int
    -- Tipo (obrigatório)
    var heroes: list<Hero> = []
    
    -- Coleções com value semantics
    const h: Hero = create_hero("Link")?
    var h2: Hero = h
    h2.heal(20)  -- h não é afetado
    
    -- For, while, repeat
    for hero, idx in heroes
        io.print(fmt "{idx}: {hero.status()}")
    end
    
    -- Match/case
    match find_hero(heroes, "Zelda")
    case value hero:
        io.print(hero.status())
    case none:
        io.print("Not found")
    end
    
    return 0
end
```

---

## Conclusão

A sintaxe de Zenith v2 é uma **linguagem deliberadamente projetada para leitura** mantendo rigor industrial. Cada aspecto—desde palavras-chave até estrutura de bloco—foi escolhido para:

1. **Aumentar legibilidade** (reduz carga cognitiva)
2. **Garantir acessibilidade** (ADHD/dislexia-friendly)
3. **Manter explicitness** (sem surpresas mágicas)
4. **Assegurar segurança de memória** (ARC + value semantics)
5. **Preservar performance** (zero-cost abstractions)

Programadores que adotam essa filosofia encontram que código Zenith é mais fácil de ler, modificar e debugar—criando um ciclo virtuoso de qualidade e produtividade.