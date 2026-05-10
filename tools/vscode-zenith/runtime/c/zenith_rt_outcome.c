static zt_bool zt_fs_is_separator(char ch) {
    return ch == '/' || ch == '\\';
}

static zt_core_error zt_fs_core_error_from_code_message(const char *code, const char *message) {
    return zt_core_error_from_message(code != NULL ? code : "fs.unknown", zt_safe_message(message));
}

static zt_core_error zt_fs_core_error_from_errno(int error_code, const char *fallback_message) {
    const char *code = "fs.io";
    const char *message = fallback_message;

    if ((message == NULL || message[0] == '\0') && error_code != 0) {
        message = strerror(error_code);
    }
    if (message == NULL || message[0] == '\0') {
        message = "filesystem error";
    }

    switch (error_code) {
        case 0:
            code = "fs.unknown";
            break;
        case ENOENT:
            code = "fs.not_found";
            break;
        case EACCES:
        case EPERM:
            code = "fs.permission_denied";
            break;
        case EEXIST:
            code = "fs.already_exists";
            break;
#ifdef ENOTDIR
        case ENOTDIR:
            code = "fs.not_a_directory";
            break;
#endif
#ifdef EISDIR
        case EISDIR:
            code = "fs.is_a_directory";
            break;
#endif
#ifdef EINVAL
        case EINVAL:
            code = "fs.invalid_path";
            break;
#endif
#ifdef ENAMETOOLONG
        case ENAMETOOLONG:
            code = "fs.invalid_path";
            break;
#endif
        default:
            code = "fs.io";
            break;
    }

    return zt_fs_core_error_from_code_message(code, message);
}

#ifdef _WIN32
static void zt_fs_windows_error_message(DWORD error_code, char *buffer, size_t buffer_size) {
    DWORD written;

    if (buffer == NULL || buffer_size == 0) {
        return;
    }

    written = FormatMessageA(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        NULL,
        error_code,
        MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
        buffer,
        (DWORD)buffer_size,
        NULL);
    if (written == 0) {
        snprintf(buffer, buffer_size, "Windows error %lu", (unsigned long)error_code);
        return;
    }

    while (written > 0 && (buffer[written - 1] == '\r' || buffer[written - 1] == '\n' || buffer[written - 1] == ' ')) {
        buffer[written - 1] = '\0';
        written -= 1;
    }
}

static zt_core_error zt_fs_core_error_from_windows(DWORD error_code, const char *fallback_message) {
    const char *code = "fs.io";
    char message_buffer[256];
    const char *message = fallback_message;

    if (message == NULL || message[0] == '\0') {
        zt_fs_windows_error_message(error_code, message_buffer, sizeof(message_buffer));
        message = message_buffer;
    }

    switch (error_code) {
        case ERROR_FILE_NOT_FOUND:
        case ERROR_PATH_NOT_FOUND:
            code = "fs.not_found";
            break;
        case ERROR_ACCESS_DENIED:
        case ERROR_SHARING_VIOLATION:
            code = "fs.permission_denied";
            break;
        case ERROR_ALREADY_EXISTS:
        case ERROR_FILE_EXISTS:
            code = "fs.already_exists";
            break;
        case ERROR_DIRECTORY:
            code = "fs.not_a_directory";
            break;
        case ERROR_INVALID_NAME:
        case ERROR_BAD_PATHNAME:
            code = "fs.invalid_path";
            break;
        default:
            code = "fs.io";
            break;
    }

    return zt_fs_core_error_from_code_message(code, message);
}
#endif

#define ZT_DEFINE_FS_FAILURE_HELPER(NAME, OUTCOME_TYPE, FAILURE_FN) \
static OUTCOME_TYPE NAME(zt_core_error error) {                     \
    OUTCOME_TYPE outcome = FAILURE_FN(error);                       \
    zt_core_error_dispose(&error);                                  \
    return outcome;                                                 \
}

ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_void_failure_error, zt_outcome_void_core_error, zt_outcome_void_core_error_failure)
ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_text_failure_error, zt_outcome_text_core_error, zt_outcome_text_core_error_failure)
ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_bool_failure_error, zt_outcome_bool_core_error, zt_outcome_bool_core_error_failure)
ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_i64_failure_error, zt_outcome_i64_core_error, zt_outcome_i64_core_error_failure)
ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_bytes_failure_error, zt_outcome_bytes_core_error, zt_outcome_bytes_core_error_failure)
ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_list_text_failure_error, zt_outcome_list_text_core_error, zt_outcome_list_text_core_error_failure)
ZT_DEFINE_FS_FAILURE_HELPER(zt_fs_outcome_optional_i64_failure_error, zt_outcome_optional_i64_core_error, zt_outcome_optional_i64_core_error_failure)

static char *zt_fs_join_path(const char *base, const char *name) {
    size_t base_len;
    size_t name_len;
    size_t total_len;
    size_t offset = 0;
    char *joined;
    zt_bool needs_sep;

    if (base == NULL || name == NULL) {
        return NULL;
    }

    base_len = strlen(base);
    name_len = strlen(name);
    needs_sep = base_len > 0 && !zt_fs_is_separator(base[base_len - 1]);

    if (!zt_try_add_size(base_len, name_len, &total_len)) {
        return NULL;
    }
    if (needs_sep && !zt_try_add_size(total_len, 1, &total_len)) {
        return NULL;
    }
    if (!zt_try_add_size(total_len, 1, &total_len)) {
        return NULL;
    }

    joined = (char *)malloc(total_len);
    if (joined == NULL) {
        return NULL;
    }

    if (base_len > 0) {
        memcpy(joined, base, base_len);
        offset = base_len;
    }
    if (needs_sep) {
#ifdef _WIN32
        joined[offset++] = '\\';
#else
        joined[offset++] = '/';
#endif
    }
    if (name_len > 0) {
        memcpy(joined + offset, name, name_len);
        offset += name_len;
    }
    joined[offset] = '\0';
    return joined;
}

static zt_outcome_void_core_error zt_fs_create_dir_path(const char *path_data) {
    struct stat info;

    if (path_data == NULL || path_data[0] == '\0') {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_code_message("fs.invalid_path", "path cannot be empty"));
    }

    if (stat(path_data, &info) == 0) {
        if (S_ISDIR(info.st_mode)) {
            return zt_outcome_void_core_error_success();
        }
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_code_message("fs.already_exists", "path already exists"));
    }
    if (errno != ENOENT) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

#ifdef _WIN32
    if (_mkdir(path_data) != 0) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
#else
    if (mkdir(path_data, 0777) != 0) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
#endif
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_fs_create_dir_all_path(const char *path_data) {
    zt_outcome_void_core_error outcome;
    char *copy;
    size_t len;
    size_t index;

    if (path_data == NULL || path_data[0] == '\0') {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_code_message("fs.invalid_path", "path cannot be empty"));
    }

    len = strlen(path_data);
    copy = (char *)malloc(len + 1);
    if (copy == NULL) {
        return zt_outcome_void_core_error_failure_message("failed to allocate directory path buffer");
    }
    memcpy(copy, path_data, len + 1);

    for (index = 0; index < len; index += 1) {
        char saved;

        if (!zt_fs_is_separator(copy[index])) {
            continue;
        }
        if (index == 0 || zt_fs_is_separator(copy[index - 1])) {
            continue;
        }
#ifdef _WIN32
        if (index == 2 && copy[1] == ':') {
            continue;
        }
#endif
        saved = copy[index];
        copy[index] = '\0';
        if (copy[0] != '\0') {
            outcome = zt_fs_create_dir_path(copy);
            if (!outcome.is_success) {
                free(copy);
                return outcome;
            }
        }
        copy[index] = saved;
    }

    outcome = zt_fs_create_dir_path(copy);
    free(copy);
    return outcome;
}

