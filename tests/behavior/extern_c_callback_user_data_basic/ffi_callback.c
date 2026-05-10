#include "runtime/c/zenith_rt.h"

zt_int zt_ffi_apply_i64_user_data(zt_int value, zt_int user_data, zt_int (*callback)(zt_int, zt_int)) {
    return callback(value, user_data);
}
