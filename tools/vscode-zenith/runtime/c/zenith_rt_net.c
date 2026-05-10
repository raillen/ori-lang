static int zt_net_startup(char *message, size_t capacity) {
#ifdef _WIN32
    static int started = 0;
    WSADATA data;
    int code;

    if (started) {
        return 1;
    }

    code = WSAStartup(MAKEWORD(2, 2), &data);
    if (code != 0) {
        snprintf(message, capacity, "net.StartupFailed: WSAStartup failed with code %d", code);
        return 0;
    }

    started = 1;
#else
    (void)message;
    (void)capacity;
#endif
    return 1;
}

static int zt_net_last_error_code(void) {
#ifdef _WIN32
    return WSAGetLastError();
#else
    return errno;
#endif
}

static int zt_net_would_block_code(int code) {
#ifdef _WIN32
    return code == WSAEWOULDBLOCK || code == WSAEINPROGRESS || code == WSAEALREADY;
#else
    return code == EINPROGRESS || code == EWOULDBLOCK || code == EAGAIN || code == EALREADY;
#endif
}

static void zt_net_format_error(char *buffer, size_t capacity, const char *prefix, int code) {
    if (buffer == NULL || capacity == 0) {
        return;
    }

#ifdef _WIN32
    snprintf(buffer, capacity, "%s (socket error %d)", prefix, code);
#else
    snprintf(buffer, capacity, "%s: %s", prefix, strerror(code));
#endif
}

static int zt_net_set_nonblocking(zt_socket_handle socket_value, char *message, size_t capacity) {
#ifdef _WIN32
    u_long mode = 1;
    if (ioctlsocket(socket_value, FIONBIO, &mode) != 0) {
        zt_net_format_error(message, capacity, "net.PlatformError: failed to set socket nonblocking", zt_net_last_error_code());
        return 0;
    }
#else
    int flags = fcntl(socket_value, F_GETFL, 0);
    if (flags < 0 || fcntl(socket_value, F_SETFL, flags | O_NONBLOCK) < 0) {
        zt_net_format_error(message, capacity, "net.PlatformError: failed to set socket nonblocking", errno);
        return 0;
    }
#endif
    return 1;
}

static int zt_net_wait_socket(zt_socket_handle socket_value, int wait_read, zt_int timeout_ms, int *out_error) {
    fd_set read_set;
    fd_set write_set;
    fd_set *read_ptr = NULL;
    fd_set *write_ptr = NULL;
    struct timeval tv;
    struct timeval *tv_ptr = NULL;
    int rc;

    if (out_error != NULL) {
        *out_error = 0;
    }

    FD_ZERO(&read_set);
    FD_ZERO(&write_set);
    if (wait_read) {
        FD_SET(socket_value, &read_set);
        read_ptr = &read_set;
    } else {
        FD_SET(socket_value, &write_set);
        write_ptr = &write_set;
    }

    if (timeout_ms >= 0) {
        tv.tv_sec = (long)(timeout_ms / 1000);
        tv.tv_usec = (long)((timeout_ms % 1000) * 1000);
        tv_ptr = &tv;
    }

    do {
#ifdef _WIN32
        rc = select(0, read_ptr, write_ptr, NULL, tv_ptr);
#else
        rc = select(socket_value + 1, read_ptr, write_ptr, NULL, tv_ptr);
#endif
    } while (rc < 0 && zt_net_last_error_code() == EINTR);

    if (rc < 0 && out_error != NULL) {
        *out_error = zt_net_last_error_code();
    }

    return rc;
}

static int zt_net_socket_error(zt_socket_handle socket_value, int *out_error) {
    int socket_error = 0;
#ifdef _WIN32
    int length = (int)sizeof(socket_error);
#else
    socklen_t length = (socklen_t)sizeof(socket_error);
#endif

    if (getsockopt(socket_value, SOL_SOCKET, SO_ERROR, (char *)&socket_error, &length) != 0) {
        if (out_error != NULL) {
            *out_error = zt_net_last_error_code();
        }
        return 0;
    }

    if (out_error != NULL) {
        *out_error = socket_error;
    }
    return 1;
}

