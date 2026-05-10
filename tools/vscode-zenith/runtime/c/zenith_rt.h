#ifndef ZENITH_NEXT_RUNTIME_C_ZENITH_RT_H
#define ZENITH_NEXT_RUNTIME_C_ZENITH_RT_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif
#ifndef ZT_EXTERN_CDECL
#if defined(_WIN32)
#define ZT_EXTERN_CDECL __cdecl
#else
#define ZT_EXTERN_CDECL
#endif
#endif

#ifndef ZT_EXTERN_STDCALL
#if defined(_WIN32)
#define ZT_EXTERN_STDCALL __stdcall
#else
#define ZT_EXTERN_STDCALL
#endif
#endif


#if defined(_MSC_VER)
#define ZT_NORETURN __declspec(noreturn)
#elif defined(__GNUC__) || defined(__clang__)
#define ZT_NORETURN __attribute__((noreturn))
#else
#define ZT_NORETURN
#endif

#define ZT_RUNTIME_ABI_VERSION_MAJOR 0
#define ZT_RUNTIME_ABI_VERSION_MINOR 4
#define ZT_EXIT_CODE_RUNTIME_ERROR 1
#define ZT_EXIT_CODE_TEST_FAILED 120
#define ZT_EXIT_CODE_TEST_SKIPPED 121

typedef int64_t zt_int;
typedef double zt_float;
typedef bool zt_bool;

typedef enum zt_heap_kind {
    ZT_HEAP_UNKNOWN = 0,
    ZT_HEAP_TEXT = 1,
    ZT_HEAP_LIST_I64 = 2,
    ZT_HEAP_LIST_TEXT = 3,
    ZT_HEAP_OPTIONAL_TEXT = 4,
    ZT_HEAP_OUTCOME_I64_TEXT = 5,
    ZT_HEAP_OUTCOME_VOID_TEXT = 6,
    ZT_HEAP_OPTIONAL_LIST_I64 = 7,
    ZT_HEAP_OUTCOME_TEXT_TEXT = 8,
    ZT_HEAP_MAP_TEXT_TEXT = 9,
    ZT_HEAP_OPTIONAL_LIST_TEXT = 10,
    ZT_HEAP_OPTIONAL_MAP_TEXT_TEXT = 11,
    ZT_HEAP_OUTCOME_LIST_I64_TEXT = 12,
    ZT_HEAP_OUTCOME_LIST_TEXT_TEXT = 13,
    ZT_HEAP_OUTCOME_MAP_TEXT_TEXT = 14,
    ZT_HEAP_BYTES = 15,
    ZT_HEAP_GRID2D_I64 = 16,
    ZT_HEAP_GRID2D_TEXT = 17,
    ZT_HEAP_PQUEUE_I64 = 18,
    ZT_HEAP_PQUEUE_TEXT = 19,
    ZT_HEAP_CIRCBUF_I64 = 20,
    ZT_HEAP_CIRCBUF_TEXT = 21,
    ZT_HEAP_BTREEMAP_TEXT_TEXT = 22,
    ZT_HEAP_BTREESET_TEXT = 23,
    ZT_HEAP_GRID3D_I64 = 24,
    ZT_HEAP_GRID3D_TEXT = 25,
    ZT_HEAP_NET_CONNECTION = 26,
    ZT_HEAP_DYN_TEXT_REPR = 27,
    ZT_HEAP_LIST_DYN_TEXT_REPR = 28,
    ZT_HEAP_LIST_F64 = 29,
    ZT_HEAP_DYN_VALUE = 30,
    ZT_HEAP_VTABLE = 31,
    ZT_HEAP_LIST_DYN = 32,
    ZT_HEAP_CLOSURE = 33,
    ZT_HEAP_LAZY_I64 = 34,
    ZT_HEAP_SET_I64 = 35,
    ZT_HEAP_SET_TEXT = 36,
    ZT_HEAP_LIST_BOOL = 37,
    ZT_HEAP_LIST_I8 = 38,
    ZT_HEAP_LIST_I16 = 39,
    ZT_HEAP_LIST_I32 = 40,
    ZT_HEAP_LIST_U8 = 41,
    ZT_HEAP_LIST_U16 = 42,
    ZT_HEAP_LIST_U32 = 43,
    ZT_HEAP_LIST_U64 = 44,
    ZT_HEAP_LIST_GENERIC = 45,
    ZT_HEAP_MAP_GENERIC = 46,
    ZT_HEAP_SET_GENERIC = 47,
    ZT_HEAP_LAZY_F64 = 48,
    ZT_HEAP_LAZY_BOOL = 49,
    ZT_HEAP_LAZY_I8 = 50,
    ZT_HEAP_LAZY_I16 = 51,
    ZT_HEAP_LAZY_I32 = 52,
    ZT_HEAP_LAZY_U8 = 53,
    ZT_HEAP_LAZY_U16 = 54,
    ZT_HEAP_LAZY_U32 = 55,
    ZT_HEAP_LAZY_U64 = 56,
    ZT_HEAP_LAZY_TEXT = 57,
    ZT_HEAP_IMMORTAL_OUTCOME_VOID_TEXT = 255
} zt_heap_kind;

typedef struct zt_header {
    uint32_t rc;
    uint32_t kind;
} zt_header;

typedef void (*zt_heap_free_fn)(void *ref);
typedef void *(*zt_heap_clone_fn)(void *ref);

static inline zt_bool zt_i64_eq(zt_int left, zt_int right) {
    return left == right;
}

typedef struct zt_text {
    zt_header header;
    size_t len;
    char *data;
} zt_text;

typedef struct zt_bytes {
    zt_header header;
    size_t len;
    uint8_t *data;
} zt_bytes;

typedef enum zt_dyn_text_repr_tag {
    ZT_DYN_TEXT_REPR_INT = 1,
    ZT_DYN_TEXT_REPR_FLOAT = 2,
    ZT_DYN_TEXT_REPR_BOOL = 3,
    ZT_DYN_TEXT_REPR_TEXT = 4
} zt_dyn_text_repr_tag;

typedef struct zt_dyn_text_repr {
    zt_header header;
    uint32_t tag;
    union {
        zt_int int_value;
        zt_float float_value;
        zt_bool bool_value;
        zt_text *text_value;
    } as;
} zt_dyn_text_repr;

typedef struct zt_list_dyn_text_repr {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_dyn_text_repr **data;
} zt_list_dyn_text_repr;



/* R3.M4: Generic dyn dispatch vtable and fat pointer infrastructure.
 * Each dyn<Trait> value is a fat pointer: (data pointer + vtable pointer).
 * The vtable contains function pointers for drop, clone, and each trait method. */

#define ZT_VTABLE_MAX_METHODS 8

typedef struct zt_vtable {
    zt_header header;
    void (*drop)(void *data);
    void (*clone_out)(void *dest, const void *src);
    size_t data_size;
    const char *trait_name;
    const char *concrete_type_name;
    uint32_t method_count;
    void (*methods[ZT_VTABLE_MAX_METHODS])(void);
} zt_vtable;

typedef struct zt_dyn_value {
    zt_header header;
    void *data;
    zt_vtable *vtable;
} zt_dyn_value;

/* Generic dyn list for heterogeneous collections */
typedef struct zt_list_dyn {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_dyn_value **data;
} zt_list_dyn;

/* R3.M6: Closure fat pointer representation */
typedef struct zt_closure {
    zt_header header;
    void *fn;
    void *ctx;
    void (*drop_ctx)(void *);
} zt_closure;

/* R3.M8: Explicit one-shot lazy value for int.
 * The thunk is forced only through std.lazy.force_int and cannot be reused. */
typedef struct zt_lazy_i64 {
    zt_header header;
    zt_closure *thunk;
    zt_bool consumed;
} zt_lazy_i64;

#define ZT_DECLARE_LAZY_STRUCT(SUFFIX) \
typedef struct zt_lazy_##SUFFIX { \
    zt_header header; \
    zt_closure *thunk; \
    zt_bool consumed; \
} zt_lazy_##SUFFIX;

ZT_DECLARE_LAZY_STRUCT(f64)
ZT_DECLARE_LAZY_STRUCT(bool)
ZT_DECLARE_LAZY_STRUCT(i8)
ZT_DECLARE_LAZY_STRUCT(i16)
ZT_DECLARE_LAZY_STRUCT(i32)
ZT_DECLARE_LAZY_STRUCT(u8)
ZT_DECLARE_LAZY_STRUCT(u16)
ZT_DECLARE_LAZY_STRUCT(u32)
ZT_DECLARE_LAZY_STRUCT(u64)
ZT_DECLARE_LAZY_STRUCT(text)

#undef ZT_DECLARE_LAZY_STRUCT

/* Runtime helper functions for dyn dispatch */
zt_dyn_value *zt_dyn_box(void *data, zt_vtable *vtable);
zt_dyn_value *zt_dyn_box_copy_owned(const void *data, size_t size, zt_vtable *vtable);
zt_dyn_value *zt_dyn_box_copy_borrowed(const void *data, size_t size, zt_vtable *vtable);
void *zt_dyn_unbox(const zt_dyn_value *dyn);
zt_vtable *zt_dyn_get_vtable(const zt_dyn_value *dyn);
void zt_dyn_drop(zt_dyn_value *dyn);
zt_dyn_value *zt_dyn_clone(const zt_dyn_value *dyn);

/* Generic dyn list helpers */
zt_list_dyn *zt_list_dyn_create(void);
zt_list_dyn *zt_list_dyn_from_array(zt_dyn_value *const *items, size_t count);
zt_list_dyn *zt_list_dyn_from_array_owned(zt_dyn_value **items, size_t count);
zt_int zt_list_dyn_len(const zt_list_dyn *list);
zt_list_dyn *zt_list_dyn_slice(const zt_list_dyn *list, zt_int start_0, zt_int end_0);
void zt_list_dyn_append(zt_list_dyn *list, zt_dyn_value *value);
zt_list_dyn *zt_list_dyn_append_owned(zt_list_dyn *list, zt_dyn_value *value);
zt_dyn_value *zt_list_dyn_get(const zt_list_dyn *list, zt_int index);
void zt_list_dyn_set(zt_list_dyn *list, zt_int index, zt_dyn_value *value);
zt_list_dyn *zt_list_dyn_set_owned(zt_list_dyn *list, zt_int index, zt_dyn_value *value);
void zt_list_dyn_free(zt_list_dyn *list);

/* Helper to call a dyn method by index (0-based, after drop/clone) */
#define ZT_DYN_CALL(dyn, method_index, return_type, ...) \
    ((return_type (*)(void *, __VA_ARGS__))((dyn)->vtable->methods[method_index]))((dyn)->data, __VA_ARGS__)

typedef struct zt_net_connection zt_net_connection;
typedef struct zt_shared_text zt_shared_text;
typedef struct zt_shared_bytes zt_shared_bytes;

typedef struct zt_list_i64 {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_int *data;
} zt_list_i64;

typedef struct zt_list_text {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_text **data;
} zt_list_text;

typedef struct zt_list_f64 {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_float *data;
} zt_list_f64;

typedef struct zt_list_bool {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_bool *data;
} zt_list_bool;

typedef struct zt_list_i8 {
    zt_header header;
    size_t len;
    size_t capacity;
    int8_t *data;
} zt_list_i8;

typedef struct zt_list_i16 {
    zt_header header;
    size_t len;
    size_t capacity;
    int16_t *data;
} zt_list_i16;

typedef struct zt_list_i32 {
    zt_header header;
    size_t len;
    size_t capacity;
    int32_t *data;
} zt_list_i32;

typedef struct zt_list_u8 {
    zt_header header;
    size_t len;
    size_t capacity;
    uint8_t *data;
} zt_list_u8;

