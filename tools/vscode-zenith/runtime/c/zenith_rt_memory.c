// Memory pool scaffolding for frequently-used types.

#define POOL_SIZE 128

typedef struct zt_pool {
    void *objects[POOL_SIZE];
    size_t count;
} zt_pool;

static zt_pool zt_text_pool = { .count = 0 };

zt_text *zt_text_pool_alloc(void) {
    if (zt_text_pool.count > 0) {
        return zt_text_pool.objects[--zt_text_pool.count];
    }
    return malloc(sizeof(zt_text));
}

void zt_text_pool_free(zt_text *text) {
    if (zt_text_pool.count < POOL_SIZE) {
        zt_text_pool.objects[zt_text_pool.count++] = text;
    } else {
        free(text);
    }
}

zt_bool zt_validate_pointer(const void *ptr) {
    return ptr != NULL;
}

void zt_runtime_safe_function_example(const zt_text *text) {
    if (!zt_validate_pointer(text)) {
        fprintf(stderr, "Erro: ponteiro nulo passado para zt_runtime_safe_function_example\n");
        return;
    }

    // ... function logic ...
}

void zt_validate_and_free_text(zt_text *value) {
    if (value == NULL || value->data == NULL || value->len == 0) {
        fprintf(stderr, "Erro: tentativa de liberar zt_text invalido\n");
        return;
    }
    zt_free_text(value);
}

void zt_validate_and_free_list_i64(zt_list_i64 *list) {
    if (list == NULL || list->data == NULL || list->len > list->capacity) {
        fprintf(stderr, "Erro: tentativa de liberar zt_list_i64 invalido\n");
        return;
    }
    zt_free_list_i64(list);
}

void zt_validate_and_free_map_text_text(zt_map_text_text *map) {
    if (map == NULL || map->keys == NULL || map->values == NULL || map->len > map->capacity) {
        fprintf(stderr, "Erro: tentativa de liberar zt_map_text_text invalido\n");
        return;
    }
    zt_free_map_text_text(map);
}
