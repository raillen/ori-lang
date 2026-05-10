zt_dyn_value *zt_dyn_box(void *data, zt_vtable *vtable) {
    zt_dyn_value *dyn;
    if (data == NULL || vtable == NULL) {
        return NULL;
    }
    dyn = (zt_dyn_value *)malloc(sizeof(zt_dyn_value));
    if (dyn == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate dyn box");
    }
    dyn->header.rc = 1;
    dyn->header.kind = ZT_HEAP_DYN_VALUE;
    dyn->data = data;
    dyn->vtable = vtable;
    /* Vtables are static const - no ref counting needed */
    return dyn;
}

static void zt_dyn_copy_bytes(void *dest, const void *src, size_t size) {
    size_t index;
    unsigned char *out = (unsigned char *)dest;
    const unsigned char *in = (const unsigned char *)src;

    for (index = 0; index < size; index += 1) {
        out[index] = in[index];
    }
}

zt_dyn_value *zt_dyn_box_copy_owned(const void *data, size_t size, zt_vtable *vtable) {
    void *copy;

    if (data == NULL || size == 0 || vtable == NULL) {
        return NULL;
    }

    copy = malloc(size);
    if (copy == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate dyn box data");
    }

    zt_dyn_copy_bytes(copy, data, size);
    return zt_dyn_box(copy, vtable);
}

zt_dyn_value *zt_dyn_box_copy_borrowed(const void *data, size_t size, zt_vtable *vtable) {
    void *copy;

    if (data == NULL || size == 0 || vtable == NULL) {
        return NULL;
    }

    copy = malloc(size);
    if (copy == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate dyn box data");
    }

    if (vtable->clone_out != NULL) {
        vtable->clone_out(copy, data);
    } else {
        zt_dyn_copy_bytes(copy, data, size);
    }

    return zt_dyn_box(copy, vtable);
}

void *zt_dyn_unbox(const zt_dyn_value *dyn) {
    if (dyn == NULL) {
        return NULL;
    }
    return dyn->data;
}

zt_vtable *zt_dyn_get_vtable(const zt_dyn_value *dyn) {
    if (dyn == NULL) {
        return NULL;
    }
    return dyn->vtable;
}

void zt_dyn_drop(zt_dyn_value *dyn) {
    if (dyn == NULL) {
        return;
    }
    if (dyn->header.rc > 0) {
        dyn->header.rc -= 1;
    }
    if (dyn->header.rc == 0) {
        if (dyn->vtable != NULL && dyn->vtable->drop != NULL) {
            dyn->vtable->drop(dyn->data);
        }
        free(dyn);
    }
}

zt_dyn_value *zt_dyn_clone(const zt_dyn_value *dyn) {
    zt_dyn_value *clone;
    void *cloned_data;
    size_t data_size;
    if (dyn == NULL) {
        return NULL;
    }
    if (dyn->vtable == NULL || dyn->vtable->clone_out == NULL) {
        zt_retain((void *)dyn);
        return (zt_dyn_value *)dyn;
    }
    clone = (zt_dyn_value *)malloc(sizeof(zt_dyn_value));
    if (clone == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate dyn clone");
    }
    data_size = dyn->vtable->data_size > 0 ? dyn->vtable->data_size : 64;
    cloned_data = malloc(data_size);
    if (cloned_data == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate cloned data");
    }
    dyn->vtable->clone_out(cloned_data, dyn->data);
    clone->header.rc = 1;
    clone->header.kind = ZT_HEAP_DYN_VALUE;
    clone->data = cloned_data;
    clone->vtable = dyn->vtable;
    return clone;
}

/* Generic dyn list implementation */
zt_list_dyn *zt_list_dyn_create(void) {
    zt_list_dyn *list;
    list = (zt_list_dyn *)malloc(sizeof(zt_list_dyn));
    if (list == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate dyn list");
    }
    list->header.rc = 1;
    list->header.kind = ZT_HEAP_LIST_DYN;
    list->len = 0;
    list->capacity = 8;
    list->data = (zt_dyn_value **)malloc(sizeof(zt_dyn_value *) * list->capacity);
    if (list->data == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate dyn list data");
    }
    return list;
}

void zt_list_dyn_append(zt_list_dyn *list, zt_dyn_value *value) {
    if (list == NULL || value == NULL) {
        return;
    }
    if (list->len >= list->capacity) {
        size_t new_capacity = list->capacity * 2;
        zt_dyn_value **new_data = (zt_dyn_value **)realloc(list->data, sizeof(zt_dyn_value *) * new_capacity);
        if (new_data == NULL) {
            zt_runtime_error(ZT_ERR_MEMORY, "failed to resize dyn list");
        }
        list->data = new_data;
        list->capacity = new_capacity;
    }
    list->data[list->len] = value;
    list->len += 1;
    zt_retain((void *)value);
}

