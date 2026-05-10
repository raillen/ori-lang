#include <stdint.h>

zt_float zt_math_pow(zt_float base, zt_float exponent) {
    return pow(base, exponent);
}

zt_float zt_math_sqrt(zt_float value) {
    return sqrt(value);
}

zt_float zt_math_floor(zt_float value) {
    return floor(value);
}

zt_float zt_math_ceil(zt_float value) {
    return ceil(value);
}

zt_float zt_math_round_half_away_from_zero(zt_float value) {
    return round(value);
}

zt_float zt_math_trunc(zt_float value) {
    return trunc(value);
}

zt_float zt_math_sin(zt_float value) {
    return sin(value);
}

zt_float zt_math_cos(zt_float value) {
    return cos(value);
}

zt_float zt_math_tan(zt_float value) {
    return tan(value);
}

zt_float zt_math_asin(zt_float value) {
    return asin(value);
}

zt_float zt_math_acos(zt_float value) {
    return acos(value);
}

zt_float zt_math_atan(zt_float value) {
    return atan(value);
}

zt_float zt_math_atan2(zt_float y, zt_float x) {
    return atan2(y, x);
}

zt_float zt_math_ln(zt_float value) {
    return log(value);
}

zt_float zt_math_log10(zt_float value) {
    return log10(value);
}

zt_float zt_math_log_ten(zt_float value) {
    return log10(value);
}

zt_float zt_math_log2(zt_float value) {
    return log2(value);
}

zt_float zt_math_log(zt_float value, zt_float base) {
    if (base <= 0.0 || base == 1.0) {
        return NAN;
    }
    return log(value) / log(base);
}

zt_float zt_math_exp(zt_float value) {
    return exp(value);
}

zt_float zt_math_infinity(void) {
    return INFINITY;
}

zt_float zt_math_nan(void) {
    return NAN;
}

zt_bool zt_math_is_nan(zt_float value) {
    return isnan(value) ? true : false;
}

zt_bool zt_math_is_infinite(zt_float value) {
    return isinf(value) ? true : false;
}

zt_bool zt_math_is_finite(zt_float value) {
    return isfinite(value) ? true : false;
}

static void zt_float_order_guard(zt_float left, zt_float right) {
    if (isnan(left) || isnan(right)) {
        zt_runtime_error_ex(
            ZT_ERR_MATH,
            "Cannot order NaN.",
            "runtime.float_nan_compare",
            zt_runtime_span_unknown());
    }
}

zt_bool zt_float_lt(zt_float left, zt_float right) {
    zt_float_order_guard(left, right);
    return left < right;
}

zt_bool zt_float_le(zt_float left, zt_float right) {
    zt_float_order_guard(left, right);
    return left <= right;
}

zt_bool zt_float_gt(zt_float left, zt_float right) {
    zt_float_order_guard(left, right);
    return left > right;
}

zt_bool zt_float_ge(zt_float left, zt_float right) {
    zt_float_order_guard(left, right);
    return left >= right;
}

zt_int zt_add_i64(zt_int a, zt_int b) {
    zt_int result;
    if (zt_try_add_i64(a, b, &result)) {
        zt_runtime_error(ZT_ERR_MATH, "arithmetic overflow");
    }
    return result;
}

zt_int zt_sub_i64(zt_int a, zt_int b) {
    zt_int result;
    if (zt_try_sub_i64(a, b, &result)) {
        zt_runtime_error(ZT_ERR_MATH, "arithmetic overflow");
    }
    return result;
}

zt_int zt_mul_i64(zt_int a, zt_int b) {
    zt_int result;
    if (zt_try_mul_i64(a, b, &result)) {
        zt_runtime_error(ZT_ERR_MATH, "arithmetic overflow");
    }
    return result;
}

zt_int zt_div_i64(zt_int a, zt_int b) {
    if (b == 0) {
        zt_runtime_error(ZT_ERR_MATH, "division by zero");
    }
    if (a == INT64_MIN && b == -1) {
        zt_runtime_error(ZT_ERR_MATH, "arithmetic overflow");
    }
    return a / b;
}

zt_int zt_rem_i64(zt_int a, zt_int b) {
    if (b == 0) {
        zt_runtime_error(ZT_ERR_MATH, "division by zero");
    }
    if (a == INT64_MIN && b == -1) {
        zt_runtime_error(ZT_ERR_MATH, "arithmetic overflow");
    }
    return a % b;
}

zt_bool zt_validate_between_i64(zt_int value, zt_int min, zt_int max) {
    return value >= min && value <= max;
}