static zt_net_connection *zt_net_connection_new(zt_socket_handle socket_value, zt_int default_timeout_ms) {
    zt_net_connection *connection;

    connection = (zt_net_connection *)calloc(1, sizeof(zt_net_connection));
    if (connection == NULL) {
        zt_net_close_socket_handle((intptr_t)socket_value);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate net.Connection");
    }

    connection->header.rc = 1;
    connection->header.kind = (uint32_t)ZT_HEAP_NET_CONNECTION;
    connection->socket_handle = (intptr_t)socket_value;
    connection->default_timeout_ms = default_timeout_ms;
    connection->closed = false;
    return connection;
}

static zt_core_error zt_net_core_error_from_prefixed_message(const char *message) {
    const char *safe_message = zt_safe_message(message);
    const char *colon = strchr(safe_message, ':');
    const char *detail = safe_message;
    zt_text *code_text;
    zt_text *message_text;
    zt_core_error error;

    if (colon != NULL && colon != safe_message) {
        size_t code_len = (size_t)(colon - safe_message);
        detail = colon + 1;
        while (*detail == ' ') detail += 1;
        code_text = zt_text_from_utf8(safe_message, code_len);
    } else {
        code_text = zt_text_from_utf8_literal("error");
    }

    message_text = zt_text_from_utf8_literal(detail);
    error = zt_core_error_make(code_text, message_text, zt_optional_text_empty());
    zt_release(code_text);
    zt_release(message_text);
    return error;
}

