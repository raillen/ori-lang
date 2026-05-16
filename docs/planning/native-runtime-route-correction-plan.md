# Plano: rota nativa de runtime e linkagem

Status: proposta de correcao arquitetural.

Data: 2026-05-14.

## Objetivo

Remover a dependencia do caminho nativo em runtime C embutido e em linkagem via
`cc` como requisito publico.

O alvo principal passa a ser:

```text
Ori source
-> parser/checker/HIR
-> Cranelift object
-> runtime Rust nativo
-> linker nativo controlado pelo Ori
-> executavel
```

O backend C continua podendo existir como backend de debug, mas nao deve definir
a semantica principal da linguagem.

## Estado atual

Hoje `ori compile` ja usa Cranelift para gerar objeto nativo, mas ainda depende
de C em dois pontos:

- `compiler/crates/ori-driver/src/pipeline.rs` chama `ensure_cc_available()`.
- O driver compila `ORI_RUNTIME_C` com `cc -c` em `build_runtime_lib()`.
- `compiler/crates/ori-codegen/src/native_backend.rs` chama `cc` para linkar o
  objeto final.
- `README.md` documenta que `ori compile` e `ori test` exigem uma toolchain C.

Ao mesmo tempo, ja existe um runtime Rust:

- `compiler/crates/ori-runtime/src/lib.rs` exporta simbolos `extern "C"` com
  `#[no_mangle]`.
- `compiler/crates/ori-runtime/Cargo.toml` ja gera `staticlib` e `rlib`.
- O runtime Rust ja tem ARC atomico.

Conclusao: o runtime Rust deve virar a fonte canonica do caminho nativo.

## Decisao arquitetural

1. O backend nativo e o runtime Rust sao o caminho principal.
2. O runtime C embutido deixa de ser usado por `ori compile` e `ori test`.
3. `ori build` pode continuar emitindo C como backend de debug.
4. A stdlib deve declarar cobertura nativa como requisito canonico.
5. A cobertura C deve ser apenas informativa para o backend de debug.

## Fora de escopo

Esta correcao nao exige:

- apagar o backend C de debug imediatamente;
- implementar um linker PE/ELF/Mach-O proprio do zero;
- implementar `async/await` junto com a troca de runtime;
- mudar a sintaxe da linguagem.

## Arquitetura alvo

### Runtime

O runtime oficial do caminho nativo deve ser `ori-runtime`.

Artefatos esperados por plataforma:

```text
runtime/
  x86_64-pc-windows-msvc/
    ori_runtime.lib
    runtime-link.json
  x86_64-pc-windows-gnu/
    libori_runtime.a
    runtime-link.json
  x86_64-unknown-linux-gnu/
    libori_runtime.a
    runtime-link.json
```

`runtime-link.json` deve registrar bibliotecas nativas exigidas pelo
`staticlib` Rust. Esse dado pode ser obtido com a saida de:

```text
rustc --print native-static-libs
```

### Linker

Criar uma camada `NativeLinker` no driver ou em `ori-codegen`.

Responsabilidades:

- receber o objeto Cranelift;
- receber a biblioteca `ori-runtime`;
- receber as bibliotecas de sistema exigidas pelo runtime;
- produzir o executavel final;
- emitir diagnostico claro quando nao houver linker disponivel.

Ordem recomendada:

1. `ORI_NATIVE_LINKER`, se definido.
2. linker empacotado pela distribuicao Ori, se existir.
3. `rust-lld` encontrado na toolchain Rust, quando compativel.
4. linker do sistema como fallback.

O fallback nao deve ser descrito como "precisa instalar C". A mensagem correta
deve falar em "linker nativo".

## Fases de implementacao

### Fase 0: auditoria e trava de regressao

- Mapear todas as chamadas a `cc`.
- Mapear todos os simbolos esperados pelo backend nativo.
- Criar teste que falha se `run_compile` depender de `ORI_RUNTIME_C`.
- Criar teste que falha se a mensagem de `ori compile` mencionar "C compiler"
  no caminho nativo.

### Fase 1: runtime Rust como runtime canonico

- Ensinar o driver a localizar o artefato `ori-runtime` do target atual.
- Em modo desenvolvimento, localizar `target/{profile}`.
- Em release, localizar `runtime/{target-triple}` junto do binario `ori`.
- Trocar `build_runtime_lib()` por `find_native_runtime_library()`.
- Manter teste `rust_runtime_exports_manifest_native_symbols`.
- Remover o teste que exige simbolos nativos em `ORI_RUNTIME_C`.

### Fase 2: camada de linkagem nativa

- Criar `NativeLinker`.
- Remover uso direto de `cc` de `ori-codegen::link`.
- Adicionar suporte inicial para Windows e Linux.
- Guardar flags por plataforma em uma estrutura testavel.
- Permitir `ORI_NATIVE_LINKER` para desenvolvimento e diagnostico.

### Fase 3: empacotamento do runtime

- Criar script de staging do runtime por target.
- Copiar `ori-runtime` staticlib para `runtime/{target-triple}`.
- Gerar `runtime-link.json`.
- Validar que `ori compile` funciona fora do workspace Cargo.

### Fase 4: remocao do runtime C embutido do caminho nativo

- Remover `ORI_RUNTIME_C` do driver nativo.
- Remover `build_runtime_lib()`.
- Atualizar testes que hoje verificam runtime C embutido.
- Atualizar `README.md`, `docs/spec/10-memory.md` e `docs/spec/12-stdlib.md`.

### Fase 5: backend C como debug backend isolado

- Manter `ori build` separado.
- Se o C backend nao suportar algo do nativo, emitir diagnostico claro.
- Evitar que novas funcionalidades sejam bloqueadas por falta de paridade no C.

## Criterios de aceite

- `ori compile` nao compila nenhum runtime C temporario.
- `ori test` usa o mesmo runtime Rust nativo.
- O caminho nativo nao chama `ensure_cc_available()`.
- O README nao diz que `ori compile` exige toolchain C.
- A spec diz que o runtime Rust e canonico para o backend nativo.
- O backend C aparece apenas como backend de debug.
- Os testes de manifest garantem que todo simbolo `native_runtime` existe no
  runtime Rust.

## Riscos

| Risco | Impacto | Mitigacao |
| --- | --- | --- |
| `staticlib` Rust exige bibliotecas de sistema | link falha fora do Cargo | gerar `runtime-link.json` por target |
| linkagem Windows diverge entre MSVC e GNU | build inconsistente | separar estrategias por target triple |
| macOS tem regras proprias de linkagem | atraso de suporte | validar Windows/Linux primeiro, macOS depois |
| runtime empacotado nao bate com target | erros de simbolo/ABI | registrar target triple e versao no metadata |
| duplicacao entre runtime Rust e C | regressao semantica | runtime Rust vira canonico; C debug pode rejeitar |

## Ordem recomendada

1. Corrigir rota nativa de runtime/linkagem.
2. Atualizar documentacao e testes do caminho nativo.
3. So depois iniciar a implementacao real de assincronicidade.

Essa ordem evita construir `async/await` sobre uma base que ainda depende de um
runtime C temporario.