static zt_outcome_text_core_error zt_host_default_read_file(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_write_file(const zt_text *path, const zt_text *value);
static zt_bool zt_host_default_path_exists(const zt_text *path);
static zt_outcome_optional_text_core_error zt_host_default_read_line_stdin(void);
static zt_outcome_text_core_error zt_host_default_read_all_stdin(void);
static zt_outcome_void_core_error zt_host_default_write_stdout(const zt_text *value);
static zt_outcome_void_core_error zt_host_default_write_stderr(const zt_text *value);
static zt_int zt_host_default_time_now_unix_ms(void);
static zt_outcome_void_core_error zt_host_default_time_sleep_ms(zt_int duration_ms);
static void zt_host_default_random_seed(zt_int seed);
static zt_int zt_host_default_random_next_i64(void);
static zt_outcome_text_core_error zt_host_default_os_current_dir(void);
static zt_outcome_void_core_error zt_host_default_os_change_dir(const zt_text *path);
static zt_list_text *zt_host_default_os_args(void);
static zt_optional_text zt_host_default_os_env(const zt_text *name);
static zt_int zt_host_default_os_pid(void);
static zt_text *zt_host_default_os_platform(void);
static zt_text *zt_host_default_os_arch(void);
static zt_outcome_void_core_error zt_host_default_fs_append_text(const zt_text *path, const zt_text *value);
static zt_outcome_bool_core_error zt_host_default_fs_is_file(const zt_text *path);
static zt_outcome_bool_core_error zt_host_default_fs_is_dir(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_fs_create_dir(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_fs_create_dir_all(const zt_text *path);
static zt_outcome_list_text_core_error zt_host_default_fs_list(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_fs_remove_file(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_fs_remove_dir(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_fs_remove_dir_all(const zt_text *path);
static zt_outcome_void_core_error zt_host_default_fs_copy_file(const zt_text *from_path, const zt_text *to_path);
static zt_outcome_void_core_error zt_host_default_fs_move(const zt_text *from_path, const zt_text *to_path);
static zt_outcome_i64_core_error zt_host_default_fs_size(const zt_text *path);
static zt_outcome_i64_core_error zt_host_default_fs_modified_at(const zt_text *path);
static zt_outcome_optional_i64_core_error zt_host_default_fs_created_at(const zt_text *path);
static char *zt_host_prepare_path_copy(const zt_text *path, const char *label, zt_core_error *out_error);
static zt_outcome_i64_core_error zt_host_default_process_run(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);
static zt_outcome_process_captured_run_core_error zt_host_default_process_run_capture(const zt_text *program, const zt_list_text *args, zt_optional_text cwd);

static zt_host_api zt_host_api_state = {
    zt_host_default_read_file,
    zt_host_default_write_file,
    zt_host_default_path_exists,
    zt_host_default_read_line_stdin,
    zt_host_default_read_all_stdin,
    zt_host_default_write_stdout,
    zt_host_default_write_stderr,
    zt_host_default_time_now_unix_ms,
    zt_host_default_time_sleep_ms,
    zt_host_default_random_seed,
    zt_host_default_random_next_i64,
    zt_host_default_os_current_dir,
    zt_host_default_os_change_dir,
    zt_host_default_os_args,
    zt_host_default_os_env,
    zt_host_default_os_pid,
    zt_host_default_os_platform,
    zt_host_default_os_arch,
    zt_host_default_process_run,
    zt_host_default_process_run_capture
};

static zt_list_text *zt_host_captured_process_args = NULL;

typedef struct zt_process_capture_redirect {
    int saved_stdout_fd;
    int saved_stderr_fd;
    zt_bool active;
#ifdef _WIN32
    HANDLE saved_stdout_handle;
    HANDLE saved_stderr_handle;
#endif
} zt_process_capture_redirect;

static zt_outcome_void_core_error zt_host_restore_process_stdio(zt_process_capture_redirect *redirect);
static void zt_process_captured_run_retain(zt_process_captured_run value);
static void zt_process_captured_run_dispose(zt_process_captured_run *value);

zt_runtime_span zt_runtime_span_unknown(void) {
    zt_runtime_span span;
    span.source_name = NULL;
    span.line = 0;
    span.column = 0;
    return span;
}

zt_runtime_span zt_runtime_make_span(const char *source_name, zt_int line, zt_int column) {
    zt_runtime_span span;
    span.source_name = source_name;
    span.line = line;
    span.column = column;
    return span;
}

zt_bool zt_runtime_span_is_known(zt_runtime_span span) {
    return span.source_name != NULL &&
           span.source_name[0] != '\0' &&
           span.line > 0 &&
           span.column > 0;
}

const zt_runtime_error_info *zt_runtime_last_error(void) {
    return &zt_last_error;
}

void zt_runtime_clear_error(void) {
    zt_last_error.has_error = false;
    zt_last_error.kind = ZT_ERR_PANIC;
    zt_last_error.message = NULL;
    zt_last_error.code = NULL;
    zt_last_error.span = zt_runtime_span_unknown();
    zt_last_error_message[0] = '\0';
    zt_last_error_code[0] = '\0';
}

const char *zt_error_kind_name(zt_error_kind kind) {
    switch (kind) {
        case ZT_ERR_CHECK:
            return "check";
        case ZT_ERR_TODO:
            return "todo";
        case ZT_ERR_UNREACHABLE:
            return "unreachable";
        case ZT_ERR_INDEX:
            return "index";
        case ZT_ERR_UNWRAP:
            return "unwrap";
        case ZT_ERR_PANIC:
            return "panic";
        case ZT_ERR_IO:
            return "io";
        case ZT_ERR_MATH:
            return "math";
        case ZT_ERR_PLATFORM:
            return "platform";
        case ZT_ERR_CONTRACT:
            return "contract";
        case ZT_ERR_TEST_FAILED:
            return "test_failed";
        case ZT_ERR_TEST_SKIPPED:
            return "test_skipped";
        default:
            return "unknown";
    }
}

static zt_header *zt_header_from_ref(void *ref) {
    return (zt_header *)ref;
}

static void zt_free_text(zt_text *value) {
    if (value == NULL) {
        return;
    }

    free(value->data);
    value->data = NULL;
    value->len = 0;
    free(value);
}

static void zt_free_bytes(zt_bytes *value) {
    if (value == NULL) {
        return;
    }

    free(value->data);
    value->data = NULL;
    value->len = 0;
    free(value);
}

static void zt_free_closure(zt_closure *closure) {
    if (closure == NULL) {
        return;
    }

    if (closure->ctx != NULL) {
        if (closure->drop_ctx != NULL) {
            closure->drop_ctx(closure->ctx);
        } else {
            free(closure->ctx);
        }
        closure->ctx = NULL;
    }
    free(closure);
}

static void zt_free_lazy_i64(zt_lazy_i64 *value) {
    if (value == NULL) {
        return;
    }

    if (value->thunk != NULL) {
        zt_release(value->thunk);
        value->thunk = NULL;
    }

    free(value);
}

#define ZT_DEFINE_FREE_LAZY(SUFFIX) \
static void zt_free_lazy_##SUFFIX(zt_lazy_##SUFFIX *value) { \
    if (value == NULL) return; \
    if (value->thunk != NULL) { \
        zt_release(value->thunk); \
        value->thunk = NULL; \
    } \
    free(value); \
}

ZT_DEFINE_FREE_LAZY(f64)
ZT_DEFINE_FREE_LAZY(bool)
ZT_DEFINE_FREE_LAZY(i8)
ZT_DEFINE_FREE_LAZY(i16)
ZT_DEFINE_FREE_LAZY(i32)
ZT_DEFINE_FREE_LAZY(u8)
ZT_DEFINE_FREE_LAZY(u16)
ZT_DEFINE_FREE_LAZY(u32)
ZT_DEFINE_FREE_LAZY(u64)
ZT_DEFINE_FREE_LAZY(text)

#undef ZT_DEFINE_FREE_LAZY

/* zt_free_list_i64 and zt_free_list_text: generated by ZT_DEFINE_LIST_IMPL */

static void zt_free_dyn_text_repr(zt_dyn_text_repr *value) {
    if (value == NULL) {
        return;
    }

    if (value->tag == (uint32_t)ZT_DYN_TEXT_REPR_TEXT && value->as.text_value != NULL) {
        zt_release(value->as.text_value);
        value->as.text_value = NULL;
    }

    free(value);
}

static void zt_free_list_dyn_text_repr(zt_list_dyn_text_repr *list) {
    size_t index;

    if (list == NULL) {
        return;
    }

    for (index = 0; index < list->len; index += 1) {
        zt_release(list->data[index]);
    }

    free(list->data);
    free(list);
}

static void zt_net_close_socket_handle(intptr_t handle) {
    zt_socket_handle socket_value = (zt_socket_handle)handle;

    if (socket_value == ZT_NET_INVALID_SOCKET) {
        return;
    }

#ifdef _WIN32
    closesocket(socket_value);
#else
    close(socket_value);
#endif
}

static void zt_free_net_connection(zt_net_connection *connection) {
    if (connection == NULL) {
        return;
    }

    if (!connection->closed) {
        zt_net_close_socket_handle(connection->socket_handle);
        connection->closed = true;
    }

    connection->socket_handle = (intptr_t)ZT_NET_INVALID_SOCKET;
    free(connection);
}
static void zt_runtime_require_text(const zt_text *value, const char *message) {
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static void zt_runtime_require_bytes(const zt_bytes *value, const char *message) {
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static void zt_runtime_require_net_connection(const zt_net_connection *connection, const char *message) {
    if (connection == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static void zt_runtime_require_list_i64(const zt_list_i64 *list, const char *message) {
    if (list == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static void zt_runtime_require_list_text(const zt_list_text *list, const char *message) {
    if (list == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

#define ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(SUFFIX) \
static void zt_runtime_require_list_##SUFFIX(const zt_list_##SUFFIX *list, const char *message) { \
    if (list == NULL) { \
        zt_runtime_error(ZT_ERR_PANIC, message); \
    } \
}

ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(f64)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(bool)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(i8)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(i16)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(i32)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(u8)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(u16)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(u32)
ZT_DEFINE_PRIMITIVE_LIST_REQUIRE(u64)

#undef ZT_DEFINE_PRIMITIVE_LIST_REQUIRE

static void zt_runtime_require_dyn_text_repr(const zt_dyn_text_repr *value, const char *message) {
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static void zt_runtime_require_list_dyn_text_repr(const zt_list_dyn_text_repr *list, const char *message) {
    if (list == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static void zt_runtime_require_map_text_text(const zt_map_text_text *map, const char *message) {
    if (map == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, message);
    }
}

static size_t zt_normalize_slice_end(size_t length, zt_int end_0) {
    if (length == 0) {
        return 0;
    }

    if (end_0 == -1) {
        return length - 1;
    }

    if (end_0 < -1) {
        zt_runtime_error(ZT_ERR_INDEX, "slice end must be -1 or a 0-based index");
    }

    if ((size_t)end_0 >= length) {
        return length - 1;
    }

    return (size_t)end_0;
}

/* ── Monomorphization: list<int> and list<text> ────────────────────────────── */
ZT_DEFINE_LIST_IMPL(i64, zt_int, ZT_HEAP_LIST_I64, 0)
ZT_DEFINE_LIST_IMPL(text, zt_text *, ZT_HEAP_LIST_TEXT, 1)
ZT_DEFINE_LIST_IMPL(f64, zt_float, ZT_HEAP_LIST_F64, 0)
ZT_DEFINE_LIST_IMPL(bool, zt_bool, ZT_HEAP_LIST_BOOL, 0)
ZT_DEFINE_LIST_IMPL(i8, int8_t, ZT_HEAP_LIST_I8, 0)
ZT_DEFINE_LIST_IMPL(i16, int16_t, ZT_HEAP_LIST_I16, 0)
ZT_DEFINE_LIST_IMPL(i32, int32_t, ZT_HEAP_LIST_I32, 0)
ZT_DEFINE_LIST_IMPL(u8, uint8_t, ZT_HEAP_LIST_U8, 0)
ZT_DEFINE_LIST_IMPL(u16, uint16_t, ZT_HEAP_LIST_U16, 0)
ZT_DEFINE_LIST_IMPL(u32, uint32_t, ZT_HEAP_LIST_U32, 0)
ZT_DEFINE_LIST_IMPL(u64, uint64_t, ZT_HEAP_LIST_U64, 0)
size_t zt_text_hash(const zt_text *value) {
    uint64_t hash = UINT64_C(1469598103934665603);
    size_t index;

    if (value == NULL || value->data == NULL) {
        return 0u;
    }

    for (index = 0; index < value->len; index += 1) {
        hash ^= (uint8_t)value->data[index];
        hash *= UINT64_C(1099511628211);
    }

    return (size_t)hash;
}

size_t zt_i64_hash(zt_int value) {
    uint64_t x = (uint64_t)value;

    x ^= x >> 33;
    x *= UINT64_C(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x *= UINT64_C(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    return (size_t)x;
}
ZT_DEFINE_MAP_IMPL(
    text_text,
    zt_text *,
    zt_text *,
    zt_optional_text,
    ZT_HEAP_MAP_TEXT_TEXT,
    1,
    1,
    zt_text_eq,
    zt_text_hash,
    zt_optional_text_present,
    zt_optional_text_empty)

zt_map_text_text *zt_map_text_text_remove(const zt_map_text_text *map, const zt_text *key) {
    zt_map_text_text *copy;
    size_t index;

    zt_runtime_require_map_text_text(map, "zt_map_text_text_remove requires map");
    zt_runtime_require_text(key, "zt_map_text_text_remove requires key");

    copy = zt_map_text_text_new();
    zt_map_text_text_reserve(copy, map->len);
    for (index = 0; index < map->len; index += 1) {
        if (!zt_text_eq(map->keys[index], key)) {
            zt_map_text_text_set(copy, map->keys[index], map->values[index]);
        }
    }
    return copy;
}

zt_list_text *zt_map_text_text_keys(const zt_map_text_text *map) {
    zt_list_text *keys;
    size_t index;

    zt_runtime_require_map_text_text(map, "zt_map_text_text_keys requires map");

    keys = zt_list_text_new();
    zt_list_text_reserve(keys, map->len);
    for (index = 0; index < map->len; index += 1) {
        zt_list_text_push(keys, map->keys[index]);
    }
    return keys;
}

zt_list_text *zt_map_text_text_values(const zt_map_text_text *map) {
    zt_list_text *values;
    size_t index;

    zt_runtime_require_map_text_text(map, "zt_map_text_text_values requires map");

    values = zt_list_text_new();
    zt_list_text_reserve(values, map->len);
    for (index = 0; index < map->len; index += 1) {
        zt_list_text_push(values, map->values[index]);
    }
    return values;
}

zt_map_text_text *zt_map_text_text_merge(const zt_map_text_text *left, const zt_map_text_text *right) {
    zt_map_text_text *copy;
    size_t index;

    zt_runtime_require_map_text_text(left, "zt_map_text_text_merge requires left map");
    zt_runtime_require_map_text_text(right, "zt_map_text_text_merge requires right map");

    copy = zt_map_text_text_new();
    zt_map_text_text_reserve(copy, zt_require_added_size(left->len, right->len, "map merge size overflow"));
    for (index = 0; index < left->len; index += 1) {
        zt_map_text_text_set(copy, left->keys[index], left->values[index]);
    }
    for (index = 0; index < right->len; index += 1) {
        zt_map_text_text_set(copy, right->keys[index], right->values[index]);
    }
    return copy;
}

/* --------------------------------------------------------------------------
 * set<int> — open-addressing hash set for zt_int
 * -------------------------------------------------------------------------- */

#define ZT_SET_EMPTY    0
#define ZT_SET_OCCUPIED 1
#define ZT_SET_DELETED  2
#define ZT_SET_INITIAL_CAP 16

static void zt_free_set_i64(zt_set_i64 *set) {
    if (set == NULL) return;
    free(set->data);
    free(set->occupied);
    free(set);
}

static void zt_set_i64_grow(zt_set_i64 *set);

zt_set_i64 *zt_set_i64_create(void) {
    zt_set_i64 *set = (zt_set_i64 *)calloc(1, sizeof(zt_set_i64));
    if (set == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate set<int>");
    set->header.rc = 1;
    set->header.kind = (uint32_t)ZT_HEAP_SET_I64;
    set->len = 0;
    set->hash_capacity = ZT_SET_INITIAL_CAP;
    set->data = (zt_int *)calloc(ZT_SET_INITIAL_CAP, sizeof(zt_int));
    set->occupied = (uint8_t *)calloc(ZT_SET_INITIAL_CAP, sizeof(uint8_t));
    if (set->data == NULL || set->occupied == NULL) {
        free(set->data);
        free(set->occupied);
        free(set);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate set<int> data");
    }
    return set;
}

zt_set_i64 *zt_set_i64_from_array(const zt_int *items, size_t count) {
    zt_set_i64 *set = zt_set_i64_create();
    size_t i;
    if (items == NULL) return set;
    for (i = 0; i < count; i += 1) {
        zt_set_i64_add(set, items[i]);
    }
    return set;
}

void zt_set_i64_add(zt_set_i64 *set, zt_int value) {
    size_t idx, i;
    if (set == NULL) return;
    if (set->len * 4 >= set->hash_capacity * 3) {
        zt_set_i64_grow(set);
    }
    idx = zt_i64_hash(value) % set->hash_capacity;
    for (i = 0; i < set->hash_capacity; i++) {
        size_t probe = (idx + i) % set->hash_capacity;
        if (set->occupied[probe] == ZT_SET_OCCUPIED && set->data[probe] == value) {
            return;
        }
        if (set->occupied[probe] != ZT_SET_OCCUPIED) {
            set->data[probe] = value;
            set->occupied[probe] = ZT_SET_OCCUPIED;
            set->len++;
            return;
        }
    }
}

zt_bool zt_set_i64_has(const zt_set_i64 *set, zt_int value) {
    size_t idx, i;
    if (set == NULL || set->hash_capacity == 0) return false;
    idx = zt_i64_hash(value) % set->hash_capacity;
    for (i = 0; i < set->hash_capacity; i++) {
        size_t probe = (idx + i) % set->hash_capacity;
        if (set->occupied[probe] == ZT_SET_EMPTY) return false;
        if (set->occupied[probe] == ZT_SET_OCCUPIED && set->data[probe] == value) return true;
    }
    return false;
}

void zt_set_i64_remove(zt_set_i64 *set, zt_int value) {
    size_t idx, i;
    if (set == NULL || set->hash_capacity == 0) return;
    idx = zt_i64_hash(value) % set->hash_capacity;
    for (i = 0; i < set->hash_capacity; i++) {
        size_t probe = (idx + i) % set->hash_capacity;
        if (set->occupied[probe] == ZT_SET_EMPTY) return;
        if (set->occupied[probe] == ZT_SET_OCCUPIED && set->data[probe] == value) {
            set->occupied[probe] = ZT_SET_DELETED;
            set->len--;
            return;
        }
    }
}

zt_int zt_set_i64_len(const zt_set_i64 *set) {
    if (set == NULL) return 0;
    return (zt_int)set->len;
}

zt_int zt_set_i64_value_at(const zt_set_i64 *set, zt_int index_0) {
    size_t i;
    size_t seen = 0;
    if (set == NULL || index_0 < 0) {
        zt_runtime_error(ZT_ERR_INDEX, "set<int> index out of bounds");
    }
    for (i = 0; i < set->hash_capacity; i += 1) {
        if (set->occupied[i] != ZT_SET_OCCUPIED) continue;
        if (seen == (size_t)index_0) {
            return set->data[i];
        }
        seen += 1;
    }
    zt_runtime_error(ZT_ERR_INDEX, "set<int> index out of bounds");
}

zt_set_i64 *zt_set_i64_union(const zt_set_i64 *left, const zt_set_i64 *right) {
    zt_set_i64 *out = zt_set_i64_create();
    size_t i;
    if (left != NULL) {
        for (i = 0; i < left->hash_capacity; i += 1) {
            if (left->occupied[i] == ZT_SET_OCCUPIED) {
                zt_set_i64_add(out, left->data[i]);
            }
        }
    }
    if (right != NULL) {
        for (i = 0; i < right->hash_capacity; i += 1) {
            if (right->occupied[i] == ZT_SET_OCCUPIED) {
                zt_set_i64_add(out, right->data[i]);
            }
        }
    }
    return out;
}

zt_set_i64 *zt_set_i64_intersect(const zt_set_i64 *left, const zt_set_i64 *right) {
    zt_set_i64 *out = zt_set_i64_create();
    size_t i;
    if (left == NULL || right == NULL) return out;
    for (i = 0; i < left->hash_capacity; i += 1) {
        if (left->occupied[i] == ZT_SET_OCCUPIED &&
                zt_set_i64_has(right, left->data[i])) {
            zt_set_i64_add(out, left->data[i]);
        }
    }
    return out;
}

zt_set_i64 *zt_set_i64_difference(const zt_set_i64 *left, const zt_set_i64 *right) {
    zt_set_i64 *out = zt_set_i64_create();
    size_t i;
    if (left == NULL) return out;
    for (i = 0; i < left->hash_capacity; i += 1) {
        if (left->occupied[i] == ZT_SET_OCCUPIED &&
                !zt_set_i64_has(right, left->data[i])) {
            zt_set_i64_add(out, left->data[i]);
        }
    }
    return out;
}

static void zt_set_i64_grow(zt_set_i64 *set) {
    size_t old_cap = set->hash_capacity;
    zt_int *old_data = set->data;
    uint8_t *old_occ = set->occupied;
    size_t new_cap = old_cap * 2;
    size_t i;

    set->hash_capacity = new_cap;
    set->data = (zt_int *)calloc(new_cap, sizeof(zt_int));
    set->occupied = (uint8_t *)calloc(new_cap, sizeof(uint8_t));
    if (set->data == NULL || set->occupied == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to grow set<int>");
    }
    set->len = 0;

    for (i = 0; i < old_cap; i++) {
        if (old_occ[i] == ZT_SET_OCCUPIED) {
            zt_set_i64_add(set, old_data[i]);
        }
    }

    free(old_data);
    free(old_occ);
}

/* --------------------------------------------------------------------------
 * set<text> — open-addressing hash set for zt_text *
 * -------------------------------------------------------------------------- */

static void zt_free_set_text(zt_set_text *set) {
    size_t i;
    if (set == NULL) return;
    for (i = 0; i < set->hash_capacity; i++) {
        if (set->occupied[i] == ZT_SET_OCCUPIED && set->data[i] != NULL) {
            zt_release(set->data[i]);
        }
    }
    free(set->data);
    free(set->occupied);
    free(set);
}

static void zt_set_text_grow(zt_set_text *set);

zt_set_text *zt_set_text_create(void) {
    zt_set_text *set = (zt_set_text *)calloc(1, sizeof(zt_set_text));
    if (set == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate set<text>");
    set->header.rc = 1;
    set->header.kind = (uint32_t)ZT_HEAP_SET_TEXT;
    set->len = 0;
    set->hash_capacity = ZT_SET_INITIAL_CAP;
    set->data = (zt_text **)calloc(ZT_SET_INITIAL_CAP, sizeof(zt_text *));
    set->occupied = (uint8_t *)calloc(ZT_SET_INITIAL_CAP, sizeof(uint8_t));
    if (set->data == NULL || set->occupied == NULL) {
        free(set->data);
        free(set->occupied);
        free(set);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate set<text> data");
    }
    return set;
}

zt_set_text *zt_set_text_from_array(zt_text *const *items, size_t count) {
    zt_set_text *set = zt_set_text_create();
    size_t i;
    if (items == NULL) return set;
    for (i = 0; i < count; i += 1) {
        zt_set_text_add(set, items[i]);
    }
    return set;
}

void zt_set_text_add(zt_set_text *set, zt_text *value) {
    size_t idx, i;
    if (set == NULL || value == NULL) return;
    if (set->len * 4 >= set->hash_capacity * 3) {
        zt_set_text_grow(set);
    }
    idx = zt_text_hash(value) % set->hash_capacity;
    for (i = 0; i < set->hash_capacity; i++) {
        size_t probe = (idx + i) % set->hash_capacity;
        if (set->occupied[probe] == ZT_SET_OCCUPIED && zt_text_eq(set->data[probe], value)) {
            return;
        }
        if (set->occupied[probe] != ZT_SET_OCCUPIED) {
            zt_retain(value);
            set->data[probe] = value;
            set->occupied[probe] = ZT_SET_OCCUPIED;
            set->len++;
            return;
        }
    }
}

zt_bool zt_set_text_has(const zt_set_text *set, const zt_text *value) {
    size_t idx, i;
    if (set == NULL || value == NULL || set->hash_capacity == 0) return false;
    idx = zt_text_hash(value) % set->hash_capacity;
    for (i = 0; i < set->hash_capacity; i++) {
        size_t probe = (idx + i) % set->hash_capacity;
        if (set->occupied[probe] == ZT_SET_EMPTY) return false;
        if (set->occupied[probe] == ZT_SET_OCCUPIED && zt_text_eq(set->data[probe], value)) return true;
    }
    return false;
}

void zt_set_text_remove(zt_set_text *set, const zt_text *value) {
    size_t idx, i;
    if (set == NULL || value == NULL || set->hash_capacity == 0) return;
    idx = zt_text_hash(value) % set->hash_capacity;
    for (i = 0; i < set->hash_capacity; i++) {
        size_t probe = (idx + i) % set->hash_capacity;
        if (set->occupied[probe] == ZT_SET_EMPTY) return;
        if (set->occupied[probe] == ZT_SET_OCCUPIED && zt_text_eq(set->data[probe], value)) {
            zt_release(set->data[probe]);
            set->data[probe] = NULL;
            set->occupied[probe] = ZT_SET_DELETED;
            set->len--;
            return;
        }
    }
}

zt_int zt_set_text_len(const zt_set_text *set) {
    if (set == NULL) return 0;
    return (zt_int)set->len;
}

zt_text *zt_set_text_value_at(const zt_set_text *set, zt_int index_0) {
    size_t i;
    size_t seen = 0;
    if (set == NULL || index_0 < 0) {
        zt_runtime_error(ZT_ERR_INDEX, "set<text> index out of bounds");
    }
    for (i = 0; i < set->hash_capacity; i += 1) {
        if (set->occupied[i] != ZT_SET_OCCUPIED || set->data[i] == NULL) continue;
        if (seen == (size_t)index_0) {
            zt_retain(set->data[i]);
            return set->data[i];
        }
        seen += 1;
    }
    zt_runtime_error(ZT_ERR_INDEX, "set<text> index out of bounds");
}

zt_set_text *zt_set_text_union(const zt_set_text *left, const zt_set_text *right) {
    zt_set_text *out = zt_set_text_create();
    size_t i;
    if (left != NULL) {
        for (i = 0; i < left->hash_capacity; i += 1) {
            if (left->occupied[i] == ZT_SET_OCCUPIED && left->data[i] != NULL) {
                zt_set_text_add(out, left->data[i]);
            }
        }
    }
    if (right != NULL) {
        for (i = 0; i < right->hash_capacity; i += 1) {
            if (right->occupied[i] == ZT_SET_OCCUPIED && right->data[i] != NULL) {
                zt_set_text_add(out, right->data[i]);
            }
        }
    }
    return out;
}

zt_set_text *zt_set_text_intersect(const zt_set_text *left, const zt_set_text *right) {
    zt_set_text *out = zt_set_text_create();
    size_t i;
    if (left == NULL || right == NULL) return out;
    for (i = 0; i < left->hash_capacity; i += 1) {
        if (left->occupied[i] == ZT_SET_OCCUPIED &&
                left->data[i] != NULL &&
                zt_set_text_has(right, left->data[i])) {
            zt_set_text_add(out, left->data[i]);
        }
    }
    return out;
}

zt_set_text *zt_set_text_difference(const zt_set_text *left, const zt_set_text *right) {
    zt_set_text *out = zt_set_text_create();
    size_t i;
    if (left == NULL) return out;
    for (i = 0; i < left->hash_capacity; i += 1) {
        if (left->occupied[i] == ZT_SET_OCCUPIED &&
                left->data[i] != NULL &&
                !zt_set_text_has(right, left->data[i])) {
            zt_set_text_add(out, left->data[i]);
        }
    }
    return out;
}

static void zt_set_text_grow(zt_set_text *set) {
    size_t old_cap = set->hash_capacity;
    zt_text **old_data = set->data;
    uint8_t *old_occ = set->occupied;
    size_t new_cap = old_cap * 2;
    size_t i;

    set->hash_capacity = new_cap;
    set->data = (zt_text **)calloc(new_cap, sizeof(zt_text *));
    set->occupied = (uint8_t *)calloc(new_cap, sizeof(uint8_t));
    if (set->data == NULL || set->occupied == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to grow set<text>");
    }
    set->len = 0;

    for (i = 0; i < old_cap; i++) {
        if (old_occ[i] == ZT_SET_OCCUPIED && old_data[i] != NULL) {
            size_t idx = zt_text_hash(old_data[i]) % new_cap;
            size_t j;
            for (j = 0; j < new_cap; j++) {
                size_t probe = (idx + j) % new_cap;
                if (set->occupied[probe] != ZT_SET_OCCUPIED) {
                    set->data[probe] = old_data[i];
                    set->occupied[probe] = ZT_SET_OCCUPIED;
                    set->len++;
                    break;
                }
            }
        }
    }

    free(old_data);
    free(old_occ);
}

static zt_bool zt_utf8_is_continuation(uint8_t byte) {
    return (byte & 0xC0u) == 0x80u;
}

static zt_bool zt_utf8_validate(const uint8_t *data, size_t len, size_t *error_index, const char **error_reason) {
    size_t index = 0;

    if (len > 0 && data == NULL) {
        if (error_index != NULL) {
            *error_index = 0;
        }
        if (error_reason != NULL) {
            *error_reason = "missing input byte buffer";
        }
        return false;
    }

    while (index < len) {
        uint8_t first = data[index];

        if (first <= 0x7Fu) {
            index += 1;
            continue;
        }

        if (first >= 0xC2u && first <= 0xDFu) {
            if (index + 1 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 2-byte sequence";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 1])) {
                if (error_index != NULL) {
                    *error_index = index + 1;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 2-byte sequence";
                }
                return false;
            }
            index += 2;
            continue;
        }

        if (first == 0xE0u) {
            if (index + 2 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 3-byte sequence";
                }
                return false;
            }
            if (!(data[index + 1] >= 0xA0u && data[index + 1] <= 0xBFu)) {
                if (error_index != NULL) {
                    *error_index = index + 1;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid second byte in 3-byte sequence";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 2])) {
                if (error_index != NULL) {
                    *error_index = index + 2;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 3-byte sequence";
                }
                return false;
            }
            index += 3;
            continue;
        }

        if ((first >= 0xE1u && first <= 0xECu) || (first >= 0xEEu && first <= 0xEFu)) {
            if (index + 2 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 3-byte sequence";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 1]) || !zt_utf8_is_continuation(data[index + 2])) {
                if (error_index != NULL) {
                    *error_index = !zt_utf8_is_continuation(data[index + 1]) ? index + 1 : index + 2;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 3-byte sequence";
                }
                return false;
            }
            index += 3;
            continue;
        }

        if (first == 0xEDu) {
            if (index + 2 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 3-byte sequence";
                }
                return false;
            }
            if (!(data[index + 1] >= 0x80u && data[index + 1] <= 0x9Fu)) {
                if (error_index != NULL) {
                    *error_index = index + 1;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid second byte for surrogate range";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 2])) {
                if (error_index != NULL) {
                    *error_index = index + 2;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 3-byte sequence";
                }
                return false;
            }
            index += 3;
            continue;
        }

        if (first == 0xF0u) {
            if (index + 3 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 4-byte sequence";
                }
                return false;
            }
            if (!(data[index + 1] >= 0x90u && data[index + 1] <= 0xBFu)) {
                if (error_index != NULL) {
                    *error_index = index + 1;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid second byte in 4-byte sequence";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 2]) || !zt_utf8_is_continuation(data[index + 3])) {
                if (error_index != NULL) {
                    *error_index = !zt_utf8_is_continuation(data[index + 2]) ? index + 2 : index + 3;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 4-byte sequence";
                }
                return false;
            }
            index += 4;
            continue;
        }

        if (first >= 0xF1u && first <= 0xF3u) {
            if (index + 3 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 4-byte sequence";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 1]) ||
                !zt_utf8_is_continuation(data[index + 2]) ||
                !zt_utf8_is_continuation(data[index + 3])) {
                if (error_index != NULL) {
                    if (!zt_utf8_is_continuation(data[index + 1])) {
                        *error_index = index + 1;
                    } else if (!zt_utf8_is_continuation(data[index + 2])) {
                        *error_index = index + 2;
                    } else {
                        *error_index = index + 3;
                    }
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 4-byte sequence";
                }
                return false;
            }
            index += 4;
            continue;
        }

        if (first == 0xF4u) {
            if (index + 3 >= len) {
                if (error_index != NULL) {
                    *error_index = index;
                }
                if (error_reason != NULL) {
                    *error_reason = "truncated 4-byte sequence";
                }
                return false;
            }
            if (!(data[index + 1] >= 0x80u && data[index + 1] <= 0x8Fu)) {
                if (error_index != NULL) {
                    *error_index = index + 1;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid second byte in 4-byte sequence";
                }
                return false;
            }
            if (!zt_utf8_is_continuation(data[index + 2]) || !zt_utf8_is_continuation(data[index + 3])) {
                if (error_index != NULL) {
                    *error_index = !zt_utf8_is_continuation(data[index + 2]) ? index + 2 : index + 3;
                }
                if (error_reason != NULL) {
                    *error_reason = "invalid continuation byte in 4-byte sequence";
                }
                return false;
            }
            index += 4;
            continue;
        }

        if (error_index != NULL) {
            *error_index = index;
        }
        if (error_reason != NULL) {
            *error_reason = "invalid leading byte";
        }
        return false;
    }

    if (error_index != NULL) {
        *error_index = len;
    }
    if (error_reason != NULL) {
        *error_reason = NULL;
    }
    return true;
}

static size_t zt_utf8_sequence_width_or_error(const uint8_t *data, size_t len, size_t offset, const char *context) {
    uint8_t first;

    if (data == NULL || offset >= len) {
        zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
    }

    first = data[offset];
    if (first <= 0x7Fu) {
        return 1;
    }

    if (first >= 0xC2u && first <= 0xDFu) {
        if (offset + 1 >= len || !zt_utf8_is_continuation(data[offset + 1])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 2;
    }

    if (first == 0xE0u) {
        if (offset + 2 >= len ||
            !(data[offset + 1] >= 0xA0u && data[offset + 1] <= 0xBFu) ||
            !zt_utf8_is_continuation(data[offset + 2])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 3;
    }

    if ((first >= 0xE1u && first <= 0xECu) || (first >= 0xEEu && first <= 0xEFu)) {
        if (offset + 2 >= len ||
            !zt_utf8_is_continuation(data[offset + 1]) ||
            !zt_utf8_is_continuation(data[offset + 2])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 3;
    }

    if (first == 0xEDu) {
        if (offset + 2 >= len ||
            !(data[offset + 1] >= 0x80u && data[offset + 1] <= 0x9Fu) ||
            !zt_utf8_is_continuation(data[offset + 2])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 3;
    }

    if (first == 0xF0u) {
        if (offset + 3 >= len ||
            !(data[offset + 1] >= 0x90u && data[offset + 1] <= 0xBFu) ||
            !zt_utf8_is_continuation(data[offset + 2]) ||
            !zt_utf8_is_continuation(data[offset + 3])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 4;
    }

    if (first >= 0xF1u && first <= 0xF3u) {
        if (offset + 3 >= len ||
            !zt_utf8_is_continuation(data[offset + 1]) ||
            !zt_utf8_is_continuation(data[offset + 2]) ||
            !zt_utf8_is_continuation(data[offset + 3])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 4;
    }

    if (first == 0xF4u) {
        if (offset + 3 >= len ||
            !(data[offset + 1] >= 0x80u && data[offset + 1] <= 0x8Fu) ||
            !zt_utf8_is_continuation(data[offset + 2]) ||
            !zt_utf8_is_continuation(data[offset + 3])) {
            zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
        }
        return 4;
    }

    zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
    return 0;
}

static size_t zt_text_codepoint_count(const zt_text *value, const char *context) {
    const uint8_t *data;
    size_t offset = 0;
    size_t count = 0;

    zt_runtime_require_text(value, "zt_text_codepoint_count requires text");
    data = (const uint8_t *)value->data;

    while (offset < value->len) {
        offset += zt_utf8_sequence_width_or_error(data, value->len, offset, context);
        count += 1;
    }

    return count;
}

static size_t zt_text_byte_offset_for_codepoint(
        const zt_text *value,
        size_t codepoint_index,
        const char *context) {
    const uint8_t *data;
    size_t offset = 0;
    size_t current = 0;

    zt_runtime_require_text(value, "zt_text_byte_offset_for_codepoint requires text");
    data = (const uint8_t *)value->data;

    while (offset < value->len && current < codepoint_index) {
        offset += zt_utf8_sequence_width_or_error(data, value->len, offset, context);
        current += 1;
    }

    if (current != codepoint_index) {
        zt_runtime_error(ZT_ERR_PLATFORM, context != NULL ? context : "invalid UTF-8 text invariant");
    }

    return offset;
}

/* zt_list_i64_reserve and zt_list_text_reserve: generated by ZT_DEFINE_LIST_IMPL */





















static void zt_free_grid2d_i64(zt_grid2d_i64 *grid);
static void zt_free_grid2d_text(zt_grid2d_text *grid);
static void zt_free_pqueue_i64(zt_pqueue_i64 *heap);
static void zt_free_pqueue_text(zt_pqueue_text *heap);
static void zt_free_circbuf_i64(zt_circbuf_i64 *buf);
static void zt_free_circbuf_text(zt_circbuf_text *buf);
static void zt_free_btreemap_text_text(zt_btreemap_text_text *map);
static void zt_free_btreeset_text(zt_btreeset_text *set);
static void zt_free_grid3d_i64(zt_grid3d_i64 *grid);
static void zt_free_grid3d_text(zt_grid3d_text *grid);
static void zt_free_net_connection(zt_net_connection *connection);
static void zt_free_lazy_f64(zt_lazy_f64 *value);
static void zt_free_lazy_bool(zt_lazy_bool *value);
static void zt_free_lazy_i8(zt_lazy_i8 *value);
static void zt_free_lazy_i16(zt_lazy_i16 *value);
static void zt_free_lazy_i32(zt_lazy_i32 *value);
static void zt_free_lazy_u8(zt_lazy_u8 *value);
static void zt_free_lazy_u16(zt_lazy_u16 *value);
static void zt_free_lazy_u32(zt_lazy_u32 *value);
static void zt_free_lazy_u64(zt_lazy_u64 *value);
static void zt_free_lazy_text(zt_lazy_text *value);

uint32_t zt_register_dynamic_heap_kind(zt_heap_free_fn free_fn, zt_heap_clone_fn clone_fn) {
    size_t index;
    uint32_t kind;

    if (free_fn == NULL || clone_fn == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "dynamic heap kinds require free and clone callbacks");
    }

    for (index = 0; index < zt_dynamic_heap_count; index += 1) {
        if (zt_dynamic_heaps[index].free_fn == free_fn &&
                zt_dynamic_heaps[index].clone_fn == clone_fn) {
            return zt_dynamic_heaps[index].kind;
        }
    }

    if (zt_dynamic_heap_count >= ZT_DYNAMIC_HEAP_CAPACITY) {
        zt_runtime_error(ZT_ERR_PLATFORM, "dynamic heap kind registry exhausted");
    }

    kind = ZT_DYNAMIC_HEAP_BASE + (uint32_t)zt_dynamic_heap_count;
    zt_dynamic_heaps[zt_dynamic_heap_count].kind = kind;
    zt_dynamic_heaps[zt_dynamic_heap_count].free_fn = free_fn;
    zt_dynamic_heaps[zt_dynamic_heap_count].clone_fn = clone_fn;
    zt_dynamic_heap_count += 1;
    return kind;
}

void zt_retain(void *ref) {
    zt_header *header;

    if (ref == NULL) {
        return;
    }

    header = zt_header_from_ref(ref);
    if (header->kind == (uint32_t)ZT_HEAP_IMMORTAL_OUTCOME_VOID_TEXT) {
        return;
    }
    if (header->rc == UINT32_MAX) {
        /* M22.E: UINT32_MAX means immortal (static closure / interned value).
         * Skip retain — the object is never freed. */
        return;
    }

    header->rc += 1;
}

void zt_release(void *ref) {
    zt_header *header;

    if (ref == NULL) {
        return;
    }

    header = zt_header_from_ref(ref);
    if (header->kind == (uint32_t)ZT_HEAP_IMMORTAL_OUTCOME_VOID_TEXT) {
        return;
    }
    if (header->rc == UINT32_MAX) {
        /* M22.E: Immortal object — skip release. */
        return;
    }
    if (header->rc == 0) {
        zt_runtime_error(ZT_ERR_PLATFORM, "release on object with rc=0");
    }

    header->rc -= 1;
    if (header->rc > 0) {
        return;
    }

    switch ((zt_heap_kind)header->kind) {
        case ZT_HEAP_TEXT:
            zt_free_text((zt_text *)ref);
            return;
        case ZT_HEAP_BYTES:
            zt_free_bytes((zt_bytes *)ref);
            return;
        case ZT_HEAP_LIST_I64:
            zt_free_list_i64((zt_list_i64 *)ref);
            return;
        case ZT_HEAP_LIST_TEXT:
            zt_free_list_text((zt_list_text *)ref);
            return;
        case ZT_HEAP_LIST_F64:
            zt_free_list_f64((zt_list_f64 *)ref);
            return;
        case ZT_HEAP_LIST_BOOL:
            zt_free_list_bool((zt_list_bool *)ref);
            return;
        case ZT_HEAP_LIST_I8:
            zt_free_list_i8((zt_list_i8 *)ref);
            return;
        case ZT_HEAP_LIST_I16:
            zt_free_list_i16((zt_list_i16 *)ref);
            return;
        case ZT_HEAP_LIST_I32:
            zt_free_list_i32((zt_list_i32 *)ref);
            return;
        case ZT_HEAP_LIST_U8:
            zt_free_list_u8((zt_list_u8 *)ref);
            return;
        case ZT_HEAP_LIST_U16:
            zt_free_list_u16((zt_list_u16 *)ref);
            return;
        case ZT_HEAP_LIST_U32:
            zt_free_list_u32((zt_list_u32 *)ref);
            return;
        case ZT_HEAP_LIST_U64:
            zt_free_list_u64((zt_list_u64 *)ref);
            return;
        case ZT_HEAP_DYN_TEXT_REPR:
            zt_free_dyn_text_repr((zt_dyn_text_repr *)ref);
            return;
        case ZT_HEAP_LIST_DYN_TEXT_REPR:
            zt_free_list_dyn_text_repr((zt_list_dyn_text_repr *)ref);
            return;
        case ZT_HEAP_DYN_VALUE:
            zt_dyn_drop((zt_dyn_value *)ref);
            return;
        case ZT_HEAP_LIST_DYN:
            zt_list_dyn_free((zt_list_dyn *)ref);
            return;
        case ZT_HEAP_MAP_TEXT_TEXT:
            zt_free_map_text_text((zt_map_text_text *)ref);
            return;
        case ZT_HEAP_GRID2D_I64:
            zt_free_grid2d_i64((zt_grid2d_i64 *)ref);
            return;
        case ZT_HEAP_GRID2D_TEXT:
            zt_free_grid2d_text((zt_grid2d_text *)ref);
            return;
        case ZT_HEAP_CLOSURE:
            zt_free_closure((zt_closure *)ref);
            return;
        case ZT_HEAP_LAZY_I64:
            zt_free_lazy_i64((zt_lazy_i64 *)ref);
            return;
        case ZT_HEAP_LAZY_F64:
            zt_free_lazy_f64((zt_lazy_f64 *)ref);
            return;
        case ZT_HEAP_LAZY_BOOL:
            zt_free_lazy_bool((zt_lazy_bool *)ref);
            return;
        case ZT_HEAP_LAZY_I8:
            zt_free_lazy_i8((zt_lazy_i8 *)ref);
            return;
        case ZT_HEAP_LAZY_I16:
            zt_free_lazy_i16((zt_lazy_i16 *)ref);
            return;
        case ZT_HEAP_LAZY_I32:
            zt_free_lazy_i32((zt_lazy_i32 *)ref);
            return;
        case ZT_HEAP_LAZY_U8:
            zt_free_lazy_u8((zt_lazy_u8 *)ref);
            return;
        case ZT_HEAP_LAZY_U16:
            zt_free_lazy_u16((zt_lazy_u16 *)ref);
            return;
        case ZT_HEAP_LAZY_U32:
            zt_free_lazy_u32((zt_lazy_u32 *)ref);
            return;
        case ZT_HEAP_LAZY_U64:
            zt_free_lazy_u64((zt_lazy_u64 *)ref);
            return;
        case ZT_HEAP_LAZY_TEXT:
            zt_free_lazy_text((zt_lazy_text *)ref);
            return;
        case ZT_HEAP_PQUEUE_I64:
            zt_free_pqueue_i64((zt_pqueue_i64 *)ref);
            return;
        case ZT_HEAP_PQUEUE_TEXT:
            zt_free_pqueue_text((zt_pqueue_text *)ref);
            return;
        case ZT_HEAP_CIRCBUF_I64:
            zt_free_circbuf_i64((zt_circbuf_i64 *)ref);
            return;
        case ZT_HEAP_CIRCBUF_TEXT:
            zt_free_circbuf_text((zt_circbuf_text *)ref);
            return;
        case ZT_HEAP_BTREEMAP_TEXT_TEXT:
            zt_free_btreemap_text_text((zt_btreemap_text_text *)ref);
            return;
        case ZT_HEAP_BTREESET_TEXT:
            zt_free_btreeset_text((zt_btreeset_text *)ref);
            return;
        case ZT_HEAP_GRID3D_I64:
            zt_free_grid3d_i64((zt_grid3d_i64 *)ref);
            return;
        case ZT_HEAP_GRID3D_TEXT:
            zt_free_grid3d_text((zt_grid3d_text *)ref);
            return;
        case ZT_HEAP_NET_CONNECTION:
            zt_free_net_connection((zt_net_connection *)ref);
            return;
        case ZT_HEAP_SET_I64:
            zt_free_set_i64((zt_set_i64 *)ref);
            return;
        case ZT_HEAP_SET_TEXT:
            zt_free_set_text((zt_set_text *)ref);
            return;
        case ZT_HEAP_LIST_GENERIC:
            zt_list_generic_free((zt_list_generic *)ref);
            return;
        case ZT_HEAP_MAP_GENERIC:
            zt_map_generic_free((zt_map_generic *)ref);
            return;
        case ZT_HEAP_SET_GENERIC:
            zt_set_generic_free((zt_set_generic *)ref);
            return;
        case ZT_HEAP_IMMORTAL_OUTCOME_VOID_TEXT:
            return;
        case ZT_HEAP_UNKNOWN:
        default:
            {
                const zt_dynamic_heap_entry *entry = zt_find_dynamic_heap_entry(header->kind);
                if (entry != NULL && entry->free_fn != NULL) {
                    entry->free_fn(ref);
                    return;
                }
                free(ref);
                return;
            }
    }
}

void *zt_deep_copy(void *ref) {
    zt_header *header;
    size_t i;

    if (ref == NULL) {
        return NULL;
    }

    header = zt_header_from_ref(ref);
    if (header->kind == (uint32_t)ZT_HEAP_IMMORTAL_OUTCOME_VOID_TEXT) {
        return ref;
    }

    switch ((zt_heap_kind)header->kind) {
        case ZT_HEAP_TEXT: {
            zt_text *t = (zt_text *)ref;
            return zt_text_from_utf8(t->data, t->len);
        }
        case ZT_HEAP_BYTES: {
            zt_bytes *b = (zt_bytes *)ref;
            return zt_bytes_from_array(b->data, b->len);
        }
        case ZT_HEAP_LIST_I64: {
            zt_list_i64 *l = (zt_list_i64 *)ref;
            return zt_list_i64_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_F64: {
            zt_list_f64 *l = (zt_list_f64 *)ref;
            return zt_list_f64_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_BOOL: {
            zt_list_bool *l = (zt_list_bool *)ref;
            return zt_list_bool_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_I8: {
            zt_list_i8 *l = (zt_list_i8 *)ref;
            return zt_list_i8_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_I16: {
            zt_list_i16 *l = (zt_list_i16 *)ref;
            return zt_list_i16_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_I32: {
            zt_list_i32 *l = (zt_list_i32 *)ref;
            return zt_list_i32_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_U8: {
            zt_list_u8 *l = (zt_list_u8 *)ref;
            return zt_list_u8_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_U16: {
            zt_list_u16 *l = (zt_list_u16 *)ref;
            return zt_list_u16_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_U32: {
            zt_list_u32 *l = (zt_list_u32 *)ref;
            return zt_list_u32_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_U64: {
            zt_list_u64 *l = (zt_list_u64 *)ref;
            return zt_list_u64_from_array(l->data, l->len);
        }
        case ZT_HEAP_LIST_TEXT: {
            zt_list_text *l = (zt_list_text *)ref;
            zt_list_text *clone = zt_list_text_new();
            zt_list_text_reserve(clone, l->len);
            for (i = 0; i < l->len; i += 1) {
                clone->data[i] = (zt_text *)zt_deep_copy(l->data[i]);
            }
            clone->len = l->len;
            return clone;
        }
        case ZT_HEAP_MAP_TEXT_TEXT: {
            return zt_map_text_text_deep_copy((const zt_map_text_text *)ref);
        }
        case ZT_HEAP_SET_I64: {
            zt_set_i64 *s = (zt_set_i64 *)ref;
            zt_set_i64 *clone = zt_set_i64_create();
            for (i = 0; i < s->hash_capacity; i++) {
                if (s->occupied[i] == ZT_SET_OCCUPIED) {
                    zt_set_i64_add(clone, s->data[i]);
                }
            }
            return clone;
        }
        case ZT_HEAP_SET_TEXT: {
            zt_set_text *s = (zt_set_text *)ref;
            zt_set_text *clone = zt_set_text_create();
            for (i = 0; i < s->hash_capacity; i++) {
                if (s->occupied[i] == ZT_SET_OCCUPIED && s->data[i] != NULL) {
                    zt_text *copy = (zt_text *)zt_deep_copy(s->data[i]);
                    zt_set_text_add(clone, copy);
                    zt_release(copy);
                }
            }
            return clone;
        }
        case ZT_HEAP_GRID2D_I64: {
            zt_grid2d_i64 *g = (zt_grid2d_i64 *)ref;
            zt_grid2d_i64 *clone = zt_grid2d_i64_new((zt_int)g->rows, (zt_int)g->cols);
            memcpy(clone->data, g->data, g->len * sizeof(zt_int));
            return clone;
        }
        case ZT_HEAP_GRID2D_TEXT: {
            zt_grid2d_text *g = (zt_grid2d_text *)ref;
            zt_grid2d_text *clone = zt_grid2d_text_new((zt_int)g->rows, (zt_int)g->cols);
            for (i = 0; i < g->len; i += 1) {
                clone->data[i] = (zt_text *)zt_deep_copy(g->data[i]);
            }
            return clone;
        }
        case ZT_HEAP_GRID3D_I64: {
            zt_grid3d_i64 *g = (zt_grid3d_i64 *)ref;
            zt_grid3d_i64 *clone = zt_grid3d_i64_new((zt_int)g->depth, (zt_int)g->rows, (zt_int)g->cols);
            memcpy(clone->data, g->data, g->len * sizeof(zt_int));
            return clone;
        }
        case ZT_HEAP_GRID3D_TEXT: {
            zt_grid3d_text *g = (zt_grid3d_text *)ref;
            zt_grid3d_text *clone = zt_grid3d_text_new((zt_int)g->depth, (zt_int)g->rows, (zt_int)g->cols);
            for (i = 0; i < g->len; i += 1) {
                clone->data[i] = (zt_text *)zt_deep_copy(g->data[i]);
            }
            return clone;
        }
        case ZT_HEAP_PQUEUE_I64: {
            zt_pqueue_i64 *q = (zt_pqueue_i64 *)ref;
            zt_pqueue_i64 *clone = zt_pqueue_i64_new();
            zt_pqueue_i64_ensure_capacity(clone, q->len);
            memcpy(clone->data, q->data, q->len * sizeof(zt_int));
            clone->len = q->len;
            return clone;
        }
        case ZT_HEAP_PQUEUE_TEXT: {
            zt_pqueue_text *q = (zt_pqueue_text *)ref;
            zt_pqueue_text *clone = zt_pqueue_text_new();
            zt_pqueue_text_ensure_capacity(clone, q->len);
            for (i = 0; i < q->len; i += 1) {
                clone->data[i] = (zt_text *)zt_deep_copy(q->data[i]);
            }
            clone->len = q->len;
            return clone;
        }
        case ZT_HEAP_CIRCBUF_I64: {
            zt_circbuf_i64 *b = (zt_circbuf_i64 *)ref;
            zt_circbuf_i64 *clone = zt_circbuf_i64_new((zt_int)b->capacity);
            memcpy(clone->data, b->data, b->capacity * sizeof(zt_int));
            clone->head = b->head;
            clone->len = b->len;
            return clone;
        }
        case ZT_HEAP_CIRCBUF_TEXT: {
            zt_circbuf_text *b = (zt_circbuf_text *)ref;
            zt_circbuf_text *clone = zt_circbuf_text_new((zt_int)b->capacity);
            for (i = 0; i < b->capacity; i += 1) {
                clone->data[i] = (zt_text *)zt_deep_copy(b->data[i]);
            }
            clone->head = b->head;
            clone->len = b->len;
            return clone;
        }
        case ZT_HEAP_BTREEMAP_TEXT_TEXT: {
            zt_btreemap_text_text *m = (zt_btreemap_text_text *)ref;
            zt_btreemap_text_text *clone = zt_btreemap_text_text_new();
            zt_btreemap_text_text_ensure_capacity(clone, m->len);
            for (i = 0; i < m->len; i += 1) {
                clone->keys[i] = (zt_text *)zt_deep_copy(m->keys[i]);
                clone->values[i] = (zt_text *)zt_deep_copy(m->values[i]);
            }
            clone->len = m->len;
            return clone;
        }
        case ZT_HEAP_BTREESET_TEXT: {
            zt_btreeset_text *s = (zt_btreeset_text *)ref;
            zt_btreeset_text *clone = zt_btreeset_text_new();
            zt_btreeset_text_ensure_capacity(clone, s->len);
            for (i = 0; i < s->len; i += 1) {
                clone->data[i] = (zt_text *)zt_deep_copy(s->data[i]);
            }
            clone->len = s->len;
            return clone;
        }
        case ZT_HEAP_DYN_TEXT_REPR: {
            zt_dyn_text_repr *d = (zt_dyn_text_repr *)ref;
            zt_dyn_text_repr *clone = (zt_dyn_text_repr *)calloc(1, sizeof(zt_dyn_text_repr));
            clone->header.rc = 1;
            clone->header.kind = (uint32_t)ZT_HEAP_DYN_TEXT_REPR;
            clone->tag = d->tag;
            clone->as = d->as;
            if (d->tag == (uint32_t)ZT_DYN_TEXT_REPR_TEXT && d->as.text_value != NULL) {
                clone->as.text_value = (zt_text *)zt_deep_copy(d->as.text_value);
            }
            return clone;
        }
        case ZT_HEAP_LIST_DYN_TEXT_REPR:
            return zt_list_dyn_text_repr_deep_copy((const zt_list_dyn_text_repr *)ref);
        case ZT_HEAP_LIST_GENERIC:
            return zt_list_generic_clone((const zt_list_generic *)ref);
        case ZT_HEAP_MAP_GENERIC:
            return zt_map_generic_clone((const zt_map_generic *)ref);
        case ZT_HEAP_SET_GENERIC:
            return zt_set_generic_clone((const zt_set_generic *)ref);
        case ZT_HEAP_UNKNOWN:
        default:
            {
                const zt_dynamic_heap_entry *entry = zt_find_dynamic_heap_entry(header->kind);
                if (entry != NULL && entry->clone_fn != NULL) {
                    return entry->clone_fn(ref);
                }
                return NULL;
            }
    }
}

typedef struct zt_shared_ops {
    void *(*snapshot)(const void *value);
} zt_shared_ops;

typedef struct zt_shared_handle {
    atomic_uint rc;
    void *value;
    const zt_shared_ops *ops;
} zt_shared_handle;

struct zt_shared_text {
    zt_shared_handle handle;
};

struct zt_shared_bytes {
    zt_shared_handle handle;
};

static void *zt_shared_text_snapshot_value(const void *value) {
    const zt_text *text = (const zt_text *)value;

    zt_runtime_require_text(text, "shared<text> snapshot requires text");
    return zt_text_from_utf8(zt_text_data(text), text->len);
}

static void *zt_shared_bytes_snapshot_value(const void *value) {
    const zt_bytes *bytes = (const zt_bytes *)value;

    zt_runtime_require_bytes(bytes, "shared<bytes> snapshot requires bytes");
    return zt_bytes_from_array(bytes->data, bytes->len);
}

static const zt_shared_ops zt_shared_text_ops = {
    zt_shared_text_snapshot_value
};

static const zt_shared_ops zt_shared_bytes_ops = {
    zt_shared_bytes_snapshot_value
};

static void zt_shared_handle_init(zt_shared_handle *handle, void *value, const zt_shared_ops *ops) {
    if (handle == NULL || value == NULL || ops == NULL || ops->snapshot == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "invalid shared handle initialization");
    }

    atomic_init(&handle->rc, 1u);
    zt_retain(value);
    handle->value = value;
    handle->ops = ops;
}

static void zt_shared_handle_retain(zt_shared_handle *handle) {
    uint32_t current;

    if (handle == NULL) {
        return;
    }

    current = atomic_load_explicit(&handle->rc, memory_order_relaxed);
    for (;;) {
        if (current == UINT32_MAX) {
            zt_runtime_error(ZT_ERR_PLATFORM, "shared reference count overflow");
        }
        if (atomic_compare_exchange_weak_explicit(
                &handle->rc,
                &current,
                current + 1,
                memory_order_relaxed,
                memory_order_relaxed)) {
            return;
        }
    }
}

static zt_bool zt_shared_handle_release(zt_shared_handle *handle) {
    uint32_t current;

    if (handle == NULL) {
        return false;
    }

    current = atomic_load_explicit(&handle->rc, memory_order_acquire);
    for (;;) {
        if (current == 0) {
            zt_runtime_error(ZT_ERR_PLATFORM, "release on shared handle with rc=0");
        }
        if (atomic_compare_exchange_weak_explicit(
                &handle->rc,
                &current,
                current - 1,
                memory_order_acq_rel,
                memory_order_acquire)) {
            return current == 1;
        }
    }
}

static const void *zt_shared_handle_borrow(const zt_shared_handle *handle, const char *message) {
    if (handle == NULL || handle->value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, message);
    }

    return handle->value;
}

static void *zt_shared_handle_snapshot(const zt_shared_handle *handle, const char *message) {
    const void *value = zt_shared_handle_borrow(handle, message);
    return handle->ops->snapshot(value);
}

static uint32_t zt_shared_handle_ref_count(const zt_shared_handle *handle, const char *message) {
    if (handle == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, message);
    }

    return atomic_load_explicit(&handle->rc, memory_order_acquire);
}

zt_shared_text *zt_shared_text_new(zt_text *value) {
    zt_shared_text *shared;

    zt_runtime_require_text(value, "zt_shared_text_new requires text");
    shared = (zt_shared_text *)calloc(1, sizeof(zt_shared_text));
    if (shared == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate shared<text> box");
    }

    zt_shared_handle_init(&shared->handle, value, &zt_shared_text_ops);
    return shared;
}

zt_shared_text *zt_shared_text_retain(zt_shared_text *shared) {
    if (shared == NULL) {
        return NULL;
    }

    zt_shared_handle_retain(&shared->handle);
    return shared;
}

void zt_shared_text_release(zt_shared_text *shared) {
    if (shared == NULL) {
        return;
    }

    if (zt_shared_handle_release(&shared->handle)) {
        zt_release(shared->handle.value);
        free(shared);
    }
}

const zt_text *zt_shared_text_borrow(const zt_shared_text *shared) {
    return (const zt_text *)zt_shared_handle_borrow(
        shared != NULL ? &shared->handle : NULL,
        "zt_shared_text_borrow requires shared text");
}

zt_text *zt_shared_text_snapshot(const zt_shared_text *shared) {
    return (zt_text *)zt_shared_handle_snapshot(
        shared != NULL ? &shared->handle : NULL,
        "zt_shared_text_snapshot requires shared text");
}

uint32_t zt_shared_text_ref_count(const zt_shared_text *shared) {
    return zt_shared_handle_ref_count(
        shared != NULL ? &shared->handle : NULL,
        "zt_shared_text_ref_count requires shared text");
}

zt_shared_bytes *zt_shared_bytes_new(zt_bytes *value) {
    zt_shared_bytes *shared;

    zt_runtime_require_bytes(value, "zt_shared_bytes_new requires bytes");
    shared = (zt_shared_bytes *)calloc(1, sizeof(zt_shared_bytes));
    if (shared == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate shared<bytes> box");
    }

    zt_shared_handle_init(&shared->handle, value, &zt_shared_bytes_ops);
    return shared;
}

zt_shared_bytes *zt_shared_bytes_retain(zt_shared_bytes *shared) {
    if (shared == NULL) {
        return NULL;
    }

    zt_shared_handle_retain(&shared->handle);
    return shared;
}

void zt_shared_bytes_release(zt_shared_bytes *shared) {
    if (shared == NULL) {
        return;
    }

    if (zt_shared_handle_release(&shared->handle)) {
        zt_release(shared->handle.value);
        free(shared);
    }
}

const zt_bytes *zt_shared_bytes_borrow(const zt_shared_bytes *shared) {
    return (const zt_bytes *)zt_shared_handle_borrow(
        shared != NULL ? &shared->handle : NULL,
        "zt_shared_bytes_borrow requires shared bytes");
}

zt_bytes *zt_shared_bytes_snapshot(const zt_shared_bytes *shared) {
    return (zt_bytes *)zt_shared_handle_snapshot(
        shared != NULL ? &shared->handle : NULL,
        "zt_shared_bytes_snapshot requires shared bytes");
}

uint32_t zt_shared_bytes_ref_count(const zt_shared_bytes *shared) {
    return zt_shared_handle_ref_count(
        shared != NULL ? &shared->handle : NULL,
        "zt_shared_bytes_ref_count requires shared bytes");
}

void zt_runtime_report_error(zt_error_kind kind, const char *message, const char *code, zt_runtime_span span) {
    zt_runtime_store_error(kind, message, code, span);
}

static jmp_buf zt_test_throws_jump;
static int zt_test_throws_active = 0;

static int zt_runtime_exit_code_for_kind(zt_error_kind kind) {
    switch (kind) {
        case ZT_ERR_TEST_FAILED:
            return ZT_EXIT_CODE_TEST_FAILED;
        case ZT_ERR_TEST_SKIPPED:
            return ZT_EXIT_CODE_TEST_SKIPPED;
        default:
            return ZT_EXIT_CODE_RUNTIME_ERROR;
    }
}

ZT_NORETURN void zt_runtime_error_ex(zt_error_kind kind, const char *message, const char *code, zt_runtime_span span) {
    zt_runtime_report_error(kind, message, code, span);
    if (zt_test_throws_active) {
        longjmp(zt_test_throws_jump, 1);
    }
    zt_runtime_print_error(&zt_last_error);
    exit(zt_runtime_exit_code_for_kind(kind));
}

ZT_NORETURN void zt_runtime_error_with_span(zt_error_kind kind, const char *message, zt_runtime_span span) {
    zt_runtime_error_ex(kind, message, NULL, span);
}

ZT_NORETURN void zt_runtime_error(zt_error_kind kind, const char *message) {
    zt_runtime_error_ex(kind, message, NULL, zt_runtime_span_unknown());
}

void zt_check(zt_bool condition, const char *message) {
    if (!condition) {
        zt_runtime_error(ZT_ERR_CHECK, message);
    }
}

ZT_NORETURN void zt_todo(const char *message) {
    const char *safe = zt_safe_message(message);
    zt_runtime_error(ZT_ERR_TODO, safe[0] != '\0' ? safe : "todo");
}

ZT_NORETURN void zt_unreachable(const char *message) {
    const char *safe = zt_safe_message(message);
    zt_runtime_error(ZT_ERR_UNREACHABLE, safe[0] != '\0' ? safe : "unreachable");
}

ZT_NORETURN void zt_panic(const char *message) {
    zt_runtime_error(ZT_ERR_PANIC, message);
}

/* ── builtins ─────────────────────────────────────────── */

void zt_builtin_print(const zt_text *value) {
    if (value != NULL && value->data != NULL) {
        fputs(value->data, stdout);
    }
    fputc('\n', stdout);
    fflush(stdout);
}

zt_text *zt_builtin_read(void) {
    zt_outcome_optional_text_core_error outcome = zt_host_read_line_stdin();
    if (outcome.is_success && outcome.value.is_present && outcome.value.value != NULL) {
        return outcome.value.value;
    }
    return zt_text_from_utf8_literal("");
}

void zt_builtin_debug(const zt_text *value) {
    if (value != NULL && value->data != NULL) {
        fprintf(stderr, "[debug] %s\n", value->data);
    } else {
        fprintf(stderr, "[debug] <nil>\n");
    }
    fflush(stderr);
}

zt_text *zt_builtin_type_name(const zt_text *value) {
    (void)value;
    return zt_text_from_utf8_literal("text");
}

zt_int zt_debug_size_of(const zt_text *value) {
    (void)value;
    return (zt_int)sizeof(zt_text *);
}

zt_list_i64 *zt_builtin_range3(zt_int start, zt_int end, zt_int step) {
    zt_list_i64 *list = zt_list_i64_new();
    zt_int i;
    if (step == 0) {
        zt_runtime_error(ZT_ERR_CONTRACT, "range step must not be zero");
    }
    if (step > 0) {
        for (i = start; i <= end; i += step) {
            zt_list_i64_push(list, i);
        }
    } else {
        for (i = start; i >= end; i += step) {
            zt_list_i64_push(list, i);
        }
    }
    return list;
}

zt_list_i64 *zt_builtin_range2(zt_int start, zt_int end) {
    return zt_builtin_range3(start, end, start <= end ? 1 : -1);
}

zt_int zt_ffi_apply_i64(zt_int value, zt_int (*callback)(zt_int)) {
    if (callback == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "extern c callback requires a valid function pointer");
    }
    return callback(value);
}

zt_int ZT_EXTERN_STDCALL zt_ffi_add_i64_stdcall(zt_int left, zt_int right) {
    return left + right;
}

/* ── end builtins ─────────────────────────────────────── */

ZT_NORETURN void zt_test_fail(zt_text *message) {
    const char *raw = (message != NULL && message->data != NULL) ? message->data : "";
    const char *final_message = raw[0] != '\0' ? raw : "test failed";
    zt_runtime_error_ex(ZT_ERR_TEST_FAILED, final_message, "test.fail", zt_runtime_span_unknown());
}

ZT_NORETURN void zt_test_skip(zt_text *reason) {
    const char *raw = (reason != NULL && reason->data != NULL) ? reason->data : "";
    const char *final_message = raw[0] != '\0' ? raw : "test skipped";
    zt_runtime_error_ex(ZT_ERR_TEST_SKIPPED, final_message, "test.skip", zt_runtime_span_unknown());
}

zt_bool zt_test_throws_closure(zt_closure *body) {
    typedef void (*zt_test_void_fn)(void *);
    zt_test_void_fn fn;

    if (body == NULL || body->fn == NULL) {
        return false;
    }

    if (setjmp(zt_test_throws_jump) != 0) {
        zt_test_throws_active = 0;
        return true;
    }

    zt_test_throws_active = 1;
    fn = (zt_test_void_fn)body->fn;
    fn(body->ctx);
    zt_test_throws_active = 0;
    return false;
}

ZT_NORETURN void zt_contract_failed(const char *message, zt_runtime_span span) {
    zt_runtime_error_with_span(ZT_ERR_CONTRACT, message, span);
}

/*
 * Concatenate `base_message` with a short value suffix and raise a contract
 * panic. Uses a dynamically-sized buffer so arbitrarily long base messages
 * are not truncated; falls back to a fixed stack buffer only if allocation
 * fails. The dynamic buffer is intentionally leaked because
 * `zt_contract_failed` never returns.
 */
static void zt_contract_failed_with_suffix(
        const char *base_message,
        const char *value_suffix,
        zt_runtime_span span) {
    const char *safe_base = zt_safe_message(base_message);
    const char *safe_suffix = value_suffix != NULL ? value_suffix : "";
    size_t base_len = strlen(safe_base);
    size_t suffix_len = strlen(safe_suffix);
    size_t total_len = base_len + suffix_len;
    char *buffer;

    buffer = (char *)malloc(total_len + 1);
    if (buffer == NULL) {
        char fallback[512];
        fallback[0] = '\0';
        zt_runtime_append_text(fallback, sizeof(fallback), safe_base);
        zt_runtime_append_text(fallback, sizeof(fallback), safe_suffix);
        zt_contract_failed(fallback, span);
        return; /* unreachable: zt_contract_failed never returns */
    }

    if (base_len > 0) memcpy(buffer, safe_base, base_len);
    if (suffix_len > 0) memcpy(buffer + base_len, safe_suffix, suffix_len);
    buffer[total_len] = '\0';

    zt_contract_failed(buffer, span);
    /* unreachable: buffer is intentionally leaked on panic */
}

void zt_contract_failed_i64(const char *message, zt_int value, zt_runtime_span span) {
    char suffix[96];
    snprintf(suffix, sizeof(suffix), " (value: %lld)", (long long)value);
    zt_contract_failed_with_suffix(message, suffix, span);
}

void zt_contract_failed_float(const char *message, zt_float value, zt_runtime_span span) {
    char suffix[96];
    snprintf(suffix, sizeof(suffix), " (value: %.17g)", (double)value);
    zt_contract_failed_with_suffix(message, suffix, span);
}

void zt_contract_failed_bool(const char *message, zt_bool value, zt_runtime_span span) {
    const char *suffix = value ? " (value: true)" : " (value: false)";
    zt_contract_failed_with_suffix(message, suffix, span);
}

static zt_text *zt_text_from_utf8_unchecked(const char *data, size_t len) {
    zt_text *value;
    size_t byte_count;

    value = (zt_text *)calloc(1, sizeof(zt_text));
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate text header");
    }

    byte_count = zt_require_added_size(len, 1, "text size overflow");
    value->data = (char *)malloc(byte_count);
    if (value->data == NULL) {
        free(value);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate text bytes");
    }

    if (data != NULL && len > 0) {
        memcpy(value->data, data, len);
    }

    value->data[len] = '\0';
    value->len = len;
    value->header.rc = 1;
    value->header.kind = (uint32_t)ZT_HEAP_TEXT;
    return value;
}

zt_text *zt_text_from_utf8(const char *data, size_t len) {
    size_t error_index = 0;
    const char *error_reason = NULL;
    char message[256];

    if (data == NULL) {
        if (len == 0) {
            return zt_text_from_utf8_unchecked("", 0);
        }
        zt_runtime_error(ZT_ERR_CONTRACT, "zt_text_from_utf8 requires UTF-8 bytes");
    }

    if (!zt_utf8_validate((const uint8_t *)data, len, &error_index, &error_reason)) {
        snprintf(
            message,
            sizeof(message),
            "zt_text_from_utf8 received invalid UTF-8 at byte %zu (%s)",
            error_index,
            error_reason != NULL ? error_reason : "invalid encoding");
        zt_runtime_error(ZT_ERR_CONTRACT, message);
    }

    return zt_text_from_utf8_unchecked(data, len);
}

zt_text *zt_text_from_utf8_literal(const char *data) {
    if (data == NULL) {
        return zt_text_from_utf8_unchecked("", 0);
    }

    return zt_text_from_utf8_unchecked(data, strlen(data));
}

zt_text *zt_text_concat(const zt_text *a, const zt_text *b) {
    zt_text *value;
    size_t left_len;
    size_t right_len;
    size_t total_len;

    zt_runtime_require_text(a, "zt_text_concat requires left text");
    zt_runtime_require_text(b, "zt_text_concat requires right text");

    left_len = a->len;
    right_len = b->len;
    if (left_len == 0) {
        zt_retain((void *)b);
        return (zt_text *)b;
    }
    if (right_len == 0) {
        zt_retain((void *)a);
        return (zt_text *)a;
    }

    total_len = zt_require_added_size(left_len, right_len, "text concat size overflow");
    value = zt_text_from_utf8_unchecked(NULL, total_len);
    if (left_len > 0) {
        memcpy(value->data, a->data, left_len);
    }
    if (right_len > 0) {
        memcpy(value->data + left_len, b->data, right_len);
    }
    value->data[total_len] = '\0';
    return value;
}

zt_closure *zt_closure_create(void *fn, void *ctx) {
    return zt_closure_create_with_drop(fn, ctx, NULL);
}

zt_closure *zt_closure_create_with_drop(void *fn, void *ctx, void (*drop_ctx)(void *)) {
    zt_closure *value = (zt_closure *)malloc(sizeof(zt_closure));
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate closure");
    }

    value->header.rc = 1;
    value->header.kind = (uint32_t)ZT_HEAP_CLOSURE;
    value->fn = fn;
    value->ctx = ctx;
    value->drop_ctx = drop_ctx;

    return value;
}

typedef struct zt_job_i64 {
    zt_closure *thunk;
    zt_int arg;
    zt_bool has_arg;
    zt_int result;
    zt_bool joined;
#ifdef _WIN32
    HANDLE thread;
#else
    zt_bool completed;
#endif
} zt_job_i64;

#define ZT_JOB_I64_MAX 64
static zt_job_i64 *zt_job_i64_slots[ZT_JOB_I64_MAX];

static zt_int zt_job_i64_run(zt_closure *thunk) {
    if (thunk == NULL || thunk->fn == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "job<int> requires a valid thunk");
    }
    return ((zt_int (*)(void *))thunk->fn)(thunk->ctx);
}

static zt_int zt_job_i64_run_with_arg(zt_closure *worker, zt_int value) {
    if (worker == NULL || worker->fn == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "job<int> requires a valid worker");
    }
    return ((zt_int (*)(void *, zt_int))worker->fn)(worker->ctx, value);
}

#ifdef _WIN32
static DWORD WINAPI zt_job_i64_thread_proc(LPVOID raw) {
    zt_job_i64 *job = (zt_job_i64 *)raw;
    job->result = job->has_arg
        ? zt_job_i64_run_with_arg(job->thunk, job->arg)
        : zt_job_i64_run(job->thunk);
    return 0;
}
#endif

static zt_int zt_job_spawn_i64_inner(zt_closure *thunk, zt_bool has_arg, zt_int arg) {
    zt_job_i64 *job;
    size_t index;

    if (thunk == NULL || thunk->fn == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "jobs.spawn_int requires a valid thunk");
    }

    for (index = 0; index < ZT_JOB_I64_MAX; index += 1) {
        if (zt_job_i64_slots[index] == NULL) break;
    }
    if (index == ZT_JOB_I64_MAX) {
        zt_runtime_error(ZT_ERR_PLATFORM, "job<int> table is full");
    }

    job = (zt_job_i64 *)calloc(1, sizeof(zt_job_i64));
    if (job == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate job<int>");
    }
    job->thunk = thunk;
    job->arg = arg;
    job->has_arg = has_arg;
    zt_retain(thunk);

#ifdef _WIN32
    job->thread = CreateThread(NULL, 0, zt_job_i64_thread_proc, job, 0, NULL);
    if (job->thread == NULL) {
        zt_release(job->thunk);
        free(job);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to spawn job<int>");
    }
#else
    job->result = job->has_arg
        ? zt_job_i64_run_with_arg(job->thunk, job->arg)
        : zt_job_i64_run(job->thunk);
    job->completed = true;
#endif

    zt_job_i64_slots[index] = job;
    return (zt_int)(index + 1);
}

zt_int zt_job_spawn_i64(zt_closure *thunk) {
    return zt_job_spawn_i64_inner(thunk, false, 0);
}

zt_int zt_job_spawn_i64_arg(zt_closure *worker, zt_int value) {
    return zt_job_spawn_i64_inner(worker, true, value);
}

zt_int zt_job_join_i64(zt_int handle) {
    size_t index;
    zt_job_i64 *job;
    zt_int result;

    if (handle <= 0 || handle > (zt_int)ZT_JOB_I64_MAX) {
        zt_runtime_error(ZT_ERR_CONTRACT, "invalid job<int> handle");
    }
    index = (size_t)(handle - 1);
    job = zt_job_i64_slots[index];
    if (job == NULL || job->joined) {
        zt_runtime_error(ZT_ERR_CONTRACT, "job<int> already joined or invalid");
    }

#ifdef _WIN32
    WaitForSingleObject(job->thread, INFINITE);
    CloseHandle(job->thread);
#endif

    job->joined = true;
    result = job->result;
    zt_job_i64_slots[index] = NULL;
    zt_release(job->thunk);
    free(job);
    return result;
}

typedef struct zt_channel_i64 {
    zt_bool has_value;
    zt_bool closed;
    zt_int value;
} zt_channel_i64;

typedef struct zt_shared_i64 {
    zt_int value;
} zt_shared_i64;

typedef struct zt_atomic_i64 {
    atomic_llong value;
} zt_atomic_i64;

#define ZT_CHANNEL_I64_MAX 64
#define ZT_SHARED_I64_MAX 64
#define ZT_ATOMIC_I64_MAX 64
static zt_channel_i64 *zt_channel_i64_slots[ZT_CHANNEL_I64_MAX];
static zt_shared_i64 *zt_shared_i64_slots[ZT_SHARED_I64_MAX];
static zt_atomic_i64 *zt_atomic_i64_slots[ZT_ATOMIC_I64_MAX];

static size_t zt_runtime_allocate_handle_slot(void **slots, size_t count, const char *message) {
    size_t index;
    for (index = 0; index < count; index += 1) {
        if (slots[index] == NULL) return index;
    }
    zt_runtime_error(ZT_ERR_PLATFORM, message);
}

static size_t zt_runtime_checked_handle_index(zt_int handle, size_t count, const char *message) {
    if (handle <= 0 || handle > (zt_int)count) {
        zt_runtime_error(ZT_ERR_CONTRACT, message);
    }
    return (size_t)(handle - 1);
}

zt_int zt_channel_i64_create(void) {
    zt_channel_i64 *channel;
    size_t index = zt_runtime_allocate_handle_slot((void **)zt_channel_i64_slots, ZT_CHANNEL_I64_MAX, "channel<int> table is full");
    channel = (zt_channel_i64 *)calloc(1, sizeof(zt_channel_i64));
    if (channel == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate channel<int>");
    }
    zt_channel_i64_slots[index] = channel;
    return (zt_int)(index + 1);
}

zt_int zt_channel_i64_send(zt_int handle, zt_int value) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_CHANNEL_I64_MAX, "invalid channel<int> handle");
    zt_channel_i64 *channel = zt_channel_i64_slots[index];
    if (channel == NULL || channel->closed) {
        zt_runtime_error(ZT_ERR_CONTRACT, "channel<int> is closed or invalid");
    }
    if (channel->has_value) {
        zt_runtime_error(ZT_ERR_CONTRACT, "channel<int> buffer is full");
    }
    channel->value = value;
    channel->has_value = true;
    return 0;
}

zt_optional_i64 zt_channel_i64_receive(zt_int handle) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_CHANNEL_I64_MAX, "invalid channel<int> handle");
    zt_channel_i64 *channel = zt_channel_i64_slots[index];
    zt_int value;
    if (channel == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "channel<int> is invalid");
    }
    if (!channel->has_value) {
        return zt_optional_i64_empty();
    }
    value = channel->value;
    channel->has_value = false;
    return zt_optional_i64_present(value);
}

zt_int zt_channel_i64_close(zt_int handle) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_CHANNEL_I64_MAX, "invalid channel<int> handle");
    zt_channel_i64 *channel = zt_channel_i64_slots[index];
    if (channel == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "channel<int> is invalid");
    }
    free(channel);
    zt_channel_i64_slots[index] = NULL;
    return 0;
}

zt_int zt_shared_i64_create(zt_int value) {
    zt_shared_i64 *shared;
    size_t index = zt_runtime_allocate_handle_slot((void **)zt_shared_i64_slots, ZT_SHARED_I64_MAX, "shared<int> table is full");
    shared = (zt_shared_i64 *)calloc(1, sizeof(zt_shared_i64));
    if (shared == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate shared<int>");
    }
    shared->value = value;
    zt_shared_i64_slots[index] = shared;
    return (zt_int)(index + 1);
}

zt_int zt_shared_i64_get(zt_int handle) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_SHARED_I64_MAX, "invalid shared<int> handle");
    zt_shared_i64 *shared = zt_shared_i64_slots[index];
    if (shared == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "shared<int> is invalid");
    }
    return shared->value;
}

zt_int zt_shared_i64_set(zt_int handle, zt_int value) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_SHARED_I64_MAX, "invalid shared<int> handle");
    zt_shared_i64 *shared = zt_shared_i64_slots[index];
    if (shared == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "shared<int> is invalid");
    }
    shared->value = value;
    return 0;
}

zt_int zt_atomic_i64_create(zt_int value) {
    zt_atomic_i64 *atomic_value;
    size_t index = zt_runtime_allocate_handle_slot((void **)zt_atomic_i64_slots, ZT_ATOMIC_I64_MAX, "atomic<int> table is full");
    atomic_value = (zt_atomic_i64 *)calloc(1, sizeof(zt_atomic_i64));
    if (atomic_value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate atomic<int>");
    }
    atomic_init(&atomic_value->value, (long long)value);
    zt_atomic_i64_slots[index] = atomic_value;
    return (zt_int)(index + 1);
}

zt_int zt_atomic_i64_load(zt_int handle) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_ATOMIC_I64_MAX, "invalid atomic<int> handle");
    zt_atomic_i64 *atomic_value = zt_atomic_i64_slots[index];
    if (atomic_value == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "atomic<int> is invalid");
    }
    return (zt_int)atomic_load(&atomic_value->value);
}

zt_int zt_atomic_i64_store(zt_int handle, zt_int value) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_ATOMIC_I64_MAX, "invalid atomic<int> handle");
    zt_atomic_i64 *atomic_value = zt_atomic_i64_slots[index];
    if (atomic_value == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "atomic<int> is invalid");
    }
    atomic_store(&atomic_value->value, (long long)value);
    return value;
}

zt_int zt_atomic_i64_add(zt_int handle, zt_int delta) {
    size_t index = zt_runtime_checked_handle_index(handle, ZT_ATOMIC_I64_MAX, "invalid atomic<int> handle");
    zt_atomic_i64 *atomic_value = zt_atomic_i64_slots[index];
    if (atomic_value == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "atomic<int> is invalid");
    }
    return (zt_int)(atomic_fetch_add(&atomic_value->value, (long long)delta) + (long long)delta);
}

zt_lazy_i64 *zt_lazy_i64_once(zt_closure *thunk) {
    zt_lazy_i64 *value;

    if (thunk == NULL || thunk->fn == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy<int> requires a valid thunk");
    }

    value = (zt_lazy_i64 *)malloc(sizeof(zt_lazy_i64));
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate lazy<int>");
    }

    value->header.rc = 1;
    value->header.kind = (uint32_t)ZT_HEAP_LAZY_I64;
    value->thunk = thunk;
    value->consumed = false;
    zt_retain(thunk);

    return value;
}

zt_int zt_lazy_i64_force(zt_lazy_i64 *value) {
    zt_closure *thunk;
    zt_int result;

    if (value == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy<int> force requires a value");
    }
    if (value->consumed || value->thunk == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy value already consumed");
    }

    value->consumed = true;
    thunk = value->thunk;
    value->thunk = NULL;
    result = ((zt_int (*)(void *))thunk->fn)(thunk->ctx);
    zt_release(thunk);
    return result;
}

zt_bool zt_lazy_i64_is_consumed(const zt_lazy_i64 *value) {
    if (value == NULL) {
        return true;
    }
    return value->consumed;
}

#define ZT_DEFINE_LAZY_VALUE_IMPL(SUFFIX, ELEM_TYPE, HEAP_KIND, DISPLAY_NAME) \
zt_lazy_##SUFFIX *zt_lazy_##SUFFIX##_once(zt_closure *thunk) { \
    zt_lazy_##SUFFIX *value; \
    if (thunk == NULL || thunk->fn == NULL) { \
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy<" DISPLAY_NAME "> requires a valid thunk"); \
    } \
    value = (zt_lazy_##SUFFIX *)malloc(sizeof(zt_lazy_##SUFFIX)); \
    if (value == NULL) { \
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate lazy<" DISPLAY_NAME ">"); \
    } \
    value->header.rc = 1; \
    value->header.kind = (uint32_t)(HEAP_KIND); \
    value->thunk = thunk; \
    value->consumed = false; \
    zt_retain(thunk); \
    return value; \
} \
ELEM_TYPE zt_lazy_##SUFFIX##_force(zt_lazy_##SUFFIX *value) { \
    zt_closure *thunk; \
    ELEM_TYPE result; \
    if (value == NULL) { \
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy<" DISPLAY_NAME "> force requires a value"); \
    } \
    if (value->consumed || value->thunk == NULL) { \
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy value already consumed"); \
    } \
    value->consumed = true; \
    thunk = value->thunk; \
    value->thunk = NULL; \
    result = ((ELEM_TYPE (*)(void *))thunk->fn)(thunk->ctx); \
    zt_release(thunk); \
    return result; \
} \
zt_bool zt_lazy_##SUFFIX##_is_consumed(const zt_lazy_##SUFFIX *value) { \
    if (value == NULL) return true; \
    return value->consumed; \
}

