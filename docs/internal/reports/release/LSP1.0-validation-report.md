# Compass LSP 1.0 Validation Report

Status: parcial, com Windows e VSIX local validados.

Data: 2026-04-28

## Resultado

- Fase 7 fechada.
- VSCode extension empacotada como `dist/zenith-vscode-1.0.0.vsix`.
- O `.vsix` inclui `zt`, `zt-lsp`, `stdlib/` e `runtime/`.
- Instalacao local validada em perfil isolado do VSCode.
- Smoke do LSP validado no Windows.

## Comandos executados

```powershell
node --check tools/vscode-zenith/extension.js
python -m json.tool tools\vscode-zenith\package.json
python -m json.tool tools\vscode-zenith\language-configuration.json
python -m json.tool tools\vscode-zenith\syntaxes\zenith.tmLanguage.json
python -m py_compile tools\package_vscode_extension.py
python tests/lsp/test_lsp_smoke.py
python tools\package_vscode_extension.py
```

Resultado: passou.

```powershell
code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --install-extension dist\zenith-vscode-1.0.0.vsix --force
code --user-data-dir .codex-vscode-user --extensions-dir .codex-vscode-extensions --list-extensions --show-versions
```

Resultado: instalou e listou `zenith-lang.zenith-vscode@1.0.0`.

```powershell
$env:ZENITH_HOME=(Resolve-Path '.codex-vscode-extensions\zenith-lang.zenith-vscode-1.0.0').Path
.codex-vscode-extensions\zenith-lang.zenith-vscode-1.0.0\bin\zt.exe check tests\behavior\check_intrinsic_basic
.codex-vscode-extensions\zenith-lang.zenith-vscode-1.0.0\bin\zt.exe build tests\behavior\check_intrinsic_basic -o .codex-vscode-user\check_intrinsic_basic.exe
.codex-vscode-extensions\zenith-lang.zenith-vscode-1.0.0\bin\zt.exe run tests\behavior\check_intrinsic_basic
```

Resultado: `check`, `build` e `run` passaram.

## Pendencias reais

- Rodar smoke LSP em Linux.
- Rodar teste visual/interativo no VSCode Extension Development Host.
- Validar manualmente `Zenith: New File` em janela do VSCode com projeto e sem projeto.

## Observacoes

- `code --install-extension` no perfil normal do usuario falhou por permissao fora do workspace.
- A instalacao em perfil isolado passou usando `--user-data-dir` e `--extensions-dir` dentro do repo.
- `vsce package` precisou de `--no-dependencies` nesta extensao, porque ela nao tem dependencias npm locais.
