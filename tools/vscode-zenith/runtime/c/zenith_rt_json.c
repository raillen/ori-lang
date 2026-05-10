static const char *zt_json_skip_whitespace(const char *cursor, const char *end) {
    while (cursor < end && isspace((unsigned char)*cursor)) {
        cursor++;
    }
    return cursor;
}

static void zt_json_buffer_reserve(char **buffer, size_t *capacity, size_t current_len, size_t additional) {
    size_t required;
    size_t next_capacity;
    char *resized;

    if (buffer == NULL || capacity == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "invalid JSON buffer state");
    }

    required = current_len + additional + 1;
    if (*capacity >= required) {
        return;
    }

    next_capacity = *capacity > 0 ? *capacity : 32;
    while (next_capacity < required) {
        next_capacity *= 2;
    }

    resized = (char *)realloc(*buffer, next_capacity);
    if (resized == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate JSON buffer");
    }

    *buffer = resized;
    *capacity = next_capacity;
}

static void zt_json_buffer_append_char(char **buffer, size_t *len, size_t *capacity, char value) {
    zt_json_buffer_reserve(buffer, capacity, *len, 1);
    (*buffer)[*len] = value;
    *len += 1;
    (*buffer)[*len] = '\0';
}

static void zt_json_buffer_append_bytes(char **buffer, size_t *len, size_t *capacity, const char *value, size_t value_len) {
    if (value_len == 0) {
        return;
    }

    zt_json_buffer_reserve(buffer, capacity, *len, value_len);
    memcpy((*buffer) + *len, value, value_len);
    *len += value_len;
    (*buffer)[*len] = '\0';
}

static void zt_json_buffer_append_escaped_text(char **buffer, size_t *len, size_t *capacity, const zt_text *value) {
    static const char *hex = "0123456789abcdef";
    size_t index;

    zt_runtime_require_text(value, "zt_json_buffer_append_escaped_text requires text");

    for (index = 0; index < value->len; index++) {
        unsigned char ch = (unsigned char)value->data[index];
        char escaped[7] = "\\u0000";

        switch (ch) {
            case '"':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\\"", 2);
                break;
            case '\\':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\\\", 2);
                break;
            case '\b':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\b", 2);
                break;
            case '\f':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\f", 2);
                break;
            case '\n':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\n", 2);
                break;
            case '\r':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\r", 2);
                break;
            case '\t':
                zt_json_buffer_append_bytes(buffer, len, capacity, "\\t", 2);
                break;
            default:
                if (ch < 0x20) {
                    escaped[4] = hex[(ch >> 4) & 0x0f];
                    escaped[5] = hex[ch & 0x0f];
                    zt_json_buffer_append_bytes(buffer, len, capacity, escaped, 6);
                } else {
                    zt_json_buffer_append_char(buffer, len, capacity, (char)ch);
                }
                break;
        }
    }
}

static zt_bool zt_json_parse_string(const char **cursor, const char *end, zt_text **out_value, const char **out_error) {
    const char *it;
    char *buffer;
    size_t len;
    size_t capacity;

    if (cursor == NULL || out_value == NULL || out_error == NULL) {
        return false;
    }

    it = *cursor;
    if (it >= end || *it != '"') {
        *out_error = "JSON string must start with quote";
        return false;
    }

    it++;
    capacity = 32;
    len = 0;
    buffer = (char *)malloc(capacity);
    if (buffer == NULL) {
        *out_error = "failed to allocate JSON string buffer";
        return false;
    }
    buffer[0] = '\0';

    while (it < end) {
        unsigned char ch = (unsigned char)*it;
        it++;

        if (ch == '"') {
            *out_value = zt_text_from_utf8(buffer, len);
            free(buffer);
            *cursor = it;
            return true;
        }

        if (ch == '\\') {
            unsigned char esc;
            if (it >= end) {
                free(buffer);
                *out_error = "unterminated JSON escape sequence";
                return false;
            }

            esc = (unsigned char)*it;
            it++;

            switch (esc) {
                case '"': zt_json_buffer_append_char(&buffer, &len, &capacity, '"'); break;
                case '\\': zt_json_buffer_append_char(&buffer, &len, &capacity, '\\'); break;
                case '/': zt_json_buffer_append_char(&buffer, &len, &capacity, '/'); break;
                case 'b': zt_json_buffer_append_char(&buffer, &len, &capacity, '\b'); break;
                case 'f': zt_json_buffer_append_char(&buffer, &len, &capacity, '\f'); break;
                case 'n': zt_json_buffer_append_char(&buffer, &len, &capacity, '\n'); break;
                case 'r': zt_json_buffer_append_char(&buffer, &len, &capacity, '\r'); break;
                case 't': zt_json_buffer_append_char(&buffer, &len, &capacity, '\t'); break;
                case 'u':
                    free(buffer);
                    *out_error = "unicode escapes are not supported in std.json MVP";
                    return false;
                default:
                    free(buffer);
                    *out_error = "invalid JSON escape sequence";
                    return false;
            }

            continue;
        }

        if (ch < 0x20) {
            free(buffer);
            *out_error = "control character in JSON string";
            return false;
        }

        zt_json_buffer_append_char(&buffer, &len, &capacity, (char)ch);
    }

    free(buffer);
    *out_error = "unterminated JSON string";
    return false;
}

