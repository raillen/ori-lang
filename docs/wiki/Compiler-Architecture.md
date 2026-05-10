# Compiler Architecture

Pipeline:

1. Lexer
2. Parser
3. AST
4. Semantic
5. HIR
6. ZIR
7. C emitter
8. Native binary

Main directories:

- `compiler/frontend/`
- `compiler/semantic/`
- `compiler/hir/`
- `compiler/zir/`
- `compiler/targets/c/`
- `compiler/driver/`
- `runtime/c/`

Map documents:

- CODE_MAP: https://github.com/raillen/zenithlang/blob/master/compiler/CODE_MAP.md
- DRIVER_MAP: https://github.com/raillen/zenithlang/blob/master/compiler/driver/DRIVER_MAP.md
- PROJECT_MAP: https://github.com/raillen/zenithlang/blob/master/compiler/project/PROJECT_MAP.md

Specs:

- Compiler model: https://github.com/raillen/zenithlang/blob/master/docs/spec/language/compiler-model.md
- Language reference: https://github.com/raillen/zenithlang/blob/master/docs/spec/language/language-reference.md