typedef struct zt_list_u16 {
    zt_header header;
    size_t len;
    size_t capacity;
    uint16_t *data;
} zt_list_u16;

typedef struct zt_list_u32 {
    zt_header header;
    size_t len;
    size_t capacity;
    uint32_t *data;
} zt_list_u32;

typedef struct zt_list_u64 {
    zt_header header;
    size_t len;
    size_t capacity;
    uint64_t *data;
} zt_list_u64;

typedef struct zt_map_text_text {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_text **keys;
    zt_text **values;
    size_t hash_capacity;
    size_t *hash_indices;
} zt_map_text_text;

typedef struct zt_set_i64 {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_int *data;
    uint8_t *occupied;
    size_t hash_capacity;
} zt_set_i64;

typedef struct zt_set_text {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_text **data;
    uint8_t *occupied;
    size_t hash_capacity;
} zt_set_text;

typedef struct zt_optional_i64 {
    zt_bool is_present;
    zt_int value;
} zt_optional_i64;

typedef struct zt_optional_f64 {
    zt_bool is_present;
    zt_float value;
} zt_optional_f64;

typedef struct zt_optional_bool {
    zt_bool is_present;
    zt_bool value;
} zt_optional_bool;

typedef struct zt_optional_i8 {
    zt_bool is_present;
    int8_t value;
} zt_optional_i8;

typedef struct zt_optional_i16 {
    zt_bool is_present;
    int16_t value;
} zt_optional_i16;

typedef struct zt_optional_i32 {
    zt_bool is_present;
    int32_t value;
} zt_optional_i32;

typedef struct zt_optional_u8 {
    zt_bool is_present;
    uint8_t value;
} zt_optional_u8;

typedef struct zt_optional_u16 {
    zt_bool is_present;
    uint16_t value;
} zt_optional_u16;

typedef struct zt_optional_u32 {
    zt_bool is_present;
    uint32_t value;
} zt_optional_u32;

typedef struct zt_optional_u64 {
    zt_bool is_present;
    uint64_t value;
} zt_optional_u64;

typedef struct zt_optional_text {
    zt_bool is_present;
    zt_text *value;
} zt_optional_text;

typedef struct zt_optional_bytes {
    zt_bool is_present;
    zt_bytes *value;
} zt_optional_bytes;

typedef struct zt_optional_list_i64 {
    zt_bool is_present;
    zt_list_i64 *value;
} zt_optional_list_i64;

typedef struct zt_optional_list_text {
    zt_bool is_present;
    zt_list_text *value;
} zt_optional_list_text;

typedef struct zt_optional_map_text_text {
    zt_bool is_present;
    zt_map_text_text *value;
} zt_optional_map_text_text;

typedef struct zt_core_error {
    zt_text *code;
    zt_text *message;
    zt_optional_text context;
} zt_core_error;

zt_core_error zt_core_error_make(zt_text *code, zt_text *message, zt_optional_text context);
zt_core_error zt_core_error_from_message(const char *code, const char *message);
zt_core_error zt_core_error_from_text(const char *code, zt_text *message);
zt_core_error zt_core_error_clone(zt_core_error error);
void zt_core_error_dispose(zt_core_error *error);
zt_text *zt_core_error_message_or_default(zt_core_error error);

typedef struct zt_process_exit_status {
    zt_int code;
} zt_process_exit_status;

typedef struct zt_process_captured_run {
    zt_process_exit_status status;
    zt_text *stdout_text;
    zt_text *stderr_text;
} zt_process_captured_run;

typedef struct zt_outcome_i64_text {
    zt_bool is_success;
    zt_int value;
    zt_text *error;
} zt_outcome_i64_text;

typedef struct zt_outcome_void_text {
    zt_bool is_success;
    zt_text *error;
} zt_outcome_void_text;

typedef struct zt_outcome_text_text {
    zt_bool is_success;
    zt_text *value;
    zt_text *error;
} zt_outcome_text_text;

typedef struct zt_outcome_list_i64_text {
    zt_bool is_success;
    zt_list_i64 *value;
    zt_text *error;
} zt_outcome_list_i64_text;

typedef struct zt_outcome_list_text_text {
    zt_bool is_success;
    zt_list_text *value;
    zt_text *error;
} zt_outcome_list_text_text;

typedef struct zt_outcome_map_text_text {
    zt_bool is_success;
    zt_map_text_text *value;
    zt_text *error;
} zt_outcome_map_text_text;

typedef struct zt_outcome_i64_core_error {
    zt_bool is_success;
    zt_int value;
    zt_core_error error;
} zt_outcome_i64_core_error;

typedef struct zt_outcome_f64_core_error {
    zt_bool is_success;
    zt_float value;
    zt_core_error error;
} zt_outcome_f64_core_error;

typedef struct zt_outcome_bool_core_error {
    zt_bool is_success;
    zt_bool value;
    zt_core_error error;
} zt_outcome_bool_core_error;

typedef struct zt_outcome_void_core_error {
    zt_bool is_success;
    zt_core_error error;
} zt_outcome_void_core_error;

typedef struct zt_outcome_text_core_error {
    zt_bool is_success;
    zt_text *value;
    zt_core_error error;
} zt_outcome_text_core_error;

typedef struct zt_outcome_bytes_core_error {
    zt_bool is_success;
    zt_bytes *value;
    zt_core_error error;
} zt_outcome_bytes_core_error;

typedef struct zt_outcome_process_captured_run_core_error {
    zt_bool is_success;
    zt_process_captured_run value;
    zt_core_error error;
} zt_outcome_process_captured_run_core_error;

typedef struct zt_outcome_optional_text_core_error {
    zt_bool is_success;
    zt_optional_text value;
    zt_core_error error;
} zt_outcome_optional_text_core_error;

typedef struct zt_outcome_optional_bytes_core_error {
    zt_bool is_success;
    zt_optional_bytes value;
    zt_core_error error;
} zt_outcome_optional_bytes_core_error;

typedef struct zt_outcome_net_connection_core_error {
    zt_bool is_success;
    zt_net_connection *value;
    zt_core_error error;
} zt_outcome_net_connection_core_error;

typedef struct zt_outcome_list_i64_core_error {
    zt_bool is_success;
    zt_list_i64 *value;
    zt_core_error error;
} zt_outcome_list_i64_core_error;

typedef struct zt_outcome_list_text_core_error {
    zt_bool is_success;
    zt_list_text *value;
    zt_core_error error;
} zt_outcome_list_text_core_error;

#define ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(SUFFIX, LIST_TYPE) \
typedef struct zt_outcome_list_##SUFFIX##_core_error { \
    zt_bool is_success; \
    LIST_TYPE *value; \
    zt_core_error error; \
} zt_outcome_list_##SUFFIX##_core_error;

ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(f64, zt_list_f64)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(bool, zt_list_bool)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(i8, zt_list_i8)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(i16, zt_list_i16)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(i32, zt_list_i32)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(u8, zt_list_u8)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(u16, zt_list_u16)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(u32, zt_list_u32)
ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME(u64, zt_list_u64)

#undef ZT_DECLARE_PRIMITIVE_LIST_CORE_ERROR_OUTCOME

typedef struct zt_outcome_map_text_text_core_error {
    zt_bool is_success;
    zt_map_text_text *value;
    zt_core_error error;
} zt_outcome_map_text_text_core_error;

typedef struct zt_outcome_optional_i64_core_error {
    zt_bool is_success;
    zt_optional_i64 value;
    zt_core_error error;
} zt_outcome_optional_i64_core_error;

typedef struct zt_grid2d_i64 {
    zt_header header;
    size_t rows;
    size_t cols;
    size_t len;
    size_t capacity;
    zt_int *data;
} zt_grid2d_i64;

typedef struct zt_grid2d_text {
    zt_header header;
    size_t rows;
    size_t cols;
    size_t len;
    size_t capacity;
    zt_text **data;
} zt_grid2d_text;

typedef struct zt_pqueue_i64 {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_int *data;
} zt_pqueue_i64;

typedef struct zt_pqueue_text {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_text **data;
} zt_pqueue_text;

typedef struct zt_circbuf_i64 {
    zt_header header;
    size_t len;
    size_t capacity;
    size_t head;
    zt_int *data;
} zt_circbuf_i64;

typedef struct zt_circbuf_text {
    zt_header header;
    size_t len;
    size_t capacity;
    size_t head;
    zt_text **data;
} zt_circbuf_text;

typedef struct zt_btreemap_text_text {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_text **keys;
    zt_text **values;
} zt_btreemap_text_text;

typedef struct zt_btreeset_text {
    zt_header header;
    size_t len;
    size_t capacity;
    zt_text **data;
} zt_btreeset_text;

typedef struct zt_grid3d_i64 {
    zt_header header;
    size_t depth;
    size_t rows;
    size_t cols;
    size_t len;
    size_t capacity;
    zt_int *data;
} zt_grid3d_i64;

typedef struct zt_grid3d_text {
    zt_header header;
    size_t depth;
    size_t rows;
    size_t cols;
    size_t len;
    size_t capacity;
    zt_text **data;
} zt_grid3d_text;

struct zt_net_connection {
    zt_header header;
    intptr_t socket_handle;
    zt_int default_timeout_ms;
    zt_bool closed;
};

typedef enum zt_error_kind {
    ZT_ERR_CHECK,
    ZT_ERR_TODO,
    ZT_ERR_UNREACHABLE,
    ZT_ERR_INDEX,
    ZT_ERR_UNWRAP,
    ZT_ERR_PANIC,
    ZT_ERR_IO,
    ZT_ERR_MATH,
    ZT_ERR_PLATFORM,
    ZT_ERR_CONTRACT,
    ZT_ERR_TEST_FAILED,
    ZT_ERR_TEST_SKIPPED
} zt_error_kind;

/* Compatibility aliases for newer runtime sections that still use the older
 * symbolic names introduced during dyn/list work. */
#define ZT_ERR_MEMORY ZT_ERR_PLATFORM
#define ZT_ERR_BOUNDS ZT_ERR_INDEX

typedef struct zt_runtime_span {
    const char *source_name;
    zt_int line;
    zt_int column;
} zt_runtime_span;

typedef struct zt_runtime_error_info {
    zt_bool has_error;
    zt_error_kind kind;
    const char *message;
    const char *code;
    zt_runtime_span span;
} zt_runtime_error_info;

const char *zt_error_kind_name(zt_error_kind kind);
zt_runtime_span zt_runtime_span_unknown(void);
zt_runtime_span zt_runtime_make_span(const char *source_name, zt_int line, zt_int column);
zt_bool zt_runtime_span_is_known(zt_runtime_span span);
/* Per-thread diagnostic slot for the current runtime thread/isolate path. */
const zt_runtime_error_info *zt_runtime_last_error(void);
void zt_runtime_clear_error(void);
void zt_runtime_report_error(zt_error_kind kind, const char *message, const char *code, zt_runtime_span span);

/*
 * Stack usage instrumentation.
 *
 * The C backend emits ZT_CHECK_STACK() at the top of generated Zenith
 * functions. The guard measures per-thread stack usage from the first
 * generated frame and raises a Zenith runtime panic before the native stack
 * reaches an OS-level fault.
 */
#if defined(_MSC_VER)
#define ZT_THREAD_LOCAL __declspec(thread)
#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
#define ZT_THREAD_LOCAL _Thread_local
#elif defined(__GNUC__) || defined(__clang__)
#define ZT_THREAD_LOCAL __thread
#else
#error "Zenith runtime requires thread-local storage support"
#endif

#ifndef ZT_MAX_STACK_SIZE
#define ZT_MAX_STACK_SIZE (1024 * 1024) /* 1 MiB default. */
#endif