static zt_bool zt_json_parse_unquoted_value(const char **cursor, const char *end, zt_text **out_value, const char **out_error) {
    const char *start;
    const char *finish;
    const char *trimmed_start;
    const char *trimmed_end;

    if (cursor == NULL || out_value == NULL || out_error == NULL) {
        return false;
    }

    start = *cursor;
    finish = start;

    while (finish < end && *finish != ',' && *finish != '}') {
        finish++;
    }

    trimmed_start = start;
    trimmed_end = finish;

    while (trimmed_start < trimmed_end && isspace((unsigned char)*trimmed_start)) {
        trimmed_start++;
    }
    while (trimmed_end > trimmed_start && isspace((unsigned char)*(trimmed_end - 1))) {
        trimmed_end--;
    }

    if (trimmed_start == trimmed_end) {
        *out_error = "expected JSON value";
        return false;
    }

    if (*trimmed_start == '{' || *trimmed_start == '[') {
        *out_error = "nested JSON values are not supported in std.json MVP";
        return false;
    }

    *out_value = zt_text_from_utf8(trimmed_start, (size_t)(trimmed_end - trimmed_start));
    *cursor = finish;
    return true;
}

zt_outcome_map_text_text_core_error zt_json_parse_map_text_text(const zt_text *input) {
    const char *cursor;
    const char *end;
    zt_map_text_text *map;

    zt_runtime_require_text(input, "zt_json_parse_map_text_text requires input");

    cursor = input->data;
    end = input->data + input->len;
    cursor = zt_json_skip_whitespace(cursor, end);

    if (cursor >= end || *cursor != '{') {
        return zt_outcome_map_text_text_core_error_failure_message("std.json.parse expects a JSON object");
    }

    cursor++;
    map = zt_map_text_text_new();
    cursor = zt_json_skip_whitespace(cursor, end);

    if (cursor < end && *cursor == '}') {
        zt_outcome_map_text_text_core_error ok;
        cursor++;
        cursor = zt_json_skip_whitespace(cursor, end);
        if (cursor != end) {
            zt_release(map);
            return zt_outcome_map_text_text_core_error_failure_message("unexpected trailing content after JSON object");
        }
        ok = zt_outcome_map_text_text_core_error_success(map);
        zt_release(map);
        return ok;
    }

    while (cursor < end) {
        zt_text *key = NULL;
        zt_text *value_text = NULL;
        const char *error_message = NULL;

        cursor = zt_json_skip_whitespace(cursor, end);
        if (cursor >= end || *cursor != '"') {
            zt_release(map);
            return zt_outcome_map_text_text_core_error_failure_message("expected quoted JSON object key");
        }

        if (!zt_json_parse_string(&cursor, end, &key, &error_message)) {
            zt_release(map);
            return zt_outcome_map_text_text_core_error_failure_message(error_message);
        }

        cursor = zt_json_skip_whitespace(cursor, end);
        if (cursor >= end || *cursor != ':') {
            zt_release(key);
            zt_release(map);
            return zt_outcome_map_text_text_core_error_failure_message("expected ':' after JSON object key");
        }

        cursor++;
        cursor = zt_json_skip_whitespace(cursor, end);

        if (cursor < end && *cursor == '"') {
            if (!zt_json_parse_string(&cursor, end, &value_text, &error_message)) {
                zt_release(key);
                zt_release(map);
                return zt_outcome_map_text_text_core_error_failure_message(error_message);
            }
        } else {
            if (!zt_json_parse_unquoted_value(&cursor, end, &value_text, &error_message)) {
                zt_release(key);
                zt_release(map);
                return zt_outcome_map_text_text_core_error_failure_message(error_message);
            }
        }

        zt_map_text_text_set(map, key, value_text);
        zt_release(key);
        zt_release(value_text);

        cursor = zt_json_skip_whitespace(cursor, end);
        if (cursor < end && *cursor == ',') {
            cursor++;
            continue;
        }

        if (cursor < end && *cursor == '}') {
            zt_outcome_map_text_text_core_error ok;
            cursor++;
            cursor = zt_json_skip_whitespace(cursor, end);
            if (cursor != end) {
                zt_release(map);
                return zt_outcome_map_text_text_core_error_failure_message("unexpected trailing content after JSON object");
            }
            ok = zt_outcome_map_text_text_core_error_success(map);
            zt_release(map);
            return ok;
        }

        zt_release(map);
        return zt_outcome_map_text_text_core_error_failure_message("expected ',' or '}' in JSON object");
    }

    zt_release(map);
    return zt_outcome_map_text_text_core_error_failure_message("unterminated JSON object");
}

