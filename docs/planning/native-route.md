# Rota nativa e backend C de debug

Status: contrato operacional.

Data: 2026-05-14.

## Resumo curto

A rota principal da linguagem Ori e nativa:

```text
codigo Ori
-> parser/checker/HIR
-> backend nativo Cranelift
-> runtime Rust ori-runtime
-> linker nativo
-> executavel
```

O backend C nao define a semantica central da linguagem. Ele permanece como
rota de debug/transpile com paridade parcial.

## Comandos

Use a rota nativa para compilar e testar:

```powershell
ori compile app.orl
ori test app.orl
```

Use a rota C apenas quando quiser inspecionar C gerado:

```powershell
ori build app.orl
```

`ori build` pode rejeitar programas que `ori compile` aceita. Isso e esperado
quando o backend C nao consegue preservar a semantica da rota nativa.

## Regras de manutencao

- `ori compile` nao deve compilar runtime C temporario.
- `ori test` deve usar o mesmo runtime nativo de `ori compile`.
- O runtime canonico e `compiler/crates/ori-runtime`.
- O pacote de release deve incluir `runtime/{target-triple}` com o artefato do
  runtime Rust e `runtime-link.json`.
- O driver le `runtime-link.json` ao lado do artefato runtime, valida
  `target`, `runtime`, `ori_version` e `abi_version`, e repassa as bibliotecas
  nativas registradas ao linker.
- `ORI_REQUIRE_PACKAGED_RUNTIME=1` ativa o modo estrito de pacote. Nesse modo,
  o driver nao volta para `target/` do workspace nem tenta construir o runtime
  com Cargo.
- Mensagens da rota nativa devem falar em runtime nativo ou linker nativo, nao
  em compilador C.
- Falta de paridade no backend C nao bloqueia feature nativa.

## Smoke de release

Use o smoke abaixo para validar um pacote temporario fora do workspace:

```powershell
.\tools\smoke_native_release.ps1
```

Ele monta uma pasta limpa com `ori`, `runtime/` e exemplos, ativa
`ORI_REQUIRE_PACKAGED_RUNTIME=1`, roda `ori compile` e roda `ori test`.

Para empacotar o runtime em Linux/macOS, use:

```sh
./tools/stage_native_runtime.sh
```

## Diagnostico esperado

Quando o backend C nao suporta um recurso, o erro deve usar
`backend.c_unsupported` e orientar o usuario a usar `ori compile`.

Quando a rota nativa nao encontra runtime ou linker, o erro deve explicar:

- qual artefato faltou;
- qual target esta sendo usado;
- qual comando ou variavel de ambiente pode corrigir o problema.

Codigos atuais da rota nativa:

- `native.runtime_missing`: runtime nativo ausente ou falha ao construir o
  fallback local.
- `native.linker_missing`: linker nativo ou `rustc` configurado nao iniciou.
- `native.abi_mismatch`: `runtime-link.json` foi gerado com ABI diferente da
  esperada pelo driver.
- `native.runtime_symbol_missing`: o linker encontrou simbolo nativo ausente,
  normalmente runtime desatualizado ou ABI incorreta.
- `native.link_failed`: o linker iniciou, mas falhou por outro motivo.

A saida padrao deve mostrar mensagem curta e acionavel. `ori compile
--native-raw` imprime stdout/stderr completos do linker para diagnostico
baixo nivel.
