#include "runtime/c/zenith_rt.h"

typedef struct zt_app_main__Point {
    zt_int x;
    zt_int y;
} zt_app_main__Point;

zt_app_main__Point zt_ffi_make_point(zt_int x, zt_int y) {
    zt_app_main__Point point;
    point.x = x;
    point.y = y;
    return point;
}
