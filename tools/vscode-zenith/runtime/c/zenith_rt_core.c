static void zt_pqueue_i64_ensure_capacity(zt_pqueue_i64 *heap, size_t needed);
static void zt_pqueue_text_ensure_capacity(zt_pqueue_text *heap, size_t needed);
static void zt_btreemap_text_text_ensure_capacity(zt_btreemap_text_text *map, size_t needed);
static void zt_btreeset_text_ensure_capacity(zt_btreeset_text *set, size_t needed);

#define ZT_DYNAMIC_HEAP_BASE 1024u
#define ZT_DYNAMIC_HEAP_CAPACITY 512u

typedef struct zt_dynamic_heap_entry {
    uint32_t kind;
    zt_heap_free_fn free_fn;
    zt_heap_clone_fn clone_fn;
} zt_dynamic_heap_entry;

static zt_dynamic_heap_entry zt_dynamic_heaps[ZT_DYNAMIC_HEAP_CAPACITY];
static size_t zt_dynamic_heap_count = 0;

static const zt_dynamic_heap_entry *zt_find_dynamic_heap_entry(uint32_t kind) {
    size_t index;

    if (kind < ZT_DYNAMIC_HEAP_BASE) {
        return NULL;
    }

    index = (size_t)(kind - ZT_DYNAMIC_HEAP_BASE);
    if (index >= zt_dynamic_heap_count) {
        return NULL;
    }

    if (zt_dynamic_heaps[index].kind != kind) {
        return NULL;
    }

    return &zt_dynamic_heaps[index];
}

static const char *zt_safe_message(const char *message) {
    return message != NULL ? message : "runtime error";
}

static zt_bool zt_text_equals_literal(const zt_text *value, const char *literal) {
    size_t literal_len;

    if (value == NULL || literal == NULL) {
        return false;
    }

    literal_len = strlen(literal);
    if (value->len != literal_len) {
        return false;
    }

    if (literal_len == 0) {
        return true;
    }

    return memcmp(value->data, literal, literal_len) == 0;
}

static void zt_runtime_append_text(char *buffer, size_t capacity, const char *text) {
    size_t length;
    size_t available;
    size_t copy_length;

    if (buffer == NULL || capacity == 0 || text == NULL) return;
    length = strlen(buffer);
    if (length >= capacity - 1) return;

    available = (capacity - 1) - length;
    copy_length = strlen(text);
    if (copy_length > available) copy_length = available;
    memcpy(buffer + length, text, copy_length);
    buffer[length + copy_length] = '\0';
}

static zt_bool zt_try_add_size(size_t left, size_t right, size_t *out) {
    if (out == NULL) {
        return false;
    }
    if (left > (SIZE_MAX - right)) {
        return false;
    }
    *out = left + right;
    return true;
}

#if !defined(ZT_FORCE_PORTABLE_OVERFLOW) && (defined(__clang__) || defined(__GNUC__))
#define ZT_USE_COMPILER_OVERFLOW_BUILTINS 1
#else
#define ZT_USE_COMPILER_OVERFLOW_BUILTINS 0
#endif

static zt_bool zt_try_add_i64(zt_int a, zt_int b, zt_int *out) {
#if ZT_USE_COMPILER_OVERFLOW_BUILTINS
    return __builtin_add_overflow(a, b, out) ? true : false;
#else
    uint64_t ua = (uint64_t)a;
    uint64_t ub = (uint64_t)b;
    uint64_t ur = ua + ub;
    if (out != NULL) {
        *out = (zt_int)ur;
    }
    return ((~(ua ^ ub) & (ua ^ ur)) >> 63) != 0u ? true : false;
#endif
}

static zt_bool zt_try_sub_i64(zt_int a, zt_int b, zt_int *out) {
#if ZT_USE_COMPILER_OVERFLOW_BUILTINS
    return __builtin_sub_overflow(a, b, out) ? true : false;
#else
    uint64_t ua = (uint64_t)a;
    uint64_t ub = (uint64_t)b;
    uint64_t ur = ua - ub;
    if (out != NULL) {
        *out = (zt_int)ur;
    }
    return (((ua ^ ub) & (ua ^ ur)) >> 63) != 0u ? true : false;
#endif
}

static zt_bool zt_try_mul_i64(zt_int a, zt_int b, zt_int *out) {
#if ZT_USE_COMPILER_OVERFLOW_BUILTINS
    return __builtin_mul_overflow(a, b, out) ? true : false;
#else
    if (out == NULL) {
        return true;
    }

    if (a == 0 || b == 0) {
        *out = 0;
        return false;
    }

    if (a == -1) {
        if (b == INT64_MIN) {
            return true;
        }
        *out = -b;
        return false;
    }

    if (b == -1) {
        if (a == INT64_MIN) {
            return true;
        }
        *out = -a;
        return false;
    }

    if (a > 0) {
        if (b > 0) {
            if (a > INT64_MAX / b) {
                return true;
            }
        } else {
            if (b < INT64_MIN / a) {
                return true;
            }
        }
    } else {
        if (b > 0) {
            if (a < INT64_MIN / b) {
                return true;
            }
        } else {
            if (a < INT64_MAX / b) {
                return true;
            }
        }
    }

    *out = a * b;
    return false;
#endif
}