extern ZT_THREAD_LOCAL uintptr_t zt_stack_base;

#define ZT_CHECK_STACK() \
    do { \
        volatile char _zt_anchor; \
        uintptr_t _zt_addr = (uintptr_t)&_zt_anchor; \
        if (zt_stack_base == 0) zt_stack_base = _zt_addr; \
        uintptr_t _zt_diff = (zt_stack_base > _zt_addr) ? (zt_stack_base - _zt_addr) : (_zt_addr - zt_stack_base); \
        if (_zt_diff > ZT_MAX_STACK_SIZE) { \
            zt_panic("Stack overflow prevented: maximum stack size exceeded"); \
        } \
    } while (0)

/* Exit hook kept in the emitted C so future depth-based guards can reuse the
 * same compiler instrumentation. The byte-usage guard does not need to pop. */
#define ZT_POP_STACK() do { } while (0)

/*
 * Alpha concurrency contract:
 * - ordinary managed values live on a single isolate/heap domain at a time;
 * - zt_retain/zt_release are not a cross-thread synchronization API;
 * - crossing a thread/isolate boundary must use explicit transfer by deep copy
 *   or a dedicated shared wrapper.
 */
void zt_retain(void *ref);
void zt_release(void *ref);
void *zt_deep_copy(void *ref);
uint32_t zt_register_dynamic_heap_kind(zt_heap_free_fn free_fn, zt_heap_clone_fn clone_fn);
zt_int zt_orc_collect_cycles(void);
zt_int zt_orc_ref_count_text(const zt_text *value);
zt_int zt_orc_ref_count_list_text(const zt_list_text *value);
zt_bool zt_orc_is_unique_text(const zt_text *value);
zt_bool zt_orc_is_unique_list_text(const zt_list_text *value);
zt_int zt_unsafe_heap_kind_text(const zt_text *value);
zt_int zt_unsafe_heap_kind_list_text(const zt_list_text *value);
zt_text *zt_unsafe_retain_text(zt_text *value);
zt_list_text *zt_unsafe_retain_list_text(zt_list_text *value);
zt_text *zt_mem_own_text(const zt_text *value);
zt_text *zt_mem_view_text(zt_text *value);
zt_text *zt_mem_edit_text(const zt_text *value);
zt_list_text *zt_mem_own_list_text(const zt_list_text *value);
zt_list_text *zt_mem_view_list_text(zt_list_text *value);
zt_list_text *zt_mem_edit_list_text(const zt_list_text *value);
void *zt_mem_own_heap(const void *value);
void *zt_mem_view_heap(void *value);
void *zt_mem_edit_heap(const void *value);

#define ZT_DECLARE_MEM_PRIMITIVE_LIST_API(SUFFIX, LIST_TYPE) \
LIST_TYPE *zt_mem_own_list_##SUFFIX(const LIST_TYPE *value); \
LIST_TYPE *zt_mem_view_list_##SUFFIX(LIST_TYPE *value); \
LIST_TYPE *zt_mem_edit_list_##SUFFIX(const LIST_TYPE *value);

ZT_DECLARE_MEM_PRIMITIVE_LIST_API(i64, zt_list_i64)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(f64, zt_list_f64)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(bool, zt_list_bool)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(i8, zt_list_i8)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(i16, zt_list_i16)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(i32, zt_list_i32)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(u8, zt_list_u8)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(u16, zt_list_u16)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(u32, zt_list_u32)
ZT_DECLARE_MEM_PRIMITIVE_LIST_API(u64, zt_list_u64)

#undef ZT_DECLARE_MEM_PRIMITIVE_LIST_API

zt_text *zt_text_pool_alloc(void);
void zt_text_pool_free(zt_text *text);
zt_bool zt_validate_pointer(const void *ptr);
void zt_runtime_safe_function_example(const zt_text *text);
void zt_validate_and_free_text(zt_text *value);
void zt_validate_and_free_list_i64(zt_list_i64 *list);
void zt_validate_and_free_map_text_text(zt_map_text_text *map);

/*
 * Shared wrappers are the narrow exception to the default single-isolate path.
 * Use them only when the host/runtime intentionally needs cross-thread sharing
 * for text/bytes snapshots. Ordinary zt_text/zt_bytes/list/map values remain
 * non-thread-safe by default.
 */
zt_shared_text *zt_shared_text_new(zt_text *value);
zt_shared_text *zt_shared_text_retain(zt_shared_text *shared);
void zt_shared_text_release(zt_shared_text *shared);
const zt_text *zt_shared_text_borrow(const zt_shared_text *shared);
zt_text *zt_shared_text_snapshot(const zt_shared_text *shared);
uint32_t zt_shared_text_ref_count(const zt_shared_text *shared);

zt_shared_bytes *zt_shared_bytes_new(zt_bytes *value);
zt_shared_bytes *zt_shared_bytes_retain(zt_shared_bytes *shared);
void zt_shared_bytes_release(zt_shared_bytes *shared);
const zt_bytes *zt_shared_bytes_borrow(const zt_shared_bytes *shared);
zt_bytes *zt_shared_bytes_snapshot(const zt_shared_bytes *shared);
uint32_t zt_shared_bytes_ref_count(const zt_shared_bytes *shared);

ZT_NORETURN void zt_runtime_error(zt_error_kind kind, const char *message);
ZT_NORETURN void zt_runtime_error_ex(zt_error_kind kind, const char *message, const char *code, zt_runtime_span span);
ZT_NORETURN void zt_runtime_error_with_span(zt_error_kind kind, const char *message, zt_runtime_span span);
void zt_check(zt_bool condition, const char *message);
ZT_NORETURN void zt_todo(const char *message);
ZT_NORETURN void zt_unreachable(const char *message);
ZT_NORETURN void zt_panic(const char *message);
ZT_NORETURN void zt_test_fail(zt_text *message);
ZT_NORETURN void zt_test_skip(zt_text *reason);
zt_bool zt_test_throws_closure(zt_closure *body);
zt_int zt_sqlite3_memory_exec(zt_text *sql);
zt_int zt_ffi_apply_i64(zt_int value, zt_int (*callback)(zt_int));
ZT_NORETURN void zt_contract_failed(const char *message, zt_runtime_span span);
void zt_contract_failed_i64(const char *message, zt_int value, zt_runtime_span span);
void zt_contract_failed_float(const char *message, zt_float value, zt_runtime_span span);
void zt_contract_failed_bool(const char *message, zt_bool value, zt_runtime_span span);

/* ── builtins ─────────────────────────────────────────── */
void zt_builtin_print(const zt_text *value);
zt_text *zt_builtin_read(void);
void zt_builtin_debug(const zt_text *value);
zt_text *zt_builtin_type_name(const zt_text *value);
zt_int zt_debug_size_of(const zt_text *value);
zt_list_i64 *zt_builtin_range2(zt_int start, zt_int end);
zt_list_i64 *zt_builtin_range3(zt_int start, zt_int end, zt_int step);
zt_float zt_int_to_float(zt_int value);
zt_text *zt_int_to_text(zt_int value);
zt_optional_i64 zt_int_parse(const zt_text *value);
zt_int zt_float_to_int(zt_float value);
zt_int zt_float_round_to_int(zt_float value);
zt_text *zt_float_to_text(zt_float value);
zt_optional_f64 zt_float_parse(const zt_text *value);
zt_text *zt_bool_to_text(zt_bool value);

zt_text *zt_text_from_utf8(const char *data, size_t len);
zt_text *zt_text_from_utf8_literal(const char *data);
zt_text *zt_text_concat(const zt_text *a, const zt_text *b);
zt_text *zt_text_deep_copy(const zt_text *value);
zt_text *zt_text_index(const zt_text *value, zt_int index_0);
zt_text *zt_text_slice(const zt_text *value, zt_int start_0, zt_int end_0);
zt_bool zt_text_eq(const zt_text *a, const zt_text *b);
size_t zt_text_hash(const zt_text *value);
size_t zt_i64_hash(zt_int value);
zt_int zt_text_len(const zt_text *value);
const char *zt_text_data(const zt_text *value);
zt_list_text *zt_text_split(const zt_text *value, const zt_text *separator);
zt_list_text *zt_text_chars(const zt_text *value);
zt_text *zt_text_to_lower_ascii(const zt_text *value);
zt_text *zt_text_to_upper_ascii(const zt_text *value);
zt_text *zt_text_capitalize_ascii(const zt_text *value);

zt_bytes *zt_bytes_empty(void);
zt_bytes *zt_bytes_from_array(const uint8_t *data, size_t len);
zt_bytes *zt_bytes_from_list_i64(const zt_list_i64 *values);
zt_list_i64 *zt_bytes_to_list_i64(const zt_bytes *value);
zt_bytes *zt_bytes_join(const zt_bytes *left, const zt_bytes *right);
zt_bool zt_bytes_starts_with(const zt_bytes *value, const zt_bytes *prefix);
zt_bool zt_bytes_ends_with(const zt_bytes *value, const zt_bytes *suffix);
zt_bool zt_bytes_contains(const zt_bytes *value, const zt_bytes *part);
zt_bytes *zt_text_to_utf8_bytes(const zt_text *value);
zt_outcome_text_text zt_text_from_utf8_bytes(const zt_bytes *value);
zt_int zt_bytes_len(const zt_bytes *value);
uint8_t zt_bytes_get(const zt_bytes *value, zt_int index_0);
zt_bytes *zt_bytes_slice(const zt_bytes *value, zt_int start_0, zt_int end_0);
zt_outcome_bytes_core_error zt_bytes_from_list_i64_result(const zt_list_i64 *values);
zt_optional_i64 zt_bytes_get_optional(const zt_bytes *value, zt_int index_0);
zt_bytes *zt_bytes_slice_clamped(const zt_bytes *value, zt_int start_0, zt_int end_0);
zt_optional_i64 zt_bytes_index_of(const zt_bytes *value, const zt_bytes *part);

zt_list_i64 *zt_list_i64_new(void);
zt_list_i64 *zt_list_i64_from_array(const zt_int *items, size_t count);
void zt_list_i64_push(zt_list_i64 *list, zt_int value);
zt_list_i64 *zt_list_i64_push_owned(zt_list_i64 *list, zt_int value);
zt_int zt_list_i64_get(const zt_list_i64 *list, zt_int index_0);
zt_optional_i64 zt_list_i64_get_optional(const zt_list_i64 *list, zt_int index_0);
zt_optional_i64 zt_list_i64_last_optional(const zt_list_i64 *list);
zt_list_i64 *zt_list_i64_rest(const zt_list_i64 *list);
zt_list_i64 *zt_list_i64_skip(const zt_list_i64 *list, zt_int count);
zt_list_i64 *zt_list_i64_append(const zt_list_i64 *list, zt_int value);
zt_list_i64 *zt_list_i64_prepend(const zt_list_i64 *list, zt_int value);
zt_bool zt_list_i64_contains(const zt_list_i64 *list, zt_int value);
zt_list_i64 *zt_list_i64_reverse(const zt_list_i64 *list);
zt_list_i64 *zt_list_i64_concat(const zt_list_i64 *left, const zt_list_i64 *right);
zt_optional_i64 zt_list_i64_index_of(const zt_list_i64 *list, zt_int value);
zt_list_i64 *zt_list_i64_map(const zt_list_i64 *list, zt_closure *mapper);
zt_list_i64 *zt_list_i64_filter(const zt_list_i64 *list, zt_closure *predicate);
zt_int zt_list_i64_reduce(const zt_list_i64 *list, zt_int initial, zt_closure *reducer);
zt_optional_i64 zt_list_i64_find(const zt_list_i64 *list, zt_closure *predicate);
zt_bool zt_list_i64_any(const zt_list_i64 *list, zt_closure *predicate);
zt_bool zt_list_i64_all(const zt_list_i64 *list, zt_closure *predicate);
zt_int zt_list_i64_count(const zt_list_i64 *list, zt_closure *predicate);
zt_list_i64 *zt_list_i64_sort_by(const zt_list_i64 *list, zt_closure *key_selector);
zt_outcome_list_i64_core_error zt_list_i64_set_result(const zt_list_i64 *list, zt_int index_0, zt_int value);
zt_outcome_list_i64_core_error zt_list_i64_remove_first(const zt_list_i64 *list);
zt_outcome_list_i64_core_error zt_list_i64_remove_last(const zt_list_i64 *list);
zt_outcome_list_i64_core_error zt_list_i64_remove_at(const zt_list_i64 *list, zt_int index_0);
zt_outcome_list_i64_core_error zt_list_i64_slice_result(const zt_list_i64 *list, zt_int start_0, zt_int end_0);
void zt_list_i64_set(zt_list_i64 *list, zt_int index_0, zt_int value);
zt_list_i64 *zt_list_i64_set_owned(zt_list_i64 *list, zt_int index_0, zt_int value);
zt_int zt_list_i64_len(const zt_list_i64 *list);
zt_list_i64 *zt_list_i64_slice(const zt_list_i64 *list, zt_int start_0, zt_int end_0);

