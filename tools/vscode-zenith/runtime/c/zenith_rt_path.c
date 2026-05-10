static zt_bool zt_path_is_separator_char(char value) {
    return value == '/' || value == '\\';
}

static zt_bool zt_path_is_drive_letter(char value) {
    return (value >= 'A' && value <= 'Z') || (value >= 'a' && value <= 'z');
}

static void zt_path_parse_prefix(const char *data, size_t len, size_t *start, zt_bool *is_absolute, char drive[3]) {
    size_t cursor = 0;

    if (start == NULL || is_absolute == NULL || drive == NULL) {
        return;
    }

    *start = 0;
    *is_absolute = false;
    drive[0] = '\0';
    drive[1] = '\0';
    drive[2] = '\0';

    if (data == NULL || len == 0) {
        return;
    }

    if (len >= 2 && zt_path_is_drive_letter(data[0]) && data[1] == ':') {
        drive[0] = (char)toupper((unsigned char)data[0]);
        drive[1] = ':';
        drive[2] = '\0';
        cursor = 2;

        if (cursor < len && zt_path_is_separator_char(data[cursor])) {
            *is_absolute = true;
            cursor += 1;
            while (cursor < len && zt_path_is_separator_char(data[cursor])) {
                cursor += 1;
            }
        }

        *start = cursor;
        return;
    }

    if (zt_path_is_separator_char(data[0])) {
        *is_absolute = true;
        cursor = 1;
        while (cursor < len && zt_path_is_separator_char(data[cursor])) {
            cursor += 1;
        }
        *start = cursor;
    }
}

static zt_bool zt_path_segment_is_dot(const char *data, size_t start, size_t length) {
    return length == 1 && data[start] == '.';
}

static zt_bool zt_path_segment_is_dot_dot(const char *data, size_t start, size_t length) {
    return length == 2 && data[start] == '.' && data[start + 1] == '.';
}

static void zt_path_collect_segments(
        const char *data,
        size_t len,
        size_t start,
        size_t *segment_starts,
        size_t *segment_lengths,
        size_t *out_count) {
    size_t index;
    size_t count;

    if (out_count == NULL) {
        return;
    }

    *out_count = 0;
    if (data == NULL || segment_starts == NULL || segment_lengths == NULL) {
        return;
    }

    index = start;
    count = 0;

    while (index < len) {
        size_t part_start;
        size_t part_len;

        while (index < len && zt_path_is_separator_char(data[index])) {
            index += 1;
        }

        part_start = index;

        while (index < len && !zt_path_is_separator_char(data[index])) {
            index += 1;
        }

        part_len = index - part_start;
        if (part_len == 0) {
            continue;
        }

        segment_starts[count] = part_start;
        segment_lengths[count] = part_len;
        count += 1;
    }

    *out_count = count;
}


