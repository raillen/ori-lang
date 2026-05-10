typedef struct zt_http_url_parts {
    char *host;
    char *path;
    zt_int port;
} zt_http_url_parts;

static zt_outcome_text_core_error zt_http_failure(const char *code, const char *message) {
    zt_core_error error = zt_core_error_from_message(code, message);
    zt_outcome_text_core_error outcome = zt_outcome_text_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

static void zt_http_url_parts_dispose(zt_http_url_parts *parts) {
    if (parts == NULL) return;
    free(parts->host);
    free(parts->path);
    parts->host = NULL;
    parts->path = NULL;
    parts->port = 0;
}

static char *zt_http_copy_range(const char *start, size_t len) {
    char *copy = (char *)malloc(len + 1);
    if (copy == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "http allocation failed");
    }
    if (len > 0) {
        memcpy(copy, start, len);
    }
    copy[len] = '\0';
    return copy;
}

static int zt_http_parse_port(const char *start, const char *end, zt_int *out_port) {
    zt_int port = 0;
    const char *cursor = start;

    if (start == end) return 0;
    while (cursor < end) {
        if (*cursor < '0' || *cursor > '9') return 0;
        port = (port * 10) + (zt_int)(*cursor - '0');
        if (port > 65535) return 0;
        cursor += 1;
    }
    if (port < 1) return 0;
    *out_port = port;
    return 1;
}

static int zt_http_parse_url(const zt_text *url, zt_http_url_parts *out_parts, const char **out_error_code, const char **out_error_message) {
    const char *raw;
    const char *rest;
    const char *slash;
    const char *authority_end;
    const char *colon;
    size_t host_len;

    zt_runtime_require_text(url, "zt_http_parse_url requires url text");
    raw = zt_text_data(url);

    if (strncmp(raw, "https://", 8) == 0) {
        *out_error_code = "http.unsupported_scheme";
        *out_error_message = "https is not supported in v1";
        return 0;
    }
    if (strncmp(raw, "http://", 7) != 0) {
        *out_error_code = "http.invalid_url";
        *out_error_message = "expected http:// URL";
        return 0;
    }

    rest = raw + 7;
    slash = strchr(rest, '/');
    authority_end = slash != NULL ? slash : raw + url->len;
    if (authority_end == rest) {
        *out_error_code = "http.invalid_url";
        *out_error_message = "missing host";
        return 0;
    }

    colon = memchr(rest, ':', (size_t)(authority_end - rest));
    out_parts->port = 80;
    if (colon != NULL) {
        if (colon == rest || !zt_http_parse_port(colon + 1, authority_end, &out_parts->port)) {
            *out_error_code = "http.invalid_url";
            *out_error_message = "invalid port";
            return 0;
        }
        host_len = (size_t)(colon - rest);
    } else {
        host_len = (size_t)(authority_end - rest);
    }

    out_parts->host = zt_http_copy_range(rest, host_len);
    out_parts->path = slash != NULL ? zt_http_copy_range(slash, strlen(slash)) : zt_http_copy_range("/", 1);
    return 1;
}

