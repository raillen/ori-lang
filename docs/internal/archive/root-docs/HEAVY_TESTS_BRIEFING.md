# Heavy Tests Briefing — Zenith Language v0.3.0-alpha.1

> **Gerado por:** Análise profunda automatizada da codebase Zenith (frontend → backend → runtime → stdlib)
> **Data:** 2026-04-22
> **Versão alvo:** `0.3.0-alpha.1` (R2.M12)
> **Pipeline:** lexer → parser → AST → binder → type checker → HIR → ZIR → verifier → C emitter → GCC/Clang → native binary

---

## 1. Contexto da Linguagem

Zenith é uma linguagem "reading-first" com semântica explícita e compilação nativa via backend C.

- **Tipos escalares:** `bool`, `int` (int64 fixo), `int8/16/32/64`, `u8/16/32/64` (aliases de `uint8..uint64`), `float` (float64 fixo), `float32/64`, `text` (UTF-8 imutável), `bytes`, `void`
- **Tipos genéricos:** `list<T>`, `map<K,V>`, `optional<T>`, `result<T,E>`, `dyn<Trait>`
- **Semântica de valor:** atribuição e passagem de parâmetro são cópias semânticas. Otimizações internas (RC, COW) não expõem aliasing mutável.
- **Gerenciamento de memória:** Reference counting não-atômico para valores gerenciados (`text`, `list`, `map`, structs com campos gerenciados). Sem coleta de ciclos no MVP.
- **Controle de mutação:** `const` (imutável binding), `var` (mutável local), `mut func` (métodos mutantes), `public var` (leitura pública, escrita apenas no namespace dono).
- **Construção:** `Type(field: value)` para structs; `Enum.Variant(field: value)` para enums.
- **Propagação:** `?` para `optional<T>` e `result<T,E>`.
- **Contracts:** `where` em parâmetros e campos gera panic `runtime.contract` se falhar (não altera tipo do construtor).
- **Traits core implícitos:** `Equatable`, `Hashable`, `TextRepresentable`, `Comparable` (este último adicionado em R2.M8, mas operadores `< <= > >=` ainda usam hardcoded numeric/text no checker).
- **FFI:** `extern c` para chamadas C; backend gera "ffi shield" (retain/release) em torno de calls.

---

## 2. Arquitetura do Compilador (Arquivos-Chave)

| Estágio | Arquivo Principal | LOC | Responsabilidade |
|---------|-------------------|-----|------------------|
| Lexer | `compiler/frontend/lexer/lexer.c` | ~489 | Tokenização de source `.zt` |
| Parser | `compiler/frontend/parser/parser.c` | ~1766 | AST construction, fmt interpolation parsing |
| AST | `compiler/frontend/ast/model.c/h` | ~540 | Nós AST e alocação em arena |
| Binder | `compiler/semantic/binder/binder.c` | ~692 | Symbol table, imports, namespace resolution |
| Type Checker | `compiler/semantic/types/checker.c` | ~2860 | Type inference, validation, trait checking |
| HIR Lowering | `compiler/hir/lowering/from_ast.c` | ~800 | Lowering AST → HIR |
| ZIR Lowering | `compiler/zir/lowering/from_hir.c` | ~1200 | Lowering HIR → ZIR |
| ZIR Verifier | `compiler/zir/verifier.c` | ~300 | Validação de ZIR |
| C Emitter | `compiler/targets/c/emitter.c` | ~7609 | Geração de código C |
| Legalization | `compiler/targets/c/legalization.c` | ~1035 | Transformações pré-emissão |
| Pipeline | `compiler/driver/pipeline.c` | ~1334 | Orquestração do build, monomorphization limit |
| Runtime | `runtime/c/zenith_rt.c` | ~1500+ | RC, containers, UTF-8, panic, FFI host API |
| Formatter | `compiler/tooling/formatter.c` | ~662 | Formatação determinística de source |

---

## 3. Bugs e Riscos Identificados (por Severidade)

### 3.1 CRÍTICO — Podem causar crash, corrupção de memória ou comportamento indefinido

