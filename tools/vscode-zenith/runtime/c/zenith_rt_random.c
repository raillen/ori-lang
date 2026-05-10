zt_outcome_f64_core_error zt_random_float_between_core(zt_float min, zt_float max) {
    zt_int raw;
    zt_float unit;

    if (max < min) {
        return zt_outcome_f64_core_error_failure_message("std.random.float_between expects max >= min");
    }
    if (max == min) {
        return zt_outcome_f64_core_error_success(min);
    }

    raw = zt_host_random_next_i64();
    if (raw < 0) raw = 0 - raw;
    unit = (zt_float)(raw % 1000000) / 1000000.0;
    return zt_outcome_f64_core_error_success(min + ((max - min) * unit));
}

static size_t zt_random_index_for_len(size_t len) {
    zt_int raw;
    if (len == 0) return 0;
    raw = zt_host_random_next_i64();
    if (raw < 0) raw = raw == INT64_MIN ? INT64_MAX : 0 - raw;
    return (size_t)(raw % (zt_int)len);
}

zt_optional_i64 zt_random_choice_i64(const zt_list_i64 *items) {
    if (items == NULL || items->len == 0) return zt_optional_i64_empty();
    return zt_optional_i64_present(items->data[zt_random_index_for_len(items->len)]);
}

zt_optional_text zt_random_choice_text(const zt_list_text *items) {
    if (items == NULL || items->len == 0) return zt_optional_text_empty();
    return zt_optional_text_present(items->data[zt_random_index_for_len(items->len)]);
}

zt_list_i64 *zt_random_shuffle_i64(const zt_list_i64 *items) {
    zt_list_i64 *copy;
    size_t index;

    if (items == NULL) return zt_list_i64_new();
    copy = zt_list_i64_from_array(items->data, items->len);
    index = copy->len;
    while (index > 1) {
        size_t swap_index = zt_random_index_for_len(index);
        size_t last_index = index - 1;
        zt_int temp = copy->data[last_index];
        copy->data[last_index] = copy->data[swap_index];
        copy->data[swap_index] = temp;
        index -= 1;
    }
    return copy;
}

zt_list_text *zt_random_shuffle_text(const zt_list_text *items) {
    zt_list_text *copy;
    size_t index;

    if (items == NULL) return zt_list_text_new();
    copy = zt_list_text_from_array(items->data, items->len);
    index = copy->len;
    while (index > 1) {
        size_t swap_index = zt_random_index_for_len(index);
        size_t last_index = index - 1;
        zt_text *temp = copy->data[last_index];
        copy->data[last_index] = copy->data[swap_index];
        copy->data[swap_index] = temp;
        index -= 1;
    }
    return copy;
}