ZT_DEFINE_LAZY_VALUE_IMPL(f64, zt_float, ZT_HEAP_LAZY_F64, "float")
ZT_DEFINE_LAZY_VALUE_IMPL(bool, zt_bool, ZT_HEAP_LAZY_BOOL, "bool")
ZT_DEFINE_LAZY_VALUE_IMPL(i8, int8_t, ZT_HEAP_LAZY_I8, "int8")
ZT_DEFINE_LAZY_VALUE_IMPL(i16, int16_t, ZT_HEAP_LAZY_I16, "int16")
ZT_DEFINE_LAZY_VALUE_IMPL(i32, int32_t, ZT_HEAP_LAZY_I32, "int32")
ZT_DEFINE_LAZY_VALUE_IMPL(u8, uint8_t, ZT_HEAP_LAZY_U8, "uint8")
ZT_DEFINE_LAZY_VALUE_IMPL(u16, uint16_t, ZT_HEAP_LAZY_U16, "uint16")
ZT_DEFINE_LAZY_VALUE_IMPL(u32, uint32_t, ZT_HEAP_LAZY_U32, "uint32")
ZT_DEFINE_LAZY_VALUE_IMPL(u64, uint64_t, ZT_HEAP_LAZY_U64, "uint64")

#undef ZT_DEFINE_LAZY_VALUE_IMPL