| ID | Componente | Descrição | Como Reproduzir |
|----|------------|-----------|-----------------|
| **LEX-01** | Lexer | `zt_lexer_read_string()` não limita tamanho de string; input grande pode consumir memória excessiva | Fornecer string literal de 10MB sem fechamento |
| **LEX-02** | Lexer | `zt_parser_normalize_hex_bytes()` usa buffer stack fixo de 1024 bytes (`char normalized[1024]`); hex literal > 512 bytes trunca silenciosamente | `hex bytes "DE AD ..."` com 600+ pares |
| **PAR-01** | Parser | `zt_parser_find_fmt_expr_end()` tracking de `in_string`/`in_triple` pode falhar com `{}` aninhados dentro de strings em interpolação | `fmt "a { fmt \"b { 1 + 2 }\" } c"` |
| **PAR-02** | Parser | Buffer `buf[ZT_PARSER_MAX_NAME_PATH_LEN + 1]` (1025) trunca name paths longos; comportamento indefinido se nome essencial truncado | Namespace com 2000+ caracteres |
| **EMI-01** | Emitter | `c_buffer_append_vformat()` chama `vsnprintf(NULL, 0)` e depois `vsnprintf()` real; buffer pode ser realocado por outra thread (não-thread-safe) | Stress emissão com múltiplas threads |
| **EMI-02** | Emitter | `c_canonicalize_type()` usa `strncpy()` sem forçar null-termination em `dest[capacity-1]` se source >= capacity | Tipo com 128+ caracteres |
| **EMI-03** | Emitter | `c_copy_sanitized()` colide símbolos: `app.main-test` e `app.main_test` viram `app_main_test` | Dois namespaces com hífen e underscore equivalentes |
| **RUN-01** | Runtime | RC (`zt_retain`/`zt_release`) é não-atômico: `header->rc += 1` sem locks. Race condition se FFI criar threads | Chamadas C que criam threads e compartilham `text`/`list` |
| **RUN-02** | Runtime | `zt_deep_copy()` para `ZT_HEAP_LIST_TEXT` não checa retorno NULL de `zt_deep_copy()` recursivo; NULL deref | Deep copy de lista com elementos text em OOM |
| **RUN-03** | Runtime | `zt_text_from_utf8()` aloca `value` e `value->data`; se segundo malloc falha, `value` vaza (não é liberado) | Forçar OOM no segundo malloc |
| **RUN-04** | Runtime | `zt_runtime_error_ex()` chama `exit()` sem chamar releases de valores gerenciados em scopes superiores | Panic em função com muitos `text`/`list` alocados |
| **PIP-01** | Pipeline | `zt_compile_c_file()` monta `compile_cmd[2048]` via `snprintf`; paths longos + flags podem truncar | Project path de 1500+ caracteres + linker flags |
| **PIP-02** | Pipeline | Scan de imports stdlib é textual via `strstr(line, "import std.")`; comentários com essa string causam falso positivo | `-- import std.io` (comentário) em arquivo .zt |

### 3.2 ALTO — Funcionalidade incorreta ou degradada em edge cases

| ID | Componente | Descrição | Como Reproduzir |
|----|------------|-----------|-----------------|
| **SEM-01** | Checker | `zt_binding_scope_dispose()` libera memória mas não invalida ponteiro; reutilização pode causar use-after-free | Re-parse de AST reutilizando scopes antigos |
| **SEM-02** | Checker | `zt_type_param_has_trait()` assume que constraint é `ZT_TYPE_RESULT`; tipos malformados bypassam | Constraint inválida em generic |
| **SEM-03** | Checker | `zt_checker_compute_integral_binary()` computa `left + right` em `long long`; signed overflow é UB em C | `9223372036854775807 + 1` em constant folding |
| **SEM-04** | Checker | `zt_parse_double_literal_value()` usa `strtod`; `1e309` é UB/inf sem tratamento | Literal float `1e309` |
| **SEM-05** | Checker | Catalog lookup (`zt_catalog_find_decl`) é O(n) linear; 1000+ símbolos degradam para quadrático | Projeto com 2000+ declarações no namespace |
| **RUN-05** | Runtime | `zt_map_text_text_new()` aloca `hash_capacity = 8` fixo; map com >1000 entries degenera para O(n) | Inserir 10k pares em map |
| **RUN-06** | Runtime | `zt_list_i64_reserve()` growth `new_capacity *= 2` pode overflow `size_t` | Lista com > SIZE_MAX/2 elementos |
| **RUN-07** | Runtime | `zt_normalize_slice_end()` comportamento inconsistente entre `text_slice` e `list_slice` para listas vazias | Slice `[0..-1]` em lista vazia vs text vazio |
| **RUN-08** | Runtime | `zt_contract_failed_*()` usam buffer stack de 512 bytes; mensagens truncam silenciosamente | `where` com mensagem de erro de 600+ chars |
| **STD-01** | Stdlib | `std.collections.circbuf_int_new(-1)` passa negativo para C que converte para `size_t` (underflow) | `circbuf_int_new(-1)` |
| **STD-02** | Stdlib | `zt_net_connection` não previne double-close; socket fd pode vazar se `close()` chamado 2x | Chamar função que fecha conexão, depois deixar gc fechar |
| **STD-03** | Stdlib | `zt_host_read_file()` não valida path traversal (`../`, `..\\`) | `read_file("../../../etc/passwd")` |
| **STD-04** | Stdlib | JSON parser (`zt_json_parse_map_text_text`) pode stack overflow em JSON deeply nested | JSON com 10000+ níveis de nesting |
| **EMI-04** | Emitter | Monomorphization não deduplica tipos com whitespace: `list<list<int>>` vs `list<list< int>>` geram duas instâncias | Usar ambas as formas no mesmo projeto |
| **EMI-05** | Emitter | FFI shield (`c_emit_ffi_shield_retain`) pode não cobrir todos os caminhos de erro em calls externas | Extern call que falha após retain |
| **FMT-01** | Formatter | Ausência de teste explícito de idempotência: `format(format(x)) == format(x)` não é garantido | Rodar formatter 2x em código complexo |

