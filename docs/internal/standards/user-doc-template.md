# User Documentation Template

> Audience: user
> Status: draft
> Surface: public
> Source of truth: no

## Objetivo

Use este template para paginas em `docs/public/` e para paginas didaticas em
`docs/reference/`.

Ele existe para manter a documentacao acessivel para pessoas com TDAH e
dislexia.

## Escopo

Use este template quando a pagina:

- ensina uma feature;
- explica um problema comum;
- compara o jeito Zenith com outra linguagem;
- mostra exemplos para usuarios.

Nao use este template para:

- spec normativa;
- decision/RFC;
- relatorio interno;
- checklist operacional.

## Dependencias

- Upstream:
  - `docs/internal/standards/documentation-style-guide.md`
  - `docs/DOCS-STRUCTURE.md`
- Downstream:
  - paginas de `docs/public/`
  - paginas de `docs/reference/`
- Codigo/Testes relacionados:
  - `tools/check_docs_paths.py`

## Conteudo principal

## Template de feature

~~~md
# Nome da Feature

> Audience: user
> Status: current | partial | planned | historical
> Surface: public | reference

## Resumo

Explique em 2 ou 3 linhas.

## Quando usar

- Caso 1.
- Caso 2.
- Caso 3.

## Forma canonica

```zt
codigo pequeno
```

## Exemplo completo pequeno

```zt
namespace app.main

public func main() -> int
    return 0
end
```

## Regras

- Regra curta.
- Regra curta.
- Regra curta.

## Erros comuns

### Erro: descricao curta

Nao escreva:

```zt
codigo ruim
```

Escreva:

```zt
codigo bom
```

## Limites atuais

- Limite atual.
- Limite atual.

## Veja tambem

- `docs/reference/...`
- `docs/spec/language/...`
- `docs/internal/decisions/language/...`
~~~

## Template de cookbook

~~~md
# Como fazer X

> Audience: user
> Status: current
> Surface: public

## Problema

Explique o problema em poucas linhas.

## Resposta curta

Use esta forma:

```zt
codigo recomendado
```

## Por que

Explique o motivo.

## Exemplo completo

```zt
namespace app.main

public func main() -> int
    return 0
end
```

## Erro comum

Nao use:

```zt
codigo ruim
```

## Quando nao usar

- Caso em que outra solucao e melhor.
~~~

## Regras de escrita para usuario

- Comece pela resposta.
- Depois explique a regra.
- Use exemplos pequenos.
- Separe `existe hoje`, `futuro` e `nao entra`.
- Evite paragrafo com mais de 4 linhas.
- Use tabelas para comparacoes.
- Nao esconda limites atuais.
- Nao mande o usuario ler decisions para aprender uma feature.

## Validacao

Antes de fechar uma pagina:

```powershell
python tools/check_docs_paths.py
git diff --check
```

Para exemplos de codigo:

- preferir exemplos baseados em `tests/behavior/`;
- marcar exemplos nao executaveis como ilustrativos;
- nao documentar feature futura como atual.

## Historico de atualizacao

- 2026-04-25: template inicial criado para guias e referencias de usuario.