zt_text *zt_path_normalize(const zt_text *value) {
    const char *data;
    size_t len;
    size_t start;
    zt_bool absolute;
    char drive[3];
    size_t max_segments;
    size_t *segment_starts;
    size_t *segment_lengths;
    size_t segment_count;
    size_t index;
    char *output;
    size_t output_capacity;
    size_t output_length;
    zt_text *result;

    zt_runtime_require_text(value, "zt_path_normalize requires text");

    data = value->data;
    len = value->len;
    start = 0;
    absolute = false;
    zt_path_parse_prefix(data, len, &start, &absolute, drive);

    max_segments = len + 1;
    segment_starts = (size_t *)malloc(max_segments * sizeof(size_t));
    segment_lengths = (size_t *)malloc(max_segments * sizeof(size_t));
    if (segment_starts == NULL || segment_lengths == NULL) {
        free(segment_starts);
        free(segment_lengths);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate path normalization buffers");
    }

    segment_count = 0;
    index = start;

    while (index < len) {
        size_t part_start;
        size_t part_len;

        while (index < len && zt_path_is_separator_char(data[index])) {
            index += 1;
        }

        part_start = index;

        while (index < len && !zt_path_is_separator_char(data[index])) {
            index += 1;
        }

        part_len = index - part_start;
        if (part_len == 0) {
            continue;
        }

        if (zt_path_segment_is_dot(data, part_start, part_len)) {
            continue;
        }

        if (zt_path_segment_is_dot_dot(data, part_start, part_len)) {
            if (segment_count > 0) {
                size_t prev_start = segment_starts[segment_count - 1];
                size_t prev_len = segment_lengths[segment_count - 1];
                if (!zt_path_segment_is_dot_dot(data, prev_start, prev_len)) {
                    segment_count -= 1;
                    continue;
                }
            }

            if (!absolute) {
                segment_starts[segment_count] = part_start;
                segment_lengths[segment_count] = part_len;
                segment_count += 1;
            }
            continue;
        }

        segment_starts[segment_count] = part_start;
        segment_lengths[segment_count] = part_len;
        segment_count += 1;
    }

    output_capacity = (len * 2) + 16;
    output = (char *)malloc(output_capacity);
    if (output == NULL) {
        free(segment_starts);
        free(segment_lengths);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate normalized path output");
    }

    output_length = 0;

    if (drive[0] != '\0') {
        output[output_length++] = drive[0];
        output[output_length++] = ':';
        if (absolute) {
            output[output_length++] = '/';
        }
    } else if (absolute) {
        output[output_length++] = '/';
    }

    if (segment_count == 0) {
        if (output_length == 0) {
            output[output_length++] = '.';
        }
    } else {
        for (index = 0; index < segment_count; index += 1) {
            zt_bool suppress_separator = (drive[0] != '\0' && !absolute && output_length == 2);
            if (output_length > 0 && output[output_length - 1] != '/' && !suppress_separator) {
                output[output_length++] = '/';
            }
            memcpy(output + output_length, data + segment_starts[index], segment_lengths[index]);
            output_length += segment_lengths[index];
        }
    }

    output[output_length] = '\0';
    result = zt_text_from_utf8(output, output_length);

    free(output);
    free(segment_starts);
    free(segment_lengths);

    return result;
}

zt_bool zt_path_is_absolute(const zt_text *value) {
    size_t start = 0;
    zt_bool absolute = false;
    char drive[3];

    zt_runtime_require_text(value, "zt_path_is_absolute requires text");
    zt_path_parse_prefix(value->data, value->len, &start, &absolute, drive);
    return absolute;
}

zt_text *zt_path_absolute(const zt_text *value, const zt_text *base) {
    zt_text *normalized_path;
    zt_text *normalized_base;
    zt_text *result;

    zt_runtime_require_text(value, "zt_path_absolute requires path");
    zt_runtime_require_text(base, "zt_path_absolute requires base");

    normalized_path = zt_path_normalize(value);
    if (zt_path_is_absolute(normalized_path)) {
        return normalized_path;
    }

    normalized_base = zt_path_normalize(base);
    if (normalized_base->len == 0 || (normalized_base->len == 1 && normalized_base->data[0] == '.')) {
        result = zt_path_normalize(normalized_path);
        zt_release(normalized_path);
        zt_release(normalized_base);
        return result;
    }

    {
        zt_text *slash = zt_text_from_utf8_literal("/");
        zt_text *combined;

        if (normalized_base->len > 0 && zt_path_is_separator_char(normalized_base->data[normalized_base->len - 1])) {
            combined = zt_text_concat(normalized_base, normalized_path);
        } else {
            zt_text *with_separator = zt_text_concat(normalized_base, slash);
            combined = zt_text_concat(with_separator, normalized_path);
            zt_release(with_separator);
        }

        result = zt_path_normalize(combined);

        zt_release(combined);
        zt_release(slash);
    }

    zt_release(normalized_path);
    zt_release(normalized_base);

    return result;
}