### 3.3 MÉDIO — Degradation, memory leaks, ou riscos de segurança menores

| ID | Componente | Descrição |
|----|------------|-----------|
| **SEM-06** | Checker | `float` literal truncado para `int` sem aviso se valor for muito grande |
| **RUN-09** | Runtime | `zt_utf8_validate()` existe mas `zt_text_from_utf8()` não o chama; text inválido pode ser criado |
| **RUN-10** | Runtime | `zt_shared_text_snapshot()` e `zt_shared_bytes_snapshot()` alocam sem checar OOM |
| **PIP-03** | Pipeline | `zt_compile_runtime_object()` escreve `.ztc-tmp/runtime/zenith_rt.o` sem file lock; builds paralelas corrompem |
| **PIP-04** | Pipeline | `zenith.ztproj` parseado manualmente; campos desconhecidos são ignorados silenciosamente |

---

## 4. Testes de Performance — Stress Points

| Área | Cenário | Métrica Alvo | Risco |
|------|---------|--------------|-------|
| **Lexer** | Source file de 10MB | Throughput > 10 MB/s | Loop linear por caractere |
| **Parser** | Expressão binária com 1000 níveis de nesting | < 100ms | Recursão left-deep |
| **Type Checker** | Projeto com 10000 símbolos | < 5s | Catalog O(n) lookup |
| **Monomorphization** | 1000 instâncias únicas de generics | < 10s | String set O(n²) |
| **C Compilation** | 100k LOC emitidos | < 30s | GCC -O0 |
| **Binary Size** | Hello world | < 500KB | Runtime bloat |
| **Runtime List** | 1M push/pop | < 1s | Realloc frequente |
| **Runtime Map** | 1M insert/lookup | < 2s | Hash collision linear probing |
| **Runtime Deep Copy** | Lista aninhada 1000 níveis | Sem stack overflow | Recursão |
| **RC Stress** | 5B retain/release cycles | Sem overflow de RC | `uint32_t` rc |

---

## 5. Lacunas de Cobertura de Testes (Conforme Análise)

1. **Zero tests para RC overflow/underflow** em retain/release
2. **Zero tests para use-after-free** em runtime (deep copy, scope dispose)
3. **Zero tests para path traversal / command injection** em pipeline e stdlib
4. **Zero tests para monomorphization explosion real** (> limit configurado)
5. **Zero tests para FFI com dados malformados** (NULL pointers, strings não-terminadas)
6. **Zero tests de idempotência** do formatter (`format(format(x)) == format(x)`)
7. **Zero tests para builds paralelas** / race conditions em `.ztc-tmp/`
8. **Zero tests para recovery de OOM** (out-of-memory)
9. **Zero tests para UTF-8 inválido** em runtime (overlong, surrogates, continuation bytes inválidos)
10. **Zero tests E2E para enum payload match** (ZIR lowering é stub para este caso)
11. **Zero tests para `public var` cross-namespace write** em múltiplos arquivos compilados separadamente
12. **Zero tests para `dyn Trait` em collections heterogêneas com 1000+ elementos**

