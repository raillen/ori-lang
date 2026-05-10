#include "runtime/c/zenith_rt.h"

typedef struct zt_app_main__Point {
    zt_int x;
    zt_int y;
} zt_app_main__Point;

zt_int zt_ffi_point_sum(zt_app_main__Point point) {
    return point.x + point.y;
}