zt_text *zt_path_relative(const zt_text *value, const zt_text *from) {
    zt_text *normalized_value;
    zt_text *normalized_from;
    size_t value_start;
    size_t from_start;
    zt_bool value_absolute;
    zt_bool from_absolute;
    char value_drive[3];
    char from_drive[3];
    size_t *value_starts;
    size_t *value_lengths;
    size_t value_count;
    size_t *from_starts;
    size_t *from_lengths;
    size_t from_count;
    size_t common;
    char *buffer;
    size_t capacity;
    size_t length;
    size_t index;
    zt_text *result;

    zt_runtime_require_text(value, "zt_path_relative requires path");
    zt_runtime_require_text(from, "zt_path_relative requires from");

    normalized_value = zt_path_normalize(value);
    normalized_from = zt_path_normalize(from);

    value_start = 0;
    value_absolute = false;
    value_drive[0] = '\0';
    value_drive[1] = '\0';
    value_drive[2] = '\0';
    zt_path_parse_prefix(normalized_value->data, normalized_value->len, &value_start, &value_absolute, value_drive);

    from_start = 0;
    from_absolute = false;
    from_drive[0] = '\0';
    from_drive[1] = '\0';
    from_drive[2] = '\0';
    zt_path_parse_prefix(normalized_from->data, normalized_from->len, &from_start, &from_absolute, from_drive);

    if (value_absolute != from_absolute || strcmp(value_drive, from_drive) != 0) {
        result = zt_text_from_utf8(normalized_value->data, normalized_value->len);
        zt_release(normalized_value);
        zt_release(normalized_from);
        return result;
    }

    value_starts = (size_t *)malloc((normalized_value->len + 1) * sizeof(size_t));
    value_lengths = (size_t *)malloc((normalized_value->len + 1) * sizeof(size_t));
    from_starts = (size_t *)malloc((normalized_from->len + 1) * sizeof(size_t));
    from_lengths = (size_t *)malloc((normalized_from->len + 1) * sizeof(size_t));
    if (value_starts == NULL || value_lengths == NULL || from_starts == NULL || from_lengths == NULL) {
        free(value_starts);
        free(value_lengths);
        free(from_starts);
        free(from_lengths);
        zt_release(normalized_value);
        zt_release(normalized_from);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate path relative buffers");
    }

    zt_path_collect_segments(normalized_value->data, normalized_value->len, value_start, value_starts, value_lengths, &value_count);
    zt_path_collect_segments(normalized_from->data, normalized_from->len, from_start, from_starts, from_lengths, &from_count);

    common = 0;
    while (common < value_count && common < from_count) {
        size_t left_len = value_lengths[common];
        size_t right_len = from_lengths[common];
        if (left_len != right_len) {
            break;
        }
        if (memcmp(normalized_value->data + value_starts[common], normalized_from->data + from_starts[common], left_len) != 0) {
            break;
        }
        common += 1;
    }

    capacity = normalized_value->len + normalized_from->len + 8;
    buffer = (char *)malloc(capacity);
    if (buffer == NULL) {
        free(value_starts);
        free(value_lengths);
        free(from_starts);
        free(from_lengths);
        zt_release(normalized_value);
        zt_release(normalized_from);
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate relative path output");
    }

    length = 0;

    for (index = common; index < from_count; index += 1) {
        if (length > 0) {
            buffer[length++] = '/';
        }
        buffer[length++] = '.';
        buffer[length++] = '.';
    }

    for (index = common; index < value_count; index += 1) {
        if (length > 0) {
            buffer[length++] = '/';
        }
        memcpy(buffer + length, normalized_value->data + value_starts[index], value_lengths[index]);
        length += value_lengths[index];
    }

    if (length == 0) {
        buffer[length++] = '.';
    }

    buffer[length] = '\0';
    result = zt_text_from_utf8(buffer, length);

    free(buffer);
    free(value_starts);
    free(value_lengths);
    free(from_starts);
    free(from_lengths);
    zt_release(normalized_value);
    zt_release(normalized_from);

    return result;
}

