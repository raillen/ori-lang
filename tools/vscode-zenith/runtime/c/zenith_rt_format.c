static uint64_t zt_u64_magnitude(zt_int value) {
    if (value >= 0) {
        return (uint64_t)value;
    }
    return (uint64_t)(-(value + 1)) + 1u;
}

static zt_int zt_format_clamp_decimals(zt_int decimals) {
    if (decimals < 0) return 0;
    if (decimals > 9) return 9;
    return decimals;
}

static zt_bool zt_unix_ms_to_utc_tm(zt_int millis, struct tm *out_tm, int *out_ms_part) {
    zt_int seconds;
    zt_int ms_part;
    time_t epoch_seconds;

    if (out_tm == NULL) return false;

    seconds = millis / 1000;
    ms_part = millis % 1000;
    if (ms_part < 0) {
        ms_part += 1000;
        seconds -= 1;
    }

    epoch_seconds = (time_t)seconds;
    if ((zt_int)epoch_seconds != seconds) {
        return false;
    }

#ifdef _WIN32
    if (gmtime_s(out_tm, &epoch_seconds) != 0) return false;
#else
    if (gmtime_r(&epoch_seconds, out_tm) == NULL) return false;
#endif

    if (out_ms_part != NULL) {
        *out_ms_part = (int)ms_part;
    }
    return true;
}

