# Runtime Sanitizer Evidence - 2026-05-07

Etapa: 3 - Confirmar sanitizer em ambiente compativel.

## Resultado

- Status final: passou.
- Comando final:

```bash
python3 tests/hardening/test_runtime_sanitizers.py
```

- Stdout final:

```text
runtime sanitizer checks ok
```

- Stderr final: vazio.

## Ambiente compativel usado

- Sistema: WSL2 Ubuntu em Linux `6.6.87.2-microsoft-standard-WSL2`.
- Arquitetura: `x86_64`.
- Python: `Python 3.12.3`.
- Compilador: `gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0`.
- Clang: indisponivel neste ambiente.

## CI revisado

- Workflow: `.github/workflows/ci.yml`.
- Runner Linux: `ubuntu-latest`.
- Compilador Linux do CI: `gcc`, instalado por `sudo apt-get install -y gcc`.
- Job/step confirmado: `Runtime sanitizer checks`.
- Comando do CI: `python tests/hardening/test_runtime_sanitizers.py`.

## Falhas encontradas e classificacao

1. Build sanitizer falhava antes da execucao.
   - Tipo: limitacao de toolchain/comando.
   - Causa: C11 estrito nao expunha APIs POSIX usadas pelo runtime.
   - Correcao: `tests/hardening/test_runtime_sanitizers.py` agora define `_POSIX_C_SOURCE=200809L` e `_DEFAULT_SOURCE`.

2. Linkedicao Linux falhava em simbolos matematicos.
   - Tipo: limitacao de toolchain/comando.
   - Causa: runtime usa `libm`, mas o comando sanitizer nao linkava `-lm`.
   - Correcao: ambientes nao Windows agora adicionam `-lm`; Windows preserva `-lws2_32`.

3. `text_utf8_bytes_roundtrip_len` falhava durante a execucao.
   - Tipo: teste obsoleto.
   - Causa: o teste comparava bytes UTF-8 com tamanho em codepoints.
   - Correcao: `tests/runtime/c/test_runtime.c` agora valida 4 bytes e 3 codepoints separadamente.

4. ASAN detectou `heap-use-after-free` em `zt_outcome_void_text`.
   - Tipo: use-after-free real.
   - Causa: `zt_outcome_void_text_propagate` fazia copia rasa do erro.
   - Correcao: `runtime/c/zenith_rt_templates.h` agora propaga sucesso/falha via construtores, retendo/clonando ownership corretamente.

## Decisao

Etapa 3 desbloqueada para prosseguir. O sanitizer passou em ambiente Linux/GCC compativel com o CI, e o achado real de use-after-free foi corrigido e revalidado.

## Validacao relacionada

- `test_outcome_propagate.c` tambem passou em WSL/GCC depois da correcao do template.
- Resultado: `Runtime outcome propagate tests OK`.
- Observacao: o build ainda emite warnings `-Wpedantic` ja existentes sobre conversao de ponteiro de objeto para ponteiro de funcao; eles nao bloquearam a execucao nem o sanitizer.