static size_t zt_require_added_size(size_t left, size_t right, const char *message) {
    size_t result = 0;

    if (!zt_try_add_size(left, right, &result)) {
        zt_runtime_error(ZT_ERR_PLATFORM, message);
    }

    return result;
}

static ZT_THREAD_LOCAL zt_runtime_error_info zt_last_error;
static ZT_THREAD_LOCAL char zt_last_error_message[256];
static ZT_THREAD_LOCAL char zt_last_error_code[64];
ZT_THREAD_LOCAL uintptr_t zt_stack_base = 0;

static void zt_runtime_store_error(zt_error_kind kind, const char *message, const char *code, zt_runtime_span span) {
    snprintf(zt_last_error_message, sizeof(zt_last_error_message), "%s", zt_safe_message(message));

    if (code != NULL && code[0] != '\0') {
        snprintf(zt_last_error_code, sizeof(zt_last_error_code), "%s", code);
        zt_last_error.code = zt_last_error_code;
    } else {
        zt_last_error_code[0] = '\0';
        zt_last_error.code = NULL;
    }

    zt_last_error.has_error = true;
    zt_last_error.kind = kind;
    zt_last_error.message = zt_last_error_message;
    zt_last_error.span = span;
}

static const char *zt_runtime_stable_code(zt_error_kind kind) {
    switch (kind) {
        case ZT_ERR_CHECK: return "runtime.check";
        case ZT_ERR_TODO: return "runtime.todo";
        case ZT_ERR_UNREACHABLE: return "runtime.unreachable";
        case ZT_ERR_PANIC: return "runtime.panic";
        case ZT_ERR_UNWRAP: return "runtime.unwrap";
        case ZT_ERR_IO: return "runtime.io";
        case ZT_ERR_INDEX: return "runtime.index";
        case ZT_ERR_MATH: return "runtime.math";
        case ZT_ERR_PLATFORM: return "runtime.platform";
        case ZT_ERR_CONTRACT: return "runtime.contract";
        case ZT_ERR_TEST_FAILED: return "test.fail";
        case ZT_ERR_TEST_SKIPPED: return "test.skip";
        default: return "runtime.error";
    }
}

static const char *zt_runtime_default_help(zt_error_kind kind) {
    switch (kind) {
        case ZT_ERR_CHECK:
            return "Handle check failures explicitly or validate inputs first.";
        case ZT_ERR_TODO:
            return "Finish the missing branch or keep it behind a clear development-only path.";
        case ZT_ERR_UNREACHABLE:
            return "Check the control flow; this path was expected to be impossible.";
        case ZT_ERR_PANIC:
            return "Avoid panic as control flow; handle recoverable failures with result/optional.";
        case ZT_ERR_UNWRAP:
            return "Check optional/result before unwrapping.";
        case ZT_ERR_IO:
            return "Verify file paths, permissions and host environment.";
        case ZT_ERR_INDEX:
            return "Guard indexes and slice bounds before access.";
        case ZT_ERR_MATH:
            return "Check for arithmetic overflow or division by zero.";
        case ZT_ERR_PLATFORM:
            return "Check platform limits and allocation failures.";
        case ZT_ERR_CONTRACT:
            return "Ensure values satisfy type or field contracts.";
        case ZT_ERR_TEST_FAILED:
            return "Use check(...) or explicit conditions before calling test.fail(...).";
        case ZT_ERR_TEST_SKIPPED:
            return "Skip marks the current test as not executed.";
        default:
            return "Review runtime preconditions for this operation.";
    }
}

static void zt_runtime_print_error(const zt_runtime_error_info *error) {
    const char *stable_code = zt_runtime_stable_code(error->kind);
    const char *help = zt_runtime_default_help(error->kind);
    const char *level = "error";

    if (error->kind == ZT_ERR_TEST_FAILED) level = "fail";
    if (error->kind == ZT_ERR_TEST_SKIPPED) level = "skip";

    fprintf(stderr, "%s[%s]\n", level, stable_code);
    fprintf(stderr, "%s\n", zt_safe_message(error->message));

    if (zt_runtime_span_is_known(error->span)) {
        const char *source_name = (error->span.source_name != NULL && error->span.source_name[0] != '\0')
            ? error->span.source_name
            : "<runtime>";
        fprintf(
            stderr,
            "\nwhere\n  %s:%lld:%lld\n",
            source_name,
            (long long)error->span.line,
            (long long)error->span.column
        );
    }

    if (error->code != NULL && error->code[0] != '\0') {
        fprintf(stderr, "\ncode\n  %s\n", error->code);
    }

    if (help != NULL && help[0] != '\0') {
        fprintf(stderr, "\nhelp\n  %s\n", help);
    }

    fprintf(stderr, "\n");
    fflush(stderr);
}