zt_list_text *zt_list_text_new(void);
zt_list_text *zt_list_text_from_array(zt_text *const *items, size_t count);
void zt_list_text_push(zt_list_text *list, zt_text *value);
zt_list_text *zt_list_text_push_owned(zt_list_text *list, zt_text *value);
zt_text *zt_list_text_get(const zt_list_text *list, zt_int index_0);
zt_text *zt_list_text_take(zt_list_text *list, zt_int index_0);
zt_optional_text zt_list_text_get_optional(const zt_list_text *list, zt_int index_0);
zt_optional_text zt_list_text_last_optional(const zt_list_text *list);
zt_list_text *zt_list_text_rest(const zt_list_text *list);
zt_list_text *zt_list_text_skip(const zt_list_text *list, zt_int count);
zt_list_text *zt_list_text_append(const zt_list_text *list, zt_text *value);
zt_list_text *zt_list_text_prepend(const zt_list_text *list, zt_text *value);
zt_bool zt_list_text_contains(const zt_list_text *list, zt_text *value);
zt_list_text *zt_list_text_reverse(const zt_list_text *list);
zt_list_text *zt_list_text_concat(const zt_list_text *left, const zt_list_text *right);
zt_optional_i64 zt_list_text_index_of(const zt_list_text *list, zt_text *value);
zt_list_text *zt_list_text_map(const zt_list_text *list, zt_closure *mapper);
zt_list_text *zt_list_text_filter(const zt_list_text *list, zt_closure *predicate);
zt_optional_text zt_list_text_find(const zt_list_text *list, zt_closure *predicate);
zt_bool zt_list_text_any(const zt_list_text *list, zt_closure *predicate);
zt_bool zt_list_text_all(const zt_list_text *list, zt_closure *predicate);
zt_int zt_list_text_count(const zt_list_text *list, zt_closure *predicate);
zt_list_text *zt_list_text_sort_by(const zt_list_text *list, zt_closure *key_selector);
zt_outcome_list_text_core_error zt_list_text_set_result(const zt_list_text *list, zt_int index_0, zt_text *value);
zt_outcome_list_text_core_error zt_list_text_remove_first(const zt_list_text *list);
zt_outcome_list_text_core_error zt_list_text_remove_last(const zt_list_text *list);
zt_outcome_list_text_core_error zt_list_text_remove_at(const zt_list_text *list, zt_int index_0);
zt_outcome_list_text_core_error zt_list_text_slice_result(const zt_list_text *list, zt_int start_0, zt_int end_0);
void zt_list_text_set(zt_list_text *list, zt_int index_0, zt_text *value);
zt_list_text *zt_list_text_set_owned(zt_list_text *list, zt_int index_0, zt_text *value);
zt_int zt_list_text_len(const zt_list_text *list);
zt_list_text *zt_list_text_slice(const zt_list_text *list, zt_int start_0, zt_int end_0);
zt_list_text *zt_list_text_deep_copy(const zt_list_text *list);

zt_list_f64 *zt_list_f64_new(void);
zt_list_f64 *zt_list_f64_from_array(const zt_float *items, size_t count);
void zt_list_f64_push(zt_list_f64 *list, zt_float value);
zt_list_f64 *zt_list_f64_push_owned(zt_list_f64 *list, zt_float value);
zt_float zt_list_f64_get(const zt_list_f64 *list, zt_int index_0);
zt_optional_f64 zt_list_f64_get_optional(const zt_list_f64 *list, zt_int index_0);
zt_optional_f64 zt_list_f64_last_optional(const zt_list_f64 *list);
zt_list_f64 *zt_list_f64_rest(const zt_list_f64 *list);
zt_list_f64 *zt_list_f64_skip(const zt_list_f64 *list, zt_int count);
zt_list_f64 *zt_list_f64_append(const zt_list_f64 *list, zt_float value);
zt_list_f64 *zt_list_f64_prepend(const zt_list_f64 *list, zt_float value);
zt_bool zt_list_f64_contains(const zt_list_f64 *list, zt_float value);
zt_list_f64 *zt_list_f64_reverse(const zt_list_f64 *list);
zt_list_f64 *zt_list_f64_concat(const zt_list_f64 *left, const zt_list_f64 *right);
zt_optional_i64 zt_list_f64_index_of(const zt_list_f64 *list, zt_float value);
void zt_list_f64_set(zt_list_f64 *list, zt_int index_0, zt_float value);
zt_list_f64 *zt_list_f64_set_owned(zt_list_f64 *list, zt_int index_0, zt_float value);
zt_int zt_list_f64_len(const zt_list_f64 *list);
zt_list_f64 *zt_list_f64_slice(const zt_list_f64 *list, zt_int start_0, zt_int end_0);
zt_list_f64 *zt_list_f64_map(const zt_list_f64 *list, zt_closure *mapper);
zt_list_f64 *zt_list_f64_filter(const zt_list_f64 *list, zt_closure *predicate);
zt_optional_f64 zt_list_f64_find(const zt_list_f64 *list, zt_closure *predicate);
zt_bool zt_list_f64_any(const zt_list_f64 *list, zt_closure *predicate);
zt_bool zt_list_f64_all(const zt_list_f64 *list, zt_closure *predicate);
zt_int zt_list_f64_count(const zt_list_f64 *list, zt_closure *predicate);
zt_list_f64 *zt_list_f64_sort_by(const zt_list_f64 *list, zt_closure *key_selector);
zt_outcome_list_f64_core_error zt_list_f64_set_result(const zt_list_f64 *list, zt_int index_0, zt_float value);
zt_outcome_list_f64_core_error zt_list_f64_remove_first(const zt_list_f64 *list);
zt_outcome_list_f64_core_error zt_list_f64_remove_last(const zt_list_f64 *list);
zt_outcome_list_f64_core_error zt_list_f64_remove_at(const zt_list_f64 *list, zt_int index_0);
zt_outcome_list_f64_core_error zt_list_f64_slice_result(const zt_list_f64 *list, zt_int start_0, zt_int end_0);

#define ZT_DECLARE_PRIMITIVE_LIST_API(SUFFIX, ELEM_TYPE) \
zt_list_##SUFFIX *zt_list_##SUFFIX##_new(void); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_from_array(const ELEM_TYPE *items, size_t count); \
void zt_list_##SUFFIX##_push(zt_list_##SUFFIX *list, ELEM_TYPE value); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_push_owned(zt_list_##SUFFIX *list, ELEM_TYPE value); \
ELEM_TYPE zt_list_##SUFFIX##_get(const zt_list_##SUFFIX *list, zt_int index_0); \
zt_optional_##SUFFIX zt_list_##SUFFIX##_get_optional(const zt_list_##SUFFIX *list, zt_int index_0); \
zt_optional_##SUFFIX zt_list_##SUFFIX##_last_optional(const zt_list_##SUFFIX *list); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_rest(const zt_list_##SUFFIX *list); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_skip(const zt_list_##SUFFIX *list, zt_int count); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_append(const zt_list_##SUFFIX *list, ELEM_TYPE value); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_prepend(const zt_list_##SUFFIX *list, ELEM_TYPE value); \
zt_bool zt_list_##SUFFIX##_contains(const zt_list_##SUFFIX *list, ELEM_TYPE value); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_reverse(const zt_list_##SUFFIX *list); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_concat(const zt_list_##SUFFIX *left, const zt_list_##SUFFIX *right); \
zt_optional_i64 zt_list_##SUFFIX##_index_of(const zt_list_##SUFFIX *list, ELEM_TYPE value); \
void zt_list_##SUFFIX##_set(zt_list_##SUFFIX *list, zt_int index_0, ELEM_TYPE value); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_set_owned(zt_list_##SUFFIX *list, zt_int index_0, ELEM_TYPE value); \
zt_int zt_list_##SUFFIX##_len(const zt_list_##SUFFIX *list); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_slice(const zt_list_##SUFFIX *list, zt_int start_0, zt_int end_0); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_map(const zt_list_##SUFFIX *list, zt_closure *mapper); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_filter(const zt_list_##SUFFIX *list, zt_closure *predicate); \
zt_optional_##SUFFIX zt_list_##SUFFIX##_find(const zt_list_##SUFFIX *list, zt_closure *predicate); \
zt_bool zt_list_##SUFFIX##_any(const zt_list_##SUFFIX *list, zt_closure *predicate); \
zt_bool zt_list_##SUFFIX##_all(const zt_list_##SUFFIX *list, zt_closure *predicate); \
zt_int zt_list_##SUFFIX##_count(const zt_list_##SUFFIX *list, zt_closure *predicate); \
zt_list_##SUFFIX *zt_list_##SUFFIX##_sort_by(const zt_list_##SUFFIX *list, zt_closure *key_selector); \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_set_result(const zt_list_##SUFFIX *list, zt_int index_0, ELEM_TYPE value); \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_remove_first(const zt_list_##SUFFIX *list); \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_remove_last(const zt_list_##SUFFIX *list); \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_remove_at(const zt_list_##SUFFIX *list, zt_int index_0); \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_slice_result(const zt_list_##SUFFIX *list, zt_int start_0, zt_int end_0);

ZT_DECLARE_PRIMITIVE_LIST_API(bool, zt_bool)
ZT_DECLARE_PRIMITIVE_LIST_API(i8, int8_t)
ZT_DECLARE_PRIMITIVE_LIST_API(i16, int16_t)
ZT_DECLARE_PRIMITIVE_LIST_API(i32, int32_t)
ZT_DECLARE_PRIMITIVE_LIST_API(u8, uint8_t)
ZT_DECLARE_PRIMITIVE_LIST_API(u16, uint16_t)
ZT_DECLARE_PRIMITIVE_LIST_API(u32, uint32_t)
ZT_DECLARE_PRIMITIVE_LIST_API(u64, uint64_t)

#undef ZT_DECLARE_PRIMITIVE_LIST_API

zt_dyn_text_repr *zt_dyn_text_repr_from_i64(zt_int value);
zt_dyn_text_repr *zt_dyn_text_repr_from_float(zt_float value);
zt_dyn_text_repr *zt_dyn_text_repr_from_bool(zt_bool value);
zt_dyn_text_repr *zt_dyn_text_repr_from_text(const zt_text *value);
zt_dyn_text_repr *zt_dyn_text_repr_from_text_owned(zt_text *value);
zt_dyn_text_repr *zt_dyn_text_repr_clone(const zt_dyn_text_repr *value);
zt_text *zt_dyn_text_repr_to_text(const zt_dyn_text_repr *value);
zt_int zt_dyn_text_repr_text_len(const zt_dyn_text_repr *value);