zt_lazy_text *zt_lazy_text_once(zt_closure *thunk) {
    zt_lazy_text *value;
    if (thunk == NULL || thunk->fn == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy<text> requires a valid thunk");
    }
    value = (zt_lazy_text *)malloc(sizeof(zt_lazy_text));
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate lazy<text>");
    }
    value->header.rc = 1;
    value->header.kind = (uint32_t)ZT_HEAP_LAZY_TEXT;
    value->thunk = thunk;
    value->consumed = false;
    zt_retain(thunk);
    return value;
}

zt_text *zt_lazy_text_force(zt_lazy_text *value) {
    zt_closure *thunk;
    zt_text *result;
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy<text> force requires a value");
    }
    if (value->consumed || value->thunk == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, "lazy value already consumed");
    }
    value->consumed = true;
    thunk = value->thunk;
    value->thunk = NULL;
    result = ((zt_text *(*)(void *))thunk->fn)(thunk->ctx);
    zt_runtime_require_text(result, "lazy<text> thunk returned null text");
    zt_release(thunk);
    return result;
}

zt_bool zt_lazy_text_is_consumed(const zt_lazy_text *value) {
    if (value == NULL) return true;
    return value->consumed;
}

zt_text *zt_text_index(const zt_text *value, zt_int index_0) {
    const uint8_t *data;
    size_t codepoint_count;
    size_t byte_offset;
    size_t byte_width;

    zt_runtime_require_text(value, "zt_text_index requires text");

    if (index_0 < 0) {
        zt_runtime_error(ZT_ERR_INDEX, "text index out of bounds");
    }

    codepoint_count = zt_text_codepoint_count(value, "text value contains invalid UTF-8");
    if ((size_t)index_0 >= codepoint_count) {
        zt_runtime_error(ZT_ERR_INDEX, "text index out of bounds");
    }

    data = (const uint8_t *)value->data;
    byte_offset = zt_text_byte_offset_for_codepoint(value, (size_t)index_0, "text value contains invalid UTF-8");
    byte_width = zt_utf8_sequence_width_or_error(data, value->len, byte_offset, "text value contains invalid UTF-8");
    return zt_text_from_utf8(value->data + byte_offset, byte_width);
}

zt_text *zt_text_slice(const zt_text *value, zt_int start_0, zt_int end_0) {
    size_t codepoint_count;
    size_t start_index;
    size_t end_index;
    size_t start_byte;
    size_t end_exclusive_byte;

    zt_runtime_require_text(value, "zt_text_slice requires text");

    if (start_0 < 0) {
        zt_runtime_error(ZT_ERR_INDEX, "slice start must be >= 0");
    }

    if (value->len == 0) {
        return zt_text_from_utf8("", 0);
    }

    codepoint_count = zt_text_codepoint_count(value, "text value contains invalid UTF-8");
    start_index = (size_t)start_0;
    end_index = zt_normalize_slice_end(codepoint_count, end_0);

    if (start_index >= codepoint_count || end_index < start_index) {
        return zt_text_from_utf8("", 0);
    }

    start_byte = zt_text_byte_offset_for_codepoint(value, start_index, "text value contains invalid UTF-8");
    if (end_index + 1 >= codepoint_count) {
        end_exclusive_byte = value->len;
    } else {
        end_exclusive_byte = zt_text_byte_offset_for_codepoint(value, end_index + 1, "text value contains invalid UTF-8");
    }

    if (start_byte == 0 && end_exclusive_byte == value->len) {
        zt_retain((void *)value);
        return (zt_text *)value;
    }

    return zt_text_from_utf8(value->data + start_byte, end_exclusive_byte - start_byte);
}