static zt_outcome_net_connection_core_error zt_net_connection_core_error_failure_prefixed(const char *message) {
    zt_core_error error = zt_net_core_error_from_prefixed_message(message);
    zt_outcome_net_connection_core_error outcome = zt_outcome_net_connection_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

static zt_outcome_optional_bytes_core_error zt_net_optional_bytes_core_error_failure_prefixed(const char *message) {
    zt_core_error error = zt_net_core_error_from_prefixed_message(message);
    zt_outcome_optional_bytes_core_error outcome = zt_outcome_optional_bytes_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

static zt_outcome_void_core_error zt_net_void_core_error_failure_prefixed(const char *message) {
    zt_core_error error = zt_net_core_error_from_prefixed_message(message);
    zt_outcome_void_core_error outcome = zt_outcome_void_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

zt_outcome_net_connection_core_error zt_net_connect(const zt_text *host, zt_int port, zt_int timeout_ms) {
    char message[256];
    char port_buffer[32];
    struct addrinfo hints;
    struct addrinfo *addresses = NULL;
    struct addrinfo *entry;
    int gai_code;
    zt_outcome_net_connection_core_error outcome;

    zt_runtime_require_text(host, "zt_net_connect requires host text");

    if (host->len == 0) {
        return zt_net_connection_core_error_failure_prefixed("net.InvalidAddress: host cannot be empty");
    }
    if (port < 1 || port > 65535) {
        return zt_net_connection_core_error_failure_prefixed("net.InvalidPort: port must be between 1 and 65535");
    }
    if (timeout_ms < -1) {
        return zt_net_connection_core_error_failure_prefixed("net.InvalidTimeout: connect timeout must be >= -1 milliseconds");
    }
    if (!zt_net_startup(message, sizeof(message))) {
        return zt_net_connection_core_error_failure_prefixed(message);
    }

    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;
    snprintf(port_buffer, sizeof(port_buffer), "%lld", (long long)port);

    gai_code = getaddrinfo(zt_text_data(host), port_buffer, &hints, &addresses);
    if (gai_code != 0) {
#ifdef _WIN32
        snprintf(message, sizeof(message), "net.DnsFailed: getaddrinfo failed with code %d", gai_code);
#else
        snprintf(message, sizeof(message), "net.DnsFailed: %s", gai_strerror(gai_code));
#endif
        return zt_net_connection_core_error_failure_prefixed(message);
    }

    snprintf(message, sizeof(message), "net.ConnectionFailed: no address was reachable");
    for (entry = addresses; entry != NULL; entry = entry->ai_next) {
        zt_socket_handle socket_value;
        int rc;
        int code;

        socket_value = socket(entry->ai_family, entry->ai_socktype, entry->ai_protocol);
        if (socket_value == ZT_NET_INVALID_SOCKET) {
            zt_net_format_error(message, sizeof(message), "net.SystemLimit: failed to create socket", zt_net_last_error_code());
            continue;
        }

        if (!zt_net_set_nonblocking(socket_value, message, sizeof(message))) {
            zt_net_close_socket_handle((intptr_t)socket_value);
            continue;
        }

        rc = connect(socket_value, entry->ai_addr, (int)entry->ai_addrlen);
        if (rc == 0) {
            zt_net_connection *connection = zt_net_connection_new(socket_value, timeout_ms);
            outcome = zt_outcome_net_connection_core_error_success(connection);
            zt_release(connection);
            freeaddrinfo(addresses);
            return outcome;
        }

        code = zt_net_last_error_code();
        if (!zt_net_would_block_code(code)) {
            zt_net_format_error(message, sizeof(message), "net.ConnectionFailed: connect failed", code);
            zt_net_close_socket_handle((intptr_t)socket_value);
            continue;
        }

        rc = zt_net_wait_socket(socket_value, 0, timeout_ms, &code);
        if (rc == 0) {
            snprintf(message, sizeof(message), "net.Timeout: connection timed out after %lld ms", (long long)timeout_ms);
            zt_net_close_socket_handle((intptr_t)socket_value);
            continue;
        }
        if (rc < 0) {
            zt_net_format_error(message, sizeof(message), "net.ConnectionFailed: connect wait failed", code);
            zt_net_close_socket_handle((intptr_t)socket_value);
            continue;
        }

        if (!zt_net_socket_error(socket_value, &code) || code != 0) {
            zt_net_format_error(message, sizeof(message), "net.ConnectionFailed: connect failed", code);
            zt_net_close_socket_handle((intptr_t)socket_value);
            continue;
        }

        {
            zt_net_connection *connection = zt_net_connection_new(socket_value, timeout_ms);
            outcome = zt_outcome_net_connection_core_error_success(connection);
            zt_release(connection);
            freeaddrinfo(addresses);
            return outcome;
        }
    }

    freeaddrinfo(addresses);
    return zt_net_connection_core_error_failure_prefixed(message);
}

static zt_int zt_net_effective_timeout_ms(const zt_net_connection *connection, zt_int timeout_ms) {
    return timeout_ms >= 0 ? timeout_ms : connection->default_timeout_ms;
}

zt_outcome_optional_bytes_core_error zt_net_read_some(zt_net_connection *connection, zt_int max, zt_int timeout_ms) {
    zt_socket_handle socket_value;
    zt_int effective_timeout;
    uint8_t *buffer;
    int wait_error = 0;
    int wait_result;
    int recv_count;
    zt_bytes *bytes_value;
    zt_optional_bytes optional;
    zt_outcome_optional_bytes_core_error outcome;

    zt_runtime_require_net_connection(connection, "zt_net_read_some requires connection");

    if (connection->closed) {
        return zt_net_optional_bytes_core_error_failure_prefixed("net.NotConnected: connection is closed");
    }
    if (max <= 0) {
        return zt_net_optional_bytes_core_error_failure_prefixed("net.InvalidReadSize: max must be > 0");
    }
    if (max > INT_MAX) {
        return zt_net_optional_bytes_core_error_failure_prefixed("net.Overflow: max is too large for this platform");
    }

    effective_timeout = zt_net_effective_timeout_ms(connection, timeout_ms);
    socket_value = (zt_socket_handle)connection->socket_handle;
    wait_result = zt_net_wait_socket(socket_value, 1, effective_timeout, &wait_error);
    if (wait_result == 0) {
        return zt_net_optional_bytes_core_error_failure_prefixed("net.Timeout: read timed out");
    }
    if (wait_result < 0) {
        char message[256];
        zt_net_format_error(message, sizeof(message), "net.ReadFailed: read wait failed", wait_error);
        return zt_net_optional_bytes_core_error_failure_prefixed(message);
    }

    buffer = (uint8_t *)malloc((size_t)max);
    if (buffer == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate net read buffer");
    }

    recv_count = recv(socket_value, (char *)buffer, (int)max, 0);
    if (recv_count == 0) {
        free(buffer);
        outcome = zt_outcome_optional_bytes_core_error_success(zt_optional_bytes_empty());
        return outcome;
    }
    if (recv_count < 0) {
        char message[256];
        int code = zt_net_last_error_code();
        free(buffer);
        zt_net_format_error(message, sizeof(message), "net.ReadFailed: recv failed", code);
        return zt_net_optional_bytes_core_error_failure_prefixed(message);
    }

    bytes_value = zt_bytes_from_array(buffer, (size_t)recv_count);
    free(buffer);
    optional = zt_optional_bytes_present(bytes_value);
    zt_release(bytes_value);
    outcome = zt_outcome_optional_bytes_core_error_success(optional);
    if (optional.is_present) {
        zt_release(optional.value);
    }
    return outcome;
}

zt_outcome_void_core_error zt_net_write_all(zt_net_connection *connection, const zt_bytes *data, zt_int timeout_ms) {
    zt_socket_handle socket_value;
    zt_int effective_timeout;
    size_t offset = 0;

    zt_runtime_require_net_connection(connection, "zt_net_write_all requires connection");
    zt_runtime_require_bytes(data, "zt_net_write_all requires data bytes");

    if (connection->closed) {
        return zt_net_void_core_error_failure_prefixed("net.NotConnected: connection is closed");
    }

    socket_value = (zt_socket_handle)connection->socket_handle;
    effective_timeout = zt_net_effective_timeout_ms(connection, timeout_ms);
    while (offset < data->len) {
        size_t remaining = data->len - offset;
        int chunk = remaining > 65536u ? 65536 : (int)remaining;
        int wait_error = 0;
        int wait_result = zt_net_wait_socket(socket_value, 0, effective_timeout, &wait_error);
        int sent;

        if (wait_result == 0) {
            return zt_net_void_core_error_failure_prefixed("net.Timeout: write timed out");
        }
        if (wait_result < 0) {
            char message[256];
            zt_net_format_error(message, sizeof(message), "net.WriteFailed: write wait failed", wait_error);
            return zt_net_void_core_error_failure_prefixed(message);
        }

        sent = send(socket_value, (const char *)(data->data + offset), chunk, 0);
        if (sent < 0) {
            char message[256];
            int code = zt_net_last_error_code();
            if (zt_net_would_block_code(code)) {
                continue;
            }
            zt_net_format_error(message, sizeof(message), "net.WriteFailed: send failed", code);
            return zt_net_void_core_error_failure_prefixed(message);
        }
        if (sent == 0) {
            return zt_net_void_core_error_failure_prefixed("net.PeerReset: connection closed while writing");
        }

        offset += (size_t)sent;
    }

    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_net_close(zt_net_connection *connection) {
    zt_runtime_require_net_connection(connection, "zt_net_close requires connection");

    if (connection->closed) {
        return zt_outcome_void_core_error_success();
    }

    zt_net_close_socket_handle(connection->socket_handle);
    connection->socket_handle = (intptr_t)ZT_NET_INVALID_SOCKET;
    connection->closed = true;
    return zt_outcome_void_core_error_success();
}

zt_bool zt_net_is_closed(const zt_net_connection *connection) {
    zt_runtime_require_net_connection(connection, "zt_net_is_closed requires connection");
    return connection->closed;
}

zt_int zt_net_error_kind_index(zt_core_error error) {
    const zt_text *code = error.code;

    if (zt_text_equals_literal(code, "net.ConnectionRefused")) return 1;
    if (zt_text_equals_literal(code, "net.HostUnreachable")) return 2;
    if (zt_text_equals_literal(code, "net.DnsFailed")) return 2;
    if (zt_text_equals_literal(code, "net.Timeout")) return 3;
    if (zt_text_equals_literal(code, "net.AddressInUse")) return 4;
    if (zt_text_equals_literal(code, "net.AlreadyConnected")) return 5;
    if (zt_text_equals_literal(code, "net.NotConnected")) return 6;
    if (zt_text_equals_literal(code, "net.NetworkDown")) return 7;
    if (zt_text_equals_literal(code, "net.Overflow")) return 8;
    if (zt_text_equals_literal(code, "net.PeerReset")) return 9;
    if (zt_text_equals_literal(code, "net.SystemLimit")) return 10;
    return 0;
}