zt_list_dyn_text_repr *zt_list_dyn_text_repr_new(void);
zt_list_dyn_text_repr *zt_list_dyn_text_repr_from_array(zt_dyn_text_repr *const *items, size_t count);
zt_list_dyn_text_repr *zt_list_dyn_text_repr_from_array_owned(zt_dyn_text_repr **items, size_t count);
void zt_list_dyn_text_repr_push(zt_list_dyn_text_repr *list, zt_dyn_text_repr *value);
zt_dyn_text_repr *zt_list_dyn_text_repr_get(const zt_list_dyn_text_repr *list, zt_int index_0);
zt_int zt_list_dyn_text_repr_len(const zt_list_dyn_text_repr *list);
zt_list_dyn_text_repr *zt_list_dyn_text_repr_slice(const zt_list_dyn_text_repr *list, zt_int start_0, zt_int end_0);
zt_list_dyn_text_repr *zt_list_dyn_text_repr_deep_copy(const zt_list_dyn_text_repr *list);

zt_text *zt_thread_boundary_copy_text(const zt_text *value);
zt_bytes *zt_thread_boundary_copy_bytes(const zt_bytes *value);
zt_list_i64 *zt_thread_boundary_copy_list_i64(const zt_list_i64 *list);
zt_list_text *zt_thread_boundary_copy_list_text(const zt_list_text *list);
zt_map_text_text *zt_thread_boundary_copy_map_text_text(const zt_map_text_text *map);
zt_dyn_text_repr *zt_thread_boundary_copy_dyn_text_repr(const zt_dyn_text_repr *value);
zt_list_dyn_text_repr *zt_thread_boundary_copy_list_dyn_text_repr(const zt_list_dyn_text_repr *list);



zt_list_i64 *zt_queue_i64_new(void);
zt_list_i64 *zt_queue_i64_enqueue(zt_list_i64 *queue, zt_int value);
zt_list_i64 *zt_queue_i64_enqueue_owned(zt_list_i64 *queue, zt_int value);
zt_optional_i64 zt_queue_i64_dequeue(zt_list_i64 *queue);
zt_optional_i64 zt_queue_i64_peek(const zt_list_i64 *queue);

zt_list_text *zt_queue_text_new(void);
zt_list_text *zt_queue_text_enqueue(zt_list_text *queue, zt_text *value);
zt_list_text *zt_queue_text_enqueue_owned(zt_list_text *queue, zt_text *value);
zt_optional_text zt_queue_text_dequeue(zt_list_text *queue);
zt_optional_text zt_queue_text_peek(const zt_list_text *queue);

zt_list_i64 *zt_stack_i64_new(void);
zt_list_i64 *zt_stack_i64_push(zt_list_i64 *stack, zt_int value);
zt_list_i64 *zt_stack_i64_push_owned(zt_list_i64 *stack, zt_int value);
zt_optional_i64 zt_stack_i64_pop(zt_list_i64 *stack);
zt_optional_i64 zt_stack_i64_peek(const zt_list_i64 *stack);

zt_list_text *zt_stack_text_new(void);
zt_list_text *zt_stack_text_push(zt_list_text *stack, zt_text *value);
zt_list_text *zt_stack_text_push_owned(zt_list_text *stack, zt_text *value);
zt_optional_text zt_stack_text_pop(zt_list_text *stack);
zt_optional_text zt_stack_text_peek(const zt_list_text *stack);

zt_map_text_text *zt_map_text_text_new(void);
zt_map_text_text *zt_map_text_text_from_arrays(zt_text *const *keys, zt_text *const *values, size_t count);
void zt_map_text_text_set(zt_map_text_text *map, zt_text *key, zt_text *value);
zt_map_text_text *zt_map_text_text_set_owned(zt_map_text_text *map, zt_text *key, zt_text *value);
zt_text *zt_map_text_text_get(const zt_map_text_text *map, const zt_text *key);
zt_optional_text zt_map_text_text_get_optional(const zt_map_text_text *map, const zt_text *key);
zt_bool zt_map_text_text_contains(const zt_map_text_text *map, const zt_text *key);
zt_map_text_text *zt_map_text_text_remove(const zt_map_text_text *map, const zt_text *key);
zt_list_text *zt_map_text_text_keys(const zt_map_text_text *map);
zt_list_text *zt_map_text_text_values(const zt_map_text_text *map);
zt_map_text_text *zt_map_text_text_merge(const zt_map_text_text *left, const zt_map_text_text *right);
zt_text *zt_map_text_text_key_at(const zt_map_text_text *map, zt_int index_0);
zt_text *zt_map_text_text_value_at(const zt_map_text_text *map, zt_int index_0);
zt_int zt_map_text_text_len(const zt_map_text_text *map);

zt_set_i64 *zt_set_i64_create(void);
zt_set_i64 *zt_set_i64_from_array(const zt_int *items, size_t count);
void zt_set_i64_add(zt_set_i64 *set, zt_int value);
zt_bool zt_set_i64_has(const zt_set_i64 *set, zt_int value);
void zt_set_i64_remove(zt_set_i64 *set, zt_int value);
zt_int zt_set_i64_len(const zt_set_i64 *set);
zt_int zt_set_i64_value_at(const zt_set_i64 *set, zt_int index_0);
zt_set_i64 *zt_set_i64_union(const zt_set_i64 *left, const zt_set_i64 *right);
zt_set_i64 *zt_set_i64_intersect(const zt_set_i64 *left, const zt_set_i64 *right);
zt_set_i64 *zt_set_i64_difference(const zt_set_i64 *left, const zt_set_i64 *right);

zt_set_text *zt_set_text_create(void);
zt_set_text *zt_set_text_from_array(zt_text *const *items, size_t count);
void zt_set_text_add(zt_set_text *set, zt_text *value);
zt_bool zt_set_text_has(const zt_set_text *set, const zt_text *value);
void zt_set_text_remove(zt_set_text *set, const zt_text *value);
zt_int zt_set_text_len(const zt_set_text *set);
zt_text *zt_set_text_value_at(const zt_set_text *set, zt_int index_0);
zt_set_text *zt_set_text_union(const zt_set_text *left, const zt_set_text *right);
zt_set_text *zt_set_text_intersect(const zt_set_text *left, const zt_set_text *right);
zt_set_text *zt_set_text_difference(const zt_set_text *left, const zt_set_text *right);

zt_grid2d_i64 *zt_grid2d_i64_new(zt_int rows, zt_int cols);
zt_int zt_grid2d_i64_get(const zt_grid2d_i64 *grid, zt_int row, zt_int col);
zt_grid2d_i64 *zt_grid2d_i64_set(zt_grid2d_i64 *grid, zt_int row, zt_int col, zt_int value);
zt_grid2d_i64 *zt_grid2d_i64_set_owned(zt_grid2d_i64 *grid, zt_int row, zt_int col, zt_int value);
zt_grid2d_i64 *zt_grid2d_i64_fill(zt_grid2d_i64 *grid, zt_int value);
zt_grid2d_i64 *zt_grid2d_i64_fill_owned(zt_grid2d_i64 *grid, zt_int value);
zt_int zt_grid2d_i64_rows(const zt_grid2d_i64 *grid);
zt_int zt_grid2d_i64_cols(const zt_grid2d_i64 *grid);
zt_list_i64 *zt_grid2d_i64_values(const zt_grid2d_i64 *grid);

zt_grid2d_text *zt_grid2d_text_new(zt_int rows, zt_int cols);
zt_text *zt_grid2d_text_get(const zt_grid2d_text *grid, zt_int row, zt_int col);
zt_grid2d_text *zt_grid2d_text_set(zt_grid2d_text *grid, zt_int row, zt_int col, zt_text *value);
zt_grid2d_text *zt_grid2d_text_set_owned(zt_grid2d_text *grid, zt_int row, zt_int col, zt_text *value);
zt_grid2d_text *zt_grid2d_text_fill(zt_grid2d_text *grid, zt_text *value);
zt_grid2d_text *zt_grid2d_text_fill_owned(zt_grid2d_text *grid, zt_text *value);
zt_int zt_grid2d_text_rows(const zt_grid2d_text *grid);
zt_int zt_grid2d_text_cols(const zt_grid2d_text *grid);
zt_list_text *zt_grid2d_text_values(const zt_grid2d_text *grid);

zt_pqueue_i64 *zt_pqueue_i64_new(void);
zt_pqueue_i64 *zt_pqueue_i64_push(zt_pqueue_i64 *heap, zt_int value);
zt_pqueue_i64 *zt_pqueue_i64_push_owned(zt_pqueue_i64 *heap, zt_int value);
zt_optional_i64 zt_pqueue_i64_pop(zt_pqueue_i64 *heap);
zt_optional_i64 zt_pqueue_i64_peek(const zt_pqueue_i64 *heap);
zt_int zt_pqueue_i64_len(const zt_pqueue_i64 *heap);
zt_list_i64 *zt_pqueue_i64_values(const zt_pqueue_i64 *heap);

zt_pqueue_text *zt_pqueue_text_new(void);
zt_pqueue_text *zt_pqueue_text_push(zt_pqueue_text *heap, zt_text *value);
zt_pqueue_text *zt_pqueue_text_push_owned(zt_pqueue_text *heap, zt_text *value);
zt_optional_text zt_pqueue_text_pop(zt_pqueue_text *heap);
zt_optional_text zt_pqueue_text_peek(const zt_pqueue_text *heap);
zt_int zt_pqueue_text_len(const zt_pqueue_text *heap);
zt_list_text *zt_pqueue_text_values(const zt_pqueue_text *heap);

zt_circbuf_i64 *zt_circbuf_i64_new(zt_int capacity);
zt_circbuf_i64 *zt_circbuf_i64_push(zt_circbuf_i64 *buf, zt_int value);
zt_circbuf_i64 *zt_circbuf_i64_push_owned(zt_circbuf_i64 *buf, zt_int value);
zt_optional_i64 zt_circbuf_i64_pop(zt_circbuf_i64 *buf);
zt_optional_i64 zt_circbuf_i64_peek(const zt_circbuf_i64 *buf);
zt_int zt_circbuf_i64_len(const zt_circbuf_i64 *buf);
zt_int zt_circbuf_i64_capacity(const zt_circbuf_i64 *buf);
zt_bool zt_circbuf_i64_is_full(const zt_circbuf_i64 *buf);
zt_list_i64 *zt_circbuf_i64_values(const zt_circbuf_i64 *buf);

zt_circbuf_text *zt_circbuf_text_new(zt_int capacity);
zt_circbuf_text *zt_circbuf_text_push(zt_circbuf_text *buf, zt_text *value);
zt_circbuf_text *zt_circbuf_text_push_owned(zt_circbuf_text *buf, zt_text *value);
zt_optional_text zt_circbuf_text_pop(zt_circbuf_text *buf);
zt_optional_text zt_circbuf_text_peek(const zt_circbuf_text *buf);
zt_int zt_circbuf_text_len(const zt_circbuf_text *buf);
zt_int zt_circbuf_text_capacity(const zt_circbuf_text *buf);
zt_bool zt_circbuf_text_is_full(const zt_circbuf_text *buf);
zt_list_text *zt_circbuf_text_values(const zt_circbuf_text *buf);