zt_list_text *zt_text_chars(const zt_text *value) {
    zt_list_text *items;
    size_t count;
    size_t index;

    zt_runtime_require_text(value, "zt_text_chars requires text");
    count = zt_text_codepoint_count(value, "text value contains invalid UTF-8");
    items = zt_list_text_new();
    zt_list_text_reserve(items, count);

    for (index = 0; index < count; index += 1) {
        zt_text *ch = zt_text_index(value, (zt_int)index);
        zt_list_text_push(items, ch);
        zt_release(ch);
    }

    return items;
}

zt_list_text *zt_text_split(const zt_text *value, const zt_text *separator) {
    zt_list_text *items;
    size_t start;
    size_t index;

    zt_runtime_require_text(value, "zt_text_split requires text");
    zt_runtime_require_text(separator, "zt_text_split requires separator");

    if (separator->len == 0) {
        return zt_text_chars(value);
    }

    items = zt_list_text_new();
    start = 0;
    index = 0;

    while (index + separator->len <= value->len) {
        if (memcmp(value->data + index, separator->data, separator->len) == 0) {
            zt_text *part = zt_text_from_utf8(value->data + start, index - start);
            zt_list_text_push(items, part);
            zt_release(part);
            index += separator->len;
            start = index;
        } else {
            index += 1;
        }
    }

    {
        zt_text *tail = zt_text_from_utf8(value->data + start, value->len - start);
        zt_list_text_push(items, tail);
        zt_release(tail);
    }

    return items;
}

zt_text *zt_text_to_lower_ascii(const zt_text *value) {
    zt_text *out;
    size_t index;

    zt_runtime_require_text(value, "zt_text_to_lower_ascii requires text");
    out = zt_text_from_utf8(value->data, value->len);
    for (index = 0; index < out->len; index += 1) {
        char ch = out->data[index];
        if (ch >= 'A' && ch <= 'Z') {
            out->data[index] = (char)(ch + ('a' - 'A'));
        }
    }
    return out;
}

zt_text *zt_text_to_upper_ascii(const zt_text *value) {
    zt_text *out;
    size_t index;

    zt_runtime_require_text(value, "zt_text_to_upper_ascii requires text");
    out = zt_text_from_utf8(value->data, value->len);
    for (index = 0; index < out->len; index += 1) {
        char ch = out->data[index];
        if (ch >= 'a' && ch <= 'z') {
            out->data[index] = (char)(ch - ('a' - 'A'));
        }
    }
    return out;
}

zt_text *zt_text_capitalize_ascii(const zt_text *value) {
    zt_text *out;
    zt_bool word_start = true;
    size_t index;

    zt_runtime_require_text(value, "zt_text_capitalize_ascii requires text");
    out = zt_text_from_utf8(value->data, value->len);

    for (index = 0; index < out->len; index += 1) {
        char ch = out->data[index];
        if (ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r') {
            word_start = true;
            continue;
        }
        if (word_start && ch >= 'a' && ch <= 'z') {
            out->data[index] = (char)(ch - ('a' - 'A'));
        }
        word_start = false;
    }

    return out;
}

zt_int zt_orc_collect_cycles(void) {
    return 0;
}

zt_int zt_orc_ref_count_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_orc_ref_count_text requires text");
    if (value->header.rc == UINT32_MAX) return -1;
    return (zt_int)value->header.rc;
}

zt_int zt_orc_ref_count_list_text(const zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_orc_ref_count_list_text requires list");
    if (value->header.rc == UINT32_MAX) return -1;
    return (zt_int)value->header.rc;
}

zt_bool zt_orc_is_unique_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_orc_is_unique_text requires text");
    return value->header.rc == 1u;
}

zt_bool zt_orc_is_unique_list_text(const zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_orc_is_unique_list_text requires list");
    return value->header.rc == 1u;
}

zt_int zt_unsafe_heap_kind_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_unsafe_heap_kind_text requires text");
    return (zt_int)value->header.kind;
}

zt_int zt_unsafe_heap_kind_list_text(const zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_unsafe_heap_kind_list_text requires list");
    return (zt_int)value->header.kind;
}

zt_text *zt_unsafe_retain_text(zt_text *value) {
    zt_runtime_require_text(value, "zt_unsafe_retain_text requires text");
    zt_retain(value);
    return value;
}

zt_list_text *zt_unsafe_retain_list_text(zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_unsafe_retain_list_text requires list");
    zt_retain(value);
    return value;
}

zt_text *zt_mem_own_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_mem_own_text requires text");
    return zt_text_deep_copy(value);
}

zt_text *zt_mem_view_text(zt_text *value) {
    zt_runtime_require_text(value, "zt_mem_view_text requires text");
    zt_retain(value);
    return value;
}

zt_text *zt_mem_edit_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_mem_edit_text requires text");
    return zt_text_deep_copy(value);
}

zt_list_text *zt_mem_own_list_text(const zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_mem_own_list_text requires list");
    return zt_list_text_deep_copy(value);
}

zt_list_text *zt_mem_view_list_text(zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_mem_view_list_text requires list");
    zt_retain(value);
    return value;
}

zt_list_text *zt_mem_edit_list_text(const zt_list_text *value) {
    zt_runtime_require_list_text(value, "zt_mem_edit_list_text requires list");
    return zt_list_text_deep_copy(value);
}

void *zt_mem_own_heap(const void *value) {
    void *copy;
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_mem_own_heap requires value");
    }
    copy = zt_deep_copy((void *)value);
    if (copy == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_mem_own_heap could not clone value");
    }
    return copy;
}

void *zt_mem_view_heap(void *value) {
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_mem_view_heap requires value");
    }
    zt_retain(value);
    return value;
}

void *zt_mem_edit_heap(const void *value) {
    void *copy;
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_mem_edit_heap requires value");
    }
    copy = zt_deep_copy((void *)value);
    if (copy == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_mem_edit_heap could not clone value");
    }
    return copy;
}

#define ZT_DEFINE_MEM_PRIMITIVE_LIST_API(SUFFIX, LIST_TYPE) \
LIST_TYPE *zt_mem_own_list_##SUFFIX(const LIST_TYPE *value) { \
    zt_runtime_require_list_##SUFFIX(value, "zt_mem_own_list_" #SUFFIX " requires list"); \
    return zt_list_##SUFFIX##_from_array(value->data, value->len); \
} \
LIST_TYPE *zt_mem_view_list_##SUFFIX(LIST_TYPE *value) { \
    zt_runtime_require_list_##SUFFIX(value, "zt_mem_view_list_" #SUFFIX " requires list"); \
    zt_retain(value); \
    return value; \
} \
LIST_TYPE *zt_mem_edit_list_##SUFFIX(const LIST_TYPE *value) { \
    zt_runtime_require_list_##SUFFIX(value, "zt_mem_edit_list_" #SUFFIX " requires list"); \
    return zt_list_##SUFFIX##_from_array(value->data, value->len); \
}

ZT_DEFINE_MEM_PRIMITIVE_LIST_API(i64, zt_list_i64)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(f64, zt_list_f64)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(bool, zt_list_bool)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(i8, zt_list_i8)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(i16, zt_list_i16)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(i32, zt_list_i32)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(u8, zt_list_u8)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(u16, zt_list_u16)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(u32, zt_list_u32)
ZT_DEFINE_MEM_PRIMITIVE_LIST_API(u64, zt_list_u64)

#undef ZT_DEFINE_MEM_PRIMITIVE_LIST_API

zt_bool zt_text_eq(const zt_text *a, const zt_text *b) {
    zt_runtime_require_text(a, "zt_text_eq requires left text");
    zt_runtime_require_text(b, "zt_text_eq requires right text");

    if (a->len != b->len) {
        return false;
    }

    if (a->len == 0) {
        return true;
    }

    if (a->data[0] != b->data[0]) {
        return false;
    }

    return memcmp(a->data, b->data, a->len) == 0;
}

zt_int zt_text_len(const zt_text *value) {
    zt_runtime_require_text(value, "zt_text_len requires text");
    return (zt_int)zt_text_codepoint_count(value, "text value contains invalid UTF-8");
}

const char *zt_text_data(const zt_text *value) {
    zt_runtime_require_text(value, "zt_text_data requires text");
    return value->data != NULL ? value->data : "";
}

zt_text *zt_text_deep_copy(const zt_text *value) {
    zt_runtime_require_text(value, "zt_text_deep_copy requires text");
    return zt_text_from_utf8(value->data, value->len);
}

typedef enum zt_regex_atom_kind {
    ZT_REGEX_ATOM_LITERAL,
    ZT_REGEX_ATOM_ANY,
    ZT_REGEX_ATOM_CLASS,
    ZT_REGEX_ATOM_DIGIT,
    ZT_REGEX_ATOM_WORD,
    ZT_REGEX_ATOM_SPACE
} zt_regex_atom_kind;

typedef struct zt_regex_atom {
    zt_regex_atom_kind kind;
    char literal;
    const char *class_start;
    size_t class_len;
    zt_bool class_negated;
} zt_regex_atom;

static zt_bool zt_regex_is_quantifier(char ch) {
    return ch == '*' || ch == '+' || ch == '?';
}

static zt_bool zt_regex_is_word_char(char ch) {
    unsigned char value = (unsigned char)ch;
    return (zt_bool)(isalnum(value) || ch == '_');
}

static zt_bool zt_regex_find_class_end(
        const char *pattern,
        size_t pattern_len,
        size_t open_index,
        size_t *close_index) {
    size_t index;
    size_t content_start;

    if (pattern == NULL || close_index == NULL || open_index >= pattern_len) {
        return false;
    }

    content_start = open_index + 1;
    if (content_start < pattern_len && pattern[content_start] == '^') {
        content_start += 1;
    }
    if (content_start >= pattern_len || pattern[content_start] == ']') {
        return false;
    }

    for (index = content_start; index < pattern_len; index += 1) {
        if (pattern[index] == '\\') {
            if (index + 1 >= pattern_len) {
                return false;
            }
            index += 1;
            continue;
        }
        if (pattern[index] == ']') {
            *close_index = index;
            return true;
        }
    }

    return false;
}

static zt_bool zt_regex_class_range_is_valid(const char *start, size_t len) {
    size_t index = 0;

    while (index < len) {
        if (start[index] == '\\') {
            if (index + 1 >= len) {
                return false;
            }
            index += 2;
            continue;
        }
        if (index + 2 < len && start[index + 1] == '-') {
            unsigned char first = (unsigned char)start[index];
            unsigned char last = (unsigned char)start[index + 2];
            if (first > last) {
                return false;
            }
            index += 3;
            continue;
        }
        index += 1;
    }

    return true;
}

static zt_bool zt_regex_validate_pattern_data(
        const char *pattern,
        size_t pattern_len,
        const char **message) {
    size_t index = 0;
    zt_bool atom_ready = false;
    zt_bool just_quantified = false;

    if (message != NULL) {
        *message = "invalid regex pattern";
    }
    if (pattern == NULL) {
        if (message != NULL) *message = "regex pattern cannot be null";
        return false;
    }
    if (pattern_len == 0) {
        return true;
    }
    if (pattern[index] == '^') {
        index += 1;
    }

    while (index < pattern_len) {
        char ch = pattern[index];

        if (ch == '$' && index + 1 == pattern_len) {
            index += 1;
            continue;
        }
        if (zt_regex_is_quantifier(ch)) {
            if (!atom_ready || just_quantified) {
                if (message != NULL) *message = "regex quantifier has no atom";
                return false;
            }
            atom_ready = false;
            just_quantified = true;
            index += 1;
            continue;
        }
        if (ch == '\\') {
            if (index + 1 >= pattern_len) {
                if (message != NULL) *message = "regex escape is incomplete";
                return false;
            }
            index += 2;
            atom_ready = true;
            just_quantified = false;
            continue;
        }
        if (ch == '[') {
            size_t close_index = 0;
            size_t class_start;
            size_t class_len;

            if (!zt_regex_find_class_end(pattern, pattern_len, index, &close_index)) {
                if (message != NULL) *message = "regex character class is not closed";
                return false;
            }
            class_start = index + 1;
            if (class_start < close_index && pattern[class_start] == '^') {
                class_start += 1;
            }
            class_len = close_index - class_start;
            if (!zt_regex_class_range_is_valid(pattern + class_start, class_len)) {
                if (message != NULL) *message = "regex character class range is invalid";
                return false;
            }
            index = close_index + 1;
            atom_ready = true;
            just_quantified = false;
            continue;
        }
        if (ch == ']') {
            if (message != NULL) *message = "regex character class is not open";
            return false;
        }

        index += 1;
        atom_ready = true;
        just_quantified = false;
    }

    return true;
}

static zt_bool zt_regex_parse_atom(
        const char *pattern,
        size_t pattern_len,
        size_t index,
        zt_regex_atom *atom,
        size_t *next_index) {
    char ch;

    if (pattern == NULL || atom == NULL || next_index == NULL || index >= pattern_len) {
        return false;
    }

    ch = pattern[index];
    atom->kind = ZT_REGEX_ATOM_LITERAL;
    atom->literal = ch;
    atom->class_start = NULL;
    atom->class_len = 0;
    atom->class_negated = false;

    if (ch == '.') {
        atom->kind = ZT_REGEX_ATOM_ANY;
        *next_index = index + 1;
        return true;
    }
    if (ch == '\\') {
        char escaped;
        if (index + 1 >= pattern_len) {
            return false;
        }
        escaped = pattern[index + 1];
        if (escaped == 'd') {
            atom->kind = ZT_REGEX_ATOM_DIGIT;
        } else if (escaped == 'w') {
            atom->kind = ZT_REGEX_ATOM_WORD;
        } else if (escaped == 's') {
            atom->kind = ZT_REGEX_ATOM_SPACE;
        } else {
            atom->kind = ZT_REGEX_ATOM_LITERAL;
            atom->literal = escaped;
        }
        *next_index = index + 2;
        return true;
    }
    if (ch == '[') {
        size_t close_index = 0;
        size_t class_start = index + 1;

        if (!zt_regex_find_class_end(pattern, pattern_len, index, &close_index)) {
            return false;
        }
        if (class_start < close_index && pattern[class_start] == '^') {
            atom->class_negated = true;
            class_start += 1;
        }
        atom->kind = ZT_REGEX_ATOM_CLASS;
        atom->class_start = pattern + class_start;
        atom->class_len = close_index - class_start;
        *next_index = close_index + 1;
        return true;
    }

    *next_index = index + 1;
    return true;
}

static zt_bool zt_regex_class_content_matches(const zt_regex_atom *atom, char ch) {
    size_t index = 0;
    zt_bool matched = false;

    while (index < atom->class_len) {
        char item = atom->class_start[index];

        if (item == '\\' && index + 1 < atom->class_len) {
            char escaped = atom->class_start[index + 1];
            if (escaped == 'd') {
                matched = (zt_bool)(matched || isdigit((unsigned char)ch));
            } else if (escaped == 'w') {
                matched = (zt_bool)(matched || zt_regex_is_word_char(ch));
            } else if (escaped == 's') {
                matched = (zt_bool)(matched || isspace((unsigned char)ch));
            } else {
                matched = (zt_bool)(matched || ch == escaped);
            }
            index += 2;
            continue;
        }
        if (index + 2 < atom->class_len && atom->class_start[index + 1] == '-') {
            unsigned char first = (unsigned char)item;
            unsigned char last = (unsigned char)atom->class_start[index + 2];
            unsigned char value = (unsigned char)ch;
            if (first <= value && value <= last) {
                matched = true;
            }
            index += 3;
            continue;
        }
        if (item == ch) {
            matched = true;
        }
        index += 1;
    }

    return atom->class_negated ? (zt_bool)!matched : matched;
}

static zt_bool zt_regex_atom_matches(const zt_regex_atom *atom, char ch) {
    switch (atom->kind) {
        case ZT_REGEX_ATOM_LITERAL:
            return (zt_bool)(ch == atom->literal);
        case ZT_REGEX_ATOM_ANY:
            return true;
        case ZT_REGEX_ATOM_CLASS:
            return zt_regex_class_content_matches(atom, ch);
        case ZT_REGEX_ATOM_DIGIT:
            return (zt_bool)isdigit((unsigned char)ch);
        case ZT_REGEX_ATOM_WORD:
            return zt_regex_is_word_char(ch);
        case ZT_REGEX_ATOM_SPACE:
            return (zt_bool)isspace((unsigned char)ch);
    }
    return false;
}

static zt_bool zt_regex_match_here(
        const char *pattern,
        size_t pattern_len,
        size_t pattern_index,
        const char *input,
        size_t input_len,
        size_t input_index,
        size_t *end_index) {
    zt_regex_atom atom;
    size_t atom_end = 0;
    size_t after_quantifier;
    char quantifier = '\0';

    if (pattern_index >= pattern_len) {
        if (end_index != NULL) *end_index = input_index;
        return true;
    }
    if (pattern_index == 0 && pattern[pattern_index] == '^') {
        return zt_regex_match_here(pattern, pattern_len, pattern_index + 1, input, input_len, input_index, end_index);
    }
    if (pattern[pattern_index] == '$' && pattern_index + 1 == pattern_len) {
        if (input_index == input_len) {
            if (end_index != NULL) *end_index = input_index;
            return true;
        }
        return false;
    }
    if (!zt_regex_parse_atom(pattern, pattern_len, pattern_index, &atom, &atom_end)) {
        return false;
    }

    after_quantifier = atom_end;
    if (atom_end < pattern_len && zt_regex_is_quantifier(pattern[atom_end])) {
        quantifier = pattern[atom_end];
        after_quantifier = atom_end + 1;
    }

    if (quantifier == '\0') {
        if (input_index >= input_len || !zt_regex_atom_matches(&atom, input[input_index])) {
            return false;
        }
        return zt_regex_match_here(pattern, pattern_len, after_quantifier, input, input_len, input_index + 1, end_index);
    }

    if (quantifier == '?') {
        if (input_index < input_len &&
                zt_regex_atom_matches(&atom, input[input_index]) &&
                zt_regex_match_here(pattern, pattern_len, after_quantifier, input, input_len, input_index + 1, end_index)) {
            return true;
        }
        return zt_regex_match_here(pattern, pattern_len, after_quantifier, input, input_len, input_index, end_index);
    }

    if (quantifier == '*' || quantifier == '+') {
        size_t count = 0;
        size_t cursor = input_index;
        size_t min_count = quantifier == '+' ? 1 : 0;

        while (cursor < input_len && zt_regex_atom_matches(&atom, input[cursor])) {
            count += 1;
            cursor += 1;
        }
        if (count < min_count) {
            return false;
        }
        while (count >= min_count) {
            if (zt_regex_match_here(pattern, pattern_len, after_quantifier, input, input_len, input_index + count, end_index)) {
                return true;
            }
            if (count == 0) {
                break;
            }
            count -= 1;
        }
        return false;
    }

    return false;
}

static zt_bool zt_regex_match_from(
        const char *pattern,
        size_t pattern_len,
        const char *input,
        size_t input_len,
        size_t start_index,
        size_t *end_index) {
    if (pattern_len > 0 && pattern[0] == '^' && start_index != 0) {
        return false;
    }
    return zt_regex_match_here(pattern, pattern_len, 0, input, input_len, start_index, end_index);
}

static zt_bool zt_regex_search_from(
        const char *pattern,
        size_t pattern_len,
        const char *input,
        size_t input_len,
        size_t start_index,
        size_t *match_start,
        size_t *match_end) {
    size_t cursor;

    if (pattern_len > 0 && pattern[0] == '^') {
        size_t end_index = 0;
        if (start_index > 0) {
            return false;
        }
        if (zt_regex_match_from(pattern, pattern_len, input, input_len, 0, &end_index)) {
            if (match_start != NULL) *match_start = 0;
            if (match_end != NULL) *match_end = end_index;
            return true;
        }
        return false;
    }

    for (cursor = start_index; cursor <= input_len; cursor += 1) {
        size_t end_index = 0;
        if (zt_regex_match_from(pattern, pattern_len, input, input_len, cursor, &end_index)) {
            if (match_start != NULL) *match_start = cursor;
            if (match_end != NULL) *match_end = end_index;
            return true;
        }
    }

    return false;
}

static void zt_regex_append_bytes(
        char **buffer,
        size_t *length,
        size_t *capacity,
        const char *data,
        size_t data_len) {
    size_t required;

    if (data_len == 0) {
        return;
    }

    if (*length > SIZE_MAX - data_len) {
        zt_runtime_error(ZT_ERR_PLATFORM, "regex output is too large");
    }
    required = *length + data_len;
    if (required > *capacity) {
        size_t next_capacity = *capacity == 0 ? 32 : *capacity;
        char *next_buffer;

        while (next_capacity < required) {
            if (next_capacity > SIZE_MAX / 2) {
                next_capacity = required;
                break;
            }
            next_capacity *= 2;
        }

        next_buffer = (char *)realloc(*buffer, next_capacity);
        if (next_buffer == NULL) {
            zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate regex output");
        }
        *buffer = next_buffer;
        *capacity = next_capacity;
    }

    memcpy(*buffer + *length, data, data_len);
    *length = required;
}

static void zt_regex_append_char(
        char **buffer,
        size_t *length,
        size_t *capacity,
        char ch) {
    zt_regex_append_bytes(buffer, length, capacity, &ch, 1);
}

static zt_bool zt_regex_escape_requires_backslash(char ch) {
    switch (ch) {
        case '.':
        case '^':
        case '$':
        case '*':
        case '+':
        case '?':
        case '[':
        case ']':
        case '\\':
            return true;
        default:
            return false;
    }
}

zt_outcome_void_core_error zt_regex_validate_core(const zt_text *pattern) {
    const char *message = NULL;

    zt_runtime_require_text(pattern, "zt_regex_validate_core requires pattern text");
    if (zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return zt_outcome_void_core_error_success();
    }
    return zt_outcome_void_core_error_failure(
        zt_core_error_from_message("regex.invalid_pattern", message));
}

zt_bool zt_regex_is_match_core(const zt_text *pattern, const zt_text *input) {
    const char *message = NULL;
    size_t match_start = 0;
    size_t match_end = 0;

    zt_runtime_require_text(pattern, "zt_regex_is_match_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_is_match_core requires input text");
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return false;
    }
    return zt_regex_search_from(pattern->data, pattern->len, input->data, input->len, 0, &match_start, &match_end);
}

zt_bool zt_regex_full_match_core(const zt_text *pattern, const zt_text *input) {
    const char *message = NULL;
    size_t match_end = 0;

    zt_runtime_require_text(pattern, "zt_regex_full_match_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_full_match_core requires input text");
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return false;
    }
    return (zt_bool)(zt_regex_match_from(pattern->data, pattern->len, input->data, input->len, 0, &match_end) &&
        match_end == input->len);
}

zt_optional_text zt_regex_first_core(const zt_text *pattern, const zt_text *input) {
    const char *message = NULL;
    size_t match_start = 0;
    size_t match_end = 0;
    zt_text *match_text;
    zt_optional_text result;

    zt_runtime_require_text(pattern, "zt_regex_first_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_first_core requires input text");
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return zt_optional_text_empty();
    }
    if (!zt_regex_search_from(pattern->data, pattern->len, input->data, input->len, 0, &match_start, &match_end)) {
        return zt_optional_text_empty();
    }

    match_text = zt_text_from_utf8(input->data + match_start, match_end - match_start);
    result = zt_optional_text_present(match_text);
    zt_release(match_text);
    return result;
}

zt_int zt_regex_count_core(const zt_text *pattern, const zt_text *input) {
    const char *message = NULL;
    size_t cursor = 0;
    zt_int count = 0;

    zt_runtime_require_text(pattern, "zt_regex_count_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_count_core requires input text");
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return 0;
    }

    while (cursor <= input->len) {
        size_t match_start = 0;
        size_t match_end = 0;

        if (!zt_regex_search_from(pattern->data, pattern->len, input->data, input->len, cursor, &match_start, &match_end)) {
            break;
        }
        if (match_end <= match_start) {
            cursor = match_start + 1;
            continue;
        }
        count += 1;
        cursor = match_end;
    }

    return count;
}

zt_list_text *zt_regex_find_all_core(const zt_text *pattern, const zt_text *input) {
    const char *message = NULL;
    zt_list_text *matches;
    size_t cursor = 0;

    zt_runtime_require_text(pattern, "zt_regex_find_all_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_find_all_core requires input text");

    matches = zt_list_text_new();
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return matches;
    }

    while (cursor <= input->len) {
        size_t match_start = 0;
        size_t match_end = 0;
        zt_text *match_text;

        if (!zt_regex_search_from(pattern->data, pattern->len, input->data, input->len, cursor, &match_start, &match_end)) {
            break;
        }
        if (match_end <= match_start) {
            cursor = match_start + 1;
            continue;
        }
        match_text = zt_text_from_utf8(input->data + match_start, match_end - match_start);
        zt_list_text_push(matches, match_text);
        zt_release(match_text);
        cursor = match_end;
    }

    return matches;
}

