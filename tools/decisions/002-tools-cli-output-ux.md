# Decision 002 - UX e output das CLIs tools

- Status: draft
- Data: 2026-04-20
- Tipo: tools / cli / ux
- Escopo: output, diagnosticos e comportamento nao interativo

## Resumo

CLIs feitas com `tools.*` devem ser boas para pessoas e para scripts.

A saida humana deve ser clara.

A saida automatizada deve ser estavel.

## Decisao

Toda CLI baseada em `tools.*` deve considerar estes modos:

1. humano: padrao
2. JSON: para integracao
3. quiet: menos ruido
4. verbose: mais detalhe
5. CI/non-interactive: sem prompts bloqueantes

## Contrato de diagnostico

Diagnosticos humanos devem seguir o modelo:

```text
ACTION: o que fazer agora
WHY: por que isso aconteceu
NEXT: proximo passo sugerido
```

Esse formato pode ser compacto ou detalhado.

Mas a ordem mental deve continuar a mesma.

## Output para scripts

Quando `--json` estiver ativo:

1. nao misturar texto humano em stdout
2. logs devem ir para stderr ou estrutura dedicada
3. campos devem ser estaveis
4. erros devem ter codigo e mensagem
5. exit code deve continuar significativo

## Prompts

Prompts sao permitidos apenas em modo interativo.

Em CI ou modo nao interativo:

1. prompt deve falhar com erro claro
2. ou usar valor padrao explicito
3. ou exigir flag obrigatoria

A CLI nao deve ficar travada esperando input invisivel.

## Acessibilidade cognitiva

Saidas humanas devem evitar:

1. parede de texto
2. excesso de cor como unica fonte de significado
3. mensagens sem acao
4. erros em cascata sem resumo
5. tabelas largas demais sem truncamento

## Racional

A filosofia Zenith e reading-first.

Ferramentas oficiais precisam seguir a mesma filosofia.

Uma CLI que compila mas confunde o usuario falha como produto.

## Consequencias

1. `tools.output` precisa ser desenhado junto com `tools.diagnostics`.
2. `tools.prompt` precisa detectar modo nao interativo.
3. `tools.table` precisa respeitar largura do terminal.
4. snapshots de output devem fazer parte dos testes.

## Fora de escopo

1. tema visual final
2. codigos de erro definitivos
3. protocolo LSP
4. UI grafica