zt_text *zt_json_stringify_map_text_text(const zt_map_text_text *value) {
    char *buffer;
    size_t len;
    size_t capacity;
    zt_int count;
    zt_int i;

    zt_runtime_require_map_text_text(value, "zt_json_stringify_map_text_text requires map");

    capacity = 64;
    len = 0;
    buffer = (char *)malloc(capacity);
    if (buffer == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate JSON buffer");
    }
    buffer[0] = '\0';

    zt_json_buffer_append_char(&buffer, &len, &capacity, '{');

    count = zt_map_text_text_len(value);
    for (i = 0; i < count; i++) {
        zt_text *key = zt_map_text_text_key_at(value, i);
        zt_text *item_value = zt_map_text_text_value_at(value, i);

        if (i > 0) {
            zt_json_buffer_append_char(&buffer, &len, &capacity, ',');
        }

        zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
        zt_json_buffer_append_escaped_text(&buffer, &len, &capacity, key);
        zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
        zt_json_buffer_append_char(&buffer, &len, &capacity, ':');
        zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
        zt_json_buffer_append_escaped_text(&buffer, &len, &capacity, item_value);
        zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
    }

    zt_json_buffer_append_char(&buffer, &len, &capacity, '}');

    {
        zt_text *result = zt_text_from_utf8(buffer, len);
        free(buffer);
        return result;
    }
}

zt_text *zt_json_pretty_map_text_text(const zt_map_text_text *value, zt_int indent) {
    char *buffer;
    size_t len;
    size_t capacity;
    zt_int count;
    zt_int i;
    size_t indent_size;

    zt_runtime_require_map_text_text(value, "zt_json_pretty_map_text_text requires map");

    indent_size = indent > 0 ? (size_t)indent : 0;
    capacity = 64;
    len = 0;
    buffer = (char *)malloc(capacity);
    if (buffer == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate JSON buffer");
    }
    buffer[0] = '\0';

    count = zt_map_text_text_len(value);
    if (count == 0) {
        zt_json_buffer_append_char(&buffer, &len, &capacity, '{');
        zt_json_buffer_append_char(&buffer, &len, &capacity, '}');
    } else {
        zt_json_buffer_append_char(&buffer, &len, &capacity, '{');
        zt_json_buffer_append_char(&buffer, &len, &capacity, '\n');

        for (i = 0; i < count; i++) {
            zt_text *key = zt_map_text_text_key_at(value, i);
            zt_text *item_value = zt_map_text_text_value_at(value, i);
            size_t spacing;

            for (spacing = 0; spacing < indent_size; spacing++) {
                zt_json_buffer_append_char(&buffer, &len, &capacity, ' ');
            }

            zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
            zt_json_buffer_append_escaped_text(&buffer, &len, &capacity, key);
            zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
            zt_json_buffer_append_char(&buffer, &len, &capacity, ':');
            zt_json_buffer_append_char(&buffer, &len, &capacity, ' ');
            zt_json_buffer_append_char(&buffer, &len, &capacity, '"');
            zt_json_buffer_append_escaped_text(&buffer, &len, &capacity, item_value);
            zt_json_buffer_append_char(&buffer, &len, &capacity, '"');

            if (i + 1 < count) {
                zt_json_buffer_append_char(&buffer, &len, &capacity, ',');
            }

            zt_json_buffer_append_char(&buffer, &len, &capacity, '\n');
        }

        zt_json_buffer_append_char(&buffer, &len, &capacity, '}');
    }

    {
        zt_text *result = zt_text_from_utf8(buffer, len);
        free(buffer);
        return result;
    }
}