zt_btreemap_text_text *zt_btreemap_text_text_new(void);
zt_btreemap_text_text *zt_btreemap_text_text_set(zt_btreemap_text_text *map, zt_text *key, zt_text *value);
zt_btreemap_text_text *zt_btreemap_text_text_set_owned(zt_btreemap_text_text *map, zt_text *key, zt_text *value);
zt_text *zt_btreemap_text_text_get(const zt_btreemap_text_text *map, const zt_text *key);
zt_optional_text zt_btreemap_text_text_get_optional(const zt_btreemap_text_text *map, const zt_text *key);
zt_bool zt_btreemap_text_text_contains(const zt_btreemap_text_text *map, const zt_text *key);
zt_btreemap_text_text *zt_btreemap_text_text_remove(zt_btreemap_text_text *map, const zt_text *key);
zt_btreemap_text_text *zt_btreemap_text_text_remove_owned(zt_btreemap_text_text *map, const zt_text *key);
zt_int zt_btreemap_text_text_len(const zt_btreemap_text_text *map);
zt_list_text *zt_btreemap_text_text_keys(const zt_btreemap_text_text *map);
zt_list_text *zt_btreemap_text_text_values(const zt_btreemap_text_text *map);

zt_btreeset_text *zt_btreeset_text_new(void);
zt_btreeset_text *zt_btreeset_text_insert(zt_btreeset_text *set, zt_text *value);
zt_btreeset_text *zt_btreeset_text_insert_owned(zt_btreeset_text *set, zt_text *value);
zt_bool zt_btreeset_text_contains(const zt_btreeset_text *set, const zt_text *value);
zt_btreeset_text *zt_btreeset_text_remove(zt_btreeset_text *set, const zt_text *value);
zt_btreeset_text *zt_btreeset_text_remove_owned(zt_btreeset_text *set, const zt_text *value);
zt_int zt_btreeset_text_len(const zt_btreeset_text *set);
zt_list_text *zt_btreeset_text_values(const zt_btreeset_text *set);

zt_grid3d_i64 *zt_grid3d_i64_new(zt_int depth, zt_int rows, zt_int cols);
zt_int zt_grid3d_i64_get(const zt_grid3d_i64 *grid, zt_int layer, zt_int row, zt_int col);
zt_grid3d_i64 *zt_grid3d_i64_set(zt_grid3d_i64 *grid, zt_int layer, zt_int row, zt_int col, zt_int value);
zt_grid3d_i64 *zt_grid3d_i64_set_owned(zt_grid3d_i64 *grid, zt_int layer, zt_int row, zt_int col, zt_int value);
zt_grid3d_i64 *zt_grid3d_i64_fill(zt_grid3d_i64 *grid, zt_int value);
zt_grid3d_i64 *zt_grid3d_i64_fill_owned(zt_grid3d_i64 *grid, zt_int value);
zt_int zt_grid3d_i64_depth(const zt_grid3d_i64 *grid);
zt_int zt_grid3d_i64_rows(const zt_grid3d_i64 *grid);
zt_int zt_grid3d_i64_cols(const zt_grid3d_i64 *grid);
zt_list_i64 *zt_grid3d_i64_values(const zt_grid3d_i64 *grid);

zt_grid3d_text *zt_grid3d_text_new(zt_int depth, zt_int rows, zt_int cols);
zt_text *zt_grid3d_text_get(const zt_grid3d_text *grid, zt_int layer, zt_int row, zt_int col);
zt_grid3d_text *zt_grid3d_text_set(zt_grid3d_text *grid, zt_int layer, zt_int row, zt_int col, zt_text *value);
zt_grid3d_text *zt_grid3d_text_set_owned(zt_grid3d_text *grid, zt_int layer, zt_int row, zt_int col, zt_text *value);
zt_grid3d_text *zt_grid3d_text_fill(zt_grid3d_text *grid, zt_text *value);
zt_grid3d_text *zt_grid3d_text_fill_owned(zt_grid3d_text *grid, zt_text *value);
zt_int zt_grid3d_text_depth(const zt_grid3d_text *grid);
zt_int zt_grid3d_text_rows(const zt_grid3d_text *grid);
zt_int zt_grid3d_text_cols(const zt_grid3d_text *grid);
zt_list_text *zt_grid3d_text_values(const zt_grid3d_text *grid);

zt_optional_i64 zt_optional_i64_present(zt_int value);
zt_optional_i64 zt_optional_i64_empty(void);
zt_bool zt_optional_i64_is_present(zt_optional_i64 value);
zt_int zt_optional_i64_coalesce(zt_optional_i64 value, zt_int fallback);
zt_int zt_optional_i64_value(zt_optional_i64 value);

#define ZT_DECLARE_PRIMITIVE_OPTIONAL_API(SUFFIX, ELEM_TYPE) \
zt_optional_##SUFFIX zt_optional_##SUFFIX##_present(ELEM_TYPE value); \
zt_optional_##SUFFIX zt_optional_##SUFFIX##_empty(void); \
zt_bool zt_optional_##SUFFIX##_is_present(zt_optional_##SUFFIX value); \
ELEM_TYPE zt_optional_##SUFFIX##_coalesce(zt_optional_##SUFFIX value, ELEM_TYPE fallback); \
ELEM_TYPE zt_optional_##SUFFIX##_value(zt_optional_##SUFFIX value);

ZT_DECLARE_PRIMITIVE_OPTIONAL_API(f64, zt_float)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(bool, zt_bool)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(i8, int8_t)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(i16, int16_t)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(i32, int32_t)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(u8, uint8_t)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(u16, uint16_t)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(u32, uint32_t)
ZT_DECLARE_PRIMITIVE_OPTIONAL_API(u64, uint64_t)

#undef ZT_DECLARE_PRIMITIVE_OPTIONAL_API

zt_optional_text zt_optional_text_present(zt_text *value);
zt_optional_text zt_optional_text_empty(void);
zt_bool zt_optional_text_is_present(zt_optional_text value);
zt_text *zt_optional_text_coalesce(zt_optional_text value, zt_text *fallback);
zt_text *zt_optional_text_value(zt_optional_text value);

zt_optional_bytes zt_optional_bytes_present(zt_bytes *value);
zt_optional_bytes zt_optional_bytes_empty(void);
zt_bool zt_optional_bytes_is_present(zt_optional_bytes value);
zt_bytes *zt_optional_bytes_coalesce(zt_optional_bytes value, zt_bytes *fallback);
zt_bytes *zt_optional_bytes_value(zt_optional_bytes value);

zt_optional_list_i64 zt_optional_list_i64_present(zt_list_i64 *value);
zt_optional_list_i64 zt_optional_list_i64_empty(void);
zt_bool zt_optional_list_i64_is_present(zt_optional_list_i64 value);
zt_list_i64 *zt_optional_list_i64_coalesce(zt_optional_list_i64 value, zt_list_i64 *fallback);
zt_list_i64 *zt_optional_list_i64_value(zt_optional_list_i64 value);

zt_optional_list_text zt_optional_list_text_present(zt_list_text *value);
zt_optional_list_text zt_optional_list_text_empty(void);
zt_bool zt_optional_list_text_is_present(zt_optional_list_text value);
zt_list_text *zt_optional_list_text_coalesce(zt_optional_list_text value, zt_list_text *fallback);

zt_optional_map_text_text zt_optional_map_text_text_present(zt_map_text_text *value);
zt_optional_map_text_text zt_optional_map_text_text_empty(void);
zt_bool zt_optional_map_text_text_is_present(zt_optional_map_text_text value);
zt_map_text_text *zt_optional_map_text_text_coalesce(zt_optional_map_text_text value, zt_map_text_text *fallback);