zt_list_text *zt_regex_split_core(const zt_text *pattern, const zt_text *input) {
    const char *message = NULL;
    zt_list_text *parts;
    size_t cursor = 0;
    size_t segment_start = 0;

    zt_runtime_require_text(pattern, "zt_regex_split_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_split_core requires input text");

    parts = zt_list_text_new();
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        zt_text *whole = zt_text_from_utf8(input->data, input->len);
        zt_list_text_push(parts, whole);
        zt_release(whole);
        return parts;
    }

    while (cursor <= input->len) {
        size_t match_start = 0;
        size_t match_end = 0;
        zt_text *part;

        if (!zt_regex_search_from(pattern->data, pattern->len, input->data, input->len, cursor, &match_start, &match_end)) {
            break;
        }
        if (match_end <= match_start) {
            cursor = match_start + 1;
            continue;
        }

        part = zt_text_from_utf8(input->data + segment_start, match_start - segment_start);
        zt_list_text_push(parts, part);
        zt_release(part);

        cursor = match_end;
        segment_start = match_end;
    }

    {
        zt_text *tail = zt_text_from_utf8(input->data + segment_start, input->len - segment_start);
        zt_list_text_push(parts, tail);
        zt_release(tail);
    }

    return parts;
}

zt_text *zt_regex_replace_all_core(const zt_text *pattern, const zt_text *input, const zt_text *replacement) {
    const char *message = NULL;
    char *buffer = NULL;
    size_t length = 0;
    size_t capacity = 0;
    size_t cursor = 0;
    size_t segment_start = 0;
    zt_text *result;

    zt_runtime_require_text(pattern, "zt_regex_replace_all_core requires pattern text");
    zt_runtime_require_text(input, "zt_regex_replace_all_core requires input text");
    zt_runtime_require_text(replacement, "zt_regex_replace_all_core requires replacement text");
    if (!zt_regex_validate_pattern_data(pattern->data, pattern->len, &message)) {
        return zt_text_from_utf8(input->data, input->len);
    }

    while (cursor <= input->len) {
        size_t match_start = 0;
        size_t match_end = 0;

        if (!zt_regex_search_from(pattern->data, pattern->len, input->data, input->len, cursor, &match_start, &match_end)) {
            break;
        }
        if (match_end <= match_start) {
            if (cursor < input->len) {
                zt_regex_append_bytes(&buffer, &length, &capacity, input->data + cursor, 1);
                cursor += 1;
                segment_start = cursor;
                continue;
            }
            break;
        }

        zt_regex_append_bytes(&buffer, &length, &capacity, input->data + segment_start, match_start - segment_start);
        zt_regex_append_bytes(&buffer, &length, &capacity, replacement->data, replacement->len);

        cursor = match_end;
        segment_start = match_end;
    }

    zt_regex_append_bytes(&buffer, &length, &capacity, input->data + segment_start, input->len - segment_start);
    result = zt_text_from_utf8(buffer, length);
    free(buffer);
    return result;
}

zt_text *zt_regex_escape_core(const zt_text *input) {
    char *buffer = NULL;
    size_t length = 0;
    size_t capacity = 0;
    size_t index;
    zt_text *result;

    zt_runtime_require_text(input, "zt_regex_escape_core requires input text");

    for (index = 0; index < input->len; index += 1) {
        char ch = input->data[index];
        if (zt_regex_escape_requires_backslash(ch)) {
            zt_regex_append_char(&buffer, &length, &capacity, '\\');
        }
        zt_regex_append_char(&buffer, &length, &capacity, ch);
    }

    result = zt_text_from_utf8(buffer, length);
    free(buffer);
    return result;
}

zt_bytes *zt_bytes_empty(void) {
    return zt_bytes_from_array(NULL, 0);
}

zt_bytes *zt_bytes_from_array(const uint8_t *data, size_t len) {
    zt_bytes *value;

    value = (zt_bytes *)calloc(1, sizeof(zt_bytes));
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate bytes header");
    }

    if (len > 0) {
        value->data = (uint8_t *)malloc(len);
        if (value->data == NULL) {
            free(value);
            zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate bytes data");
        }
        if (data != NULL) {
            memcpy(value->data, data, len);
        } else {
            memset(value->data, 0, len);
        }
    }

    value->len = len;
    value->header.rc = 1;
    value->header.kind = (uint32_t)ZT_HEAP_BYTES;
    return value;
}

zt_bytes *zt_bytes_from_list_i64(const zt_list_i64 *values) {
    zt_bytes *bytes_value;
    size_t index;

    zt_runtime_require_list_i64(values, "zt_bytes_from_list_i64 requires list<int>");

    if (values->len == 0) {
        return zt_bytes_from_array(NULL, 0);
    }

    bytes_value = zt_bytes_from_array(NULL, values->len);
    for (index = 0; index < values->len; index += 1) {
        zt_int item = values->data[index];
        if (item < 0 || item > 255) {
            zt_release(bytes_value);
            zt_runtime_error(ZT_ERR_CHECK, "std.bytes.from_list expects items in the 0..255 range");
        }
        bytes_value->data[index] = (uint8_t)item;
    }

    return bytes_value;
}

zt_outcome_bytes_core_error zt_bytes_from_list_i64_result(const zt_list_i64 *values) {
    zt_bytes *bytes_value;
    size_t index;
    zt_outcome_bytes_core_error outcome;

    zt_runtime_require_list_i64(values, "zt_bytes_from_list_i64_result requires list<int>");

    for (index = 0; index < values->len; index += 1) {
        if (values->data[index] < 0 || values->data[index] > 255) {
            return zt_outcome_bytes_core_error_failure_message("bytes.from_list value must be in 0..255");
        }
    }

    bytes_value = zt_bytes_from_list_i64(values);
    outcome = zt_outcome_bytes_core_error_success(bytes_value);
    zt_release(bytes_value);
    return outcome;
}

zt_list_i64 *zt_bytes_to_list_i64(const zt_bytes *value) {
    zt_list_i64 *list;
    size_t index;

    zt_runtime_require_bytes(value, "zt_bytes_to_list_i64 requires bytes");

    list = zt_list_i64_new();
    if (value->len == 0) {
        return list;
    }

    zt_list_i64_reserve(list, value->len);
    for (index = 0; index < value->len; index += 1) {
        list->data[index] = (zt_int)value->data[index];
    }
    list->len = value->len;

    return list;
}

zt_bytes *zt_bytes_join(const zt_bytes *left, const zt_bytes *right) {
    zt_bytes *joined;
    size_t total_len;

    zt_runtime_require_bytes(left, "zt_bytes_join requires left bytes");
    zt_runtime_require_bytes(right, "zt_bytes_join requires right bytes");

    total_len = zt_require_added_size(left->len, right->len, "bytes join size overflow");
    joined = zt_bytes_from_array(NULL, total_len);
    if (left->len > 0) {
        memcpy(joined->data, left->data, left->len);
    }
    if (right->len > 0) {
        memcpy(joined->data + left->len, right->data, right->len);
    }

    return joined;
}

zt_bool zt_bytes_starts_with(const zt_bytes *value, const zt_bytes *prefix) {
    zt_runtime_require_bytes(value, "zt_bytes_starts_with requires bytes value");
    zt_runtime_require_bytes(prefix, "zt_bytes_starts_with requires bytes prefix");

    if (prefix->len > value->len) {
        return false;
    }

    if (prefix->len == 0) {
        return true;
    }

    return memcmp(value->data, prefix->data, prefix->len) == 0;
}

zt_bool zt_bytes_ends_with(const zt_bytes *value, const zt_bytes *suffix) {
    zt_runtime_require_bytes(value, "zt_bytes_ends_with requires bytes value");
    zt_runtime_require_bytes(suffix, "zt_bytes_ends_with requires bytes suffix");

    if (suffix->len > value->len) {
        return false;
    }

    if (suffix->len == 0) {
        return true;
    }

    return memcmp(value->data + (value->len - suffix->len), suffix->data, suffix->len) == 0;
}

zt_bool zt_bytes_contains(const zt_bytes *value, const zt_bytes *part) {
    size_t index;

    zt_runtime_require_bytes(value, "zt_bytes_contains requires bytes value");
    zt_runtime_require_bytes(part, "zt_bytes_contains requires bytes part");

    if (part->len == 0) {
        return true;
    }

    if (part->len > value->len) {
        return false;
    }

    for (index = 0; index + part->len <= value->len; index += 1) {
        if (memcmp(value->data + index, part->data, part->len) == 0) {
            return true;
        }
    }

    return false;
}
zt_bytes *zt_text_to_utf8_bytes(const zt_text *value) {
    zt_runtime_require_text(value, "zt_text_to_utf8_bytes requires text");
    return zt_bytes_from_array((const uint8_t *)value->data, value->len);
}

zt_outcome_text_text zt_text_from_utf8_bytes(const zt_bytes *value) {
    size_t error_index;
    const char *error_reason;
    char message[192];
    zt_text *text;
    zt_outcome_text_text outcome;

    zt_runtime_require_bytes(value, "zt_text_from_utf8_bytes requires bytes");

    if (!zt_utf8_validate(value->data, value->len, &error_index, &error_reason)) {
        snprintf(
            message,
            sizeof(message),
            "invalid UTF-8 at byte %llu: %s",
            (unsigned long long)error_index,
            error_reason != NULL ? error_reason : "malformed sequence"
        );
        return zt_outcome_text_text_failure_message(message);
    }

    text = zt_text_from_utf8((const char *)value->data, value->len);
    outcome = zt_outcome_text_text_success(text);
    zt_release(text);
    return outcome;
}

zt_int zt_bytes_len(const zt_bytes *value) {
    zt_runtime_require_bytes(value, "zt_bytes_len requires bytes");
    return (zt_int)value->len;
}

uint8_t zt_bytes_get(const zt_bytes *value, zt_int index_0) {
    zt_runtime_require_bytes(value, "zt_bytes_get requires bytes");

    if (index_0 < 0 || (size_t)index_0 >= value->len) {
        zt_runtime_error(ZT_ERR_INDEX, "bytes index out of bounds");
    }

    return value->data[(size_t)index_0];
}

zt_bytes *zt_bytes_slice(const zt_bytes *value, zt_int start_0, zt_int end_0) {
    size_t start_pos;
    size_t end_pos;

    zt_runtime_require_bytes(value, "zt_bytes_slice requires bytes");

    if (start_0 < 0) {
        zt_runtime_error(ZT_ERR_INDEX, "slice start must be >= 0");
    }

    if (value->len == 0) {
        return zt_bytes_from_array(NULL, 0);
    }

    start_pos = (size_t)start_0;
    end_pos = zt_normalize_slice_end(value->len, end_0);

    if (start_pos >= value->len || end_pos < start_pos) {
        return zt_bytes_from_array(NULL, 0);
    }

    return zt_bytes_from_array(value->data + start_pos, end_pos - start_pos + 1);
}

zt_optional_i64 zt_bytes_get_optional(const zt_bytes *value, zt_int index_0) {
    zt_runtime_require_bytes(value, "zt_bytes_get_optional requires bytes");
    if (index_0 < 0 || (size_t)index_0 >= value->len) {
        return zt_optional_i64_empty();
    }
    return zt_optional_i64_present((zt_int)value->data[index_0]);
}

zt_bytes *zt_bytes_slice_clamped(const zt_bytes *value, zt_int start_0, zt_int end_0) {
    zt_int start = start_0;
    zt_int end = end_0;

    zt_runtime_require_bytes(value, "zt_bytes_slice_clamped requires bytes");

    if (value->len == 0) {
        return zt_bytes_empty();
    }
    if (start < 0) {
        start = 0;
    }
    if (end < start) {
        return zt_bytes_empty();
    }
    if ((size_t)start >= value->len) {
        return zt_bytes_empty();
    }
    if ((size_t)end >= value->len) {
        end = (zt_int)value->len - 1;
    }

    return zt_bytes_slice(value, start, end);
}

zt_optional_i64 zt_bytes_index_of(const zt_bytes *value, const zt_bytes *part) {
    size_t index;

    zt_runtime_require_bytes(value, "zt_bytes_index_of requires bytes value");
    zt_runtime_require_bytes(part, "zt_bytes_index_of requires bytes part");

    if (part->len == 0) {
        return zt_optional_i64_present(0);
    }
    if (part->len > value->len) {
        return zt_optional_i64_empty();
    }

    for (index = 0; index + part->len <= value->len; index += 1) {
        if (memcmp(value->data + index, part->data, part->len) == 0) {
            return zt_optional_i64_present((zt_int)index);
        }
    }

    return zt_optional_i64_empty();
}

/* zt_list_i64: new, from_array, push, push_owned, get, set, set_owned, len, slice
 * generated by ZT_DEFINE_LIST_IMPL(i64, zt_int, ZT_HEAP_LIST_I64, 0) */

zt_optional_i64 zt_list_i64_get_optional(const zt_list_i64 *list, zt_int index_0) {
    zt_runtime_require_list_i64(list, "zt_list_i64_get_optional requires list");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        return zt_optional_i64_empty();
    }
    return zt_optional_i64_present(list->data[index_0]);
}

zt_optional_i64 zt_list_i64_last_optional(const zt_list_i64 *list) {
    zt_runtime_require_list_i64(list, "zt_list_i64_last_optional requires list");
    if (list->len == 0) {
        return zt_optional_i64_empty();
    }
    return zt_optional_i64_present(list->data[list->len - 1]);
}

zt_list_i64 *zt_list_i64_rest(const zt_list_i64 *list) {
    zt_runtime_require_list_i64(list, "zt_list_i64_rest requires list");
    if (list->len <= 1) {
        return zt_list_i64_new();
    }
    return zt_list_i64_slice(list, 1, (zt_int)list->len - 1);
}

zt_list_i64 *zt_list_i64_skip(const zt_list_i64 *list, zt_int count) {
    zt_runtime_require_list_i64(list, "zt_list_i64_skip requires list");
    if (count <= 0) {
        if (list->len == 0) return zt_list_i64_new();
        return zt_list_i64_slice(list, 0, (zt_int)list->len - 1);
    }
    if ((size_t)count >= list->len) {
        return zt_list_i64_new();
    }
    return zt_list_i64_slice(list, count, (zt_int)list->len - 1);
}

zt_list_i64 *zt_list_i64_append(const zt_list_i64 *list, zt_int value) {
    zt_list_i64 *copy;

    zt_runtime_require_list_i64(list, "zt_list_i64_append requires list");

    copy = zt_list_i64_from_array(list->data, list->len);
    zt_list_i64_push(copy, value);
    return copy;
}

zt_list_i64 *zt_list_i64_prepend(const zt_list_i64 *list, zt_int value) {
    zt_list_i64 *copy;
    size_t index;

    zt_runtime_require_list_i64(list, "zt_list_i64_prepend requires list");

    copy = zt_list_i64_new();
    zt_list_i64_reserve(copy, zt_require_added_size(list->len, 1, "list prepend size overflow"));
    zt_list_i64_push(copy, value);
    for (index = 0; index < list->len; index += 1) {
        zt_list_i64_push(copy, list->data[index]);
    }
    return copy;
}

zt_bool zt_list_i64_contains(const zt_list_i64 *list, zt_int value) {
    size_t index;

    zt_runtime_require_list_i64(list, "zt_list_i64_contains requires list");

    for (index = 0; index < list->len; index += 1) {
        if (list->data[index] == value) return true;
    }
    return false;
}

zt_list_i64 *zt_list_i64_reverse(const zt_list_i64 *list) {
    zt_list_i64 *copy;
    size_t remaining;

    zt_runtime_require_list_i64(list, "zt_list_i64_reverse requires list");

    copy = zt_list_i64_new();
    zt_list_i64_reserve(copy, list->len);
    remaining = list->len;
    while (remaining > 0) {
        remaining -= 1;
        zt_list_i64_push(copy, list->data[remaining]);
    }
    return copy;
}

zt_list_i64 *zt_list_i64_concat(const zt_list_i64 *left, const zt_list_i64 *right) {
    zt_list_i64 *copy;
    size_t index;

    zt_runtime_require_list_i64(left, "zt_list_i64_concat requires left list");
    zt_runtime_require_list_i64(right, "zt_list_i64_concat requires right list");

    copy = zt_list_i64_new();
    zt_list_i64_reserve(copy, zt_require_added_size(left->len, right->len, "list concat size overflow"));
    for (index = 0; index < left->len; index += 1) {
        zt_list_i64_push(copy, left->data[index]);
    }
    for (index = 0; index < right->len; index += 1) {
        zt_list_i64_push(copy, right->data[index]);
    }
    return copy;
}

zt_optional_i64 zt_list_i64_index_of(const zt_list_i64 *list, zt_int value) {
    size_t index;

    zt_runtime_require_list_i64(list, "zt_list_i64_index_of requires list");

    for (index = 0; index < list->len; index += 1) {
        if (list->data[index] == value) return zt_optional_i64_present((zt_int)index);
    }
    return zt_optional_i64_empty();
}

static void zt_runtime_require_closure_value(zt_closure *closure, const char *message) {
    if (closure == NULL || closure->fn == NULL) {
        zt_runtime_error(ZT_ERR_CONTRACT, message);
    }
}