static zt_bool zt_json_skip_raw_string(const char **cursor, const char *end) {
    const char *it;
    if (cursor == NULL || *cursor >= end || **cursor != '"') return false;
    it = *cursor + 1;
    while (it < end) {
        unsigned char ch = (unsigned char)*it++;
        if (ch == '"') {
            *cursor = it;
            return true;
        }
        if (ch == '\\') {
            if (it >= end) return false;
            if (*it == 'u') {
                size_t n;
                it++;
                for (n = 0; n < 4; n += 1) {
                    if (it >= end || !isxdigit((unsigned char)*it)) return false;
                    it++;
                }
            } else {
                unsigned char esc = (unsigned char)*it++;
                if (!(esc == '"' || esc == '\\' || esc == '/' || esc == 'b' ||
                        esc == 'f' || esc == 'n' || esc == 'r' || esc == 't')) return false;
            }
        } else if (ch < 0x20) {
            return false;
        }
    }
    return false;
}

static zt_bool zt_json_skip_raw_value(const char **cursor, const char *end);

static zt_bool zt_json_skip_raw_array(const char **cursor, const char *end) {
    const char *it = *cursor + 1;
    it = zt_json_skip_whitespace(it, end);
    if (it < end && *it == ']') {
        *cursor = it + 1;
        return true;
    }
    while (it < end) {
        if (!zt_json_skip_raw_value(&it, end)) return false;
        it = zt_json_skip_whitespace(it, end);
        if (it < end && *it == ',') {
            it++;
            it = zt_json_skip_whitespace(it, end);
            continue;
        }
        if (it < end && *it == ']') {
            *cursor = it + 1;
            return true;
        }
        return false;
    }
    return false;
}

static zt_bool zt_json_skip_raw_object(const char **cursor, const char *end) {
    const char *it = *cursor + 1;
    it = zt_json_skip_whitespace(it, end);
    if (it < end && *it == '}') {
        *cursor = it + 1;
        return true;
    }
    while (it < end) {
        if (!zt_json_skip_raw_string(&it, end)) return false;
        it = zt_json_skip_whitespace(it, end);
        if (it >= end || *it != ':') return false;
        it++;
        it = zt_json_skip_whitespace(it, end);
        if (!zt_json_skip_raw_value(&it, end)) return false;
        it = zt_json_skip_whitespace(it, end);
        if (it < end && *it == ',') {
            it++;
            it = zt_json_skip_whitespace(it, end);
            continue;
        }
        if (it < end && *it == '}') {
            *cursor = it + 1;
            return true;
        }
        return false;
    }
    return false;
}

static zt_bool zt_json_skip_raw_number(const char **cursor, const char *end) {
    const char *it = *cursor;
    if (it < end && *it == '-') it++;
    if (it >= end) return false;
    if (*it == '0') {
        it++;
    } else if (isdigit((unsigned char)*it)) {
        while (it < end && isdigit((unsigned char)*it)) it++;
    } else {
        return false;
    }
    if (it < end && *it == '.') {
        it++;
        if (it >= end || !isdigit((unsigned char)*it)) return false;
        while (it < end && isdigit((unsigned char)*it)) it++;
    }
    if (it < end && (*it == 'e' || *it == 'E')) {
        it++;
        if (it < end && (*it == '+' || *it == '-')) it++;
        if (it >= end || !isdigit((unsigned char)*it)) return false;
        while (it < end && isdigit((unsigned char)*it)) it++;
    }
    *cursor = it;
    return true;
}

static zt_bool zt_json_match_literal(const char **cursor, const char *end, const char *literal) {
    size_t len = strlen(literal);
    if ((size_t)(end - *cursor) < len) return false;
    if (memcmp(*cursor, literal, len) != 0) return false;
    *cursor += len;
    return true;
}

static zt_bool zt_json_consumed_all_number(const char *cursor) {
    if (cursor == NULL) return false;
    while (*cursor != '\0') {
        if (!isspace((unsigned char)*cursor)) return false;
        cursor++;
    }
    return true;
}

