# Decisions de Ferramentas Zenith

- Escopo: bibliotecas `tools.*` e comandos oficiais `zt.*`
- Fora de escopo: semantica da linguagem, sintaxe central e stdlib universal

## Objetivo

Esta pasta guarda decisoes sobre ferramentas criadas depois da estabilizacao da linguagem.

Ela existe para nao misturar:

1. decisoes da linguagem central
2. decisoes da stdlib universal
3. decisoes de experiencia de ferramentas

## Regra de organizacao

Use esta pasta para decisoes sobre:

1. `tools.*`
2. UX de CLI
3. output e diagnosticos de ferramentas
4. scaffolds
5. comandos `zt.*`
6. integracao entre ferramentas oficiais

Nao use esta pasta para:

1. sintaxe da linguagem
2. semantica de tipos
3. runtime central
4. backend C
5. regras canonicas da stdlib

## Decisions iniciais

1. `001-tools-namespace-boundary.md`
2. `002-tools-cli-output-ux.md`
3. `003-tools-cli-model.md`
4. `004-tools-output-model.md`
5. `005-tools-diagnostics-model.md`