#define ZT_DEFINE_VALUE_LIST_HOF_IMPL(SUFFIX, ELEM_TYPE) \
zt_list_##SUFFIX *zt_list_##SUFFIX##_map(const zt_list_##SUFFIX *list, zt_closure *mapper) { \
    zt_list_##SUFFIX *out; \
    size_t index; \
    ELEM_TYPE (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_map requires list"); \
    zt_runtime_require_closure_value(mapper, "zt_list_" #SUFFIX "_map requires mapper"); \
    fn = (ELEM_TYPE (*)(void *, ELEM_TYPE))mapper->fn; \
    out = zt_list_##SUFFIX##_new(); \
    zt_list_##SUFFIX##_reserve(out, list->len); \
    for (index = 0; index < list->len; index += 1) { \
        zt_list_##SUFFIX##_push(out, fn(mapper->ctx, list->data[index])); \
    } \
    return out; \
} \
zt_list_##SUFFIX *zt_list_##SUFFIX##_filter(const zt_list_##SUFFIX *list, zt_closure *predicate) { \
    zt_list_##SUFFIX *out; \
    size_t index; \
    zt_bool (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_filter requires list"); \
    zt_runtime_require_closure_value(predicate, "zt_list_" #SUFFIX "_filter requires predicate"); \
    fn = (zt_bool (*)(void *, ELEM_TYPE))predicate->fn; \
    out = zt_list_##SUFFIX##_new(); \
    zt_list_##SUFFIX##_reserve(out, list->len); \
    for (index = 0; index < list->len; index += 1) { \
        ELEM_TYPE value = list->data[index]; \
        if (fn(predicate->ctx, value)) { \
            zt_list_##SUFFIX##_push(out, value); \
        } \
    } \
    return out; \
} \
zt_optional_##SUFFIX zt_list_##SUFFIX##_find(const zt_list_##SUFFIX *list, zt_closure *predicate) { \
    size_t index; \
    zt_bool (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_find requires list"); \
    zt_runtime_require_closure_value(predicate, "zt_list_" #SUFFIX "_find requires predicate"); \
    fn = (zt_bool (*)(void *, ELEM_TYPE))predicate->fn; \
    for (index = 0; index < list->len; index += 1) { \
        ELEM_TYPE value = list->data[index]; \
        if (fn(predicate->ctx, value)) { \
            return zt_optional_##SUFFIX##_present(value); \
        } \
    } \
    return zt_optional_##SUFFIX##_empty(); \
} \
zt_bool zt_list_##SUFFIX##_any(const zt_list_##SUFFIX *list, zt_closure *predicate) { \
    size_t index; \
    zt_bool (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_any requires list"); \
    zt_runtime_require_closure_value(predicate, "zt_list_" #SUFFIX "_any requires predicate"); \
    fn = (zt_bool (*)(void *, ELEM_TYPE))predicate->fn; \
    for (index = 0; index < list->len; index += 1) { \
        if (fn(predicate->ctx, list->data[index])) return true; \
    } \
    return false; \
} \
zt_bool zt_list_##SUFFIX##_all(const zt_list_##SUFFIX *list, zt_closure *predicate) { \
    size_t index; \
    zt_bool (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_all requires list"); \
    zt_runtime_require_closure_value(predicate, "zt_list_" #SUFFIX "_all requires predicate"); \
    fn = (zt_bool (*)(void *, ELEM_TYPE))predicate->fn; \
    for (index = 0; index < list->len; index += 1) { \
        if (!fn(predicate->ctx, list->data[index])) return false; \
    } \
    return true; \
} \
zt_int zt_list_##SUFFIX##_count(const zt_list_##SUFFIX *list, zt_closure *predicate) { \
    zt_int total = 0; \
    size_t index; \
    zt_bool (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_count requires list"); \
    zt_runtime_require_closure_value(predicate, "zt_list_" #SUFFIX "_count requires predicate"); \
    fn = (zt_bool (*)(void *, ELEM_TYPE))predicate->fn; \
    for (index = 0; index < list->len; index += 1) { \
        if (fn(predicate->ctx, list->data[index])) total += 1; \
    } \
    return total; \
} \
zt_list_##SUFFIX *zt_list_##SUFFIX##_sort_by(const zt_list_##SUFFIX *list, zt_closure *key_selector) { \
    zt_list_##SUFFIX *out; \
    size_t index; \
    zt_int (*fn)(void *, ELEM_TYPE); \
    if (list == NULL) zt_runtime_error(ZT_ERR_PANIC, "zt_list_" #SUFFIX "_sort_by requires list"); \
    zt_runtime_require_closure_value(key_selector, "zt_list_" #SUFFIX "_sort_by requires key selector"); \
    fn = (zt_int (*)(void *, ELEM_TYPE))key_selector->fn; \
    out = zt_list_##SUFFIX##_from_array(list->data, list->len); \
    for (index = 1; index < out->len; index += 1) { \
        ELEM_TYPE value = out->data[index]; \
        zt_int key = fn(key_selector->ctx, value); \
        size_t pos = index; \
        while (pos > 0 && fn(key_selector->ctx, out->data[pos - 1]) > key) { \
            out->data[pos] = out->data[pos - 1]; \
            pos -= 1; \
        } \
        out->data[pos] = value; \
    } \
    return out; \
}

ZT_DEFINE_VALUE_LIST_HOF_IMPL(f64, zt_float)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(bool, zt_bool)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(i8, int8_t)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(i16, int16_t)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(i32, int32_t)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(u8, uint8_t)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(u16, uint16_t)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(u32, uint32_t)
ZT_DEFINE_VALUE_LIST_HOF_IMPL(u64, uint64_t)

#undef ZT_DEFINE_VALUE_LIST_HOF_IMPL

zt_list_i64 *zt_list_i64_map(const zt_list_i64 *list, zt_closure *mapper) {
    zt_list_i64 *out;
    size_t index;
    zt_int (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_map requires list");
    zt_runtime_require_closure_value(mapper, "zt_list_i64_map requires mapper");

    fn = (zt_int (*)(void *, zt_int))mapper->fn;
    out = zt_list_i64_new();
    zt_list_i64_reserve(out, list->len);
    for (index = 0; index < list->len; index += 1) {
        zt_list_i64_push(out, fn(mapper->ctx, list->data[index]));
    }
    return out;
}

zt_list_i64 *zt_list_i64_filter(const zt_list_i64 *list, zt_closure *predicate) {
    zt_list_i64 *out;
    size_t index;
    zt_bool (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_filter requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_i64_filter requires predicate");

    fn = (zt_bool (*)(void *, zt_int))predicate->fn;
    out = zt_list_i64_new();
    zt_list_i64_reserve(out, list->len);
    for (index = 0; index < list->len; index += 1) {
        zt_int value = list->data[index];
        if (fn(predicate->ctx, value)) {
            zt_list_i64_push(out, value);
        }
    }
    return out;
}

zt_int zt_list_i64_reduce(const zt_list_i64 *list, zt_int initial, zt_closure *reducer) {
    zt_int acc = initial;
    size_t index;
    zt_int (*fn)(void *, zt_int, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_reduce requires list");
    zt_runtime_require_closure_value(reducer, "zt_list_i64_reduce requires reducer");

    fn = (zt_int (*)(void *, zt_int, zt_int))reducer->fn;
    for (index = 0; index < list->len; index += 1) {
        acc = fn(reducer->ctx, acc, list->data[index]);
    }
    return acc;
}

zt_optional_i64 zt_list_i64_find(const zt_list_i64 *list, zt_closure *predicate) {
    size_t index;
    zt_bool (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_find requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_i64_find requires predicate");

    fn = (zt_bool (*)(void *, zt_int))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        zt_int value = list->data[index];
        if (fn(predicate->ctx, value)) {
            return zt_optional_i64_present(value);
        }
    }
    return zt_optional_i64_empty();
}

zt_bool zt_list_i64_any(const zt_list_i64 *list, zt_closure *predicate) {
    size_t index;
    zt_bool (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_any requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_i64_any requires predicate");

    fn = (zt_bool (*)(void *, zt_int))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        if (fn(predicate->ctx, list->data[index])) return true;
    }
    return false;
}

zt_bool zt_list_i64_all(const zt_list_i64 *list, zt_closure *predicate) {
    size_t index;
    zt_bool (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_all requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_i64_all requires predicate");

    fn = (zt_bool (*)(void *, zt_int))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        if (!fn(predicate->ctx, list->data[index])) return false;
    }
    return true;
}

zt_int zt_list_i64_count(const zt_list_i64 *list, zt_closure *predicate) {
    zt_int total = 0;
    size_t index;
    zt_bool (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_count requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_i64_count requires predicate");

    fn = (zt_bool (*)(void *, zt_int))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        if (fn(predicate->ctx, list->data[index])) {
            total += 1;
        }
    }
    return total;
}

zt_list_i64 *zt_list_i64_sort_by(const zt_list_i64 *list, zt_closure *key_selector) {
    zt_list_i64 *out;
    size_t index;
    zt_int (*fn)(void *, zt_int);

    zt_runtime_require_list_i64(list, "zt_list_i64_sort_by requires list");
    zt_runtime_require_closure_value(key_selector, "zt_list_i64_sort_by requires key selector");

    fn = (zt_int (*)(void *, zt_int))key_selector->fn;
    out = zt_list_i64_from_array(list->data, list->len);
    for (index = 1; index < out->len; index += 1) {
        zt_int value = out->data[index];
        zt_int key = fn(key_selector->ctx, value);
        size_t pos = index;
        while (pos > 0 && fn(key_selector->ctx, out->data[pos - 1]) > key) {
            out->data[pos] = out->data[pos - 1];
            pos -= 1;
        }
        out->data[pos] = value;
    }
    return out;
}

zt_list_text *zt_list_text_map(const zt_list_text *list, zt_closure *mapper) {
    zt_list_text *out;
    size_t index;
    zt_text *(*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_map requires list");
    zt_runtime_require_closure_value(mapper, "zt_list_text_map requires mapper");

    fn = (zt_text *(*)(void *, zt_text *))mapper->fn;
    out = zt_list_text_new();
    zt_list_text_reserve(out, list->len);
    for (index = 0; index < list->len; index += 1) {
        zt_text *mapped = fn(mapper->ctx, list->data[index]);
        zt_runtime_require_text(mapped, "zt_list_text_map mapper returned null text");
        zt_list_text_push(out, mapped);
        zt_release(mapped);
    }
    return out;
}

zt_list_text *zt_list_text_filter(const zt_list_text *list, zt_closure *predicate) {
    zt_list_text *out;
    size_t index;
    zt_bool (*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_filter requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_text_filter requires predicate");

    fn = (zt_bool (*)(void *, zt_text *))predicate->fn;
    out = zt_list_text_new();
    zt_list_text_reserve(out, list->len);
    for (index = 0; index < list->len; index += 1) {
        zt_text *value = list->data[index];
        zt_runtime_require_text(value, "list<text> entry cannot be null");
        if (fn(predicate->ctx, value)) {
            zt_list_text_push(out, value);
        }
    }
    return out;
}

zt_optional_text zt_list_text_find(const zt_list_text *list, zt_closure *predicate) {
    size_t index;
    zt_bool (*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_find requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_text_find requires predicate");

    fn = (zt_bool (*)(void *, zt_text *))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        zt_text *value = list->data[index];
        zt_runtime_require_text(value, "list<text> entry cannot be null");
        if (fn(predicate->ctx, value)) {
            return zt_optional_text_present(value);
        }
    }
    return zt_optional_text_empty();
}

zt_bool zt_list_text_any(const zt_list_text *list, zt_closure *predicate) {
    size_t index;
    zt_bool (*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_any requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_text_any requires predicate");

    fn = (zt_bool (*)(void *, zt_text *))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        zt_text *value = list->data[index];
        zt_runtime_require_text(value, "list<text> entry cannot be null");
        if (fn(predicate->ctx, value)) return true;
    }
    return false;
}

zt_bool zt_list_text_all(const zt_list_text *list, zt_closure *predicate) {
    size_t index;
    zt_bool (*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_all requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_text_all requires predicate");

    fn = (zt_bool (*)(void *, zt_text *))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        zt_text *value = list->data[index];
        zt_runtime_require_text(value, "list<text> entry cannot be null");
        if (!fn(predicate->ctx, value)) return false;
    }
    return true;
}

zt_int zt_list_text_count(const zt_list_text *list, zt_closure *predicate) {
    zt_int total = 0;
    size_t index;
    zt_bool (*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_count requires list");
    zt_runtime_require_closure_value(predicate, "zt_list_text_count requires predicate");

    fn = (zt_bool (*)(void *, zt_text *))predicate->fn;
    for (index = 0; index < list->len; index += 1) {
        zt_text *value = list->data[index];
        zt_runtime_require_text(value, "list<text> entry cannot be null");
        if (fn(predicate->ctx, value)) {
            total += 1;
        }
    }
    return total;
}

zt_list_text *zt_list_text_sort_by(const zt_list_text *list, zt_closure *key_selector) {
    zt_list_text *out;
    size_t index;
    zt_int (*fn)(void *, zt_text *);

    zt_runtime_require_list_text(list, "zt_list_text_sort_by requires list");
    zt_runtime_require_closure_value(key_selector, "zt_list_text_sort_by requires key selector");

    fn = (zt_int (*)(void *, zt_text *))key_selector->fn;
    out = zt_list_text_from_array(list->data, list->len);
    for (index = 1; index < out->len; index += 1) {
        zt_text *value = out->data[index];
        zt_int key = fn(key_selector->ctx, value);
        size_t pos = index;
        while (pos > 0 && fn(key_selector->ctx, out->data[pos - 1]) > key) {
            out->data[pos] = out->data[pos - 1];
            pos -= 1;
        }
        out->data[pos] = value;
    }
    return out;
}

zt_outcome_list_i64_core_error zt_list_i64_set_result(const zt_list_i64 *list, zt_int index_0, zt_int value) {
    zt_list_i64 *copy;

    zt_runtime_require_list_i64(list, "zt_list_i64_set_result requires list");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        return zt_outcome_list_i64_core_error_failure_message("list.set index out of bounds");
    }

    copy = zt_list_i64_from_array(list->data, list->len);
    zt_list_i64_set(copy, index_0, value);
    return zt_outcome_list_i64_core_error_success(copy);
}

zt_outcome_list_i64_core_error zt_list_i64_remove_at(const zt_list_i64 *list, zt_int index_0) {
    zt_list_i64 *copy;
    size_t index;

    zt_runtime_require_list_i64(list, "zt_list_i64_remove_at requires list");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        return zt_outcome_list_i64_core_error_failure_message("list.remove_at index out of bounds");
    }

    copy = zt_list_i64_new();
    zt_list_i64_reserve(copy, list->len > 0 ? list->len - 1 : 0);
    for (index = 0; index < list->len; index += 1) {
        if (index != (size_t)index_0) {
            zt_list_i64_push(copy, list->data[index]);
        }
    }
    return zt_outcome_list_i64_core_error_success(copy);
}

zt_outcome_list_i64_core_error zt_list_i64_remove_first(const zt_list_i64 *list) {
    return zt_list_i64_remove_at(list, 0);
}

zt_outcome_list_i64_core_error zt_list_i64_remove_last(const zt_list_i64 *list) {
    zt_runtime_require_list_i64(list, "zt_list_i64_remove_last requires list");
    if (list->len == 0) {
        return zt_outcome_list_i64_core_error_failure_message("list.remove_last requires a non-empty list");
    }
    return zt_list_i64_remove_at(list, (zt_int)list->len - 1);
}

zt_outcome_list_i64_core_error zt_list_i64_slice_result(const zt_list_i64 *list, zt_int start_0, zt_int end_0) {
    zt_runtime_require_list_i64(list, "zt_list_i64_slice_result requires list");
    if (start_0 < 0) {
        return zt_outcome_list_i64_core_error_failure_message("list.slice start must be >= 0");
    }
    return zt_outcome_list_i64_core_error_success(zt_list_i64_slice(list, start_0, end_0));
}

#define ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(SUFFIX, ELEM_TYPE) \
zt_optional_##SUFFIX zt_list_##SUFFIX##_get_optional(const zt_list_##SUFFIX *list, zt_int index_0) { \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_get_optional requires list"); \
    if (index_0 < 0 || (size_t)index_0 >= list->len) { \
        return zt_optional_##SUFFIX##_empty(); \
    } \
    return zt_optional_##SUFFIX##_present(list->data[index_0]); \
} \
 \
zt_optional_##SUFFIX zt_list_##SUFFIX##_last_optional(const zt_list_##SUFFIX *list) { \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_last_optional requires list"); \
    if (list->len == 0) { \
        return zt_optional_##SUFFIX##_empty(); \
    } \
    return zt_optional_##SUFFIX##_present(list->data[list->len - 1]); \
} \
 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_rest(const zt_list_##SUFFIX *list) { \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_rest requires list"); \
    if (list->len <= 1) { \
        return zt_list_##SUFFIX##_new(); \
    } \
    return zt_list_##SUFFIX##_slice(list, 1, (zt_int)list->len - 1); \
} \
 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_skip(const zt_list_##SUFFIX *list, zt_int count) { \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_skip requires list"); \
    if (count <= 0) { \
        if (list->len == 0) return zt_list_##SUFFIX##_new(); \
        return zt_list_##SUFFIX##_slice(list, 0, (zt_int)list->len - 1); \
    } \
    if ((size_t)count >= list->len) { \
        return zt_list_##SUFFIX##_new(); \
    } \
    return zt_list_##SUFFIX##_slice(list, count, (zt_int)list->len - 1); \
} \
 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_append(const zt_list_##SUFFIX *list, ELEM_TYPE value) { \
    zt_list_##SUFFIX *copy; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_append requires list"); \
    copy = zt_list_##SUFFIX##_from_array(list->data, list->len); \
    zt_list_##SUFFIX##_push(copy, value); \
    return copy; \
} \
 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_prepend(const zt_list_##SUFFIX *list, ELEM_TYPE value) { \
    zt_list_##SUFFIX *copy; \
    size_t index; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_prepend requires list"); \
    copy = zt_list_##SUFFIX##_new(); \
    zt_list_##SUFFIX##_reserve(copy, zt_require_added_size(list->len, 1, "list prepend size overflow")); \
    zt_list_##SUFFIX##_push(copy, value); \
    for (index = 0; index < list->len; index += 1) { \
        zt_list_##SUFFIX##_push(copy, list->data[index]); \
    } \
    return copy; \
} \
 \
zt_bool zt_list_##SUFFIX##_contains(const zt_list_##SUFFIX *list, ELEM_TYPE value) { \
    size_t index; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_contains requires list"); \
    for (index = 0; index < list->len; index += 1) { \
        if (list->data[index] == value) return true; \
    } \
    return false; \
} \
 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_reverse(const zt_list_##SUFFIX *list) { \
    zt_list_##SUFFIX *copy; \
    size_t remaining; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_reverse requires list"); \
    copy = zt_list_##SUFFIX##_new(); \
    zt_list_##SUFFIX##_reserve(copy, list->len); \
    remaining = list->len; \
    while (remaining > 0) { \
        remaining -= 1; \
        zt_list_##SUFFIX##_push(copy, list->data[remaining]); \
    } \
    return copy; \
} \
 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_concat(const zt_list_##SUFFIX *left, const zt_list_##SUFFIX *right) { \
    zt_list_##SUFFIX *copy; \
    size_t index; \
    zt_runtime_require_list_##SUFFIX(left, "zt_list_" #SUFFIX "_concat requires left list"); \
    zt_runtime_require_list_##SUFFIX(right, "zt_list_" #SUFFIX "_concat requires right list"); \
    copy = zt_list_##SUFFIX##_new(); \
    zt_list_##SUFFIX##_reserve(copy, zt_require_added_size(left->len, right->len, "list concat size overflow")); \
    for (index = 0; index < left->len; index += 1) { \
        zt_list_##SUFFIX##_push(copy, left->data[index]); \
    } \
    for (index = 0; index < right->len; index += 1) { \
        zt_list_##SUFFIX##_push(copy, right->data[index]); \
    } \
    return copy; \
} \
 \
zt_optional_i64 zt_list_##SUFFIX##_index_of(const zt_list_##SUFFIX *list, ELEM_TYPE value) { \
    size_t index; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_index_of requires list"); \
    for (index = 0; index < list->len; index += 1) { \
        if (list->data[index] == value) return zt_optional_i64_present((zt_int)index); \
    } \
    return zt_optional_i64_empty(); \
} \
 \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_set_result(const zt_list_##SUFFIX *list, zt_int index_0, ELEM_TYPE value) { \
    zt_list_##SUFFIX *copy; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_set_result requires list"); \
    if (index_0 < 0 || (size_t)index_0 >= list->len) { \
        return zt_outcome_list_##SUFFIX##_core_error_failure_message("list.set index out of bounds"); \
    } \
    copy = zt_list_##SUFFIX##_from_array(list->data, list->len); \
    zt_list_##SUFFIX##_set(copy, index_0, value); \
    return zt_outcome_list_##SUFFIX##_core_error_success(copy); \
} \
 \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_remove_at(const zt_list_##SUFFIX *list, zt_int index_0) { \
    zt_list_##SUFFIX *copy; \
    size_t index; \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_remove_at requires list"); \
    if (index_0 < 0 || (size_t)index_0 >= list->len) { \
        return zt_outcome_list_##SUFFIX##_core_error_failure_message("list.remove_at index out of bounds"); \
    } \
    copy = zt_list_##SUFFIX##_new(); \
    zt_list_##SUFFIX##_reserve(copy, list->len > 0 ? list->len - 1 : 0); \
    for (index = 0; index < list->len; index += 1) { \
        if (index != (size_t)index_0) { \
            zt_list_##SUFFIX##_push(copy, list->data[index]); \
        } \
    } \
    return zt_outcome_list_##SUFFIX##_core_error_success(copy); \
} \
 \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_remove_first(const zt_list_##SUFFIX *list) { \
    return zt_list_##SUFFIX##_remove_at(list, 0); \
} \
 \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_remove_last(const zt_list_##SUFFIX *list) { \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_remove_last requires list"); \
    if (list->len == 0) { \
        return zt_outcome_list_##SUFFIX##_core_error_failure_message("list.remove_last requires a non-empty list"); \
    } \
    return zt_list_##SUFFIX##_remove_at(list, (zt_int)list->len - 1); \
} \
 \
zt_outcome_list_##SUFFIX##_core_error zt_list_##SUFFIX##_slice_result(const zt_list_##SUFFIX *list, zt_int start_0, zt_int end_0) { \
    zt_runtime_require_list_##SUFFIX(list, "zt_list_" #SUFFIX "_slice_result requires list"); \
    if (start_0 < 0) { \
        return zt_outcome_list_##SUFFIX##_core_error_failure_message("list.slice start must be >= 0"); \
    } \
    return zt_outcome_list_##SUFFIX##_core_error_success(zt_list_##SUFFIX##_slice(list, start_0, end_0)); \
}

ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(f64, zt_float)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(bool, zt_bool)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(i8, int8_t)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(i16, int16_t)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(i32, int32_t)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(u8, uint8_t)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(u16, uint16_t)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(u32, uint32_t)
ZT_DEFINE_PRIMITIVE_LIST_VALUE_API(u64, uint64_t)

#undef ZT_DEFINE_PRIMITIVE_LIST_VALUE_API

/* zt_list_text: new, from_array, push, push_owned, get, set, set_owned, len, slice
 * generated by ZT_DEFINE_LIST_IMPL(text, zt_text *, ZT_HEAP_LIST_TEXT, 1) */

zt_text *zt_list_text_take(zt_list_text *list, zt_int index_0) {
    zt_text *value;

    zt_runtime_require_list_text(list, "zt_list_text_take requires list");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        zt_runtime_error(ZT_ERR_INDEX, "list<text> index out of bounds");
    }

    value = list->data[index_0];
    zt_runtime_require_text(value, "list<text> entry cannot be null");

    if (list->header.rc > 1u) {
        zt_retain(value);
        return value;
    }

    list->data[index_0] = NULL;
    return value;
}

zt_optional_text zt_list_text_get_optional(const zt_list_text *list, zt_int index_0) {
    zt_text *value;

    zt_runtime_require_list_text(list, "zt_list_text_get_optional requires list");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        return zt_optional_text_empty();
    }

    value = list->data[index_0];
    zt_runtime_require_text(value, "list<text> entry cannot be null");
    return zt_optional_text_present(value);
}

zt_optional_text zt_list_text_last_optional(const zt_list_text *list) {
    zt_text *value;

    zt_runtime_require_list_text(list, "zt_list_text_last_optional requires list");

    if (list->len == 0) {
        return zt_optional_text_empty();
    }

    value = list->data[list->len - 1];
    zt_runtime_require_text(value, "list<text> entry cannot be null");
    return zt_optional_text_present(value);
}

zt_list_text *zt_list_text_rest(const zt_list_text *list) {
    zt_runtime_require_list_text(list, "zt_list_text_rest requires list");
    if (list->len <= 1) {
        return zt_list_text_new();
    }
    return zt_list_text_slice(list, 1, (zt_int)list->len - 1);
}

zt_list_text *zt_list_text_skip(const zt_list_text *list, zt_int count) {
    zt_runtime_require_list_text(list, "zt_list_text_skip requires list");
    if (count <= 0) {
        if (list->len == 0) return zt_list_text_new();
        return zt_list_text_slice(list, 0, (zt_int)list->len - 1);
    }
    if ((size_t)count >= list->len) {
        return zt_list_text_new();
    }
    return zt_list_text_slice(list, count, (zt_int)list->len - 1);
}

zt_list_text *zt_list_text_append(const zt_list_text *list, zt_text *value) {
    zt_list_text *copy;

    zt_runtime_require_list_text(list, "zt_list_text_append requires list");
    zt_runtime_require_text(value, "zt_list_text_append requires value");

    copy = zt_list_text_from_array(list->data, list->len);
    zt_list_text_push(copy, value);
    return copy;
}

zt_list_text *zt_list_text_prepend(const zt_list_text *list, zt_text *value) {
    zt_list_text *copy;
    size_t index;

    zt_runtime_require_list_text(list, "zt_list_text_prepend requires list");
    zt_runtime_require_text(value, "zt_list_text_prepend requires value");

    copy = zt_list_text_new();
    zt_list_text_reserve(copy, zt_require_added_size(list->len, 1, "list prepend size overflow"));
    zt_list_text_push(copy, value);
    for (index = 0; index < list->len; index += 1) {
        zt_list_text_push(copy, list->data[index]);
    }
    return copy;
}

zt_bool zt_list_text_contains(const zt_list_text *list, zt_text *value) {
    size_t index;

    zt_runtime_require_list_text(list, "zt_list_text_contains requires list");
    zt_runtime_require_text(value, "zt_list_text_contains requires value");

    for (index = 0; index < list->len; index += 1) {
        if (zt_text_eq(list->data[index], value)) return true;
    }
    return false;
}

zt_list_text *zt_list_text_reverse(const zt_list_text *list) {
    zt_list_text *copy;
    size_t remaining;

    zt_runtime_require_list_text(list, "zt_list_text_reverse requires list");

    copy = zt_list_text_new();
    zt_list_text_reserve(copy, list->len);
    remaining = list->len;
    while (remaining > 0) {
        remaining -= 1;
        zt_list_text_push(copy, list->data[remaining]);
    }
    return copy;
}

zt_list_text *zt_list_text_concat(const zt_list_text *left, const zt_list_text *right) {
    zt_list_text *copy;
    size_t index;

    zt_runtime_require_list_text(left, "zt_list_text_concat requires left list");
    zt_runtime_require_list_text(right, "zt_list_text_concat requires right list");

    copy = zt_list_text_new();
    zt_list_text_reserve(copy, zt_require_added_size(left->len, right->len, "list concat size overflow"));
    for (index = 0; index < left->len; index += 1) {
        zt_list_text_push(copy, left->data[index]);
    }
    for (index = 0; index < right->len; index += 1) {
        zt_list_text_push(copy, right->data[index]);
    }
    return copy;
}

zt_optional_i64 zt_list_text_index_of(const zt_list_text *list, zt_text *value) {
    size_t index;

    zt_runtime_require_list_text(list, "zt_list_text_index_of requires list");
    zt_runtime_require_text(value, "zt_list_text_index_of requires value");

    for (index = 0; index < list->len; index += 1) {
        if (zt_text_eq(list->data[index], value)) return zt_optional_i64_present((zt_int)index);
    }
    return zt_optional_i64_empty();
}

zt_outcome_list_text_core_error zt_list_text_set_result(const zt_list_text *list, zt_int index_0, zt_text *value) {
    zt_list_text *copy;

    zt_runtime_require_list_text(list, "zt_list_text_set_result requires list");
    zt_runtime_require_text(value, "zt_list_text_set_result requires value");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        return zt_outcome_list_text_core_error_failure_message("list.set index out of bounds");
    }

    copy = zt_list_text_from_array(list->data, list->len);
    zt_list_text_set(copy, index_0, value);
    return zt_outcome_list_text_core_error_success(copy);
}

zt_outcome_list_text_core_error zt_list_text_remove_at(const zt_list_text *list, zt_int index_0) {
    zt_list_text *copy;
    size_t index;

    zt_runtime_require_list_text(list, "zt_list_text_remove_at requires list");

    if (index_0 < 0 || (size_t)index_0 >= list->len) {
        return zt_outcome_list_text_core_error_failure_message("list.remove_at index out of bounds");
    }

    copy = zt_list_text_new();
    zt_list_text_reserve(copy, list->len > 0 ? list->len - 1 : 0);
    for (index = 0; index < list->len; index += 1) {
        if (index != (size_t)index_0) {
            zt_list_text_push(copy, list->data[index]);
        }
    }
    return zt_outcome_list_text_core_error_success(copy);
}

zt_outcome_list_text_core_error zt_list_text_remove_first(const zt_list_text *list) {
    return zt_list_text_remove_at(list, 0);
}

zt_outcome_list_text_core_error zt_list_text_remove_last(const zt_list_text *list) {
    zt_runtime_require_list_text(list, "zt_list_text_remove_last requires list");
    if (list->len == 0) {
        return zt_outcome_list_text_core_error_failure_message("list.remove_last requires a non-empty list");
    }
    return zt_list_text_remove_at(list, (zt_int)list->len - 1);
}

zt_outcome_list_text_core_error zt_list_text_slice_result(const zt_list_text *list, zt_int start_0, zt_int end_0) {
    zt_runtime_require_list_text(list, "zt_list_text_slice_result requires list");
    if (start_0 < 0) {
        return zt_outcome_list_text_core_error_failure_message("list.slice start must be >= 0");
    }
    return zt_outcome_list_text_core_error_success(zt_list_text_slice(list, start_0, end_0));
}

zt_list_i64 *zt_queue_i64_new(void) {
    return zt_list_i64_new();
}

zt_list_i64 *zt_queue_i64_enqueue(zt_list_i64 *queue, zt_int value) {
    return zt_list_i64_push_owned(queue, value);
}

zt_list_i64 *zt_queue_i64_enqueue_owned(zt_list_i64 *queue, zt_int value) {
    return zt_queue_i64_enqueue(queue, value);
}

zt_optional_i64 zt_queue_i64_dequeue(zt_list_i64 *queue) {
    zt_int value;

    zt_runtime_require_list_i64(queue, "zt_queue_i64_dequeue requires queue");
    if (queue->len == 0) {
        return zt_optional_i64_empty();
    }

    value = queue->data[0];
    if (queue->len > 1) {
        memmove(queue->data, queue->data + 1, (queue->len - 1) * sizeof(zt_int));
    }
    queue->len -= 1;
    return zt_optional_i64_present(value);
}

zt_optional_i64 zt_queue_i64_peek(const zt_list_i64 *queue) {
    zt_runtime_require_list_i64(queue, "zt_queue_i64_peek requires queue");
    if (queue->len == 0) {
        return zt_optional_i64_empty();
    }
    return zt_optional_i64_present(queue->data[0]);
}

zt_list_text *zt_queue_text_new(void) {
    return zt_list_text_new();
}

zt_list_text *zt_queue_text_enqueue(zt_list_text *queue, zt_text *value) {
    return zt_list_text_push_owned(queue, value);
}

zt_list_text *zt_queue_text_enqueue_owned(zt_list_text *queue, zt_text *value) {
    return zt_queue_text_enqueue(queue, value);
}