static zt_bool zt_json_skip_raw_value(const char **cursor, const char *end) {
    const char *it;
    if (cursor == NULL) return false;
    it = zt_json_skip_whitespace(*cursor, end);
    if (it >= end) return false;
    if (*it == '"') {
        if (!zt_json_skip_raw_string(&it, end)) return false;
    } else if (*it == '{') {
        if (!zt_json_skip_raw_object(&it, end)) return false;
    } else if (*it == '[') {
        if (!zt_json_skip_raw_array(&it, end)) return false;
    } else if (*it == '-' || isdigit((unsigned char)*it)) {
        if (!zt_json_skip_raw_number(&it, end)) return false;
    } else if (!zt_json_match_literal(&it, end, "true") &&
               !zt_json_match_literal(&it, end, "false") &&
               !zt_json_match_literal(&it, end, "null")) {
        return false;
    }
    *cursor = it;
    return true;
}

static zt_bool zt_json_raw_bounds(const zt_text *input, const char **start, const char **finish) {
    const char *cursor;
    const char *end;
    zt_runtime_require_text(input, "json helper requires text");
    cursor = zt_json_skip_whitespace(input->data, input->data + input->len);
    end = input->data + input->len;
    if (start != NULL) *start = cursor;
    if (!zt_json_skip_raw_value(&cursor, end)) return false;
    cursor = zt_json_skip_whitespace(cursor, end);
    if (cursor != end) return false;
    if (finish != NULL) *finish = cursor;
    return true;
}

zt_outcome_text_core_error zt_json_validate_full(const zt_text *input) {
    const char *start;
    const char *finish;
    zt_text *raw;
    zt_outcome_text_core_error out;
    if (!zt_json_raw_bounds(input, &start, &finish)) {
        return zt_outcome_text_core_error_failure_message("std.json.parse_value expects valid JSON");
    }
    raw = zt_text_from_utf8(start, (size_t)(finish - start));
    out = zt_outcome_text_core_error_success(raw);
    zt_release(raw);
    return out;
}

zt_text *zt_json_pretty_full(const zt_text *input, zt_int indent) {
    const char *start;
    const char *finish;
    (void)indent;
    if (!zt_json_raw_bounds(input, &start, &finish)) {
        return zt_text_from_utf8_literal("null");
    }
    return zt_text_from_utf8(start, (size_t)(finish - start));
}

zt_int zt_json_kind_index(const zt_text *input) {
    const char *start;
    if (!zt_json_raw_bounds(input, &start, NULL)) return 0;
    if (*start == 't' || *start == 'f') return 1;
    if (*start == '-' || isdigit((unsigned char)*start)) return 2;
    if (*start == '"') return 3;
    if (*start == '[') return 4;
    if (*start == '{') return 5;
    return 0;
}

zt_optional_text zt_json_as_text(const zt_text *input) {
    const char *cursor;
    const char *end;
    zt_text *parsed = NULL;
    const char *error = NULL;
    zt_optional_text result;
    if (!zt_json_raw_bounds(input, &cursor, NULL) || *cursor != '"') return zt_optional_text_empty();
    end = input->data + input->len;
    if (!zt_json_parse_string(&cursor, end, &parsed, &error)) return zt_optional_text_empty();
    result = zt_optional_text_present(parsed);
    zt_release(parsed);
    return result;
}

zt_optional_i64 zt_json_as_int(const zt_text *input) {
    const char *start;
    char buffer[96];
    char *endptr = NULL;
    long long parsed;
    if (!zt_json_raw_bounds(input, &start, NULL)) return zt_optional_i64_empty();
    if (!(start[0] == '-' || isdigit((unsigned char)start[0]))) return zt_optional_i64_empty();
    if (strchr(start, '.') != NULL || strchr(start, 'e') != NULL || strchr(start, 'E') != NULL) return zt_optional_i64_empty();
    snprintf(buffer, sizeof(buffer), "%.*s", (int)(input->data + input->len - start), start);
    errno = 0;
    parsed = strtoll(buffer, &endptr, 10);
    if (errno != 0 || endptr == buffer || !zt_json_consumed_all_number(endptr)) return zt_optional_i64_empty();
    return zt_optional_i64_present((zt_int)parsed);
}

zt_optional_f64 zt_json_as_float(const zt_text *input) {
    const char *start;
    char buffer[128];
    char *endptr = NULL;
    double parsed;
    if (!zt_json_raw_bounds(input, &start, NULL)) return zt_optional_f64_empty();
    if (!(start[0] == '-' || isdigit((unsigned char)start[0]))) return zt_optional_f64_empty();
    snprintf(buffer, sizeof(buffer), "%.*s", (int)(input->data + input->len - start), start);
    errno = 0;
    parsed = strtod(buffer, &endptr);
    if (errno != 0 || endptr == buffer || !zt_json_consumed_all_number(endptr)) return zt_optional_f64_empty();
    return zt_optional_f64_present((zt_float)parsed);
}