#define ZT_DECLARE_OUTCOME_VALUE_API(SUFFIX, VALUE_TYPE, ERROR_TYPE) \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(VALUE_TYPE value); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(ERROR_TYPE error); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message); \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome); \
VALUE_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(zt_outcome_##SUFFIX outcome); \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome);

#define ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(SUFFIX, VALUE_TYPE, ERROR_TYPE) \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(VALUE_TYPE value); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(ERROR_TYPE error); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_text(zt_text *message); \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome); \
VALUE_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(zt_outcome_##SUFFIX outcome); \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome);

#define ZT_DECLARE_OUTCOME_VOID_API(SUFFIX, ERROR_TYPE) \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(void); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(ERROR_TYPE error); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message); \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(zt_outcome_##SUFFIX outcome); \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome);

#define ZT_DECLARE_OUTCOME_VOID_API_WITH_FAILURE_TEXT(SUFFIX, ERROR_TYPE) \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(void); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(ERROR_TYPE error); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_text(zt_text *message); \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome); \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(zt_outcome_##SUFFIX outcome); \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome);

ZT_DECLARE_OUTCOME_VALUE_API(i64_text, zt_int, zt_text *)
ZT_DECLARE_OUTCOME_VOID_API(void_text, zt_text *)

typedef struct zt_host_api {
    zt_outcome_text_core_error (*read_file)(const zt_text *path);
    zt_outcome_void_core_error (*write_file)(const zt_text *path, const zt_text *value);
    zt_bool (*path_exists)(const zt_text *path);
    zt_outcome_optional_text_core_error (*read_line_stdin)(void);
    zt_outcome_text_core_error (*read_all_stdin)(void);
    zt_outcome_void_core_error (*write_stdout)(const zt_text *value);
    zt_outcome_void_core_error (*write_stderr)(const zt_text *value);
    zt_int (*time_now_unix_ms)(void);
    zt_outcome_void_core_error (*time_sleep_ms)(zt_int duration_ms);
    void (*random_seed)(zt_int seed);
    zt_int (*random_next_i64)(void);
    zt_outcome_text_core_error (*os_current_dir)(void);
    zt_outcome_void_core_error (*os_change_dir)(const zt_text *path);
    zt_list_text *(*os_args)(void);
    zt_optional_text (*os_env)(const zt_text *name);
    zt_int (*os_pid)(void);
    zt_text *(*os_platform)(void);
    zt_text *(*os_arch)(void);
    zt_outcome_i64_core_error (*process_run)(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);
    zt_outcome_process_captured_run_core_error (*process_run_capture)(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);
} zt_host_api;

void zt_runtime_capture_process_args(int argc, char **argv);
void zt_host_set_api(const zt_host_api *api);
const zt_host_api *zt_host_get_api(void);
zt_outcome_text_core_error zt_host_read_file(const zt_text *path);
zt_outcome_void_core_error zt_host_write_file(const zt_text *path, const zt_text *value);
zt_bool zt_host_path_exists(const zt_text *path);
zt_outcome_optional_text_core_error zt_host_read_line_stdin(void);
zt_outcome_text_core_error zt_host_read_all_stdin(void);
zt_outcome_void_core_error zt_host_write_stdout(const zt_text *value);
zt_outcome_void_core_error zt_host_write_stderr(const zt_text *value);
zt_bool zt_host_console_is_terminal(const zt_text *stream);
zt_int zt_host_console_columns(void);
zt_int zt_host_console_rows(void);
zt_outcome_void_core_error zt_host_console_clear(void);
zt_outcome_void_core_error zt_host_console_set_color(const zt_text *name);
zt_outcome_void_core_error zt_host_console_set_style(const zt_text *name);
zt_outcome_void_core_error zt_host_console_reset_style(void);
zt_outcome_optional_text_core_error zt_host_console_read_key(void);
zt_int zt_host_time_now_unix_ms(void);
zt_outcome_void_core_error zt_host_time_sleep_ms(zt_int duration_ms);
void zt_host_random_seed(zt_int seed);
zt_int zt_host_random_next_i64(void);
zt_outcome_text_core_error zt_host_os_current_dir(void);
zt_outcome_void_core_error zt_host_os_change_dir(const zt_text *path);
zt_outcome_text_core_error zt_host_os_current_dir_core(void);
zt_outcome_void_core_error zt_host_os_change_dir_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_append_text_core(const zt_text *path, const zt_text *value);
zt_outcome_bytes_core_error zt_host_fs_read_bytes_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_write_bytes_core(const zt_text *path, const zt_bytes *value);
zt_outcome_bool_core_error zt_host_fs_is_file_core(const zt_text *path);
zt_outcome_bool_core_error zt_host_fs_is_dir_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_create_dir_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_create_dir_all_core(const zt_text *path);
zt_outcome_list_text_core_error zt_host_fs_list_core(const zt_text *path);
zt_outcome_list_text_core_error zt_host_fs_walk_dir_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_remove_file_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_remove_dir_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_remove_dir_all_core(const zt_text *path);
zt_outcome_void_core_error zt_host_fs_copy_file_core(const zt_text *from_path, const zt_text *to_path);
zt_outcome_void_core_error zt_host_fs_move_core(const zt_text *from_path, const zt_text *to_path);
zt_outcome_i64_core_error zt_host_fs_size_core(const zt_text *path);
zt_outcome_i64_core_error zt_host_fs_modified_at_core(const zt_text *path);
zt_outcome_optional_i64_core_error zt_host_fs_created_at_core(const zt_text *path);
zt_outcome_void_core_error zt_regex_validate_core(const zt_text *pattern);
zt_bool zt_regex_is_match_core(const zt_text *pattern, const zt_text *input);
zt_bool zt_regex_full_match_core(const zt_text *pattern, const zt_text *input);
zt_optional_text zt_regex_first_core(const zt_text *pattern, const zt_text *input);
zt_int zt_regex_count_core(const zt_text *pattern, const zt_text *input);
zt_list_text *zt_regex_find_all_core(const zt_text *pattern, const zt_text *input);
zt_list_text *zt_regex_split_core(const zt_text *pattern, const zt_text *input);
zt_text *zt_regex_replace_all_core(const zt_text *pattern, const zt_text *input, const zt_text *replacement);
zt_text *zt_regex_escape_core(const zt_text *input);
zt_list_text *zt_host_os_args(void);
zt_optional_text zt_host_os_env(const zt_text *name);
zt_int zt_host_os_pid(void);

zt_closure *zt_closure_create(void *fn, void *ctx);
zt_closure *zt_closure_create_with_drop(void *fn, void *ctx, void (*drop_ctx)(void *));
zt_int zt_job_spawn_i64(zt_closure *thunk);
zt_int zt_job_spawn_i64_arg(zt_closure *worker, zt_int value);
zt_int zt_job_join_i64(zt_int handle);
zt_int zt_channel_i64_create(void);
zt_int zt_channel_i64_send(zt_int handle, zt_int value);
zt_optional_i64 zt_channel_i64_receive(zt_int handle);
zt_int zt_channel_i64_close(zt_int handle);
zt_int zt_shared_i64_create(zt_int value);
zt_int zt_shared_i64_get(zt_int handle);
zt_int zt_shared_i64_set(zt_int handle, zt_int value);
zt_int zt_atomic_i64_create(zt_int value);
zt_int zt_atomic_i64_load(zt_int handle);
zt_int zt_atomic_i64_store(zt_int handle, zt_int value);
zt_int zt_atomic_i64_add(zt_int handle, zt_int delta);
zt_lazy_i64 *zt_lazy_i64_once(zt_closure *thunk);
zt_int zt_lazy_i64_force(zt_lazy_i64 *value);
zt_bool zt_lazy_i64_is_consumed(const zt_lazy_i64 *value);
zt_lazy_f64 *zt_lazy_f64_once(zt_closure *thunk);
zt_float zt_lazy_f64_force(zt_lazy_f64 *value);
zt_bool zt_lazy_f64_is_consumed(const zt_lazy_f64 *value);
zt_lazy_bool *zt_lazy_bool_once(zt_closure *thunk);
zt_bool zt_lazy_bool_force(zt_lazy_bool *value);
zt_bool zt_lazy_bool_is_consumed(const zt_lazy_bool *value);
zt_lazy_i8 *zt_lazy_i8_once(zt_closure *thunk);
int8_t zt_lazy_i8_force(zt_lazy_i8 *value);
zt_bool zt_lazy_i8_is_consumed(const zt_lazy_i8 *value);
zt_lazy_i16 *zt_lazy_i16_once(zt_closure *thunk);
int16_t zt_lazy_i16_force(zt_lazy_i16 *value);
zt_bool zt_lazy_i16_is_consumed(const zt_lazy_i16 *value);
zt_lazy_i32 *zt_lazy_i32_once(zt_closure *thunk);
int32_t zt_lazy_i32_force(zt_lazy_i32 *value);
zt_bool zt_lazy_i32_is_consumed(const zt_lazy_i32 *value);
zt_lazy_u8 *zt_lazy_u8_once(zt_closure *thunk);
uint8_t zt_lazy_u8_force(zt_lazy_u8 *value);
zt_bool zt_lazy_u8_is_consumed(const zt_lazy_u8 *value);
zt_lazy_u16 *zt_lazy_u16_once(zt_closure *thunk);
uint16_t zt_lazy_u16_force(zt_lazy_u16 *value);
zt_bool zt_lazy_u16_is_consumed(const zt_lazy_u16 *value);
zt_lazy_u32 *zt_lazy_u32_once(zt_closure *thunk);
uint32_t zt_lazy_u32_force(zt_lazy_u32 *value);
zt_bool zt_lazy_u32_is_consumed(const zt_lazy_u32 *value);
zt_lazy_u64 *zt_lazy_u64_once(zt_closure *thunk);
uint64_t zt_lazy_u64_force(zt_lazy_u64 *value);
zt_bool zt_lazy_u64_is_consumed(const zt_lazy_u64 *value);
zt_lazy_text *zt_lazy_text_once(zt_closure *thunk);
zt_text *zt_lazy_text_force(zt_lazy_text *value);
zt_bool zt_lazy_text_is_consumed(const zt_lazy_text *value);

zt_text *zt_host_os_platform(void);
zt_text *zt_host_os_arch(void);
zt_outcome_i64_core_error zt_host_process_run(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);
zt_outcome_process_captured_run_core_error zt_host_process_run_capture(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);
zt_outcome_i64_core_error zt_host_process_run_core(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);
zt_outcome_process_captured_run_core_error zt_host_process_run_capture_core(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);

ZT_DECLARE_OUTCOME_VALUE_API(text_text, zt_text *, zt_text *)

zt_bool zt_outcome_text_text_eq(zt_outcome_text_text left, zt_outcome_text_text right);
ZT_DECLARE_OUTCOME_VALUE_API(list_i64_text, zt_list_i64 *, zt_text *)
ZT_DECLARE_OUTCOME_VALUE_API(list_text_text, zt_list_text *, zt_text *)
ZT_DECLARE_OUTCOME_VALUE_API(map_text_text, zt_map_text_text *, zt_text *)

ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(i64_core_error, zt_int, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(f64_core_error, zt_float, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(bool_core_error, zt_bool, zt_core_error)
ZT_DECLARE_OUTCOME_VOID_API_WITH_FAILURE_TEXT(void_core_error, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(text_core_error, zt_text *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(bytes_core_error, zt_bytes *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API(process_captured_run_core_error, zt_process_captured_run, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(optional_text_core_error, zt_optional_text, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(optional_bytes_core_error, zt_optional_bytes, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(net_connection_core_error, zt_net_connection *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_i64_core_error, zt_list_i64 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_text_core_error, zt_list_text *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_f64_core_error, zt_list_f64 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_bool_core_error, zt_list_bool *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_i8_core_error, zt_list_i8 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_i16_core_error, zt_list_i16 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_i32_core_error, zt_list_i32 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_u8_core_error, zt_list_u8 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_u16_core_error, zt_list_u16 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_u32_core_error, zt_list_u32 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(list_u64_core_error, zt_list_u64 *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(map_text_text_core_error, zt_map_text_text *, zt_core_error)
ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT(optional_i64_core_error, zt_optional_i64, zt_core_error)

#undef ZT_DECLARE_OUTCOME_VALUE_API
#undef ZT_DECLARE_OUTCOME_VALUE_API_WITH_FAILURE_TEXT
#undef ZT_DECLARE_OUTCOME_VOID_API
#undef ZT_DECLARE_OUTCOME_VOID_API_WITH_FAILURE_TEXT

zt_outcome_map_text_text_core_error zt_json_parse_map_text_text(const zt_text *input);
zt_text *zt_json_stringify_map_text_text(const zt_map_text_text *value);
zt_text *zt_json_pretty_map_text_text(const zt_map_text_text *value, zt_int indent);
zt_outcome_text_core_error zt_json_validate_full(const zt_text *input);
zt_text *zt_json_pretty_full(const zt_text *input, zt_int indent);
zt_int zt_json_kind_index(const zt_text *input);
zt_optional_text zt_json_as_text(const zt_text *input);
zt_optional_i64 zt_json_as_int(const zt_text *input);
zt_optional_f64 zt_json_as_float(const zt_text *input);
zt_optional_bool zt_json_as_bool(const zt_text *input);
zt_optional_text zt_json_get_raw(const zt_text *input, const zt_text *key);
zt_optional_text zt_json_at_raw(const zt_text *input, zt_int index);
zt_int zt_json_len(const zt_text *input);

zt_text *zt_encoding_hex_encode(const zt_bytes *data);
zt_outcome_bytes_core_error zt_encoding_hex_decode(const zt_text *text_value);
zt_text *zt_encoding_base64_encode(const zt_bytes *data);
zt_outcome_bytes_core_error zt_encoding_base64_decode(const zt_text *text_value);

zt_text *zt_hash_sha256_text(const zt_text *value);
zt_text *zt_hash_sha256_bytes(const zt_bytes *value);
zt_text *zt_hash_md5_text(const zt_text *value);
zt_text *zt_hash_md5_bytes(const zt_bytes *value);

zt_outcome_f64_core_error zt_random_float_between_core(zt_float min, zt_float max);
zt_optional_i64 zt_random_choice_i64(const zt_list_i64 *items);
zt_optional_text zt_random_choice_text(const zt_list_text *items);
zt_list_i64 *zt_random_shuffle_i64(const zt_list_i64 *items);
zt_list_text *zt_random_shuffle_text(const zt_list_text *items);

zt_text *zt_format_number(zt_float value, zt_int decimals);
zt_text *zt_format_percent(zt_float value, zt_int decimals);
zt_text *zt_format_date(zt_int millis, const zt_text *style);
zt_text *zt_format_datetime(zt_int millis, const zt_text *style, const zt_text *locale);
zt_text *zt_format_date_pattern(zt_int millis, const zt_text *pattern);
zt_text *zt_format_datetime_pattern(zt_int millis, const zt_text *pattern);
zt_text *zt_format_hex_i64(zt_int value);
zt_text *zt_format_bin_i64(zt_int value);
zt_text *zt_format_bytes_binary(zt_int value, zt_int decimals);
zt_text *zt_format_bytes_decimal(zt_int value, zt_int decimals);

zt_float zt_math_pow(zt_float base, zt_float exponent);
zt_float zt_math_sqrt(zt_float value);
zt_float zt_math_floor(zt_float value);
zt_float zt_math_ceil(zt_float value);
zt_float zt_math_round_half_away_from_zero(zt_float value);
zt_float zt_math_trunc(zt_float value);
zt_float zt_math_sin(zt_float value);
zt_float zt_math_cos(zt_float value);
zt_float zt_math_tan(zt_float value);
zt_float zt_math_asin(zt_float value);
zt_float zt_math_acos(zt_float value);
zt_float zt_math_atan(zt_float value);
zt_float zt_math_atan2(zt_float y, zt_float x);
zt_float zt_math_ln(zt_float value);
zt_float zt_math_log10(zt_float value);
zt_float zt_math_log_ten(zt_float value);
zt_float zt_math_log2(zt_float value);
zt_float zt_math_log(zt_float value, zt_float base);
zt_float zt_math_exp(zt_float value);
zt_float zt_math_infinity(void);
zt_float zt_math_nan(void);
zt_bool zt_math_is_nan(zt_float value);
zt_bool zt_math_is_infinite(zt_float value);
zt_bool zt_math_is_finite(zt_float value);
zt_bool zt_float_lt(zt_float left, zt_float right);
zt_bool zt_float_le(zt_float left, zt_float right);
zt_bool zt_float_gt(zt_float left, zt_float right);
zt_bool zt_float_ge(zt_float left, zt_float right);

zt_outcome_net_connection_core_error zt_net_connect(const zt_text *host, zt_int port, zt_int timeout_ms);
zt_outcome_optional_bytes_core_error zt_net_read_some(zt_net_connection *connection, zt_int max, zt_int timeout_ms);
zt_outcome_void_core_error zt_net_write_all(zt_net_connection *connection, const zt_bytes *data, zt_int timeout_ms);
zt_outcome_void_core_error zt_net_close(zt_net_connection *connection);
zt_bool zt_net_is_closed(const zt_net_connection *connection);
zt_int zt_net_error_kind_index(zt_core_error error);
zt_outcome_text_core_error zt_http_get_core(const zt_text *url);
zt_outcome_text_core_error zt_http_post_core(const zt_text *url, const zt_text *body, const zt_text *content_type);

zt_outcome_i64_core_error zt_borealis_open_window(const zt_text *title, zt_int width, zt_int height, zt_int target_fps, zt_int backend_id);
zt_outcome_void_core_error zt_borealis_close_window(zt_int window_id);
zt_bool zt_borealis_window_should_close(zt_int window_id);
zt_outcome_void_core_error zt_borealis_begin_frame(zt_int window_id, zt_int clear_r, zt_int clear_g, zt_int clear_b, zt_int clear_a);
zt_outcome_void_core_error zt_borealis_end_frame(zt_int window_id);
zt_outcome_void_core_error zt_borealis_draw_rect(
    zt_int window_id,
    zt_float x,
    zt_float y,
    zt_float width,
    zt_float height,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_draw_line(
    zt_int window_id,
    zt_float x1,
    zt_float y1,
    zt_float x2,
    zt_float y2,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_draw_rect_outline(
    zt_int window_id,
    zt_float x,
    zt_float y,
    zt_float width,
    zt_float height,
    zt_float thickness,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_draw_circle(
    zt_int window_id,
    zt_float x,
    zt_float y,
    zt_float radius,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_draw_circle_outline(
    zt_int window_id,
    zt_float x,
    zt_float y,
    zt_float radius,
    zt_float thickness,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_draw_text(
    zt_int window_id,
    const zt_text *value,
    zt_int x,
    zt_int y,
    zt_int size,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_bool zt_borealis_is_key_down(zt_int window_id, zt_int input_code);
zt_bool zt_borealis_is_key_pressed(zt_int window_id, zt_int input_code);
zt_bool zt_borealis_is_key_released(zt_int window_id, zt_int input_code);
zt_outcome_void_core_error zt_borealis_stub_set_key_down(zt_int window_id, zt_int input_code, zt_bool is_down);
zt_outcome_void_core_error zt_borealis_stub_reset_input(zt_int window_id);
zt_bool zt_borealis_raylib_available(void);
zt_text *zt_borealis_raylib_loaded_path(void);
zt_outcome_void_core_error zt_borealis_raylib_draw_triangle(
    zt_int window_id,
    zt_float x1,
    zt_float y1,
    zt_float x2,
    zt_float y2,
    zt_float x3,
    zt_float y3,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_raylib_draw_ellipse(
    zt_int window_id,
    zt_float x,
    zt_float y,
    zt_float radius_h,
    zt_float radius_v,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_int zt_borealis_raylib_measure_text(const zt_text *value, zt_int font_size);
zt_outcome_i64_core_error zt_borealis_raylib_load_texture(const zt_text *path);
zt_outcome_void_core_error zt_borealis_raylib_unload_texture(zt_int texture_handle);
zt_int zt_borealis_raylib_texture_width(zt_int texture_handle);
zt_int zt_borealis_raylib_texture_height(zt_int texture_handle);
zt_outcome_void_core_error zt_borealis_raylib_draw_texture(
    zt_int window_id,
    zt_int texture_handle,
    zt_float x,
    zt_float y,
    zt_int tint_r,
    zt_int tint_g,
    zt_int tint_b,
    zt_int tint_a);
zt_outcome_void_core_error zt_borealis_raylib_draw_texture_ex(
    zt_int window_id,
    zt_int texture_handle,
    zt_float x,
    zt_float y,
    zt_float rotation,
    zt_float scale,
    zt_int tint_r,
    zt_int tint_g,
    zt_int tint_b,
    zt_int tint_a);
zt_outcome_void_core_error zt_borealis_raylib_init_audio_device(void);
zt_outcome_void_core_error zt_borealis_raylib_close_audio_device(void);
zt_bool zt_borealis_raylib_is_audio_device_ready(void);
zt_outcome_void_core_error zt_borealis_raylib_set_master_volume(zt_float volume);
zt_outcome_i64_core_error zt_borealis_raylib_load_sound(const zt_text *path);
zt_outcome_void_core_error zt_borealis_raylib_unload_sound(zt_int sound_handle);
zt_outcome_void_core_error zt_borealis_raylib_play_sound(zt_int sound_handle);
zt_outcome_void_core_error zt_borealis_raylib_stop_sound(zt_int sound_handle);
zt_outcome_void_core_error zt_borealis_raylib_set_sound_volume(zt_int sound_handle, zt_float volume);
zt_outcome_void_core_error zt_borealis_raylib_begin_mode3d(
    zt_int window_id,
    zt_float position_x,
    zt_float position_y,
    zt_float position_z,
    zt_float target_x,
    zt_float target_y,
    zt_float target_z,
    zt_float up_x,
    zt_float up_y,
    zt_float up_z,
    zt_float fov_y,
    zt_int projection);
zt_outcome_void_core_error zt_borealis_raylib_end_mode3d(zt_int window_id);
zt_outcome_void_core_error zt_borealis_raylib_draw_cube(
    zt_int window_id,
    zt_float x,
    zt_float y,
    zt_float z,
    zt_float width,
    zt_float height,
    zt_float depth,
    zt_int color_r,
    zt_int color_g,
    zt_int color_b,
    zt_int color_a);
zt_outcome_void_core_error zt_borealis_raylib_draw_grid(
    zt_int window_id,
    zt_int slices,
    zt_float spacing);
zt_outcome_i64_core_error zt_borealis_raylib_load_model(const zt_text *path);
zt_outcome_void_core_error zt_borealis_raylib_unload_model(zt_int model_handle);
zt_outcome_void_core_error zt_borealis_raylib_draw_model(
    zt_int window_id,
    zt_int model_handle,
    zt_float position_x,
    zt_float position_y,
    zt_float position_z,
    zt_float rotation_x,
    zt_float rotation_y,
    zt_float rotation_z,
    zt_float scale_x,
    zt_float scale_y,
    zt_float scale_z,
    zt_int tint_r,
    zt_int tint_g,
    zt_int tint_b,
    zt_int tint_a);
zt_outcome_void_core_error zt_borealis_raylib_draw_billboard(
    zt_int window_id,
    zt_int texture_handle,
    zt_float camera_position_x,
    zt_float camera_position_y,
    zt_float camera_position_z,
    zt_float camera_target_x,
    zt_float camera_target_y,
    zt_float camera_target_z,
    zt_float camera_up_x,
    zt_float camera_up_y,
    zt_float camera_up_z,
    zt_float camera_fov_y,
    zt_int camera_projection,
    zt_float position_x,
    zt_float position_y,
    zt_float position_z,
    zt_float size_x,
    zt_float size_y,
    zt_int tint_r,
    zt_int tint_g,
    zt_int tint_b,
    zt_int tint_a);
zt_float zt_borealis_raylib_vector2_length(zt_float x, zt_float y);
zt_float zt_borealis_raylib_vector2_distance(zt_float ax, zt_float ay, zt_float bx, zt_float by);
zt_float zt_borealis_raylib_lerp(zt_float start, zt_float finish, zt_float amount);
zt_float zt_borealis_raylib_ease_linear(zt_float t, zt_float b, zt_float c, zt_float d);
zt_float zt_borealis_raylib_ease_sine_in(zt_float t, zt_float b, zt_float c, zt_float d);
zt_float zt_borealis_raylib_ease_sine_out(zt_float t, zt_float b, zt_float c, zt_float d);
zt_float zt_borealis_raylib_ease_sine_in_out(zt_float t, zt_float b, zt_float c, zt_float d);
zt_float zt_borealis_raylib_ease_quad_in(zt_float t, zt_float b, zt_float c, zt_float d);
zt_float zt_borealis_raylib_ease_quad_out(zt_float t, zt_float b, zt_float c, zt_float d);
zt_float zt_borealis_raylib_ease_quad_in_out(zt_float t, zt_float b, zt_float c, zt_float d);

typedef struct zt_borealis_desktop_api {
    zt_outcome_i64_core_error (*open_window)(const zt_text *title, zt_int width, zt_int height, zt_int target_fps, zt_int backend_id);
    zt_outcome_void_core_error (*close_window)(zt_int window_id);
    zt_bool (*window_should_close)(zt_int window_id);
    zt_outcome_void_core_error (*begin_frame)(zt_int window_id, zt_int clear_r, zt_int clear_g, zt_int clear_b, zt_int clear_a);
    zt_outcome_void_core_error (*end_frame)(zt_int window_id);
    zt_outcome_void_core_error (*draw_rect)(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a);
    zt_outcome_void_core_error (*draw_line)(
        zt_int window_id,
        zt_float x1,
        zt_float y1,
        zt_float x2,
        zt_float y2,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a);
    zt_outcome_void_core_error (*draw_rect_outline)(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height,
        zt_float thickness,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a);
    zt_outcome_void_core_error (*draw_circle)(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a);
    zt_outcome_void_core_error (*draw_circle_outline)(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius,
        zt_float thickness,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a);
    zt_outcome_void_core_error (*draw_text)(
        zt_int window_id,
        const zt_text *value,
        zt_int x,
        zt_int y,
        zt_int size,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a);
    zt_bool (*is_key_down)(zt_int window_id, zt_int input_code);
    zt_bool (*is_key_pressed)(zt_int window_id, zt_int input_code);
    zt_bool (*is_key_released)(zt_int window_id, zt_int input_code);
} zt_borealis_desktop_api;

void zt_borealis_set_desktop_api(const zt_borealis_desktop_api *api);
const zt_borealis_desktop_api *zt_borealis_get_desktop_api(void);

zt_text *zt_path_normalize(const zt_text *value);
zt_bool zt_path_is_absolute(const zt_text *value);
zt_text *zt_path_absolute(const zt_text *value, const zt_text *base);
zt_text *zt_path_relative(const zt_text *value, const zt_text *from);

zt_int zt_add_i64(zt_int a, zt_int b);
zt_int zt_sub_i64(zt_int a, zt_int b);
zt_int zt_mul_i64(zt_int a, zt_int b);
zt_int zt_div_i64(zt_int a, zt_int b);
zt_int zt_rem_i64(zt_int a, zt_int b);
zt_bool zt_validate_between_i64(zt_int value, zt_int min, zt_int max);

#include "zenith_collections_generic.h"

#ifdef __cplusplus
}
#endif

#endif