zt_optional_text zt_queue_text_dequeue(zt_list_text *queue) {
    zt_text *value;
    zt_optional_text opt;

    zt_runtime_require_list_text(queue, "zt_queue_text_dequeue requires queue");
    if (queue->len == 0) {
        return zt_optional_text_empty();
    }

    value = queue->data[0];
    if (queue->len > 1) {
        memmove(queue->data, queue->data + 1, (queue->len - 1) * sizeof(zt_text *));
    }
    queue->len -= 1;
    queue->data[queue->len] = NULL;

    zt_runtime_require_text(value, "queue<text> entry cannot be null");
    opt.is_present = true;
    opt.value = value;
    return opt;
}

zt_optional_text zt_queue_text_peek(const zt_list_text *queue) {
    zt_text *value;

    zt_runtime_require_list_text(queue, "zt_queue_text_peek requires queue");
    if (queue->len == 0) {
        return zt_optional_text_empty();
    }

    value = queue->data[0];
    zt_runtime_require_text(value, "queue<text> entry cannot be null");
    return zt_optional_text_present(value);
}

zt_list_i64 *zt_stack_i64_new(void) {
    return zt_list_i64_new();
}

zt_list_i64 *zt_stack_i64_push(zt_list_i64 *stack, zt_int value) {
    return zt_list_i64_push_owned(stack, value);
}

zt_list_i64 *zt_stack_i64_push_owned(zt_list_i64 *stack, zt_int value) {
    return zt_stack_i64_push(stack, value);
}

zt_optional_i64 zt_stack_i64_pop(zt_list_i64 *stack) {
    zt_int value;

    zt_runtime_require_list_i64(stack, "zt_stack_i64_pop requires stack");
    if (stack->len == 0) {
        return zt_optional_i64_empty();
    }

    value = stack->data[stack->len - 1];
    stack->len -= 1;
    return zt_optional_i64_present(value);
}

zt_optional_i64 zt_stack_i64_peek(const zt_list_i64 *stack) {
    zt_runtime_require_list_i64(stack, "zt_stack_i64_peek requires stack");
    if (stack->len == 0) {
        return zt_optional_i64_empty();
    }
    return zt_optional_i64_present(stack->data[stack->len - 1]);
}

zt_list_text *zt_stack_text_new(void) {
    return zt_list_text_new();
}

zt_list_text *zt_stack_text_push(zt_list_text *stack, zt_text *value) {
    return zt_list_text_push_owned(stack, value);
}

zt_list_text *zt_stack_text_push_owned(zt_list_text *stack, zt_text *value) {
    return zt_stack_text_push(stack, value);
}

zt_optional_text zt_stack_text_pop(zt_list_text *stack) {
    zt_text *value;
    zt_optional_text opt;

    zt_runtime_require_list_text(stack, "zt_stack_text_pop requires stack");
    if (stack->len == 0) {
        return zt_optional_text_empty();
    }

    value = stack->data[stack->len - 1];
    stack->len -= 1;
    stack->data[stack->len] = NULL;

    zt_runtime_require_text(value, "stack<text> entry cannot be null");
    opt.is_present = true;
    opt.value = value;
    return opt;
}

zt_optional_text zt_stack_text_peek(const zt_list_text *stack) {
    zt_text *value;

    zt_runtime_require_list_text(stack, "zt_stack_text_peek requires stack");
    if (stack->len == 0) {
        return zt_optional_text_empty();
    }

    value = stack->data[stack->len - 1];
    zt_runtime_require_text(value, "stack<text> entry cannot be null");
    return zt_optional_text_present(value);
}



static zt_dyn_text_repr *zt_dyn_text_repr_alloc(zt_dyn_text_repr_tag tag) {
    zt_dyn_text_repr *value = (zt_dyn_text_repr *)calloc(1, sizeof(zt_dyn_text_repr));
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate dyn<TextRepresentable> box");
    }
    value->header.rc = 1;
    value->header.kind = (uint32_t)ZT_HEAP_DYN_TEXT_REPR;
    value->tag = (uint32_t)tag;
    return value;
}

zt_dyn_text_repr *zt_dyn_text_repr_from_i64(zt_int value) {
    zt_dyn_text_repr *boxed = zt_dyn_text_repr_alloc(ZT_DYN_TEXT_REPR_INT);
    boxed->as.int_value = value;
    return boxed;
}

zt_dyn_text_repr *zt_dyn_text_repr_from_float(zt_float value) {
    zt_dyn_text_repr *boxed = zt_dyn_text_repr_alloc(ZT_DYN_TEXT_REPR_FLOAT);
    boxed->as.float_value = value;
    return boxed;
}

zt_dyn_text_repr *zt_dyn_text_repr_from_bool(zt_bool value) {
    zt_dyn_text_repr *boxed = zt_dyn_text_repr_alloc(ZT_DYN_TEXT_REPR_BOOL);
    boxed->as.bool_value = value;
    return boxed;
}

zt_dyn_text_repr *zt_dyn_text_repr_from_text_owned(zt_text *value) {
    zt_dyn_text_repr *boxed;
    zt_runtime_require_text(value, "zt_dyn_text_repr_from_text_owned requires text");
    boxed = zt_dyn_text_repr_alloc(ZT_DYN_TEXT_REPR_TEXT);
    boxed->as.text_value = value;
    return boxed;
}

zt_dyn_text_repr *zt_dyn_text_repr_from_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_dyn_text_repr_from_text requires text");
    return zt_dyn_text_repr_from_text_owned(zt_text_deep_copy(value));
}

zt_dyn_text_repr *zt_dyn_text_repr_clone(const zt_dyn_text_repr *value) {
    zt_runtime_require_dyn_text_repr(value, "zt_dyn_text_repr_clone requires value");

    switch ((zt_dyn_text_repr_tag)value->tag) {
        case ZT_DYN_TEXT_REPR_INT:
            return zt_dyn_text_repr_from_i64(value->as.int_value);
        case ZT_DYN_TEXT_REPR_FLOAT:
            return zt_dyn_text_repr_from_float(value->as.float_value);
        case ZT_DYN_TEXT_REPR_BOOL:
            return zt_dyn_text_repr_from_bool(value->as.bool_value);
        case ZT_DYN_TEXT_REPR_TEXT:
            zt_runtime_require_text(value->as.text_value, "dyn<TextRepresentable> text payload cannot be null");
            return zt_dyn_text_repr_from_text(value->as.text_value);
        default:
            zt_runtime_error(ZT_ERR_PANIC, "unknown dyn<TextRepresentable> tag in clone");
            return NULL;
    }
}

zt_text *zt_dyn_text_repr_to_text(const zt_dyn_text_repr *value) {
    char buffer[96];

    zt_runtime_require_dyn_text_repr(value, "zt_dyn_text_repr_to_text requires value");

    switch ((zt_dyn_text_repr_tag)value->tag) {
        case ZT_DYN_TEXT_REPR_INT:
            snprintf(buffer, sizeof(buffer), "%lld", (long long)value->as.int_value);
            return zt_text_from_utf8_literal(buffer);
        case ZT_DYN_TEXT_REPR_FLOAT:
            snprintf(buffer, sizeof(buffer), "%.17g", (double)value->as.float_value);
            return zt_text_from_utf8_literal(buffer);
        case ZT_DYN_TEXT_REPR_BOOL:
            return zt_text_from_utf8_literal(value->as.bool_value ? "true" : "false");
        case ZT_DYN_TEXT_REPR_TEXT:
            zt_runtime_require_text(value->as.text_value, "dyn<TextRepresentable> text payload cannot be null");
            return zt_text_deep_copy(value->as.text_value);
        default:
            zt_runtime_error(ZT_ERR_PANIC, "unknown dyn<TextRepresentable> tag in to_text");
            return NULL;
    }
}

zt_int zt_dyn_text_repr_text_len(const zt_dyn_text_repr *value) {
    zt_text *text;
    zt_int length;

    zt_runtime_require_dyn_text_repr(value, "zt_dyn_text_repr_text_len requires value");
    text = zt_dyn_text_repr_to_text(value);
    length = zt_text_len(text);
    zt_release(text);
    return length;
}

zt_list_dyn_text_repr *zt_list_dyn_text_repr_new(void) {
    zt_list_dyn_text_repr *list = (zt_list_dyn_text_repr *)calloc(1, sizeof(zt_list_dyn_text_repr));

    if (list == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate list<dyn<TextRepresentable>>");
    }

    list->header.rc = 1;
    list->header.kind = (uint32_t)ZT_HEAP_LIST_DYN_TEXT_REPR;
    list->capacity = 4;
    list->data = (zt_dyn_text_repr **)calloc(list->capacity, sizeof(zt_dyn_text_repr *));
    if (list->data == NULL) {
        free(list);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate list<dyn<TextRepresentable>> data");
    }

    return list;
}

static void zt_list_dyn_text_repr_reserve(zt_list_dyn_text_repr *list, size_t needed) {
    size_t new_capacity;
    zt_dyn_text_repr **new_data;

    zt_runtime_require_list_dyn_text_repr(list, "zt_list_dyn_text_repr_reserve requires list");
    if (needed <= list->capacity) {
        return;
    }

    new_capacity = list->capacity == 0 ? 4 : list->capacity;
    while (new_capacity < needed) {
        if (new_capacity > SIZE_MAX / 2) {
            zt_runtime_error(ZT_ERR_PLATFORM, "list<dyn<TextRepresentable>> capacity overflow");
        }
        new_capacity *= 2;
    }

    new_data = (zt_dyn_text_repr **)realloc(list->data, new_capacity * sizeof(zt_dyn_text_repr *));
    if (new_data == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to grow list<dyn<TextRepresentable>>");
    }

    list->data = new_data;
    list->capacity = new_capacity;
}

void zt_list_dyn_text_repr_push(zt_list_dyn_text_repr *list, zt_dyn_text_repr *value) {
    zt_runtime_require_list_dyn_text_repr(list, "zt_list_dyn_text_repr_push requires list");
    zt_runtime_require_dyn_text_repr(value, "zt_list_dyn_text_repr_push requires value");

    zt_list_dyn_text_repr_reserve(list, list->len + 1);
    zt_retain(value);
    list->data[list->len] = value;
    list->len += 1;
}

zt_list_dyn_text_repr *zt_list_dyn_text_repr_from_array(zt_dyn_text_repr *const *items, size_t count) {
    zt_list_dyn_text_repr *list = zt_list_dyn_text_repr_new();
    size_t index;

    zt_list_dyn_text_repr_reserve(list, count);
    for (index = 0; index < count; index += 1) {
        zt_list_dyn_text_repr_push(list, items[index]);
    }

    return list;
}

zt_list_dyn_text_repr *zt_list_dyn_text_repr_from_array_owned(zt_dyn_text_repr **items, size_t count) {
    zt_list_dyn_text_repr *list = zt_list_dyn_text_repr_new();
    size_t index;

    zt_list_dyn_text_repr_reserve(list, count);
    for (index = 0; index < count; index += 1) {
        zt_runtime_require_dyn_text_repr(items[index], "zt_list_dyn_text_repr_from_array_owned requires value");
        list->data[list->len] = items[index];
        list->len += 1;
    }

    return list;
}

zt_dyn_text_repr *zt_list_dyn_text_repr_get(const zt_list_dyn_text_repr *list, zt_int index_0) {
    zt_int index;
    zt_dyn_text_repr *value;

    zt_runtime_require_list_dyn_text_repr(list, "zt_list_dyn_text_repr_get requires list");
    index = index_0 < 0 ? (zt_int)list->len + index_0 : index_0;
    if (index < 0 || index >= (zt_int)list->len) {
        zt_runtime_error(ZT_ERR_BOUNDS, "list<dyn<TextRepresentable>> index out of bounds");
    }

    value = list->data[index];
    zt_retain(value);
    return value;
}

zt_int zt_list_dyn_text_repr_len(const zt_list_dyn_text_repr *list) {
    zt_runtime_require_list_dyn_text_repr(list, "zt_list_dyn_text_repr_len requires list");
    return (zt_int)list->len;
}

zt_list_dyn_text_repr *zt_list_dyn_text_repr_slice(const zt_list_dyn_text_repr *list, zt_int start_0, zt_int end_0) {
    zt_list_dyn_text_repr *slice;
    size_t start_pos;
    size_t end_pos;
    size_t index;

    zt_runtime_require_list_dyn_text_repr(list, "zt_list_dyn_text_repr_slice requires list");
    if (start_0 < 0) {
        zt_runtime_error(ZT_ERR_INDEX, "slice start must be >= 0");
    }
    if (list->len == 0) {
        return zt_list_dyn_text_repr_new();
    }

    start_pos = (size_t)start_0;
    end_pos = zt_normalize_slice_end(list->len, end_0);
    if (start_pos >= list->len || end_pos < start_pos) {
        return zt_list_dyn_text_repr_new();
    }

    slice = zt_list_dyn_text_repr_new();
    zt_list_dyn_text_repr_reserve(slice, end_pos - start_pos + 1);
    for (index = start_pos; index <= end_pos; index += 1) {
        zt_list_dyn_text_repr_push(slice, list->data[index]);
    }

    return slice;
}

zt_list_dyn_text_repr *zt_list_dyn_text_repr_deep_copy(const zt_list_dyn_text_repr *list) {
    zt_list_dyn_text_repr *copy;
    size_t index;

    zt_runtime_require_list_dyn_text_repr(list, "zt_list_dyn_text_repr_deep_copy requires list");
    copy = zt_list_dyn_text_repr_new();
    zt_list_dyn_text_repr_reserve(copy, list->len);

    for (index = 0; index < list->len; index += 1) {
        copy->data[copy->len] = zt_dyn_text_repr_clone(list->data[index]);
        copy->len += 1;
    }

    return copy;
}

zt_text *zt_thread_boundary_copy_text(const zt_text *value) {
    zt_runtime_require_text(value, "zt_thread_boundary_copy_text requires text");
    return zt_text_deep_copy(value);
}

zt_bytes *zt_thread_boundary_copy_bytes(const zt_bytes *value) {
    zt_runtime_require_bytes(value, "zt_thread_boundary_copy_bytes requires bytes");
    return zt_bytes_from_array(value->data, value->len);
}

zt_list_i64 *zt_thread_boundary_copy_list_i64(const zt_list_i64 *list) {
    zt_runtime_require_list_i64(list, "zt_thread_boundary_copy_list_i64 requires list");
    return zt_list_i64_from_array(list->data, list->len);
}

zt_list_text *zt_thread_boundary_copy_list_text(const zt_list_text *list) {
    zt_runtime_require_list_text(list, "zt_thread_boundary_copy_list_text requires list");
    return zt_list_text_deep_copy(list);
}

zt_map_text_text *zt_thread_boundary_copy_map_text_text(const zt_map_text_text *map) {
    zt_runtime_require_map_text_text(map, "zt_thread_boundary_copy_map_text_text requires map");
    return (zt_map_text_text *)zt_deep_copy((void *)map);
}

zt_dyn_text_repr *zt_thread_boundary_copy_dyn_text_repr(const zt_dyn_text_repr *value) {
    zt_runtime_require_dyn_text_repr(value, "zt_thread_boundary_copy_dyn_text_repr requires value");
    return zt_dyn_text_repr_clone(value);
}

/* ── Monomorphization: optional<T> ─────────────────────────────────────────── */
zt_list_dyn_text_repr *zt_thread_boundary_copy_list_dyn_text_repr(const zt_list_dyn_text_repr *list) {
    zt_runtime_require_list_dyn_text_repr(list, "zt_thread_boundary_copy_list_dyn_text_repr requires list");
    return zt_list_dyn_text_repr_deep_copy(list);
}

ZT_DEFINE_OPTIONAL_IMPL(i64,          zt_int,           0)
ZT_DEFINE_OPTIONAL_IMPL(f64,          zt_float,         0)
ZT_DEFINE_OPTIONAL_IMPL(bool,         zt_bool,          0)
ZT_DEFINE_OPTIONAL_IMPL(i8,           int8_t,           0)
ZT_DEFINE_OPTIONAL_IMPL(i16,          int16_t,          0)
ZT_DEFINE_OPTIONAL_IMPL(i32,          int32_t,          0)
ZT_DEFINE_OPTIONAL_IMPL(u8,           uint8_t,          0)
ZT_DEFINE_OPTIONAL_IMPL(u16,          uint16_t,         0)
ZT_DEFINE_OPTIONAL_IMPL(u32,          uint32_t,         0)
ZT_DEFINE_OPTIONAL_IMPL(u64,          uint64_t,         0)
ZT_DEFINE_OPTIONAL_IMPL(text,         zt_text *,        1)
ZT_DEFINE_OPTIONAL_IMPL(bytes,        zt_bytes *,       1)
ZT_DEFINE_OPTIONAL_IMPL(list_i64,     zt_list_i64 *,    1)
ZT_DEFINE_OPTIONAL_IMPL(list_text,    zt_list_text *,   1)
ZT_DEFINE_OPTIONAL_IMPL(map_text_text, zt_map_text_text *, 1)


zt_core_error zt_core_error_make(zt_text *code, zt_text *message, zt_optional_text context) {
    zt_core_error error;

    zt_runtime_require_text(code, "core.Error requires code text");
    zt_runtime_require_text(message, "core.Error requires message text");

    error.code = code;
    error.message = message;
    error.context = context;
    zt_retain(code);
    zt_retain(message);
    if (context.is_present && context.value != NULL) {
        zt_retain(context.value);
    }

    return error;
}

zt_core_error zt_core_error_from_message(const char *code, const char *message) {
    zt_text *code_text;
    zt_text *message_text;
    zt_core_error error;

    code_text = zt_text_from_utf8_literal(code != NULL ? code : "error");
    message_text = zt_text_from_utf8_literal(zt_safe_message(message));
    error = zt_core_error_make(code_text, message_text, zt_optional_text_empty());
    zt_release(code_text);
    zt_release(message_text);
    return error;
}

zt_core_error zt_core_error_from_text(const char *code, zt_text *message) {
    zt_text *code_text;
    zt_core_error error;

    zt_runtime_require_text(message, "core.Error message cannot be null");
    code_text = zt_text_from_utf8_literal(code != NULL ? code : "error");
    error = zt_core_error_make(code_text, message, zt_optional_text_empty());
    zt_release(code_text);
    return error;
}

zt_core_error zt_core_error_clone(zt_core_error error) {
    zt_core_error copy;
    copy.code = error.code;
    copy.message = error.message;
    copy.context = error.context;
    if (copy.code != NULL) zt_retain(copy.code);
    if (copy.message != NULL) zt_retain(copy.message);
    if (copy.context.is_present && copy.context.value != NULL) zt_retain(copy.context.value);
    return copy;
}

void zt_core_error_dispose(zt_core_error *error) {
    if (error == NULL) return;
    if (error->code != NULL) zt_release(error->code);
    if (error->message != NULL) zt_release(error->message);
    if (error->context.is_present && error->context.value != NULL) zt_release(error->context.value);
    error->code = NULL;
    error->message = NULL;
    error->context = zt_optional_text_empty();
}

zt_text *zt_core_error_message_or_default(zt_core_error error) {
    if (error.message != NULL) {
        zt_retain(error.message);
        return error.message;
    }
    return zt_text_from_utf8_literal("error");
}

zt_outcome_process_captured_run_core_error zt_outcome_process_captured_run_core_error_success(zt_process_captured_run value) {
    zt_outcome_process_captured_run_core_error outcome;
    outcome.is_success = true;
    outcome.value = value;
    outcome.error = (zt_core_error){0};
    zt_process_captured_run_retain(value);
    return outcome;
}

zt_outcome_process_captured_run_core_error zt_outcome_process_captured_run_core_error_failure(zt_core_error error) {
    zt_outcome_process_captured_run_core_error outcome;
    outcome.is_success = false;
    outcome.value.status.code = 0;
    outcome.value.stdout_text = NULL;
    outcome.value.stderr_text = NULL;
    outcome.error = error.message != NULL ? zt_core_error_clone(error) : zt_core_error_from_message("error", "error");
    return outcome;
}

zt_outcome_process_captured_run_core_error zt_outcome_process_captured_run_core_error_failure_message(const char *message) {
    zt_core_error error = zt_core_error_from_message("error", message);
    zt_outcome_process_captured_run_core_error outcome = zt_outcome_process_captured_run_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

zt_bool zt_outcome_process_captured_run_core_error_is_success(zt_outcome_process_captured_run_core_error outcome) {
    return outcome.is_success;
}

zt_process_captured_run zt_outcome_process_captured_run_core_error_value(zt_outcome_process_captured_run_core_error outcome) {
    zt_process_captured_run value;
    if (!outcome.is_success) {
        zt_runtime_error(ZT_ERR_UNWRAP, "outcome_value on failure");
    }
    value = outcome.value;
    zt_process_captured_run_retain(value);
    return value;
}

zt_outcome_process_captured_run_core_error zt_outcome_process_captured_run_core_error_propagate(zt_outcome_process_captured_run_core_error outcome) {
    if (outcome.is_success) {
        return zt_outcome_process_captured_run_core_error_success(outcome.value);
    }
    return zt_outcome_process_captured_run_core_error_failure(outcome.error);
}

void zt_outcome_process_captured_run_core_error_dispose(zt_outcome_process_captured_run_core_error *outcome) {
    if (outcome == NULL) {
        return;
    }
    if (outcome->is_success) {
        zt_process_captured_run_dispose(&outcome->value);
    } else {
        zt_core_error_dispose(&outcome->error);
    }
    memset(outcome, 0, sizeof(*outcome));
}

/* ── Monomorphization: outcome<V,text> ─────────────────────────────────────── */
ZT_DEFINE_OUTCOME_IMPL(i64_text,  zt_int,     zt_text *, 0)
ZT_DEFINE_OUTCOME_IMPL(text_text, zt_text *,  zt_text *, 1)


zt_bool zt_outcome_text_text_eq(zt_outcome_text_text left, zt_outcome_text_text right) {
    if (left.is_success != right.is_success) {
        return false;
    }

    if (left.is_success) {
        if (left.value == NULL || right.value == NULL) {
            return left.value == right.value;
        }
        return zt_text_eq(left.value, right.value);
    }

    if (left.error == NULL || right.error == NULL) {
        return left.error == right.error;
    }

    return zt_text_eq(left.error, right.error);
}

ZT_DEFINE_OUTCOME_IMPL(list_i64_text,  zt_list_i64 *,  zt_text *, 1)
ZT_DEFINE_OUTCOME_IMPL(list_text_text, zt_list_text *, zt_text *, 1)


ZT_DEFINE_OUTCOME_IMPL(map_text_text, zt_map_text_text *, zt_text *, 1)

ZT_DEFINE_OUTCOME_VOID_TEXT_ERROR_IMPL(void_text)

/* ── R2.M1 (T2.2): outcome<V, core.Error> via macros ────────────────────────── */
ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL(i64_core_error, zt_int, 0)
ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL(f64_core_error, zt_float, 0.0)
ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL(bool_core_error, zt_bool, false)
ZT_DEFINE_OUTCOME_VOID_CORE_ERROR_IMPL(void_core_error)
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(text_core_error, zt_text *, zt_runtime_require_text, "text")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(bytes_core_error, zt_bytes *, zt_runtime_require_bytes, "bytes")
ZT_DEFINE_OUTCOME_CORE_ERROR_OPTIONAL_PTR_IMPL(
    optional_text_core_error, zt_optional_text, zt_runtime_require_text, "text", zt_optional_text_empty)
ZT_DEFINE_OUTCOME_CORE_ERROR_OPTIONAL_PTR_IMPL(
    optional_bytes_core_error, zt_optional_bytes, zt_runtime_require_bytes, "bytes", zt_optional_bytes_empty)
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    net_connection_core_error, zt_net_connection *, zt_runtime_require_net_connection, "net.Connection")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_i64_core_error, zt_list_i64 *, zt_runtime_require_list_i64, "list<int>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_text_core_error, zt_list_text *, zt_runtime_require_list_text, "list<text>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_f64_core_error, zt_list_f64 *, zt_runtime_require_list_f64, "list<float>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_bool_core_error, zt_list_bool *, zt_runtime_require_list_bool, "list<bool>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_i8_core_error, zt_list_i8 *, zt_runtime_require_list_i8, "list<int8>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_i16_core_error, zt_list_i16 *, zt_runtime_require_list_i16, "list<int16>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_i32_core_error, zt_list_i32 *, zt_runtime_require_list_i32, "list<int32>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_u8_core_error, zt_list_u8 *, zt_runtime_require_list_u8, "list<uint8>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_u16_core_error, zt_list_u16 *, zt_runtime_require_list_u16, "list<uint16>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_u32_core_error, zt_list_u32 *, zt_runtime_require_list_u32, "list<uint32>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    list_u64_core_error, zt_list_u64 *, zt_runtime_require_list_u64, "list<uint64>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
    map_text_text_core_error, zt_map_text_text *, zt_runtime_require_map_text_text, "map<text,text>")
ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL(
    optional_i64_core_error, zt_optional_i64, zt_optional_i64_empty())