zt_list_dyn *zt_list_dyn_append_owned(zt_list_dyn *list, zt_dyn_value *value) {
    if (list == NULL || value == NULL) {
        return list;
    }
    if (list->header.rc > 1u) {
        list = zt_list_dyn_from_array(list->data, list->len);
    }
    if (list->len >= list->capacity) {
        size_t new_capacity = list->capacity * 2;
        zt_dyn_value **new_data = (zt_dyn_value **)realloc(list->data, sizeof(zt_dyn_value *) * new_capacity);
        if (new_data == NULL) {
            zt_runtime_error(ZT_ERR_MEMORY, "failed to resize dyn list");
        }
        list->data = new_data;
        list->capacity = new_capacity;
    }
    list->data[list->len] = value;
    list->len += 1;
    return list;
}

zt_dyn_value *zt_list_dyn_get(const zt_list_dyn *list, zt_int index) {
    zt_int idx;
    if (list == NULL) {
        return NULL;
    }
    idx = index;
    if (idx < 0) {
        idx = (zt_int)list->len + idx;
    }
    if (idx < 0 || idx >= (zt_int)list->len) {
        zt_runtime_error(ZT_ERR_BOUNDS, "dyn list index out of bounds");
    }
    zt_retain((void *)list->data[idx]);
    return list->data[idx];
}

void zt_list_dyn_set(zt_list_dyn *list, zt_int index, zt_dyn_value *value) {
    zt_int idx;

    if (list == NULL || value == NULL) {
        return;
    }

    idx = index;
    if (idx < 0) {
        idx = (zt_int)list->len + idx;
    }
    if (idx < 0 || idx >= (zt_int)list->len) {
        zt_runtime_error(ZT_ERR_BOUNDS, "dyn list index out of bounds");
    }

    zt_retain((void *)value);
    zt_dyn_drop(list->data[idx]);
    list->data[idx] = value;
}

zt_list_dyn *zt_list_dyn_set_owned(zt_list_dyn *list, zt_int index, zt_dyn_value *value) {
    zt_int idx;

    if (list == NULL || value == NULL) {
        return list;
    }
    if (list->header.rc > 1u) {
        list = zt_list_dyn_from_array(list->data, list->len);
    }

    idx = index;
    if (idx < 0) {
        idx = (zt_int)list->len + idx;
    }
    if (idx < 0 || idx >= (zt_int)list->len) {
        zt_runtime_error(ZT_ERR_BOUNDS, "dyn list index out of bounds");
    }

    zt_dyn_drop(list->data[idx]);
    list->data[idx] = value;
    return list;
}

void zt_list_dyn_free(zt_list_dyn *list) {
    size_t i;
    if (list == NULL) {
        return;
    }
    if (list->header.rc > 0) {
        list->header.rc -= 1;
    }
    if (list->header.rc == 0) {
        for (i = 0; i < list->len; i += 1) {
            zt_dyn_drop(list->data[i]);
        }
        free(list->data);
        free(list);
    }
}

zt_list_dyn *zt_list_dyn_from_array(zt_dyn_value *const *items, size_t count) {
    zt_list_dyn *list = zt_list_dyn_create();
    size_t i;
    for (i = 0; i < count; i += 1) {
        zt_list_dyn_append(list, items[i]);
    }
    return list;
}

zt_list_dyn *zt_list_dyn_from_array_owned(zt_dyn_value **items, size_t count) {
    zt_list_dyn *list = zt_list_dyn_create();
    size_t i;

    if (count > list->capacity) {
        zt_dyn_value **new_data = (zt_dyn_value **)realloc(list->data, sizeof(zt_dyn_value *) * count);
        if (new_data == NULL) {
            zt_runtime_error(ZT_ERR_MEMORY, "failed to resize dyn list");
        }
        list->data = new_data;
        list->capacity = count;
    }

    for (i = 0; i < count; i += 1) {
        if (items[i] == NULL) {
            zt_runtime_error(ZT_ERR_PANIC, "zt_list_dyn_from_array_owned requires value");
        }
        list->data[i] = items[i];
    }
    list->len = count;
    return list;
}

zt_int zt_list_dyn_len(const zt_list_dyn *list) {
    if (list == NULL) return 0;
    return (zt_int)list->len;
}

zt_list_dyn *zt_list_dyn_slice(const zt_list_dyn *list, zt_int start_0, zt_int end_0) {
    zt_list_dyn *sliced;
    zt_int s, e, count;
    zt_int i;

    if (list == NULL) return NULL;
    if (list->len == 0) return zt_list_dyn_create();

    s = start_0 < 0 ? (zt_int)list->len + start_0 : start_0;
    e = end_0 < 0 ? (zt_int)list->len + end_0 : end_0;

    if (s < 0) s = 0;
    if (s > (zt_int)list->len) s = (zt_int)list->len;
    if (e < s) return zt_list_dyn_create();
    if (e >= (zt_int)list->len) e = (zt_int)list->len - 1;

    if (s >= (zt_int)list->len) {
        return zt_list_dyn_create();
    }

    count = e - s + 1;
    sliced = zt_list_dyn_create();
    for (i = 0; i < count; i += 1) {
        zt_list_dyn_append(sliced, list->data[s + i]);
    }
    return sliced;
}








