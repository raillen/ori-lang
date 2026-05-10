# Zenith Next Tests

Esta pasta concentra fixtures e harnesses pequenos para o compilador Next.

Categorias:

- `frontend/`: lexer e parser.
- `semantic/`: binder, tipos, constraints e HIR.
- `zir/`: ZIR estruturada, printer e verifier.
- `targets/c/`: legalization e emitter C.
- `runtime/c/`: runtime C.
- `driver/`: manifesto e pipeline de projeto.
- `conformance/`: smoke tests end-to-end do caminho real `.zt -> C -> exe`.
- `behavior/`: projetos Zenith usados como fixtures observaveis. A matriz M16 fica em `behavior/MATRIX.md`.
- `fixtures/`: fixtures antigas ou de nivel baixo.

Regras:

- O compilador Next e testado por harness em C.
- Fontes `.zt` sao fixtures de usuario.
- `.zir` textual e fixture/debug, nao entrada principal do usuario.
- Testes de conformance devem usar o driver real sempre que possivel.
