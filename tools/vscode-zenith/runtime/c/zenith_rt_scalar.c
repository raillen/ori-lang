zt_float zt_int_to_float(zt_int value) {
    return (zt_float)value;
}

zt_text *zt_int_to_text(zt_int value) {
    char buffer[64];
    snprintf(buffer, sizeof(buffer), "%lld", (long long)value);
    return zt_text_from_utf8_literal(buffer);
}

static zt_bool zt_parse_consumed_all(const char *cursor) {
    if (cursor == NULL) return false;
    while (*cursor != '\0') {
        if (!isspace((unsigned char)*cursor)) return false;
        cursor++;
    }
    return true;
}

zt_optional_i64 zt_int_parse(const zt_text *value) {
    char *end = NULL;
    long long parsed;

    zt_runtime_require_text(value, "int.parse requires text");
    if (value->data == NULL || value->len == 0) {
        return zt_optional_i64_empty();
    }

    errno = 0;
    parsed = strtoll(value->data, &end, 10);
    if (end == value->data || errno == ERANGE || !zt_parse_consumed_all(end)) {
        return zt_optional_i64_empty();
    }

    return zt_optional_i64_present((zt_int)parsed);
}

zt_int zt_float_to_int(zt_float value) {
    return (zt_int)value;
}

zt_int zt_float_round_to_int(zt_float value) {
    return (zt_int)round(value);
}

zt_text *zt_float_to_text(zt_float value) {
    char buffer[128];
    snprintf(buffer, sizeof(buffer), "%.17g", (double)value);
    return zt_text_from_utf8_literal(buffer);
}

zt_optional_f64 zt_float_parse(const zt_text *value) {
    char *end = NULL;
    double parsed;

    zt_runtime_require_text(value, "float.parse requires text");
    if (value->data == NULL || value->len == 0) {
        return zt_optional_f64_empty();
    }

    errno = 0;
    parsed = strtod(value->data, &end);
    if (end == value->data || errno == ERANGE || !zt_parse_consumed_all(end)) {
        return zt_optional_f64_empty();
    }

    return zt_optional_f64_present((zt_float)parsed);
}

zt_text *zt_bool_to_text(zt_bool value) {
    return zt_text_from_utf8_literal(value ? "true" : "false");
}