zt_text *zt_format_number(zt_float value, zt_int decimals) {
    char format_spec[16];
    char buffer[128];
    zt_int clamped_decimals = zt_format_clamp_decimals(decimals);

    snprintf(format_spec, sizeof(format_spec), "%%.%df", (int)clamped_decimals);
    snprintf(buffer, sizeof(buffer), format_spec, (double)value);
    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_percent(zt_float value, zt_int decimals) {
    char format_spec[20];
    char buffer[128];
    zt_int clamped_decimals = zt_format_clamp_decimals(decimals);

    snprintf(format_spec, sizeof(format_spec), "%%.%df%%%%", (int)clamped_decimals);
    snprintf(buffer, sizeof(buffer), format_spec, (double)(value * 100.0));
    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_date(zt_int millis, const zt_text *style) {
    struct tm utc_tm;
    const char *pattern = "%Y-%m-%d";
    char buffer[128];

    if (zt_text_equals_literal(style, "long")) {
        pattern = "%A, %d %B %Y";
    } else if (zt_text_equals_literal(style, "short")) {
        pattern = "%Y-%m-%d";
    } else if (zt_text_equals_literal(style, "iso")) {
        pattern = "%Y-%m-%d";
    }

    if (!zt_unix_ms_to_utc_tm(millis, &utc_tm, NULL)) {
        return zt_text_from_utf8_literal("1970-01-01");
    }

    if (strftime(buffer, sizeof(buffer), pattern, &utc_tm) == 0) {
        return zt_text_from_utf8_literal("1970-01-01");
    }
    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_datetime(zt_int millis, const zt_text *style, const zt_text *locale) {
    struct tm utc_tm;
    int ms_part = 0;
    char buffer[160];
    (void)locale;

    if (!zt_unix_ms_to_utc_tm(millis, &utc_tm, &ms_part)) {
        return zt_text_from_utf8_literal("1970-01-01T00:00:00.000Z");
    }

    if (zt_text_equals_literal(style, "short")) {
        if (strftime(buffer, sizeof(buffer), "%Y-%m-%d %H:%M", &utc_tm) == 0) {
            return zt_text_from_utf8_literal("1970-01-01 00:00");
        }
        return zt_text_from_utf8_literal(buffer);
    }

    if (zt_text_equals_literal(style, "long")) {
        if (strftime(buffer, sizeof(buffer), "%Y-%m-%d %H:%M:%S UTC", &utc_tm) == 0) {
            return zt_text_from_utf8_literal("1970-01-01 00:00:00 UTC");
        }
        return zt_text_from_utf8_literal(buffer);
    }

    if (strftime(buffer, sizeof(buffer), "%Y-%m-%dT%H:%M:%S", &utc_tm) == 0) {
        return zt_text_from_utf8_literal("1970-01-01T00:00:00.000Z");
    }

    {
        char iso_buffer[176];
        snprintf(iso_buffer, sizeof(iso_buffer), "%s.%03dZ", buffer, ms_part);
        return zt_text_from_utf8_literal(iso_buffer);
    }
}

zt_text *zt_format_date_pattern(zt_int millis, const zt_text *pattern) {
    struct tm utc_tm;
    const char *strftime_pattern = "%Y-%m-%d";
    char buffer[128];

    if (zt_text_equals_literal(pattern, "yyyy-MM-dd")) {
        strftime_pattern = "%Y-%m-%d";
    } else if (zt_text_equals_literal(pattern, "dd/MM/yyyy")) {
        strftime_pattern = "%d/%m/%Y";
    } else if (zt_text_equals_literal(pattern, "MM/dd/yyyy")) {
        strftime_pattern = "%m/%d/%Y";
    }

    if (!zt_unix_ms_to_utc_tm(millis, &utc_tm, NULL)) {
        return zt_text_from_utf8_literal("1970-01-01");
    }

    if (strftime(buffer, sizeof(buffer), strftime_pattern, &utc_tm) == 0) {
        return zt_text_from_utf8_literal("1970-01-01");
    }
    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_datetime_pattern(zt_int millis, const zt_text *pattern) {
    struct tm utc_tm;
    const char *strftime_pattern = "%Y-%m-%dT%H:%M:%S";
    char buffer[160];

    if (zt_text_equals_literal(pattern, "yyyy-MM-dd HH:mm:ss")) {
        strftime_pattern = "%Y-%m-%d %H:%M:%S";
    } else if (zt_text_equals_literal(pattern, "yyyy-MM-ddTHH:mm:ss")) {
        strftime_pattern = "%Y-%m-%dT%H:%M:%S";
    } else if (zt_text_equals_literal(pattern, "HH:mm:ss")) {
        strftime_pattern = "%H:%M:%S";
    }

    if (!zt_unix_ms_to_utc_tm(millis, &utc_tm, NULL)) {
        return zt_text_from_utf8_literal("1970-01-01T00:00:00");
    }

    if (strftime(buffer, sizeof(buffer), strftime_pattern, &utc_tm) == 0) {
        return zt_text_from_utf8_literal("1970-01-01T00:00:00");
    }
    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_hex_i64(zt_int value) {
    char buffer[80];
    uint64_t magnitude = zt_u64_magnitude(value);

    if (value < 0) {
        snprintf(buffer, sizeof(buffer), "-%llx", (unsigned long long)magnitude);
    } else {
        snprintf(buffer, sizeof(buffer), "%llx", (unsigned long long)magnitude);
    }

    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_bin_i64(zt_int value) {
    char digits[65];
    size_t count = 0;
    uint64_t magnitude = zt_u64_magnitude(value);
    char buffer[96];

    if (magnitude == 0) {
        digits[count++] = '0';
    } else {
        while (magnitude > 0 && count < sizeof(digits)) {
            digits[count++] = (char)('0' + (magnitude & 1u));
            magnitude >>= 1u;
        }
    }

    if (value < 0) {
        size_t out = 0;
        buffer[out++] = '-';
        while (count > 0 && out + 1 < sizeof(buffer)) {
            buffer[out++] = digits[--count];
        }
        buffer[out] = '\0';
    } else {
        size_t out = 0;
        while (count > 0 && out + 1 < sizeof(buffer)) {
            buffer[out++] = digits[--count];
        }
        buffer[out] = '\0';
    }

    return zt_text_from_utf8_literal(buffer);
}

static zt_text *zt_format_bytes_impl(zt_int value, zt_float base, const char *const *units, size_t unit_count, zt_int decimals) {
    zt_float scaled = (zt_float)value;
    size_t unit_index = 0;
    zt_int clamped_decimals = decimals;
    char format_spec[16];
    char buffer[96];

    if (clamped_decimals < 0) {
        clamped_decimals = 0;
    }
    if (clamped_decimals > 6) {
        clamped_decimals = 6;
    }

    while ((scaled <= -base || scaled >= base) && (unit_index + 1) < unit_count) {
        scaled /= base;
        unit_index += 1;
    }

    snprintf(format_spec, sizeof(format_spec), "%%.%df %%s", (int)clamped_decimals);
    snprintf(buffer, sizeof(buffer), format_spec, (double)scaled, units[unit_index]);
    return zt_text_from_utf8_literal(buffer);
}

zt_text *zt_format_bytes_binary(zt_int value, zt_int decimals) {
    static const char *const units[] = {"B", "KiB", "MiB", "GiB", "TiB", "PiB"};
    return zt_format_bytes_impl(value, 1024.0, units, sizeof(units) / sizeof(units[0]), decimals);
}

zt_text *zt_format_bytes_decimal(zt_int value, zt_int decimals) {
    static const char *const units[] = {"B", "KB", "MB", "GB", "TB", "PB"};
    return zt_format_bytes_impl(value, 1000.0, units, sizeof(units) / sizeof(units[0]), decimals);
}