---

## 6. Estratégia Recomendada para Heavy Tests

### 6.1 Fuzzing Direcionado
- **Lexer fuzz:** Strings não-terminadas, escape sequences malformadas (`\x`, `\u`), bytes não-printáveis (0x00-0x1F), UTF-8 inválido
- **Parser fuzz:** Deep nesting (2000+), blocos sem `end`, keywords em posições inesperadas, fmt interpolation malformada
- **Semantic fuzz:** ASTs válidas sintaticamente mas com tipos incompatíveis, ciclos de import, símbolos duplicados
- **Runtime fuzz:** Chamadas FFI com ponteiros aleatórios, valores `NaN`/`Inf`, índices negativos, capacidades negativas

### 6.2 Property-Based Testing
- **Idempotência do formatter:** `forall code . parse(format(code)) == parse(format(format(code)))`
- **Value semantics:** `a = b; mutate(b); a == original_a`
- **RC invariants:** `retain(x); release(x);` → rc final == rc inicial
- **Slice invariants:** `len(slice(list, a, b)) == max(0, b - a + 1)` (para bounds válidos)

### 6.3 Stress Tests
- **Compilation stress:** Gerar código Zenith com 10k+ funções, 100k+ linhas, 1000+ instâncias genéricas
- **Runtime stress:** Alocar até OOM em list/maps; medir tempo de degradação
- **Parallel stress:** 10 builds simultâneas do mesmo projeto

### 6.4 Security Tests
- **Injection:** `build.linker_flags = "; rm -rf /"`, paths com `..`, symlinks maliciosos
- **DoS:** JSON de 1MB com nesting 10000+, strings de 10MB

---

## 7. Comandos de Build e Execução

```bash
# Build do compilador
python build.py

# Check de projeto
./zt.exe check <projeto>/zenith.ztproj

# Build e run
./zt.exe run <projeto>/zenith.ztproj

# Testes existentes
python run_all_tests.py

# Fuzzing existente
python tests/fuzz/fuzz_lexer.py --iters 1000 --seed 20260422
python tests/fuzz/fuzz_parser.py --iters 1000 --seed 20260422 --timeout 15.0

# Performance gate
python tests/perf/run_perf.py --suite quick --release-gate
```

---

## 8. Decisões de Design Relevantes para Tests

- **Value semantics:** Testar que `const a: list<int> = [1,2,3]; var b = a; b[0] = 10;` não altera `a`
- **No null:** Testar que `none` é o único valor ausente; verificar que parser não aceita `null`
- **No implicit conversion:** Testar que `const x: int = 1.5` falha; `to_int(1.5)` é necessário
- **Explicit `return`:** Funções sem `-> Type` não retornam valor; verificar que `return` sem valor funciona
- **Mutating methods:** `mut func` requer receiver `var`; testar chamada em `const` deve falhar
- **Where contracts:** Construtor direto com campo `where` que falha deve panic, NÃO retornar `result`
- **Public var:** Leitura cross-namespace permitida; escrita cross-namespace deve falhar com `mutability.invalid_update`

---

## 9. Referências aos Arquivos de Código

- **Lexer:** `compiler/frontend/lexer/lexer.c:168-192` (string reading), `lexer.c:278-308` (hex bytes)
- **Parser:** `compiler/frontend/parser/parser.c:348-424` (fmt expr end), `parser.c:599-620` (name path)
- **Checker:** `compiler/semantic/types/checker.c:533-556` (integral binary), `checker.c:836-941` (assignability)
- **Emitter:** `compiler/targets/c/emitter.c:52-81` (canonicalize type), `emitter.c:128-150` (buffer reserve), `emitter.c:235-261` (sanitize)
- **Runtime:** `runtime/c/zenith_rt.c:865-975` (retain/release), `zenith_rt.c:977-1140` (deep_copy), `zenith_rt.c:1457-1480` (text from utf8)
- **Pipeline:** `compiler/driver/pipeline.c:441-689` (compile_project), `pipeline.c:1165-1225` (compile_c_file)

---

*Este documento deve ser usado como contexto primário para geração de testes automatizados, fuzzers, e cenários de stress test da linguagem Zenith.*