zt_optional_bool zt_json_as_bool(const zt_text *input) {
    const char *start;
    if (!zt_json_raw_bounds(input, &start, NULL)) return zt_optional_bool_empty();
    if (strncmp(start, "true", 4) == 0) return zt_optional_bool_present(true);
    if (strncmp(start, "false", 5) == 0) return zt_optional_bool_present(false);
    return zt_optional_bool_empty();
}

static zt_optional_text zt_json_raw_slice_optional(const char *start, const char *finish) {
    zt_text *raw = zt_text_from_utf8(start, (size_t)(finish - start));
    zt_optional_text result = zt_optional_text_present(raw);
    zt_release(raw);
    return result;
}

zt_optional_text zt_json_get_raw(const zt_text *input, const zt_text *key) {
    const char *it;
    const char *end;
    zt_runtime_require_text(key, "json.get requires key");
    if (!zt_json_raw_bounds(input, &it, NULL) || *it != '{') return zt_optional_text_empty();
    end = input->data + input->len;
    it++;
    it = zt_json_skip_whitespace(it, end);
    while (it < end && *it != '}') {
        zt_text *parsed_key = NULL;
        const char *error = NULL;
        const char *value_start;
        const char *value_end;
        if (!zt_json_parse_string(&it, end, &parsed_key, &error)) return zt_optional_text_empty();
        it = zt_json_skip_whitespace(it, end);
        if (it >= end || *it != ':') {
            zt_release(parsed_key);
            return zt_optional_text_empty();
        }
        it++;
        it = zt_json_skip_whitespace(it, end);
        value_start = it;
        if (!zt_json_skip_raw_value(&it, end)) {
            zt_release(parsed_key);
            return zt_optional_text_empty();
        }
        value_end = it;
        if (zt_text_eq(parsed_key, key)) {
            zt_release(parsed_key);
            return zt_json_raw_slice_optional(value_start, value_end);
        }
        zt_release(parsed_key);
        it = zt_json_skip_whitespace(it, end);
        if (it < end && *it == ',') {
            it++;
            it = zt_json_skip_whitespace(it, end);
        }
    }
    return zt_optional_text_empty();
}

zt_optional_text zt_json_at_raw(const zt_text *input, zt_int index) {
    const char *it;
    const char *end;
    zt_int current = 0;
    if (index < 0 || !zt_json_raw_bounds(input, &it, NULL) || *it != '[') return zt_optional_text_empty();
    end = input->data + input->len;
    it++;
    it = zt_json_skip_whitespace(it, end);
    while (it < end && *it != ']') {
        const char *value_start = it;
        const char *value_end;
        if (!zt_json_skip_raw_value(&it, end)) return zt_optional_text_empty();
        value_end = it;
        if (current == index) return zt_json_raw_slice_optional(value_start, value_end);
        current += 1;
        it = zt_json_skip_whitespace(it, end);
        if (it < end && *it == ',') {
            it++;
            it = zt_json_skip_whitespace(it, end);
        }
    }
    return zt_optional_text_empty();
}

zt_int zt_json_len(const zt_text *input) {
    const char *it;
    const char *end;
    zt_int count = 0;
    if (!zt_json_raw_bounds(input, &it, NULL)) return 0;
    end = input->data + input->len;
    if (*it != '[' && *it != '{') return 0;
    if (*it == '[') {
        it++;
        it = zt_json_skip_whitespace(it, end);
        while (it < end && *it != ']') {
            if (!zt_json_skip_raw_value(&it, end)) return 0;
            count += 1;
            it = zt_json_skip_whitespace(it, end);
            if (it < end && *it == ',') it++;
            it = zt_json_skip_whitespace(it, end);
        }
        return count;
    }
    it++;
    it = zt_json_skip_whitespace(it, end);
    while (it < end && *it != '}') {
        if (!zt_json_skip_raw_string(&it, end)) return 0;
        it = zt_json_skip_whitespace(it, end);
        if (it >= end || *it != ':') return 0;
        it++;
        if (!zt_json_skip_raw_value(&it, end)) return 0;
        count += 1;
        it = zt_json_skip_whitespace(it, end);
        if (it < end && *it == ',') it++;
        it = zt_json_skip_whitespace(it, end);
    }
    return count;
}

