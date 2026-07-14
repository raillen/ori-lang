# Daily / weekly stages

## Fast daily (`tools/qa/daily_fast.sh`) ~15–40 min

1. `cargo check --workspace`  
2. Crate units: lexer, parser, types, hir (as available)  
3. `ori_spec` + `diagnostic_catalog`  
4. `memory_arc` + `security_robustness` (subset ok if timeout)  
5. Residual gate: `compile_runs_lang_res_product_surface_native`  
6. Optional: `examples` smoke first N

## Full daily / weekly (`tools/qa/daily_full.sh`)

Fast +  

7. `cargo test --workspace`  
8. `multifile_imports`  
9. `concurrency_async` full  
10. `tools/qa/examples_smoke.sh`  
11. `tools/qa/perf_daily.sh`  

## Perf (`tools/qa/perf_daily.sh`)

- `tools/microbench_lang_perf.sh` if present  
- `cargo test -p ori-driver --test performance_guard`  
- Record wall times in log (no hard fail unless guard fails)

## Residual audit (`tools/qa/residual_audit.sh`)

- Runs LANG-RES product surface test  
- Runs known negative residual (for-without-iterator ABI)  
- Prints Spec 14 pointer  

## Environment

```bash
export ORI_USE_SYSTEM_LINKER=1   # if needed for AOT smoke
# stage runtime if link fails — see ori-testing skill
```
