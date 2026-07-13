# Política de mesclagem da stdlib (M2)

> **Status:** decisão aceita (2026-07-13)  
> **Ordem tática:** M2 (este documento) → M3 ABI → M1 Rust-indep → M4 self-host  
> **Código de merge em massa:** **ainda não** — esta fatia é **docs + exemplos**.

---

## Decisão (resumo)

| ID | Tema | Decisão |
|----|------|---------|
| **M2.D** | Modelo de API | **D** — namespace **público canônico** = `ori.X` (um módulo por domínio) |
| **M2.A** | Layout preferido no disco | **A** — preferir `stdlib/X.orl` = `module ori.X` |
| **M2.B** | Split em pastas | **B** só se necessário: algoritmos pesados (`graph`, `tree`, …), `math` (vec/mat), ou pai ≫ ~400 linhas |
| **M2.compat** | `ori.X.utils` / `ori.X.algorithms` | **Alias silencioso** por enquanto (código antigo continua compilando) |
| **M2.examples** | Exemplos e docs de uso | **Só estilo canônico** `ori.X` (sem ensinar `.utils` como API nova) |
| **M2.code** | Merge físico de arquivos | **Depois** desta fatia de docs (lotes: fs, io, time, …) |

**Rejeitado:** C puro (sempre forçar `utils`/`algorithms` como API pública).  
**Rejeitado:** inferir tipo de layout só “pelo uso” em docs; a regra acima é normativa de produto.

---

## Camadas (inalteradas)

| Camada | Papel | Onde vive |
|--------|--------|-----------|
| **L1** | Hot path, FFI, runtime Rust | Manifesto `stdlib.rs` + `ori-runtime` |
| **L2** | Wrappers / ergonomia `.orl` | Preferência: mesmo `ori.X` / `stdlib/X.orl` |
| **L3** | Algoritmos puros `.orl` | Preferência: mesmo `ori.X` se couber; senão submódulo interno |

Mesclar **não** significa portar L1 para `.orl`.

---

## Regras de import (produto)

### Canônico (ensinar e usar em exemplos)

```ori
import ori.io = io
import ori.fs = fs
import ori.string = str
import ori.path = path
import ori.list = lists

-- seletivo no pai (quando o pai `.orl` expõe o símbolo)
import ori.string (is_empty, truncate = cut)
import ori.fs (read_text_or)
```

### Compatibilidade (não ensinar; não remover ainda)

```ori
import ori.fs.utils = fu          -- ainda funciona (alias silencioso)
import ori.string.utils = su      -- ainda funciona
import ori.queue.utils = qu       -- ainda funciona até existir pai flat completo
```

Quando o código de merge criar/estender `stdlib/X.orl`, helpers que hoje só
existem em `X/utils.orl` passam a ser importáveis via `ori.X` (pai). Até lá,
código interno/testes podem ainda tocar `.utils`; **exemplos e guias não**.

---

## Regras de layout no disco (para o código futuro)

1. **Preferir** `stdlib/<name>.orl` com `module ori.<name>`.
2. **Evitar** duplicar a mesma função em `X.orl` e `X/utils.orl` (hoje: `fs` é o caso-modelo a deduplicar no lote 1 de código).
3. **Manter pasta** `X/` quando:
   - há vários artefatos irmãos (`math/vec2.orl`, `mat3.orl`);
   - algorithms grandes e estáveis (`graph/algorithms.orl`, `tree/…`);
   - o pai único ficaria ilegível (>~400 linhas) **e** a divisão for por tema, não por “utils” genérico.
4. **README/spec** listam a API como `ori.X`, não como árvore de pastas.
5. Submódulos legados `ori.X.utils` / `ori.X.algorithms` = **compat**, não marca.

---

## Plano de implementação de código

| Fase | Trabalho | Status |
|------|----------|--------|
| **M2.1–2.6** | Pais `ori.X` + utils/algorithms compat + testes | ✅ |
| **path.relative** | multi-call un-ignored | ✅ |
| **layout** | Cargo em `compiler/`; examples mini-projetos; `_archive/` | ✅ |
| **result-ctors** | `ok`/`err` | ✅ |

---

## Critério de pronto M2

- [x] Política + README + spec
- [x] Pais `stdlib/X.orl` para domínios com helpers
- [x] Compat `ori.X.utils` / `algorithms`
- [x] Exemplos como projetos
- [x] Suíte stdlib/flatten/official examples

---

## Histórico

| Data | Evento |
|------|--------|
| 2026-07-13 | Recomendação D+A/B aceita; alias silencioso; docs primeiro; exemplos no estilo `ori.X` |
| 2026-07-13 | **M2 fechado** — merge pais + layout monorepo + examples + path.relative |
