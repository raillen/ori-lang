#include "runtime/c/zenith_rt.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static zt_int zt_app_main__main(void *zt_ctx);

static zt_int zt_app_main__main(void *zt_ctx) {
    zt_int a;
    zt_int b;
    goto zt_block_entry;

zt_block_entry:
#if defined(__GNUC__) || defined(__clang__)
    __attribute__((unused));
#endif
    a = 40;
    b = 2;
    return zt_add_i64(a, b);
}

int main(int argc, char **argv) {
    zt_runtime_capture_process_args(argc, argv);
    return (int)zt_app_main__main(NULL);
}