static zt_outcome_text_core_error zt_http_request_core(const zt_text *url, const zt_text *method, const zt_text *body, const zt_text *content_type) {
    zt_http_url_parts parts = {0};
    const char *error_code = NULL;
    const char *error_message = NULL;
    zt_text *host_text = NULL;
    zt_outcome_net_connection_core_error connect_outcome;
    zt_net_connection *connection = NULL;
    char *request = NULL;
    size_t request_len;
    zt_bytes *request_bytes = NULL;
    char *raw = NULL;
    size_t raw_len = 0;
    size_t raw_cap = 0;
    zt_outcome_text_core_error result;

    if (!zt_http_parse_url(url, &parts, &error_code, &error_message)) {
        return zt_http_failure(error_code, error_message);
    }

    host_text = zt_text_from_utf8_literal(parts.host);
    connect_outcome = zt_net_connect(host_text, parts.port, 3000);
    zt_release(host_text);
    if (!connect_outcome.is_success) {
        zt_http_url_parts_dispose(&parts);
        result = zt_outcome_text_core_error_failure(connect_outcome.error);
        zt_core_error_dispose(&connect_outcome.error);
        return result;
    }
    connection = connect_outcome.value;

    request_len = strlen(zt_text_data(method)) + strlen(parts.path) + strlen(parts.host) + body->len + content_type->len + 160;
    request = (char *)malloc(request_len);
    if (request == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "http request allocation failed");
    }

    if (zt_text_equals_literal(method, "POST")) {
        snprintf(request, request_len,
            "POST %s HTTP/1.1\r\nHost: %s\r\nConnection: close\r\nUser-Agent: Zenith/1\r\nContent-Type: %s\r\nContent-Length: %zu\r\n\r\n%s",
            parts.path, parts.host, zt_text_data(content_type), body->len, zt_text_data(body));
    } else {
        snprintf(request, request_len,
            "GET %s HTTP/1.1\r\nHost: %s\r\nConnection: close\r\nUser-Agent: Zenith/1\r\n\r\n",
            parts.path, parts.host);
    }

    request_bytes = zt_bytes_from_array((const uint8_t *)request, strlen(request));
    {
        zt_outcome_void_core_error write_outcome = zt_net_write_all(connection, request_bytes, 3000);
        zt_release(request_bytes);
        free(request);
        request = NULL;
        if (!write_outcome.is_success) {
            zt_net_close(connection);
            zt_release(connection);
            zt_http_url_parts_dispose(&parts);
            result = zt_outcome_text_core_error_failure(write_outcome.error);
            zt_core_error_dispose(&write_outcome.error);
            return result;
        }
    }

    for (;;) {
        zt_outcome_optional_bytes_core_error read_outcome = zt_net_read_some(connection, 4096, 1000);
        if (!read_outcome.is_success) {
            if (zt_text_equals_literal(read_outcome.error.code, "net.Timeout") && raw_len > 0) {
                zt_core_error_dispose(&read_outcome.error);
                break;
            }
            free(raw);
            zt_net_close(connection);
            zt_release(connection);
            zt_http_url_parts_dispose(&parts);
            result = zt_outcome_text_core_error_failure(read_outcome.error);
            zt_core_error_dispose(&read_outcome.error);
            return result;
        }

        if (!read_outcome.value.is_present) {
            break;
        }

        if (raw_len + read_outcome.value.value->len + 1 > raw_cap) {
            size_t next_cap = raw_cap == 0 ? 8192 : raw_cap * 2;
            while (next_cap < raw_len + read_outcome.value.value->len + 1) {
                next_cap *= 2;
            }
            raw = (char *)realloc(raw, next_cap);
            if (raw == NULL) {
                zt_runtime_error(ZT_ERR_PLATFORM, "http response allocation failed");
            }
            raw_cap = next_cap;
        }
        memcpy(raw + raw_len, read_outcome.value.value->data, read_outcome.value.value->len);
        raw_len += read_outcome.value.value->len;
        raw[raw_len] = '\0';
        zt_release(read_outcome.value.value);
    }

    zt_net_close(connection);
    zt_release(connection);
    zt_http_url_parts_dispose(&parts);

    if (raw == NULL) {
        return zt_http_failure("http.invalid_response", "empty response");
    }

    {
        zt_text *raw_text = zt_text_from_utf8(raw, raw_len);
        free(raw);
        result = zt_outcome_text_core_error_success(raw_text);
        zt_release(raw_text);
        return result;
    }
}

zt_outcome_text_core_error zt_http_get_core(const zt_text *url) {
    zt_text *method = zt_text_from_utf8_literal("GET");
    zt_text *empty = zt_text_from_utf8_literal("");
    zt_outcome_text_core_error result = zt_http_request_core(url, method, empty, empty);
    zt_release(method);
    zt_release(empty);
    return result;
}

zt_outcome_text_core_error zt_http_post_core(const zt_text *url, const zt_text *body, const zt_text *content_type) {
    zt_text *method = zt_text_from_utf8_literal("POST");
    zt_outcome_text_core_error result = zt_http_request_core(url, method, body, content_type);
    zt_release(method);
    return result;
}
