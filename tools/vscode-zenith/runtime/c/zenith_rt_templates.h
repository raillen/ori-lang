/*
 * zenith_rt_templates.h - Monomorphization templates for the Zenith runtime.
 *
 * These macros generate type-specialized data structures and functions
 * for the Zenith generic type system (list<T>, optional<T>, outcome<T,E>, map<K,V>).
 *
 * Usage:
 *   ZT_DEFINE_LIST(i64, zt_int, ZT_HEAP_LIST_I64, 0)      // list<int>  - value type
 *   ZT_DEFINE_LIST(f64, zt_float, ZT_HEAP_LIST_F64, 0)    // list<float> - value type
 *   ZT_DEFINE_LIST(text, zt_text *, ZT_HEAP_LIST_TEXT, 1) // list<text> - managed type
 *
 * IS_PTR = 0: element is a value type (copied by value, no retain/release)
 * IS_PTR = 1: element is a managed/pointer type (needs zt_retain/zt_release)
 */

#ifndef ZENITH_RT_TEMPLATES_H
#define ZENITH_RT_TEMPLATES_H

#include <stdlib.h>
#include <string.h>

#define ZT_TEMPLATE_CAT_INNER(a, b) a##b
#define ZT_TEMPLATE_CAT(a, b) ZT_TEMPLATE_CAT_INNER(a, b)
#define ZT_TEMPLATE_IF_0(...)
#define ZT_TEMPLATE_IF_1(...) __VA_ARGS__
#define ZT_TEMPLATE_IF(flag, ...) ZT_TEMPLATE_CAT(ZT_TEMPLATE_IF_, flag)(__VA_ARGS__)
#define ZT_TEMPLATE_IF_NOT_0(...) __VA_ARGS__
#define ZT_TEMPLATE_IF_NOT_1(...)
#define ZT_TEMPLATE_IF_NOT(flag, ...) ZT_TEMPLATE_CAT(ZT_TEMPLATE_IF_NOT_, flag)(__VA_ARGS__)

/* --------------------------------------------------------------------------
 * ZT_DEFINE_LIST(SUFFIX, ELEM_TYPE, HEAP_KIND, IS_PTR)
 *
 * Generates:
 *   - typedef struct zt_list_##SUFFIX
 *   - static void zt_free_list_##SUFFIX(...)
 *   - static void zt_list_##SUFFIX##_reserve(...)
 *   - zt_list_##SUFFIX *zt_list_##SUFFIX##_new(void)
 *   - zt_list_##SUFFIX *zt_list_##SUFFIX##_from_array(...)
 *   - void zt_list_##SUFFIX##_push(...)
 *   - zt_list_##SUFFIX *zt_list_##SUFFIX##_push_owned(...)
 *   - ELEM_TYPE zt_list_##SUFFIX##_get(...)
 *   - void zt_list_##SUFFIX##_set(...)
 *   - zt_list_##SUFFIX *zt_list_##SUFFIX##_set_owned(...)
 *   - zt_int zt_list_##SUFFIX##_len(...)
 *   - zt_list_##SUFFIX *zt_list_##SUFFIX##_slice(...)
 * -------------------------------------------------------------------------- */

#define ZT_DEFINE_LIST_STRUCT(SUFFIX, ELEM_TYPE, HEAP_KIND, IS_PTR)             \
                                                                                 \
typedef struct zt_list_##SUFFIX {                                                \
    zt_header header;                                                            \
    size_t len;                                                                  \
    size_t capacity;                                                             \
    ELEM_TYPE *data;                                                             \
} zt_list_##SUFFIX;

#define ZT_DEFINE_LIST_IMPL(SUFFIX, ELEM_TYPE, HEAP_KIND, IS_PTR)               \
static void zt_free_list_##SUFFIX(zt_list_##SUFFIX *list) {                     \
    if (list == NULL) return;                                                    \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        size_t _i;                                                               \
        for (_i = 0; _i < list->len; _i += 1) {                                 \
            zt_release((void *)list->data[_i]);                                 \
        }                                                                        \
    )                                                                            \
    free(list->data);                                                            \
    list->data = NULL;                                                           \
    list->len = 0;                                                               \
    list->capacity = 0;                                                          \
    free(list);                                                                  \
}                                                                                \
                                                                                 \
static void zt_list_##SUFFIX##_reserve(zt_list_##SUFFIX *list,                  \
                                        size_t min_capacity) {                   \
    size_t new_capacity;                                                         \
    ELEM_TYPE *new_data;                                                         \
                                                                                 \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_reserve requires list");                        \
    }                                                                            \
    if (min_capacity <= list->capacity) return;                                  \
                                                                                 \
    new_capacity = list->capacity > 0 ? list->capacity : 4;                     \
    while (new_capacity < min_capacity) {                                        \
        if (new_capacity > SIZE_MAX / 2) {                                      \
            zt_runtime_error(ZT_ERR_PLATFORM, "list capacity overflow");        \
        }                                                                        \
        new_capacity *= 2;                                                       \
    }                                                                            \
                                                                                 \
    new_data = (ELEM_TYPE *)realloc(list->data,                                 \
                                     new_capacity * sizeof(ELEM_TYPE));          \
    if (new_data == NULL) {                                                      \
        zt_runtime_error(ZT_ERR_PLATFORM,                                       \
            "failed to grow list<" #SUFFIX "> buffer");                          \
    }                                                                            \
    list->data = new_data;                                                       \
    list->capacity = new_capacity;                                               \
}                                                                                \
                                                                                 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_new(void) {                                \
    zt_list_##SUFFIX *list;                                                      \
    list = (zt_list_##SUFFIX *)calloc(1, sizeof(zt_list_##SUFFIX));             \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PLATFORM,                                       \
            "failed to allocate list<" #SUFFIX "> header");                      \
    }                                                                            \
    list->header.rc = 1;                                                         \
    list->header.kind = (uint32_t)(HEAP_KIND);                                  \
    return list;                                                                 \
}                                                                                \
                                                                                 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_from_array(                                \
        ELEM_TYPE const *items, size_t count) {                                  \
    zt_list_##SUFFIX *list = zt_list_##SUFFIX##_new();                          \
    if (count == 0) return list;                                                 \
    if (items == NULL) {                                                         \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_from_array requires items");                    \
    }                                                                            \
    zt_list_##SUFFIX##_reserve(list, count);                                    \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        size_t _i;                                                               \
        for (_i = 0; _i < count; _i += 1) {                                     \
            zt_retain((void *)items[_i]);                                        \
            list->data[_i] = items[_i];                                          \
        }                                                                        \
    )                                                                            \
    ZT_TEMPLATE_IF_NOT(IS_PTR,                                                   \
        memcpy(list->data, items, count * sizeof(ELEM_TYPE));                    \
    )                                                                            \
    list->len = count;                                                           \
    return list;                                                                 \
}                                                                                \
                                                                                 \
void zt_list_##SUFFIX##_push(zt_list_##SUFFIX *list, ELEM_TYPE value) {         \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_push requires list");                           \
    }                                                                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)value == NULL) {                                      \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_list_" #SUFFIX "_push requires non-null element");           \
        }                                                                        \
    )                                                                            \
    zt_list_##SUFFIX##_reserve(list, list->len + 1);                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        zt_retain((void *)value);                                                \
    )                                                                            \
    list->data[list->len] = value;                                               \
    list->len += 1;                                                              \
}                                                                                \
                                                                                 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_push_owned(                                \
        zt_list_##SUFFIX *list, ELEM_TYPE value) {                               \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_push_owned requires list");                     \
    }                                                                            \
    if (list->header.rc > 1u) {                                                  \
        list = zt_list_##SUFFIX##_from_array(list->data, list->len);            \
    } else {                                                                     \
        zt_retain(list);                                                         \
    }                                                                            \
    zt_list_##SUFFIX##_push(list, value);                                       \
    return list;                                                                 \
}                                                                                \
                                                                                 \
ELEM_TYPE zt_list_##SUFFIX##_get(const zt_list_##SUFFIX *list,                  \
                                   zt_int index_0) {                             \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_get requires list");                            \
    }                                                                            \
    if (index_0 < 0 || (size_t)index_0 >= list->len) {                          \
        zt_runtime_error(ZT_ERR_INDEX,                                          \
            "list<" #SUFFIX "> index out of bounds");                            \
    }                                                                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)list->data[index_0] == NULL) {                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "list<" #SUFFIX "> entry cannot be null");                       \
        }                                                                        \
        zt_retain((void *)list->data[index_0]);                                  \
    )                                                                            \
    return list->data[index_0];                                                  \
}                                                                                \
                                                                                 \
void zt_list_##SUFFIX##_set(zt_list_##SUFFIX *list,                             \
                              zt_int index_0, ELEM_TYPE value) {                 \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_set requires list");                            \
    }                                                                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)value == NULL) {                                      \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_list_" #SUFFIX "_set requires non-null element");            \
        }                                                                        \
    )                                                                            \
    if (index_0 < 0 || (size_t)index_0 >= list->len) {                          \
        zt_runtime_error(ZT_ERR_INDEX,                                          \
            "list<" #SUFFIX "> index out of bounds");                            \
    }                                                                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        zt_retain((void *)value);                                                \
        zt_release((void *)list->data[index_0]);                                 \
    )                                                                            \
    list->data[index_0] = value;                                                 \
}                                                                                \
                                                                                 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_set_owned(                                 \
        zt_list_##SUFFIX *list, zt_int index_0, ELEM_TYPE value) {               \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_set_owned requires list");                      \
    }                                                                            \
    if (list->header.rc > 1u) {                                                  \
        list = zt_list_##SUFFIX##_from_array(list->data, list->len);            \
    } else {                                                                     \
        zt_retain(list);                                                         \
    }                                                                            \
    zt_list_##SUFFIX##_set(list, index_0, value);                               \
    return list;                                                                 \
}                                                                                \
                                                                                 \
zt_int zt_list_##SUFFIX##_len(const zt_list_##SUFFIX *list) {                   \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_len requires list");                            \
    }                                                                            \
    return (zt_int)list->len;                                                    \
}                                                                                \
                                                                                 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_deep_copy(                                 \
        const zt_list_##SUFFIX *list) {                                          \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_deep_copy requires list");                      \
    }                                                                            \
    ZT_TEMPLATE_IF_NOT(IS_PTR,                                                   \
        return zt_list_##SUFFIX##_from_array(list->data, list->len);             \
    )                                                                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        zt_list_##SUFFIX *clone;                                                 \
        size_t _i;                                                               \
        clone = zt_list_##SUFFIX##_new();                                        \
        zt_list_##SUFFIX##_reserve(clone, list->len);                            \
        for (_i = 0; _i < list->len; _i += 1) {                                 \
            clone->data[_i] = (ELEM_TYPE)zt_deep_copy((void *)list->data[_i]);  \
        }                                                                        \
        clone->len = list->len;                                                  \
        return clone;                                                            \
    )                                                                            \
}                                                                                \
                                                                                 \
zt_list_##SUFFIX *zt_list_##SUFFIX##_slice(                                     \
        const zt_list_##SUFFIX *list, zt_int start_0, zt_int end_0) {            \
    size_t start_pos, end_pos;                                                   \
    if (list == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_list_" #SUFFIX "_slice requires list");                          \
    }                                                                            \
    if (start_0 < 0) {                                                           \
        zt_runtime_error(ZT_ERR_INDEX, "slice start must be >= 0");             \
    }                                                                            \
    if (list->len == 0) return zt_list_##SUFFIX##_new();                        \
    start_pos = (size_t)start_0;                                                 \
    end_pos = zt_normalize_slice_end(list->len, end_0);                         \
    if (start_pos >= list->len || end_pos < start_pos) {                        \
        return zt_list_##SUFFIX##_new();                                        \
    }                                                                            \
    return zt_list_##SUFFIX##_from_array(                                       \
        list->data + start_pos, end_pos - start_pos + 1);                       \
}

/* Combined: struct + impl */
#define ZT_DEFINE_LIST(SUFFIX, ELEM_TYPE, HEAP_KIND, IS_PTR)                    \
    ZT_DEFINE_LIST_STRUCT(SUFFIX, ELEM_TYPE, HEAP_KIND, IS_PTR)                 \
    ZT_DEFINE_LIST_IMPL(SUFFIX, ELEM_TYPE, HEAP_KIND, IS_PTR)

#define ZT_DEFINE_MAP_STRUCT(                                                   \
        SUFFIX,                                                                 \
        KEY_TYPE,                                                               \
        VALUE_TYPE,                                                             \
        OPTIONAL_VALUE_TYPE,                                                    \
        HEAP_KIND,                                                              \
        KEY_IS_PTR,                                                             \
        VALUE_IS_PTR,                                                           \
        KEY_EQ_FN,                                                              \
        KEY_HASH_FN,                                                            \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN)                                                      \
                                                                                 \
typedef struct zt_map_##SUFFIX {                                                \
    zt_header header;                                                           \
    size_t len;                                                                 \
    size_t capacity;                                                            \
    KEY_TYPE *keys;                                                             \
    VALUE_TYPE *values;                                                         \
    size_t hash_capacity;                                                       \
    size_t *hash_indices;                                                       \
} zt_map_##SUFFIX;

#define ZT_DEFINE_MAP_IMPL(                                                     \
        SUFFIX,                                                                 \
        KEY_TYPE,                                                               \
        VALUE_TYPE,                                                             \
        OPTIONAL_VALUE_TYPE,                                                    \
        HEAP_KIND,                                                              \
        KEY_IS_PTR,                                                             \
        VALUE_IS_PTR,                                                           \
        KEY_EQ_FN,                                                              \
        KEY_HASH_FN,                                                            \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN)                                                      \
static void zt_free_map_##SUFFIX(zt_map_##SUFFIX *map) {                       \
    size_t _i;                                                                  \
    if (map == NULL) return;                                                    \
    for (_i = 0; _i < map->len; _i += 1) {                                      \
        ZT_TEMPLATE_IF(KEY_IS_PTR,                                              \
            zt_release((void *)map->keys[_i]);                                  \
        )                                                                       \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            zt_release((void *)map->values[_i]);                                \
        )                                                                       \
    }                                                                           \
    free(map->keys);                                                            \
    free(map->values);                                                          \
    free(map->hash_indices);                                                    \
    map->keys = NULL;                                                           \
    map->values = NULL;                                                         \
    map->hash_indices = NULL;                                                   \
    map->len = 0;                                                               \
    map->capacity = 0;                                                          \
    map->hash_capacity = 0;                                                     \
    free(map);                                                                  \
}                                                                               \
                                                                                \
static void zt_map_##SUFFIX##_reserve(zt_map_##SUFFIX *map,                    \
                                      size_t min_capacity) {                    \
    size_t new_capacity;                                                        \
    KEY_TYPE *new_keys;                                                         \
    VALUE_TYPE *new_values;                                                     \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_reserve requires map");                         \
    }                                                                           \
    if (min_capacity <= map->capacity) return;                                  \
    new_capacity = map->capacity > 0 ? map->capacity : 4;                       \
    while (new_capacity < min_capacity) {                                       \
        if (new_capacity > SIZE_MAX / 2) {                                      \
            zt_runtime_error(ZT_ERR_PLATFORM, "map capacity overflow");         \
        }                                                                       \
        new_capacity *= 2;                                                      \
    }                                                                           \
    new_keys = (KEY_TYPE *)realloc(map->keys, new_capacity * sizeof(KEY_TYPE)); \
    if (new_keys == NULL) {                                                     \
        zt_runtime_error(ZT_ERR_PLATFORM,                                       \
            "failed to grow map<" #SUFFIX "> key buffer");                      \
    }                                                                           \
    new_values = (VALUE_TYPE *)realloc(                                         \
        map->values,                                                            \
        new_capacity * sizeof(VALUE_TYPE));                                     \
    if (new_values == NULL) {                                                   \
        zt_runtime_error(ZT_ERR_PLATFORM,                                       \
            "failed to grow map<" #SUFFIX "> value buffer");                    \
    }                                                                           \
    map->keys = new_keys;                                                       \
    map->values = new_values;                                                   \
    map->capacity = new_capacity;                                               \
}                                                                               \
                                                                                \
static size_t zt_map_##SUFFIX##_hash_target_capacity(size_t min_items) {        \
    size_t target = 8;                                                          \
    size_t needed = min_items > 0 ? min_items : 1;                              \
    while (target < needed * 2u) {                                              \
        if (target > SIZE_MAX / 2u) {                                           \
            zt_runtime_error(ZT_ERR_PLATFORM, "map hash capacity overflow");    \
        }                                                                       \
        target *= 2u;                                                           \
    }                                                                           \
    return target;                                                              \
}                                                                               \
                                                                                \
static void zt_map_##SUFFIX##_hash_clear(                                       \
        size_t *hash_indices,                                                   \
        size_t hash_capacity) {                                                 \
    size_t _i;                                                                  \
    if (hash_indices == NULL) return;                                           \
    for (_i = 0; _i < hash_capacity; _i += 1) {                                 \
        hash_indices[_i] = 0;                                                   \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_map_##SUFFIX##_hash_insert_index(                                \
        zt_map_##SUFFIX *map,                                                   \
        size_t index) {                                                         \
    size_t slot;                                                                \
    size_t mask;                                                                \
    if (map == NULL || map->hash_indices == NULL || map->hash_capacity == 0) {  \
        return;                                                                 \
    }                                                                           \
    mask = map->hash_capacity - 1u;                                             \
    slot = KEY_HASH_FN(map->keys[index]) & mask;                                \
    while (map->hash_indices[slot] != 0) {                                      \
        slot = (slot + 1u) & mask;                                              \
    }                                                                           \
    map->hash_indices[slot] = index + 1u;                                       \
}                                                                               \
                                                                                \
static void zt_map_##SUFFIX##_rebuild_hash(                                     \
        zt_map_##SUFFIX *map,                                                   \
        size_t min_items) {                                                     \
    size_t target_capacity;                                                     \
    size_t *new_indices = NULL;                                                 \
    size_t _i;                                                                  \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_rebuild_hash requires map");                    \
    }                                                                           \
    target_capacity = zt_map_##SUFFIX##_hash_target_capacity(min_items);        \
    if (map->hash_capacity != target_capacity || map->hash_indices == NULL) {   \
        new_indices = (size_t *)calloc(target_capacity, sizeof(size_t));        \
        if (new_indices == NULL) {                                              \
            zt_runtime_error(ZT_ERR_PLATFORM,                                   \
                "failed to allocate map<" #SUFFIX "> hash index");              \
        }                                                                       \
        free(map->hash_indices);                                                \
        map->hash_indices = new_indices;                                        \
        map->hash_capacity = target_capacity;                                   \
    } else {                                                                    \
        zt_map_##SUFFIX##_hash_clear(map->hash_indices, map->hash_capacity);    \
    }                                                                           \
    for (_i = 0; _i < map->len; _i += 1) {                                      \
        zt_map_##SUFFIX##_hash_insert_index(map, _i);                           \
    }                                                                           \
}                                                                               \
                                                                                \
static size_t zt_map_##SUFFIX##_find_index(                                     \
        const zt_map_##SUFFIX *map,                                             \
        const KEY_TYPE key,                                                     \
        zt_bool *found) {                                                       \
    size_t _i;                                                                  \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_find_index requires map");                      \
    }                                                                           \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_map_" #SUFFIX "_find_index requires key");                  \
        }                                                                       \
    )                                                                           \
    if (map->hash_capacity > 0 && map->hash_indices != NULL) {                  \
        size_t slot;                                                            \
        size_t start;                                                           \
        size_t mask = map->hash_capacity - 1u;                                  \
        slot = KEY_HASH_FN(key) & mask;                                         \
        start = slot;                                                           \
        while (map->hash_indices[slot] != 0) {                                  \
            _i = map->hash_indices[slot] - 1u;                                  \
            if (KEY_EQ_FN(map->keys[_i], key)) {                                \
                if (found != NULL) *found = true;                               \
                return _i;                                                      \
            }                                                                   \
            slot = (slot + 1u) & mask;                                          \
            if (slot == start) {                                                \
                break;                                                          \
            }                                                                   \
        }                                                                       \
        if (found != NULL) *found = false;                                      \
        return map->len;                                                        \
    }                                                                           \
    for (_i = 0; _i < map->len; _i += 1) {                                      \
        if (KEY_EQ_FN(map->keys[_i], key)) {                                    \
            if (found != NULL) *found = true;                                   \
            return _i;                                                          \
        }                                                                       \
    }                                                                           \
    if (found != NULL) *found = false;                                          \
    return map->len;                                                            \
}                                                                               \
                                                                                \
static zt_map_##SUFFIX *zt_map_##SUFFIX##_alloc(void) {                        \
    zt_map_##SUFFIX *map;                                                       \
    map = (zt_map_##SUFFIX *)calloc(1, sizeof(zt_map_##SUFFIX));                \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PLATFORM,                                       \
            "failed to allocate map<" #SUFFIX "> header");                      \
    }                                                                           \
    map->header.rc = 1;                                                         \
    map->header.kind = (uint32_t)(HEAP_KIND);                                   \
    return map;                                                                 \
}                                                                               \
                                                                                \
zt_map_##SUFFIX *zt_map_##SUFFIX##_new(void) {                                 \
    return zt_map_##SUFFIX##_alloc();                                           \
}                                                                               \
                                                                                \
void zt_map_##SUFFIX##_set(                                                     \
        zt_map_##SUFFIX *map,                                                   \
        KEY_TYPE key,                                                           \
        VALUE_TYPE value);                                                      \
                                                                                \
zt_map_##SUFFIX *zt_map_##SUFFIX##_from_arrays(                                 \
        KEY_TYPE const *keys,                                                   \
        VALUE_TYPE const *values,                                               \
        size_t count) {                                                         \
    zt_map_##SUFFIX *map;                                                       \
    size_t _i;                                                                  \
    map = zt_map_##SUFFIX##_new();                                              \
    if (count == 0) return map;                                                 \
    if (keys == NULL || values == NULL) {                                       \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_from_arrays requires key/value arrays");        \
    }                                                                           \
    for (_i = 0; _i < count; _i += 1) {                                         \
        zt_map_##SUFFIX##_set(map, keys[_i], values[_i]);                       \
    }                                                                           \
    return map;                                                                 \
}                                                                               \
                                                                                \
void zt_map_##SUFFIX##_set(                                                     \
        zt_map_##SUFFIX *map,                                                   \
        KEY_TYPE key,                                                           \
        VALUE_TYPE value) {                                                     \
    zt_bool found;                                                              \
    size_t index;                                                               \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_set requires map");                             \
    }                                                                           \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_map_" #SUFFIX "_set requires key");                         \
        }                                                                       \
    )                                                                           \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)value == NULL) {                                      \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_map_" #SUFFIX "_set requires value");                       \
        }                                                                       \
    )                                                                           \
    if (map->len > 0 && (map->hash_capacity == 0 || map->hash_indices == NULL)) { \
        zt_map_##SUFFIX##_rebuild_hash(map, map->len);                          \
    }                                                                           \
    index = zt_map_##SUFFIX##_find_index(map, key, &found);                     \
    if (found) {                                                                \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            zt_retain((void *)value);                                           \
            zt_release((void *)map->values[index]);                             \
        )                                                                       \
        map->values[index] = value;                                             \
        return;                                                                 \
    }                                                                           \
    zt_map_##SUFFIX##_reserve(map, map->len + 1);                               \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        zt_retain((void *)key);                                                 \
    )                                                                           \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        zt_retain((void *)value);                                               \
    )                                                                           \
    map->keys[map->len] = key;                                                  \
    map->values[map->len] = value;                                              \
    map->len += 1;                                                              \
    if (map->hash_capacity == 0 || map->hash_indices == NULL ||                 \
            (map->len * 10u) >= (map->hash_capacity * 7u)) {                    \
        zt_map_##SUFFIX##_rebuild_hash(map, map->len);                          \
    } else {                                                                    \
        zt_map_##SUFFIX##_hash_insert_index(map, map->len - 1u);                \
    }                                                                           \
}                                                                               \
                                                                                \
static zt_map_##SUFFIX *zt_map_##SUFFIX##_deep_copy(                            \
        const zt_map_##SUFFIX *map) {                                           \
    zt_map_##SUFFIX *clone;                                                     \
    size_t _i;                                                                  \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_deep_copy requires map");                       \
    }                                                                           \
    clone = zt_map_##SUFFIX##_new();                                            \
    zt_map_##SUFFIX##_reserve(clone, map->len);                                 \
    for (_i = 0; _i < map->len; _i += 1) {                                      \
        KEY_TYPE key = map->keys[_i];                                           \
        VALUE_TYPE value = map->values[_i];                                     \
        ZT_TEMPLATE_IF(KEY_IS_PTR,                                              \
            key = (KEY_TYPE)zt_deep_copy((void *)map->keys[_i]);                \
        )                                                                       \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            value = (VALUE_TYPE)zt_deep_copy((void *)map->values[_i]);          \
        )                                                                       \
        zt_map_##SUFFIX##_set(clone, key, value);                               \
        ZT_TEMPLATE_IF(KEY_IS_PTR,                                              \
            zt_release((void *)key);                                            \
        )                                                                       \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            zt_release((void *)value);                                          \
        )                                                                       \
    }                                                                           \
    return clone;                                                               \
}                                                                               \
                                                                                \
zt_map_##SUFFIX *zt_map_##SUFFIX##_set_owned(                                   \
        zt_map_##SUFFIX *map,                                                   \
        KEY_TYPE key,                                                           \
        VALUE_TYPE value) {                                                     \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_set_owned requires map");                       \
    }                                                                           \
    if (map->header.rc > 1u) {                                                  \
        map = zt_map_##SUFFIX##_deep_copy(map);                                 \
    } else {                                                                    \
        zt_retain(map);                                                         \
    }                                                                           \
    zt_map_##SUFFIX##_set(map, key, value);                                     \
    return map;                                                                 \
}                                                                               \
                                                                                \
VALUE_TYPE zt_map_##SUFFIX##_get(                                               \
        const zt_map_##SUFFIX *map,                                             \
        const KEY_TYPE key) {                                                   \
    zt_bool found;                                                              \
    size_t index;                                                               \
    VALUE_TYPE value;                                                           \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_get requires map");                             \
    }                                                                           \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_map_" #SUFFIX "_get requires key");                         \
        }                                                                       \
    )                                                                           \
    index = zt_map_##SUFFIX##_find_index(map, key, &found);                     \
    if (!found) {                                                               \
        zt_runtime_error(ZT_ERR_INDEX, "map<" #SUFFIX "> key not found");      \
    }                                                                           \
    value = map->values[index];                                                 \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)value == NULL) {                                      \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "map<" #SUFFIX "> value cannot be null");                       \
        }                                                                       \
        zt_retain((void *)value);                                               \
    )                                                                           \
    return value;                                                               \
}                                                                               \
                                                                                \
OPTIONAL_VALUE_TYPE zt_map_##SUFFIX##_get_optional(                             \
        const zt_map_##SUFFIX *map,                                             \
        const KEY_TYPE key) {                                                   \
    zt_bool found;                                                              \
    size_t index;                                                               \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_get_optional requires map");                    \
    }                                                                           \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_map_" #SUFFIX "_get_optional requires key");                \
        }                                                                       \
    )                                                                           \
    index = zt_map_##SUFFIX##_find_index(map, key, &found);                     \
    if (!found) {                                                               \
        return OPTIONAL_EMPTY_FN();                                             \
    }                                                                           \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)map->values[index] == NULL) {                         \
            return OPTIONAL_EMPTY_FN();                                         \
        }                                                                       \
    )                                                                           \
    return OPTIONAL_PRESENT_FN(map->values[index]);                             \
}                                                                               \
                                                                                \
zt_bool zt_map_##SUFFIX##_contains(                                             \
        const zt_map_##SUFFIX *map,                                             \
        const KEY_TYPE key) {                                                   \
    zt_bool found;                                                              \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_contains requires map");                        \
    }                                                                           \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "zt_map_" #SUFFIX "_contains requires key");                    \
        }                                                                       \
    )                                                                           \
    (void)zt_map_##SUFFIX##_find_index(map, key, &found);                       \
    return found;                                                               \
}                                                                               \
                                                                                \
KEY_TYPE zt_map_##SUFFIX##_key_at(                                              \
        const zt_map_##SUFFIX *map,                                             \
        zt_int index_0) {                                                       \
    KEY_TYPE key;                                                               \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_key_at requires map");                          \
    }                                                                           \
    if (index_0 < 0 || (size_t)index_0 >= map->len) {                           \
        zt_runtime_error(ZT_ERR_INDEX,                                          \
            "map<" #SUFFIX "> iteration key index out of bounds");              \
    }                                                                           \
    key = map->keys[(size_t)index_0];                                           \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "map<" #SUFFIX "> key cannot be null");                         \
        }                                                                       \
        zt_retain((void *)key);                                                 \
    )                                                                           \
    return key;                                                                 \
}                                                                               \
                                                                                \
VALUE_TYPE zt_map_##SUFFIX##_value_at(                                          \
        const zt_map_##SUFFIX *map,                                             \
        zt_int index_0) {                                                       \
    VALUE_TYPE value;                                                           \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_value_at requires map");                        \
    }                                                                           \
    if (index_0 < 0 || (size_t)index_0 >= map->len) {                           \
        zt_runtime_error(ZT_ERR_INDEX,                                          \
            "map<" #SUFFIX "> iteration value index out of bounds");            \
    }                                                                           \
    value = map->values[(size_t)index_0];                                       \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)value == NULL) {                                      \
            zt_runtime_error(ZT_ERR_PANIC,                                      \
                "map<" #SUFFIX "> value cannot be null");                       \
        }                                                                       \
        zt_retain((void *)value);                                               \
    )                                                                           \
    return value;                                                               \
}                                                                               \
                                                                                \
zt_int zt_map_##SUFFIX##_len(const zt_map_##SUFFIX *map) {                     \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_map_" #SUFFIX "_len requires map");                             \
    }                                                                           \
    return (zt_int)map->len;                                                    \
}

#define ZT_DEFINE_MAP(                                                          \
        SUFFIX,                                                                 \
        KEY_TYPE,                                                               \
        VALUE_TYPE,                                                             \
        OPTIONAL_VALUE_TYPE,                                                    \
        HEAP_KIND,                                                              \
        KEY_IS_PTR,                                                             \
        VALUE_IS_PTR,                                                           \
        KEY_EQ_FN,                                                              \
        KEY_HASH_FN,                                                            \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN)                                                      \
    ZT_DEFINE_MAP_STRUCT(                                                       \
        SUFFIX,                                                                 \
        KEY_TYPE,                                                               \
        VALUE_TYPE,                                                             \
        OPTIONAL_VALUE_TYPE,                                                    \
        HEAP_KIND,                                                              \
        KEY_IS_PTR,                                                             \
        VALUE_IS_PTR,                                                           \
        KEY_EQ_FN,                                                              \
        KEY_HASH_FN,                                                            \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN)                                                      \
    ZT_DEFINE_MAP_IMPL(                                                         \
        SUFFIX,                                                                 \
        KEY_TYPE,                                                               \
        VALUE_TYPE,                                                             \
        OPTIONAL_VALUE_TYPE,                                                    \
        HEAP_KIND,                                                              \
        KEY_IS_PTR,                                                             \
        VALUE_IS_PTR,                                                           \
        KEY_EQ_FN,                                                              \
        KEY_HASH_FN,                                                            \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN)

#define ZT_DEFINE_GRID2D_IMPL(                                                  \
        SUFFIX,                                                                 \
        ELEM_TYPE,                                                              \
        HEAP_KIND,                                                              \
        IS_PTR,                                                                 \
        TYPE_LABEL,                                                             \
        EMPTY_VALUE_EXPR)                                                       \
static void zt_runtime_require_grid2d_##SUFFIX(                                 \
        const zt_grid2d_##SUFFIX *grid,                                         \
        const char *message) {                                                  \
    if (grid == NULL) {                                                         \
        zt_runtime_error(ZT_ERR_PANIC, message);                                \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_free_grid2d_##SUFFIX(zt_grid2d_##SUFFIX *grid) {                \
    if (grid == NULL) return;                                                   \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (size_t _i = 0; _i < grid->len; _i += 1) {                         \
            zt_release(grid->data[_i]);                                         \
        }                                                                       \
    )                                                                           \
    free(grid->data);                                                           \
    grid->data = NULL;                                                          \
    grid->rows = 0;                                                             \
    grid->cols = 0;                                                             \
    grid->len = 0;                                                              \
    grid->capacity = 0;                                                         \
    free(grid);                                                                 \
}                                                                               \
                                                                                \
zt_grid2d_##SUFFIX *zt_grid2d_##SUFFIX##_new(zt_int rows, zt_int cols) {       \
    zt_grid2d_##SUFFIX *grid;                                                   \
    size_t total;                                                               \
    if (rows <= 0 || cols <= 0) {                                               \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "grid2d<" TYPE_LABEL "> dimensions must be positive");              \
    }                                                                           \
    total = (size_t)rows * (size_t)cols;                                        \
    grid = (zt_grid2d_##SUFFIX *)calloc(1, sizeof(zt_grid2d_##SUFFIX));         \
    if (grid == NULL) {                                                         \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate grid2d<" TYPE_LABEL "> header");                \
    }                                                                           \
    grid->header.rc = 1;                                                        \
    grid->header.kind = (uint32_t)(HEAP_KIND);                                  \
    grid->rows = (size_t)rows;                                                  \
    grid->cols = (size_t)cols;                                                  \
    grid->len = total;                                                          \
    grid->capacity = total;                                                     \
    grid->data = (ELEM_TYPE *)calloc(total, sizeof(ELEM_TYPE));                 \
    if (grid->data == NULL) {                                                   \
        free(grid);                                                             \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate grid2d<" TYPE_LABEL "> data");                  \
    }                                                                           \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (size_t _i = 0; _i < total; _i += 1) {                             \
            grid->data[_i] = (EMPTY_VALUE_EXPR);                                \
        }                                                                       \
    )                                                                           \
    return grid;                                                                \
}                                                                               \
                                                                                \
ELEM_TYPE zt_grid2d_##SUFFIX##_get(                                             \
        const zt_grid2d_##SUFFIX *grid,                                         \
        zt_int row,                                                             \
        zt_int col) {                                                           \
    size_t _index;                                                              \
    ELEM_TYPE value;                                                            \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_get requires grid");                             \
    if (row < 0 || (size_t)row >= grid->rows ||                                 \
            col < 0 || (size_t)col >= grid->cols) {                             \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "grid2d<" TYPE_LABEL "> index out of bounds");                      \
    }                                                                           \
    _index = (size_t)row * grid->cols + (size_t)col;                            \
    value = grid->data[_index];                                                 \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value == NULL) return (EMPTY_VALUE_EXPR);             \
        zt_retain((void *)value);                                               \
    )                                                                           \
    return value;                                                               \
}                                                                               \
                                                                                \
zt_grid2d_##SUFFIX *zt_grid2d_##SUFFIX##_set(                                   \
        zt_grid2d_##SUFFIX *grid,                                               \
        zt_int row,                                                             \
        zt_int col,                                                             \
        ELEM_TYPE value) {                                                      \
    size_t _index;                                                              \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_set requires grid");                             \
    if (row < 0 || (size_t)row >= grid->rows ||                                 \
            col < 0 || (size_t)col >= grid->cols) {                             \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "grid2d<" TYPE_LABEL "> index out of bounds");                      \
    }                                                                           \
    _index = (size_t)row * grid->cols + (size_t)col;                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        ELEM_TYPE old = grid->data[_index];                                     \
        if ((const void *)value != NULL) zt_retain((void *)value);              \
        if ((const void *)old != NULL) zt_release(old);                         \
    )                                                                           \
    grid->data[_index] = value;                                                 \
    return grid;                                                                \
}                                                                               \
                                                                                \
zt_grid2d_##SUFFIX *zt_grid2d_##SUFFIX##_set_owned(                             \
        zt_grid2d_##SUFFIX *grid,                                               \
        zt_int row,                                                             \
        zt_int col,                                                             \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_set_owned requires grid");                       \
    if (grid->header.rc > 1u) {                                                 \
        size_t _i;                                                              \
        zt_grid2d_##SUFFIX *clone = zt_grid2d_##SUFFIX##_new(                   \
            (zt_int)grid->rows,                                                 \
            (zt_int)grid->cols);                                                \
        for (_i = 0; _i < grid->len; _i += 1) {                                \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_release(clone->data[_i]);                                \
                }                                                               \
            )                                                                   \
            clone->data[_i] = grid->data[_i];                                   \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain(clone->data[_i]);                                 \
                }                                                               \
            )                                                                   \
        }                                                                       \
        grid = clone;                                                           \
    } else {                                                                    \
        zt_retain(grid);                                                        \
    }                                                                           \
    return zt_grid2d_##SUFFIX##_set(grid, row, col, value);                     \
}                                                                               \
                                                                                \
zt_grid2d_##SUFFIX *zt_grid2d_##SUFFIX##_fill(                                  \
        zt_grid2d_##SUFFIX *grid,                                               \
        ELEM_TYPE value) {                                                      \
    size_t _i;                                                                  \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_fill requires grid");                            \
    for (_i = 0; _i < grid->len; _i += 1) {                                    \
        ZT_TEMPLATE_IF(IS_PTR,                                                  \
            ELEM_TYPE old = grid->data[_i];                                     \
            if ((const void *)value != NULL) zt_retain((void *)value);          \
            grid->data[_i] = value;                                             \
            if ((const void *)old != NULL) zt_release(old);                     \
        )                                                                       \
        ZT_TEMPLATE_IF_NOT(IS_PTR,                                              \
            grid->data[_i] = value;                                             \
        )                                                                       \
    }                                                                           \
    return grid;                                                                \
}                                                                               \
                                                                                \
zt_grid2d_##SUFFIX *zt_grid2d_##SUFFIX##_fill_owned(                            \
        zt_grid2d_##SUFFIX *grid,                                               \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_fill_owned requires grid");                      \
    if (grid->header.rc > 1u) {                                                 \
        size_t _i;                                                              \
        zt_grid2d_##SUFFIX *clone = zt_grid2d_##SUFFIX##_new(                   \
            (zt_int)grid->rows,                                                 \
            (zt_int)grid->cols);                                                \
        for (_i = 0; _i < grid->len; _i += 1) {                                \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_release(clone->data[_i]);                                \
                }                                                               \
            )                                                                   \
            clone->data[_i] = grid->data[_i];                                   \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain(clone->data[_i]);                                 \
                }                                                               \
            )                                                                   \
        }                                                                       \
        grid = clone;                                                           \
    } else {                                                                    \
        zt_retain(grid);                                                        \
    }                                                                           \
    return zt_grid2d_##SUFFIX##_fill(grid, value);                              \
}                                                                               \
                                                                                \
zt_int zt_grid2d_##SUFFIX##_rows(const zt_grid2d_##SUFFIX *grid) {             \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_rows requires grid");                            \
    return (zt_int)grid->rows;                                                  \
}                                                                               \
                                                                                \
zt_int zt_grid2d_##SUFFIX##_cols(const zt_grid2d_##SUFFIX *grid) {             \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_cols requires grid");                            \
    return (zt_int)grid->cols;                                                  \
}                                                                               \
                                                                                \
zt_list_##SUFFIX *zt_grid2d_##SUFFIX##_values(                                  \
        const zt_grid2d_##SUFFIX *grid) {                                       \
    zt_runtime_require_grid2d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid2d_" #SUFFIX "_values requires grid");                          \
    return zt_list_##SUFFIX##_from_array(                                       \
        (ELEM_TYPE const *)grid->data,                                           \
        grid->len);                                                             \
}

#define ZT_DEFINE_GRID3D_IMPL(                                                  \
        SUFFIX,                                                                 \
        ELEM_TYPE,                                                              \
        HEAP_KIND,                                                              \
        IS_PTR,                                                                 \
        TYPE_LABEL,                                                             \
        EMPTY_VALUE_EXPR)                                                       \
static void zt_runtime_require_grid3d_##SUFFIX(                                 \
        const zt_grid3d_##SUFFIX *grid,                                         \
        const char *message) {                                                  \
    if (grid == NULL) {                                                         \
        zt_runtime_error(ZT_ERR_PANIC, message);                                \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_free_grid3d_##SUFFIX(zt_grid3d_##SUFFIX *grid) {                \
    if (grid == NULL) return;                                                   \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (size_t _i = 0; _i < grid->len; _i += 1) {                         \
            zt_release(grid->data[_i]);                                         \
        }                                                                       \
    )                                                                           \
    free(grid->data);                                                           \
    grid->data = NULL;                                                          \
    grid->depth = 0;                                                            \
    grid->rows = 0;                                                             \
    grid->cols = 0;                                                             \
    grid->len = 0;                                                              \
    grid->capacity = 0;                                                         \
    free(grid);                                                                 \
}                                                                               \
                                                                                \
zt_grid3d_##SUFFIX *zt_grid3d_##SUFFIX##_new(                                   \
        zt_int depth,                                                           \
        zt_int rows,                                                            \
        zt_int cols) {                                                          \
    zt_grid3d_##SUFFIX *grid;                                                   \
    size_t total;                                                               \
    if (depth <= 0 || rows <= 0 || cols <= 0) {                                 \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "grid3d<" TYPE_LABEL "> dimensions must be positive");              \
    }                                                                           \
    total = (size_t)depth * (size_t)rows * (size_t)cols;                        \
    grid = (zt_grid3d_##SUFFIX *)calloc(1, sizeof(zt_grid3d_##SUFFIX));         \
    if (grid == NULL) {                                                         \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate grid3d<" TYPE_LABEL ">");                       \
    }                                                                           \
    grid->header.rc = 1;                                                        \
    grid->header.kind = (uint32_t)(HEAP_KIND);                                  \
    grid->depth = (size_t)depth;                                                \
    grid->rows = (size_t)rows;                                                  \
    grid->cols = (size_t)cols;                                                  \
    grid->len = total;                                                          \
    grid->capacity = total;                                                     \
    grid->data = (ELEM_TYPE *)calloc(total, sizeof(ELEM_TYPE));                 \
    if (grid->data == NULL) {                                                   \
        free(grid);                                                             \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate grid3d<" TYPE_LABEL "> data");                  \
    }                                                                           \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (size_t _i = 0; _i < total; _i += 1) {                             \
            grid->data[_i] = (EMPTY_VALUE_EXPR);                                \
        }                                                                       \
    )                                                                           \
    return grid;                                                                \
}                                                                               \
                                                                                \
ELEM_TYPE zt_grid3d_##SUFFIX##_get(                                             \
        const zt_grid3d_##SUFFIX *grid,                                         \
        zt_int layer,                                                           \
        zt_int row,                                                             \
        zt_int col) {                                                           \
    size_t _index;                                                              \
    ELEM_TYPE value;                                                            \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_get requires grid");                             \
    if (layer < 0 || (size_t)layer >= grid->depth ||                            \
            row < 0 || (size_t)row >= grid->rows ||                             \
            col < 0 || (size_t)col >= grid->cols) {                             \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "grid3d<" TYPE_LABEL "> index out of bounds");                      \
    }                                                                           \
    _index = ((size_t)layer * grid->rows + (size_t)row) * grid->cols +          \
        (size_t)col;                                                            \
    value = grid->data[_index];                                                 \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value == NULL) return (EMPTY_VALUE_EXPR);             \
        zt_retain((void *)value);                                               \
    )                                                                           \
    return value;                                                               \
}                                                                               \
                                                                                \
zt_grid3d_##SUFFIX *zt_grid3d_##SUFFIX##_set(                                   \
        zt_grid3d_##SUFFIX *grid,                                               \
        zt_int layer,                                                           \
        zt_int row,                                                             \
        zt_int col,                                                             \
        ELEM_TYPE value) {                                                      \
    size_t _index;                                                              \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_set requires grid");                             \
    if (layer < 0 || (size_t)layer >= grid->depth ||                            \
            row < 0 || (size_t)row >= grid->rows ||                             \
            col < 0 || (size_t)col >= grid->cols) {                             \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "grid3d<" TYPE_LABEL "> index out of bounds");                      \
    }                                                                           \
    _index = ((size_t)layer * grid->rows + (size_t)row) * grid->cols +          \
        (size_t)col;                                                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        ELEM_TYPE old = grid->data[_index];                                     \
        if ((const void *)value != NULL) zt_retain((void *)value);              \
        if ((const void *)old != NULL) zt_release(old);                         \
    )                                                                           \
    grid->data[_index] = value;                                                 \
    return grid;                                                                \
}                                                                               \
                                                                                \
zt_grid3d_##SUFFIX *zt_grid3d_##SUFFIX##_set_owned(                             \
        zt_grid3d_##SUFFIX *grid,                                               \
        zt_int layer,                                                           \
        zt_int row,                                                             \
        zt_int col,                                                             \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_set_owned requires grid");                       \
    if (grid->header.rc > 1u) {                                                 \
        size_t _i;                                                              \
        zt_grid3d_##SUFFIX *clone = zt_grid3d_##SUFFIX##_new(                   \
            (zt_int)grid->depth,                                                \
            (zt_int)grid->rows,                                                 \
            (zt_int)grid->cols);                                                \
        for (_i = 0; _i < grid->len; _i += 1) {                                \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_release(clone->data[_i]);                                \
                }                                                               \
            )                                                                   \
            clone->data[_i] = grid->data[_i];                                   \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain(clone->data[_i]);                                 \
                }                                                               \
            )                                                                   \
        }                                                                       \
        grid = clone;                                                           \
    } else {                                                                    \
        zt_retain(grid);                                                        \
    }                                                                           \
    return zt_grid3d_##SUFFIX##_set(grid, layer, row, col, value);              \
}                                                                               \
                                                                                \
zt_grid3d_##SUFFIX *zt_grid3d_##SUFFIX##_fill(                                  \
        zt_grid3d_##SUFFIX *grid,                                               \
        ELEM_TYPE value) {                                                      \
    size_t _i;                                                                  \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_fill requires grid");                            \
    for (_i = 0; _i < grid->len; _i += 1) {                                    \
        ZT_TEMPLATE_IF(IS_PTR,                                                  \
            ELEM_TYPE old = grid->data[_i];                                     \
            if ((const void *)value != NULL) zt_retain((void *)value);          \
            grid->data[_i] = value;                                             \
            if ((const void *)old != NULL) zt_release(old);                     \
        )                                                                       \
        ZT_TEMPLATE_IF_NOT(IS_PTR,                                              \
            grid->data[_i] = value;                                             \
        )                                                                       \
    }                                                                           \
    return grid;                                                                \
}                                                                               \
                                                                                \
zt_grid3d_##SUFFIX *zt_grid3d_##SUFFIX##_fill_owned(                            \
        zt_grid3d_##SUFFIX *grid,                                               \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_fill_owned requires grid");                      \
    if (grid->header.rc > 1u) {                                                 \
        size_t _i;                                                              \
        zt_grid3d_##SUFFIX *clone = zt_grid3d_##SUFFIX##_new(                   \
            (zt_int)grid->depth,                                                \
            (zt_int)grid->rows,                                                 \
            (zt_int)grid->cols);                                                \
        for (_i = 0; _i < grid->len; _i += 1) {                                \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_release(clone->data[_i]);                                \
                }                                                               \
            )                                                                   \
            clone->data[_i] = grid->data[_i];                                   \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain(clone->data[_i]);                                 \
                }                                                               \
            )                                                                   \
        }                                                                       \
        grid = clone;                                                           \
    } else {                                                                    \
        zt_retain(grid);                                                        \
    }                                                                           \
    return zt_grid3d_##SUFFIX##_fill(grid, value);                              \
}                                                                               \
                                                                                \
zt_int zt_grid3d_##SUFFIX##_depth(const zt_grid3d_##SUFFIX *grid) {            \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_depth requires grid");                           \
    return (zt_int)grid->depth;                                                 \
}                                                                               \
                                                                                \
zt_int zt_grid3d_##SUFFIX##_rows(const zt_grid3d_##SUFFIX *grid) {             \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_rows requires grid");                            \
    return (zt_int)grid->rows;                                                  \
}                                                                               \
                                                                                \
zt_int zt_grid3d_##SUFFIX##_cols(const zt_grid3d_##SUFFIX *grid) {             \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_cols requires grid");                            \
    return (zt_int)grid->cols;                                                  \
}                                                                               \
                                                                                \
zt_list_##SUFFIX *zt_grid3d_##SUFFIX##_values(                                  \
        const zt_grid3d_##SUFFIX *grid) {                                       \
    zt_runtime_require_grid3d_##SUFFIX(                                         \
        grid,                                                                   \
        "zt_grid3d_" #SUFFIX "_values requires grid");                          \
    return zt_list_##SUFFIX##_from_array(                                       \
        (ELEM_TYPE const *)grid->data,                                           \
        grid->len);                                                             \
}

#define ZT_DEFINE_PQUEUE_IMPL(                                                  \
        SUFFIX,                                                                 \
        ELEM_TYPE,                                                              \
        HEAP_KIND,                                                              \
        OPTIONAL_TYPE,                                                          \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN,                                                      \
        IS_PTR,                                                                 \
        TYPE_LABEL,                                                             \
        LESS_EXPR)                                                              \
static void zt_runtime_require_pqueue_##SUFFIX(                                 \
        const zt_pqueue_##SUFFIX *heap,                                         \
        const char *message) {                                                  \
    if (heap == NULL) {                                                         \
        zt_runtime_error(ZT_ERR_PANIC, message);                                \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_free_pqueue_##SUFFIX(zt_pqueue_##SUFFIX *heap) {                \
    if (heap == NULL) return;                                                   \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (size_t _i = 0; _i < heap->len; _i += 1) {                         \
            zt_release(heap->data[_i]);                                         \
        }                                                                       \
    )                                                                           \
    free(heap->data);                                                           \
    heap->data = NULL;                                                          \
    heap->len = 0;                                                              \
    heap->capacity = 0;                                                         \
    free(heap);                                                                 \
}                                                                               \
                                                                                \
zt_pqueue_##SUFFIX *zt_pqueue_##SUFFIX##_new(void) {                           \
    zt_pqueue_##SUFFIX *heap;                                                   \
    heap = (zt_pqueue_##SUFFIX *)calloc(1, sizeof(zt_pqueue_##SUFFIX));         \
    if (heap == NULL) {                                                         \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate pqueue<" TYPE_LABEL "> header");                \
    }                                                                           \
    heap->header.rc = 1;                                                        \
    heap->header.kind = (uint32_t)(HEAP_KIND);                                  \
    return heap;                                                                \
}                                                                               \
                                                                                \
static void zt_pqueue_##SUFFIX##_ensure_capacity(                               \
        zt_pqueue_##SUFFIX *heap,                                               \
        size_t needed) {                                                        \
    size_t new_cap;                                                             \
    ELEM_TYPE *new_data;                                                        \
    if (heap->capacity >= needed) return;                                       \
    new_cap = heap->capacity == 0 ? 8 : heap->capacity * 2;                     \
    while (new_cap < needed) new_cap *= 2;                                      \
    new_data = (ELEM_TYPE *)realloc(heap->data, new_cap * sizeof(ELEM_TYPE));   \
    if (new_data == NULL) {                                                     \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to grow pqueue<" TYPE_LABEL ">");                           \
    }                                                                           \
    heap->data = new_data;                                                      \
    heap->capacity = new_cap;                                                   \
}                                                                               \
                                                                                \
static void zt_pqueue_##SUFFIX##_sift_up(                                       \
        zt_pqueue_##SUFFIX *heap,                                               \
        size_t index) {                                                         \
    while (index > 0) {                                                         \
        size_t parent = (index - 1) / 2;                                        \
        ELEM_TYPE _lhs = heap->data[index];                                     \
        ELEM_TYPE _rhs = heap->data[parent];                                    \
        if (!(LESS_EXPR)) break;                                                \
        {                                                                       \
            ELEM_TYPE temp = heap->data[index];                                 \
            heap->data[index] = heap->data[parent];                             \
            heap->data[parent] = temp;                                          \
        }                                                                       \
        index = parent;                                                         \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_pqueue_##SUFFIX##_sift_down(                                     \
        zt_pqueue_##SUFFIX *heap,                                               \
        size_t index) {                                                         \
    while (1) {                                                                 \
        size_t left = 2 * index + 1;                                            \
        size_t right = 2 * index + 2;                                           \
        size_t smallest = index;                                                \
        if (left < heap->len) {                                                 \
            ELEM_TYPE _lhs = heap->data[left];                                  \
            ELEM_TYPE _rhs = heap->data[smallest];                              \
            if (LESS_EXPR) smallest = left;                                     \
        }                                                                       \
        if (right < heap->len) {                                                \
            ELEM_TYPE _lhs = heap->data[right];                                 \
            ELEM_TYPE _rhs = heap->data[smallest];                              \
            if (LESS_EXPR) smallest = right;                                    \
        }                                                                       \
        if (smallest == index) break;                                           \
        {                                                                       \
            ELEM_TYPE temp = heap->data[index];                                 \
            heap->data[index] = heap->data[smallest];                           \
            heap->data[smallest] = temp;                                        \
        }                                                                       \
        index = smallest;                                                       \
    }                                                                           \
}                                                                               \
                                                                                \
zt_pqueue_##SUFFIX *zt_pqueue_##SUFFIX##_push(                                  \
        zt_pqueue_##SUFFIX *heap,                                               \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_pqueue_##SUFFIX(                                         \
        heap,                                                                   \
        "zt_pqueue_" #SUFFIX "_push requires heap");                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value != NULL) zt_retain((void *)value);              \
    )                                                                           \
    zt_pqueue_##SUFFIX##_ensure_capacity(heap, heap->len + 1);                  \
    heap->data[heap->len] = value;                                              \
    heap->len += 1;                                                             \
    zt_pqueue_##SUFFIX##_sift_up(heap, heap->len - 1);                          \
    return heap;                                                                \
}                                                                               \
                                                                                \
zt_pqueue_##SUFFIX *zt_pqueue_##SUFFIX##_push_owned(                            \
        zt_pqueue_##SUFFIX *heap,                                               \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_pqueue_##SUFFIX(                                         \
        heap,                                                                   \
        "zt_pqueue_" #SUFFIX "_push_owned requires heap");                      \
    if (heap->header.rc > 1u) {                                                 \
        size_t _i;                                                              \
        zt_pqueue_##SUFFIX *clone = zt_pqueue_##SUFFIX##_new();                 \
        zt_pqueue_##SUFFIX##_ensure_capacity(clone, heap->len);                 \
        for (_i = 0; _i < heap->len; _i += 1) {                                \
            clone->data[_i] = heap->data[_i];                                   \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain((void *)clone->data[_i]);                         \
                }                                                               \
            )                                                                   \
        }                                                                       \
        clone->len = heap->len;                                                 \
        heap = clone;                                                           \
    } else {                                                                    \
        zt_retain(heap);                                                        \
    }                                                                           \
    return zt_pqueue_##SUFFIX##_push(heap, value);                              \
}                                                                               \
                                                                                \
OPTIONAL_TYPE zt_pqueue_##SUFFIX##_pop(zt_pqueue_##SUFFIX *heap) {             \
    ELEM_TYPE result;                                                           \
    zt_runtime_require_pqueue_##SUFFIX(                                         \
        heap,                                                                   \
        "zt_pqueue_" #SUFFIX "_pop requires heap");                             \
    if (heap->len == 0) {                                                       \
        return OPTIONAL_EMPTY_FN();                                             \
    }                                                                           \
    result = heap->data[0];                                                     \
    heap->len -= 1;                                                             \
    if (heap->len > 0) {                                                        \
        heap->data[0] = heap->data[heap->len];                                  \
        zt_pqueue_##SUFFIX##_sift_down(heap, 0);                                \
    }                                                                           \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        OPTIONAL_TYPE opt;                                                      \
        heap->data[heap->len] = (ELEM_TYPE)NULL;                                \
        opt.is_present = true;                                                  \
        opt.value = result;                                                     \
        return opt;                                                             \
    )                                                                           \
    ZT_TEMPLATE_IF_NOT(IS_PTR,                                                  \
        return OPTIONAL_PRESENT_FN(result);                                     \
    )                                                                           \
    return OPTIONAL_EMPTY_FN(); /* unreachable */                               \
}                                                                               \
                                                                                \
OPTIONAL_TYPE zt_pqueue_##SUFFIX##_peek(const zt_pqueue_##SUFFIX *heap) {      \
    zt_runtime_require_pqueue_##SUFFIX(                                         \
        heap,                                                                   \
        "zt_pqueue_" #SUFFIX "_peek requires heap");                            \
    if (heap->len == 0) {                                                       \
        return OPTIONAL_EMPTY_FN();                                             \
    }                                                                           \
    return OPTIONAL_PRESENT_FN(heap->data[0]);                                  \
}                                                                               \
                                                                                \
zt_int zt_pqueue_##SUFFIX##_len(const zt_pqueue_##SUFFIX *heap) {              \
    zt_runtime_require_pqueue_##SUFFIX(                                         \
        heap,                                                                   \
        "zt_pqueue_" #SUFFIX "_len requires heap");                             \
    return (zt_int)heap->len;                                                   \
}                                                                               \
                                                                                \
zt_list_##SUFFIX *zt_pqueue_##SUFFIX##_values(                                  \
        const zt_pqueue_##SUFFIX *heap) {                                       \
    zt_pqueue_##SUFFIX *clone;                                                  \
    zt_list_##SUFFIX *out;                                                      \
    size_t _i;                                                                  \
    zt_runtime_require_pqueue_##SUFFIX(                                         \
        heap,                                                                   \
        "zt_pqueue_" #SUFFIX "_values requires heap");                          \
    clone = zt_pqueue_##SUFFIX##_new();                                         \
    zt_pqueue_##SUFFIX##_ensure_capacity(clone, heap->len);                     \
    for (_i = 0; _i < heap->len; _i += 1) {                                    \
        clone->data[_i] = heap->data[_i];                                       \
        ZT_TEMPLATE_IF(IS_PTR,                                                  \
            if ((const void *)clone->data[_i] != NULL) {                        \
                zt_retain((void *)clone->data[_i]);                             \
            }                                                                   \
        )                                                                       \
    }                                                                           \
    clone->len = heap->len;                                                     \
    out = zt_list_##SUFFIX##_new();                                             \
    while (clone->len > 0) {                                                    \
        OPTIONAL_TYPE opt = zt_pqueue_##SUFFIX##_pop(clone);                    \
        if (!opt.is_present) break;                                             \
        zt_list_##SUFFIX##_push(out, opt.value);                                \
        ZT_TEMPLATE_IF(IS_PTR,                                                  \
            if ((const void *)opt.value != NULL) {                              \
                zt_release((void *)opt.value);                                  \
            }                                                                   \
        )                                                                       \
    }                                                                           \
    zt_release(clone);                                                          \
    return out;                                                                 \
}

#define ZT_DEFINE_CIRCBUF_IMPL(                                                 \
        SUFFIX,                                                                 \
        ELEM_TYPE,                                                              \
        HEAP_KIND,                                                              \
        OPTIONAL_TYPE,                                                          \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN,                                                      \
        IS_PTR,                                                                 \
        TYPE_LABEL)                                                             \
static void zt_runtime_require_circbuf_##SUFFIX(                                \
        const zt_circbuf_##SUFFIX *buf,                                         \
        const char *message) {                                                  \
    if (buf == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC, message);                                \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_free_circbuf_##SUFFIX(zt_circbuf_##SUFFIX *buf) {               \
    if (buf == NULL) return;                                                    \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (size_t _i = 0; _i < buf->capacity; _i += 1) {                     \
            zt_release(buf->data[_i]);                                          \
        }                                                                       \
    )                                                                           \
    free(buf->data);                                                            \
    buf->data = NULL;                                                           \
    buf->capacity = 0;                                                          \
    buf->head = 0;                                                              \
    buf->len = 0;                                                               \
    free(buf);                                                                  \
}                                                                               \
                                                                                \
zt_circbuf_##SUFFIX *zt_circbuf_##SUFFIX##_new(zt_int capacity) {              \
    zt_circbuf_##SUFFIX *buf;                                                   \
    if (capacity <= 0) {                                                        \
        zt_runtime_error(                                                       \
            ZT_ERR_INDEX,                                                       \
            "circbuf<" TYPE_LABEL "> capacity must be positive");               \
    }                                                                           \
    buf = (zt_circbuf_##SUFFIX *)calloc(1, sizeof(zt_circbuf_##SUFFIX));       \
    if (buf == NULL) {                                                          \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate circbuf<" TYPE_LABEL "> header");               \
    }                                                                           \
    buf->header.rc = 1;                                                         \
    buf->header.kind = (uint32_t)(HEAP_KIND);                                   \
    buf->capacity = (size_t)capacity;                                           \
    buf->head = 0;                                                              \
    buf->len = 0;                                                               \
    buf->data = (ELEM_TYPE *)calloc((size_t)capacity, sizeof(ELEM_TYPE));       \
    if (buf->data == NULL) {                                                    \
        free(buf);                                                              \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate circbuf<" TYPE_LABEL "> data");                 \
    }                                                                           \
    return buf;                                                                 \
}                                                                               \
                                                                                \
zt_circbuf_##SUFFIX *zt_circbuf_##SUFFIX##_push(                                \
        zt_circbuf_##SUFFIX *buf,                                               \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_push requires buf");                            \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value != NULL) zt_retain((void *)value);              \
    )                                                                           \
    if (buf->len < buf->capacity) {                                             \
        size_t pos = (buf->head + buf->len) % buf->capacity;                    \
        buf->data[pos] = value;                                                 \
        buf->len += 1;                                                          \
    } else {                                                                    \
        ZT_TEMPLATE_IF(IS_PTR,                                                  \
            ELEM_TYPE old = buf->data[buf->head];                               \
            buf->data[buf->head] = value;                                       \
            buf->head = (buf->head + 1) % buf->capacity;                        \
            if ((const void *)old != NULL) zt_release((void *)old);             \
        )                                                                       \
        ZT_TEMPLATE_IF_NOT(IS_PTR,                                              \
            buf->data[buf->head] = value;                                       \
            buf->head = (buf->head + 1) % buf->capacity;                        \
        )                                                                       \
    }                                                                           \
    return buf;                                                                 \
}                                                                               \
                                                                                \
zt_circbuf_##SUFFIX *zt_circbuf_##SUFFIX##_push_owned(                          \
        zt_circbuf_##SUFFIX *buf,                                               \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_push_owned requires buf");                      \
    if (buf->header.rc > 1u) {                                                  \
        size_t _i;                                                              \
        zt_circbuf_##SUFFIX *clone = zt_circbuf_##SUFFIX##_new(                 \
            (zt_int)buf->capacity);                                             \
        for (_i = 0; _i < buf->len; _i += 1) {                                 \
            clone->data[_i] = buf->data[(buf->head + _i) % buf->capacity];      \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain((void *)clone->data[_i]);                         \
                }                                                               \
            )                                                                   \
        }                                                                       \
        clone->len = buf->len;                                                  \
        buf = clone;                                                            \
    } else {                                                                    \
        zt_retain(buf);                                                         \
    }                                                                           \
    return zt_circbuf_##SUFFIX##_push(buf, value);                              \
}                                                                               \
                                                                                \
OPTIONAL_TYPE zt_circbuf_##SUFFIX##_pop(zt_circbuf_##SUFFIX *buf) {            \
    ELEM_TYPE value;                                                            \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_pop requires buf");                             \
    if (buf->len == 0) {                                                        \
        return OPTIONAL_EMPTY_FN();                                             \
    }                                                                           \
    value = buf->data[buf->head];                                               \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        buf->data[buf->head] = (ELEM_TYPE)NULL;                                 \
    )                                                                           \
    buf->head = (buf->head + 1) % buf->capacity;                                \
    buf->len -= 1;                                                              \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        OPTIONAL_TYPE opt;                                                      \
        opt.is_present = true;                                                  \
        opt.value = value;                                                      \
        return opt;                                                             \
    )                                                                           \
    ZT_TEMPLATE_IF_NOT(IS_PTR,                                                  \
        return OPTIONAL_PRESENT_FN(value);                                      \
    )                                                                           \
    return OPTIONAL_EMPTY_FN(); /* unreachable */                               \
}                                                                               \
                                                                                \
OPTIONAL_TYPE zt_circbuf_##SUFFIX##_peek(const zt_circbuf_##SUFFIX *buf) {     \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_peek requires buf");                            \
    if (buf->len == 0) {                                                        \
        return OPTIONAL_EMPTY_FN();                                             \
    }                                                                           \
    return OPTIONAL_PRESENT_FN(buf->data[buf->head]);                           \
}                                                                               \
                                                                                \
zt_int zt_circbuf_##SUFFIX##_len(const zt_circbuf_##SUFFIX *buf) {             \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_len requires buf");                             \
    return (zt_int)buf->len;                                                    \
}                                                                               \
                                                                                \
zt_int zt_circbuf_##SUFFIX##_capacity(const zt_circbuf_##SUFFIX *buf) {        \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_capacity requires buf");                        \
    return (zt_int)buf->capacity;                                               \
}                                                                               \
                                                                                \
zt_bool zt_circbuf_##SUFFIX##_is_full(const zt_circbuf_##SUFFIX *buf) {        \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_is_full requires buf");                         \
    return buf->len >= buf->capacity;                                           \
}                                                                               \
                                                                                \
zt_list_##SUFFIX *zt_circbuf_##SUFFIX##_values(                                 \
        const zt_circbuf_##SUFFIX *buf) {                                       \
    zt_list_##SUFFIX *out;                                                      \
    size_t _i;                                                                  \
    zt_runtime_require_circbuf_##SUFFIX(                                        \
        buf,                                                                    \
        "zt_circbuf_" #SUFFIX "_values requires buf");                          \
    out = zt_list_##SUFFIX##_new();                                             \
    for (_i = 0; _i < buf->len; _i += 1) {                                     \
        zt_list_##SUFFIX##_push(                                                \
            out,                                                                \
            buf->data[(buf->head + _i) % buf->capacity]);                       \
    }                                                                           \
    return out;                                                                 \
}

#define ZT_DEFINE_BTREEMAP_IMPL(                                                \
        SUFFIX,                                                                 \
        KEY_TYPE,                                                               \
        VALUE_TYPE,                                                             \
        HEAP_KIND,                                                              \
        OPTIONAL_PRESENT_FN,                                                    \
        OPTIONAL_EMPTY_FN,                                                      \
        KEY_IS_PTR,                                                             \
        VALUE_IS_PTR,                                                           \
        TYPE_LABEL,                                                             \
        KEY_CMP_EXPR,                                                           \
        GET_MISSING_EXPR)                                                       \
static void zt_runtime_require_btreemap_##SUFFIX(                               \
        const zt_btreemap_##SUFFIX *map,                                        \
        const char *message) {                                                  \
    if (map == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC, message);                                \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_free_btreemap_##SUFFIX(zt_btreemap_##SUFFIX *map) {             \
    size_t _i;                                                                  \
    if (map == NULL) return;                                                    \
    for (_i = 0; _i < map->len; _i += 1) {                                     \
        ZT_TEMPLATE_IF(KEY_IS_PTR,                                              \
            zt_release((void *)map->keys[_i]);                                  \
        )                                                                       \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            zt_release((void *)map->values[_i]);                                \
        )                                                                       \
    }                                                                           \
    free(map->keys);                                                            \
    free(map->values);                                                          \
    map->keys = NULL;                                                           \
    map->values = NULL;                                                         \
    map->len = 0;                                                               \
    map->capacity = 0;                                                          \
    free(map);                                                                  \
}                                                                               \
                                                                                \
static size_t zt_btreemap_##SUFFIX##_find(                                      \
        const zt_btreemap_##SUFFIX *map,                                        \
        const KEY_TYPE key,                                                     \
        size_t *pos) {                                                          \
    size_t lo = 0;                                                              \
    size_t hi = map->len;                                                       \
    while (lo < hi) {                                                           \
        size_t mid = lo + (hi - lo) / 2;                                        \
        KEY_TYPE _lhs = map->keys[mid];                                         \
        int cmp = (KEY_CMP_EXPR);                                               \
        if (cmp == 0) {                                                         \
            if (pos != NULL) *pos = mid;                                        \
            return mid;                                                         \
        }                                                                       \
        if (cmp < 0) {                                                          \
            lo = mid + 1;                                                       \
        } else {                                                                \
            hi = mid;                                                           \
        }                                                                       \
    }                                                                           \
    if (pos != NULL) *pos = lo;                                                 \
    return map->len;                                                            \
}                                                                               \
                                                                                \
static void zt_btreemap_##SUFFIX##_ensure_capacity(                             \
        zt_btreemap_##SUFFIX *map,                                              \
        size_t needed) {                                                        \
    size_t new_cap;                                                             \
    KEY_TYPE *new_keys;                                                         \
    VALUE_TYPE *new_values;                                                     \
    if (map->capacity >= needed) return;                                        \
    new_cap = map->capacity == 0 ? 8 : map->capacity * 2;                       \
    while (new_cap < needed) new_cap *= 2;                                      \
    new_keys = (KEY_TYPE *)realloc(map->keys, new_cap * sizeof(KEY_TYPE));      \
    new_values = (VALUE_TYPE *)realloc(                                         \
        map->values,                                                            \
        new_cap * sizeof(VALUE_TYPE));                                          \
    if (new_keys == NULL || new_values == NULL) {                               \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to grow btreemap<" TYPE_LABEL ">");                         \
    }                                                                           \
    map->keys = new_keys;                                                       \
    map->values = new_values;                                                   \
    map->capacity = new_cap;                                                    \
}                                                                               \
                                                                                \
zt_btreemap_##SUFFIX *zt_btreemap_##SUFFIX##_new(void) {                       \
    zt_btreemap_##SUFFIX *map;                                                  \
    map = (zt_btreemap_##SUFFIX *)calloc(1, sizeof(zt_btreemap_##SUFFIX));      \
    if (map == NULL) {                                                          \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate btreemap<" TYPE_LABEL ">");                     \
    }                                                                           \
    map->header.rc = 1;                                                         \
    map->header.kind = (uint32_t)(HEAP_KIND);                                   \
    return map;                                                                 \
}                                                                               \
                                                                                \
zt_btreemap_##SUFFIX *zt_btreemap_##SUFFIX##_set(                               \
        zt_btreemap_##SUFFIX *map,                                              \
        KEY_TYPE key,                                                           \
        VALUE_TYPE value) {                                                     \
    size_t pos;                                                                 \
    size_t idx;                                                                 \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_set requires map");                            \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) {                                        \
            zt_runtime_error(                                                   \
                ZT_ERR_PANIC,                                                   \
                "btreemap<" TYPE_LABEL "> key cannot be null");                 \
        }                                                                       \
    )                                                                           \
    idx = zt_btreemap_##SUFFIX##_find(map, key, &pos);                          \
    if (idx < map->len) {                                                       \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            VALUE_TYPE old_value = map->values[idx];                            \
            if ((const void *)value != NULL) zt_retain((void *)value);          \
            map->values[idx] = value;                                           \
            if ((const void *)old_value != NULL) zt_release((void *)old_value); \
            return map;                                                         \
        )                                                                       \
        ZT_TEMPLATE_IF_NOT(VALUE_IS_PTR,                                        \
            map->values[idx] = value;                                           \
            return map;                                                         \
        )                                                                       \
    }                                                                           \
    zt_btreemap_##SUFFIX##_ensure_capacity(map, map->len + 1);                  \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key != NULL) zt_retain((void *)key);                  \
    )                                                                           \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)value != NULL) zt_retain((void *)value);              \
    )                                                                           \
    if (pos < map->len) {                                                       \
        memmove(map->keys + pos + 1, map->keys + pos,                           \
            (map->len - pos) * sizeof(KEY_TYPE));                               \
        memmove(map->values + pos + 1, map->values + pos,                       \
            (map->len - pos) * sizeof(VALUE_TYPE));                             \
    }                                                                           \
    map->keys[pos] = key;                                                       \
    map->values[pos] = value;                                                   \
    map->len += 1;                                                              \
    return map;                                                                 \
}                                                                               \
                                                                                \
zt_btreemap_##SUFFIX *zt_btreemap_##SUFFIX##_set_owned(                         \
        zt_btreemap_##SUFFIX *map,                                              \
        KEY_TYPE key,                                                           \
        VALUE_TYPE value) {                                                     \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_set_owned requires map");                      \
    if (map->header.rc > 1u) {                                                  \
        size_t _i;                                                              \
        zt_btreemap_##SUFFIX *clone = zt_btreemap_##SUFFIX##_new();             \
        zt_btreemap_##SUFFIX##_ensure_capacity(clone, map->len);                \
        for (_i = 0; _i < map->len; _i += 1) {                                 \
            clone->keys[_i] = map->keys[_i];                                    \
            clone->values[_i] = map->values[_i];                                \
            ZT_TEMPLATE_IF(KEY_IS_PTR,                                          \
                if ((const void *)clone->keys[_i] != NULL) {                    \
                    zt_retain((void *)clone->keys[_i]);                         \
                }                                                               \
            )                                                                   \
            ZT_TEMPLATE_IF(VALUE_IS_PTR,                                        \
                if ((const void *)clone->values[_i] != NULL) {                  \
                    zt_retain((void *)clone->values[_i]);                       \
                }                                                               \
            )                                                                   \
        }                                                                       \
        clone->len = map->len;                                                  \
        map = clone;                                                            \
    } else {                                                                    \
        zt_retain(map);                                                         \
    }                                                                           \
    return zt_btreemap_##SUFFIX##_set(map, key, value);                         \
}                                                                               \
                                                                                \
VALUE_TYPE zt_btreemap_##SUFFIX##_get(                                          \
        const zt_btreemap_##SUFFIX *map,                                        \
        const KEY_TYPE key) {                                                   \
    size_t idx;                                                                 \
    VALUE_TYPE value;                                                           \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_get requires map");                            \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) return (GET_MISSING_EXPR);               \
    )                                                                           \
    idx = zt_btreemap_##SUFFIX##_find(map, key, NULL);                          \
    if (idx >= map->len) return (GET_MISSING_EXPR);                             \
    value = map->values[idx];                                                   \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)value != NULL) zt_retain((void *)value);              \
    )                                                                           \
    return value;                                                               \
}                                                                               \
                                                                                \
zt_optional_text zt_btreemap_##SUFFIX##_get_optional(                           \
        const zt_btreemap_##SUFFIX *map,                                        \
        const KEY_TYPE key) {                                                   \
    size_t idx;                                                                 \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_get_optional requires map");                   \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) return OPTIONAL_EMPTY_FN();              \
    )                                                                           \
    idx = zt_btreemap_##SUFFIX##_find(map, key, NULL);                          \
    if (idx >= map->len) return OPTIONAL_EMPTY_FN();                            \
    ZT_TEMPLATE_IF(VALUE_IS_PTR,                                                \
        if ((const void *)map->values[idx] == NULL) return OPTIONAL_EMPTY_FN(); \
    )                                                                           \
    return OPTIONAL_PRESENT_FN(map->values[idx]);                               \
}                                                                               \
                                                                                \
zt_bool zt_btreemap_##SUFFIX##_contains(                                        \
        const zt_btreemap_##SUFFIX *map,                                        \
        const KEY_TYPE key) {                                                   \
    size_t idx;                                                                 \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_contains requires map");                       \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) return 0;                                \
    )                                                                           \
    idx = zt_btreemap_##SUFFIX##_find(map, key, NULL);                          \
    return idx < map->len ? 1 : 0;                                              \
}                                                                               \
                                                                                \
zt_btreemap_##SUFFIX *zt_btreemap_##SUFFIX##_remove(                            \
        zt_btreemap_##SUFFIX *map,                                              \
        const KEY_TYPE key) {                                                   \
    size_t idx;                                                                 \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_remove requires map");                         \
    ZT_TEMPLATE_IF(KEY_IS_PTR,                                                  \
        if ((const void *)key == NULL) return map;                              \
    )                                                                           \
    idx = zt_btreemap_##SUFFIX##_find(map, key, NULL);                          \
    if (idx >= map->len) return map;                                            \
    {                                                                           \
        KEY_TYPE old_key = map->keys[idx];                                      \
        VALUE_TYPE old_value = map->values[idx];                                \
        if (idx + 1 < map->len) {                                               \
            memmove(map->keys + idx, map->keys + idx + 1,                       \
                (map->len - idx - 1) * sizeof(KEY_TYPE));                       \
            memmove(map->values + idx, map->values + idx + 1,                   \
                (map->len - idx - 1) * sizeof(VALUE_TYPE));                     \
        }                                                                       \
        map->len -= 1;                                                          \
        ZT_TEMPLATE_IF(KEY_IS_PTR,                                              \
            if ((const void *)old_key != NULL) zt_release((void *)old_key);     \
        )                                                                       \
        ZT_TEMPLATE_IF(VALUE_IS_PTR,                                            \
            if ((const void *)old_value != NULL) {                              \
                zt_release((void *)old_value);                                  \
            }                                                                   \
        )                                                                       \
    }                                                                           \
    return map;                                                                 \
}                                                                               \
                                                                                \
zt_btreemap_##SUFFIX *zt_btreemap_##SUFFIX##_remove_owned(                      \
        zt_btreemap_##SUFFIX *map,                                              \
        const KEY_TYPE key) {                                                   \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_remove_owned requires map");                   \
    if (map->header.rc > 1u) {                                                  \
        size_t _i;                                                              \
        zt_btreemap_##SUFFIX *clone = zt_btreemap_##SUFFIX##_new();             \
        zt_btreemap_##SUFFIX##_ensure_capacity(clone, map->len);                \
        for (_i = 0; _i < map->len; _i += 1) {                                 \
            clone->keys[_i] = map->keys[_i];                                    \
            clone->values[_i] = map->values[_i];                                \
            ZT_TEMPLATE_IF(KEY_IS_PTR,                                          \
                if ((const void *)clone->keys[_i] != NULL) {                    \
                    zt_retain((void *)clone->keys[_i]);                         \
                }                                                               \
            )                                                                   \
            ZT_TEMPLATE_IF(VALUE_IS_PTR,                                        \
                if ((const void *)clone->values[_i] != NULL) {                  \
                    zt_retain((void *)clone->values[_i]);                       \
                }                                                               \
            )                                                                   \
        }                                                                       \
        clone->len = map->len;                                                  \
        map = clone;                                                            \
    } else {                                                                    \
        zt_retain(map);                                                         \
    }                                                                           \
    return zt_btreemap_##SUFFIX##_remove(map, key);                             \
}                                                                               \
                                                                                \
zt_int zt_btreemap_##SUFFIX##_len(const zt_btreemap_##SUFFIX *map) {           \
    zt_runtime_require_btreemap_##SUFFIX(                                       \
        map,                                                                    \
        "zt_btreemap_" #SUFFIX "_len requires map");                            \
    return (zt_int)map->len;                                                    \
}

#define ZT_DEFINE_BTREESET_IMPL(                                                \
        SUFFIX,                                                                 \
        ELEM_TYPE,                                                              \
        HEAP_KIND,                                                              \
        IS_PTR,                                                                 \
        TYPE_LABEL,                                                             \
        CMP_EXPR)                                                               \
static void zt_runtime_require_btreeset_##SUFFIX(                               \
        const zt_btreeset_##SUFFIX *set,                                        \
        const char *message) {                                                  \
    if (set == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC, message);                                \
    }                                                                           \
}                                                                               \
                                                                                \
static void zt_free_btreeset_##SUFFIX(zt_btreeset_##SUFFIX *set) {             \
    size_t _i;                                                                  \
    if (set == NULL) return;                                                    \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        for (_i = 0; _i < set->len; _i += 1) {                                 \
            zt_release((void *)set->data[_i]);                                  \
        }                                                                       \
    )                                                                           \
    free(set->data);                                                            \
    set->data = NULL;                                                           \
    set->len = 0;                                                               \
    set->capacity = 0;                                                          \
    free(set);                                                                  \
}                                                                               \
                                                                                \
static size_t zt_btreeset_##SUFFIX##_find(                                      \
        const zt_btreeset_##SUFFIX *set,                                        \
        const ELEM_TYPE value,                                                  \
        size_t *pos) {                                                          \
    size_t lo = 0;                                                              \
    size_t hi = set->len;                                                       \
    while (lo < hi) {                                                           \
        size_t mid = lo + (hi - lo) / 2;                                        \
        ELEM_TYPE _lhs = set->data[mid];                                        \
        int cmp = (CMP_EXPR);                                                   \
        if (cmp == 0) {                                                         \
            if (pos != NULL) *pos = mid;                                        \
            return mid;                                                         \
        }                                                                       \
        if (cmp < 0) {                                                          \
            lo = mid + 1;                                                       \
        } else {                                                                \
            hi = mid;                                                           \
        }                                                                       \
    }                                                                           \
    if (pos != NULL) *pos = lo;                                                 \
    return set->len;                                                            \
}                                                                               \
                                                                                \
static void zt_btreeset_##SUFFIX##_ensure_capacity(                             \
        zt_btreeset_##SUFFIX *set,                                              \
        size_t needed) {                                                        \
    size_t new_cap;                                                             \
    ELEM_TYPE *new_data;                                                        \
    if (set->capacity >= needed) return;                                        \
    new_cap = set->capacity == 0 ? 8 : set->capacity * 2;                       \
    while (new_cap < needed) new_cap *= 2;                                      \
    new_data = (ELEM_TYPE *)realloc(set->data, new_cap * sizeof(ELEM_TYPE));    \
    if (new_data == NULL) {                                                     \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to grow btreeset<" TYPE_LABEL ">");                         \
    }                                                                           \
    set->data = new_data;                                                       \
    set->capacity = new_cap;                                                    \
}                                                                               \
                                                                                \
zt_btreeset_##SUFFIX *zt_btreeset_##SUFFIX##_new(void) {                       \
    zt_btreeset_##SUFFIX *set;                                                  \
    set = (zt_btreeset_##SUFFIX *)calloc(1, sizeof(zt_btreeset_##SUFFIX));      \
    if (set == NULL) {                                                          \
        zt_runtime_error(                                                       \
            ZT_ERR_PLATFORM,                                                    \
            "failed to allocate btreeset<" TYPE_LABEL ">");                     \
    }                                                                           \
    set->header.rc = 1;                                                         \
    set->header.kind = (uint32_t)(HEAP_KIND);                                   \
    return set;                                                                 \
}                                                                               \
                                                                                \
zt_btreeset_##SUFFIX *zt_btreeset_##SUFFIX##_insert(                            \
        zt_btreeset_##SUFFIX *set,                                              \
        ELEM_TYPE value) {                                                      \
    size_t idx;                                                                 \
    size_t pos;                                                                 \
    zt_runtime_require_btreeset_##SUFFIX(                                       \
        set,                                                                    \
        "zt_btreeset_" #SUFFIX "_insert requires set");                         \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value == NULL) {                                      \
            zt_runtime_error(                                                   \
                ZT_ERR_PANIC,                                                   \
                "btreeset<" TYPE_LABEL "> value cannot be null");               \
        }                                                                       \
    )                                                                           \
    idx = zt_btreeset_##SUFFIX##_find(set, value, &pos);                        \
    if (idx < set->len) return set;                                             \
    zt_btreeset_##SUFFIX##_ensure_capacity(set, set->len + 1);                  \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        zt_retain((void *)value);                                               \
    )                                                                           \
    if (pos < set->len) {                                                       \
        memmove(set->data + pos + 1, set->data + pos,                           \
            (set->len - pos) * sizeof(ELEM_TYPE));                              \
    }                                                                           \
    set->data[pos] = value;                                                     \
    set->len += 1;                                                              \
    return set;                                                                 \
}                                                                               \
                                                                                \
zt_btreeset_##SUFFIX *zt_btreeset_##SUFFIX##_insert_owned(                      \
        zt_btreeset_##SUFFIX *set,                                              \
        ELEM_TYPE value) {                                                      \
    zt_runtime_require_btreeset_##SUFFIX(                                       \
        set,                                                                    \
        "zt_btreeset_" #SUFFIX "_insert_owned requires set");                   \
    if (set->header.rc > 1u) {                                                  \
        size_t _i;                                                              \
        zt_btreeset_##SUFFIX *clone = zt_btreeset_##SUFFIX##_new();             \
        zt_btreeset_##SUFFIX##_ensure_capacity(clone, set->len);                \
        for (_i = 0; _i < set->len; _i += 1) {                                 \
            clone->data[_i] = set->data[_i];                                    \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain((void *)clone->data[_i]);                         \
                }                                                               \
            )                                                                   \
        }                                                                       \
        clone->len = set->len;                                                  \
        set = clone;                                                            \
    } else {                                                                    \
        zt_retain(set);                                                         \
    }                                                                           \
    return zt_btreeset_##SUFFIX##_insert(set, value);                           \
}                                                                               \
                                                                                \
zt_bool zt_btreeset_##SUFFIX##_contains(                                        \
        const zt_btreeset_##SUFFIX *set,                                        \
        const ELEM_TYPE value) {                                                \
    zt_runtime_require_btreeset_##SUFFIX(                                       \
        set,                                                                    \
        "zt_btreeset_" #SUFFIX "_contains requires set");                       \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value == NULL) return 0;                              \
    )                                                                           \
    return zt_btreeset_##SUFFIX##_find(set, value, NULL) < set->len ? 1 : 0;    \
}                                                                               \
                                                                                \
zt_btreeset_##SUFFIX *zt_btreeset_##SUFFIX##_remove(                            \
        zt_btreeset_##SUFFIX *set,                                              \
        const ELEM_TYPE value) {                                                \
    size_t idx;                                                                 \
    zt_runtime_require_btreeset_##SUFFIX(                                       \
        set,                                                                    \
        "zt_btreeset_" #SUFFIX "_remove requires set");                         \
    ZT_TEMPLATE_IF(IS_PTR,                                                      \
        if ((const void *)value == NULL) return set;                            \
    )                                                                           \
    idx = zt_btreeset_##SUFFIX##_find(set, value, NULL);                        \
    if (idx >= set->len) return set;                                            \
    {                                                                           \
        ELEM_TYPE old = set->data[idx];                                         \
        if (idx + 1 < set->len) {                                               \
            memmove(set->data + idx, set->data + idx + 1,                       \
                (set->len - idx - 1) * sizeof(ELEM_TYPE));                      \
        }                                                                       \
        set->len -= 1;                                                          \
        ZT_TEMPLATE_IF(IS_PTR,                                                  \
            if ((const void *)old != NULL) zt_release((void *)old);             \
        )                                                                       \
    }                                                                           \
    return set;                                                                 \
}                                                                               \
                                                                                \
zt_btreeset_##SUFFIX *zt_btreeset_##SUFFIX##_remove_owned(                      \
        zt_btreeset_##SUFFIX *set,                                              \
        const ELEM_TYPE value) {                                                \
    zt_runtime_require_btreeset_##SUFFIX(                                       \
        set,                                                                    \
        "zt_btreeset_" #SUFFIX "_remove_owned requires set");                   \
    if (set->header.rc > 1u) {                                                  \
        size_t _i;                                                              \
        zt_btreeset_##SUFFIX *clone = zt_btreeset_##SUFFIX##_new();             \
        zt_btreeset_##SUFFIX##_ensure_capacity(clone, set->len);                \
        for (_i = 0; _i < set->len; _i += 1) {                                 \
            clone->data[_i] = set->data[_i];                                    \
            ZT_TEMPLATE_IF(IS_PTR,                                              \
                if ((const void *)clone->data[_i] != NULL) {                    \
                    zt_retain((void *)clone->data[_i]);                         \
                }                                                               \
            )                                                                   \
        }                                                                       \
        clone->len = set->len;                                                  \
        set = clone;                                                            \
    } else {                                                                    \
        zt_retain(set);                                                         \
    }                                                                           \
    return zt_btreeset_##SUFFIX##_remove(set, value);                           \
}                                                                               \
                                                                                \
zt_int zt_btreeset_##SUFFIX##_len(const zt_btreeset_##SUFFIX *set) {           \
    zt_runtime_require_btreeset_##SUFFIX(                                       \
        set,                                                                    \
        "zt_btreeset_" #SUFFIX "_len requires set");                            \
    return (zt_int)set->len;                                                    \
}

/* --------------------------------------------------------------------------
 * ZT_DEFINE_OPTIONAL(SUFFIX, ELEM_TYPE, IS_PTR)
 *
 * Generates:
 *   - typedef struct zt_optional_##SUFFIX
 *   - zt_optional_##SUFFIX zt_optional_##SUFFIX##_present(ELEM_TYPE value)
 *   - zt_optional_##SUFFIX zt_optional_##SUFFIX##_empty(void)
 *   - zt_bool zt_optional_##SUFFIX##_is_present(zt_optional_##SUFFIX value)
 *   - ELEM_TYPE zt_optional_##SUFFIX##_coalesce(zt_optional_##SUFFIX value, ELEM_TYPE fallback)
 * -------------------------------------------------------------------------- */

#define ZT_DEFINE_OPTIONAL(SUFFIX, ELEM_TYPE, IS_PTR)                           \
                                                                                 \
typedef struct zt_optional_##SUFFIX {                                            \
    zt_bool is_present;                                                          \
    ELEM_TYPE value;                                                             \
} zt_optional_##SUFFIX;                                                         \
                                                                                 \
static inline zt_optional_##SUFFIX zt_optional_##SUFFIX##_present(              \
        ELEM_TYPE value) {                                                       \
    zt_optional_##SUFFIX result;                                                 \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)value != NULL) { zt_retain((void *)value); }           \
    )                                                                            \
    result.is_present = true;                                                    \
    result.value = value;                                                         \
    return result;                                                               \
}                                                                                \
                                                                                 \
static inline zt_optional_##SUFFIX zt_optional_##SUFFIX##_empty(void) {         \
    zt_optional_##SUFFIX result;                                                 \
    result.value = (ELEM_TYPE)0;                                          \
    result.is_present = false;                                                   \
    return result;                                                               \
}                                                                                \
                                                                                 \
static inline zt_bool zt_optional_##SUFFIX##_is_present(                        \
        zt_optional_##SUFFIX opt) {                                              \
    return opt.is_present;                                                        \
}                                                                                \
                                                                                 \
static inline ELEM_TYPE zt_optional_##SUFFIX##_coalesce(                        \
        zt_optional_##SUFFIX opt, ELEM_TYPE fallback) {                          \
    ELEM_TYPE selected = opt.is_present ? opt.value : fallback;                  \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)selected != NULL) { zt_retain((void *)selected); }     \
    )                                                                            \
    return selected;                                                             \
}

/* --------------------------------------------------------------------------
 * ZT_DEFINE_OPTIONAL_IMPL(SUFFIX, ELEM_TYPE, IS_PTR)
 *
 * Like ZT_DEFINE_OPTIONAL but only generates function implementations.
 * Use this in zenith_rt.c where structs are already declared in zenith_rt.h.
 * -------------------------------------------------------------------------- */



#define ZT_DEFINE_OPTIONAL_IMPL(SUFFIX, ELEM_TYPE, IS_PTR)                      \
                                                                                 \
zt_optional_##SUFFIX zt_optional_##SUFFIX##_present(ELEM_TYPE value) {          \
    zt_optional_##SUFFIX result;                                                 \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)value != NULL) { zt_retain((void *)value); }           \
    )                                                                            \
    result.is_present = true;                                                    \
    result.value = value;                                                         \
    return result;                                                               \
}                                                                                \
                                                                                 \
zt_optional_##SUFFIX zt_optional_##SUFFIX##_empty(void) {                       \
    zt_optional_##SUFFIX result;                                                 \
    result.value = (ELEM_TYPE)0;                                          \
    result.is_present = false;                                                   \
    return result;                                                               \
}                                                                                \
                                                                                 \
zt_bool zt_optional_##SUFFIX##_is_present(zt_optional_##SUFFIX opt) {           \
    return opt.is_present;                                                        \
}                                                                                \
                                                                                 \
ELEM_TYPE zt_optional_##SUFFIX##_coalesce(                                       \
        zt_optional_##SUFFIX opt, ELEM_TYPE fallback) {                          \
    ELEM_TYPE selected = opt.is_present ? opt.value : fallback;                  \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)selected != NULL) { zt_retain((void *)selected); }     \
    )                                                                            \
    return selected;                                                             \
}                                                                                \
                                                                                 \
ELEM_TYPE zt_optional_##SUFFIX##_value(zt_optional_##SUFFIX opt) {               \
    ELEM_TYPE val = opt.value;                                                     \
    ZT_TEMPLATE_IF(IS_PTR,                                                       \
        if ((const void *)val != NULL) { zt_retain((void *)val); }               \
    )                                                                            \
    return val;                                                                   \
}

/* --------------------------------------------------------------------------
 * ZT_DEFINE_OUTCOME_IMPL(SUFFIX, VAL_TYPE, ERR_TYPE, VAL_IS_PTR)
 *
 * Generates outcome<VAL_TYPE, ERR_TYPE> function implementations.
 * VAL_IS_PTR = 0 for value types (int, void), 1 for managed pointers.
 * Error type is currently assumed to be managed text (zt_text *).
 *
 * Generates:
 *   - zt_outcome_##SUFFIX##_success(VAL_TYPE value)
 *   - zt_outcome_##SUFFIX##_failure(ERR_TYPE error)
 *   - zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome)
 *   - VAL_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome)
 *   - zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(zt_outcome_##SUFFIX outcome)
 *   - zt_outcome_##SUFFIX##_failure_message(const char *message)
 *   - zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome)
 * -------------------------------------------------------------------------- */

#define ZT_DEFINE_OUTCOME_IMPL(SUFFIX, VAL_TYPE, ERR_TYPE, VAL_IS_PTR)          \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(VAL_TYPE value) {             \
    zt_outcome_##SUFFIX outcome;                                                 \
    \
    if (VAL_IS_PTR && (const void *)value != NULL) { zt_retain((void *)value); }                               \
    outcome.is_success = true;                                                   \
    outcome.value = value;                                                        \
    outcome.error = NULL;                                                         \
    return outcome;                                                              \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(ERR_TYPE error) {             \
    zt_outcome_##SUFFIX outcome;                                                 \
    \
    if (error == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "zt_outcome_" #SUFFIX "_failure requires error");                    \
    }                                                                            \
    zt_retain((void *)error);                                                    \
    outcome.is_success = false;                                                 \
    outcome.value = (VAL_TYPE)0;                                                \
    outcome.error = error;                                                      \
    return outcome;                                                              \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(                      \
        const char *message) {                                                   \
    zt_text *error = zt_text_from_utf8_literal(zt_safe_message(message));        \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);          \
    zt_release(error);                                                           \
    return outcome;                                                              \
}                                                                                \
                                                                                 \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome) {         \
    return outcome.is_success;                                                    \
}                                                                                \
                                                                                 \
VAL_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome) {             \
    if (!outcome.is_success) {                                                   \
        zt_runtime_error(ZT_ERR_UNWRAP, "outcome_value on failure");            \
    }                                                                            \
    if (VAL_IS_PTR && (void *)outcome.value == NULL) {                          \
        zt_runtime_error(ZT_ERR_PANIC,                                          \
            "outcome<" #SUFFIX "> success value cannot be null");                \
    }                                                                            \
    if (VAL_IS_PTR && (const void *)outcome.value != NULL) { zt_retain((void *)outcome.value); }                       \
    return outcome.value;                                                         \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(                            \
        zt_outcome_##SUFFIX outcome) {                                           \
    if (outcome.is_success) {                                                    \
        if (VAL_IS_PTR) {                                                        \
            return zt_outcome_##SUFFIX##_success(outcome.value);                 \
        }                                                                        \
        return outcome;                                                          \
    }                                                                            \
    return zt_outcome_##SUFFIX##_failure(outcome.error);                         \
}                                                                                \
                                                                                 \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome) {              \
    if (outcome == NULL) return;                                                  \
    if (outcome->is_success) {                                                    \
        if (VAL_IS_PTR && (const void *)outcome->value != NULL) {                \
            zt_release((void *)outcome->value);                                  \
        }                                                                        \
    } else if (outcome->error != NULL) {                                          \
        zt_release((void *)outcome->error);                                      \
    }                                                                            \
    memset(outcome, 0, sizeof(*outcome));                                        \
}

#define ZT_DEFINE_OUTCOME_VOID_TEXT_ERROR_IMPL(SUFFIX)                          \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(void) {                       \
    zt_outcome_##SUFFIX outcome;                                                 \
    outcome.is_success = true;                                                   \
    outcome.error = NULL;                                                        \
    return outcome;                                                              \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(zt_text *error) {             \
    zt_outcome_##SUFFIX outcome;                                                 \
    if (error == NULL) {                                                          \
        zt_runtime_error(ZT_ERR_PANIC,                                           \
            "zt_outcome_" #SUFFIX "_failure requires error");                   \
    }                                                                            \
    zt_retain(error);                                                            \
    outcome.is_success = false;                                                  \
    outcome.error = error;                                                       \
    return outcome;                                                              \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(                      \
        const char *message) {                                                   \
    zt_text *error = zt_text_from_utf8_literal(zt_safe_message(message));        \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);          \
    zt_release(error);                                                           \
    return outcome;                                                              \
}                                                                                \
                                                                                 \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome) {         \
    return outcome.is_success;                                                   \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(                            \
        zt_outcome_##SUFFIX outcome) {                                           \
    if (outcome.is_success) return zt_outcome_##SUFFIX##_success();              \
    return zt_outcome_##SUFFIX##_failure(outcome.error);                         \
}                                                                                \
                                                                                 \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome) {              \
    if (outcome == NULL) return;                                                  \
    if (!outcome->is_success && outcome->error != NULL) {                        \
        zt_release(outcome->error);                                              \
    }                                                                            \
    memset(outcome, 0, sizeof(*outcome));                                        \
}

/* --------------------------------------------------------------------------
 * R2.M1 (T2.2): Outcome typedef structs.
 *
 * These macros generate the outcome struct typedefs themselves, eliminating
 * the boilerplate of hand-written `typedef struct zt_outcome_X {...}`.
 *
 * - ZT_DEFINE_OUTCOME_VALUE_STRUCT: outcome with a value field.
 * - ZT_DEFINE_OUTCOME_VOID_STRUCT: outcome without a value field.
 * -------------------------------------------------------------------------- */

#define ZT_DEFINE_OUTCOME_VALUE_STRUCT(SUFFIX, VAL_TYPE, ERR_TYPE)              \
typedef struct zt_outcome_##SUFFIX {                                             \
    zt_bool is_success;                                                          \
    VAL_TYPE value;                                                              \
    ERR_TYPE error;                                                              \
} zt_outcome_##SUFFIX;

#define ZT_DEFINE_OUTCOME_VOID_STRUCT(SUFFIX, ERR_TYPE)                         \
typedef struct zt_outcome_##SUFFIX {                                             \
    zt_bool is_success;                                                          \
    ERR_TYPE error;                                                              \
} zt_outcome_##SUFFIX;

/* --------------------------------------------------------------------------
 * R2.M1 (T2.2): outcome<V, core.Error> implementations.
 *
 * Generates impl for outcome variants whose error is `zt_core_error` (a value
 * struct holding code/message/context). Three patterns are exposed:
 *
 *   ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL(SUFFIX, VAL_TYPE, VAL_DEFAULT)
 *     For primitive values that need no retain/release (zt_int, zt_bool,
 *     zt_optional_i64, etc.). VAL_DEFAULT is the zero/empty literal used on
 *     failure (e.g. `0`, `false`, `zt_optional_i64_empty()`).
 *
 *   ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(
 *       SUFFIX, VAL_TYPE, REQUIRE_FN, ERR_NAME_LITERAL)
 *     For managed pointers (zt_text *, zt_list_* *, zt_map_* *, etc.).
 *     REQUIRE_FN is the runtime requirement helper (e.g.
 *     `zt_runtime_require_text`); ERR_NAME_LITERAL is the user-visible type
 *     name string used in panic messages.
 *
 *   ZT_DEFINE_OUTCOME_VOID_CORE_ERROR_IMPL(SUFFIX)
 *     For void-valued outcomes.
 *
 * All three accept failure(zt_core_error), failure_message(const char *) and
 * failure_text(zt_text *) constructors and follow the same dispose semantics
 * (release retained value pointer, dispose stored core error).
 * -------------------------------------------------------------------------- */

#define ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL(                            \
        SUFFIX, VAL_TYPE, VAL_DEFAULT)                                          \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(VAL_TYPE value) {             \
    zt_outcome_##SUFFIX outcome;                                                \
    outcome.is_success = true;                                                  \
    outcome.value = value;                                                      \
    outcome.error = (zt_core_error){0};                                         \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(zt_core_error error) {        \
    zt_outcome_##SUFFIX outcome;                                                \
    outcome.is_success = false;                                                 \
    outcome.value = (VAL_DEFAULT);                                              \
    outcome.error = error.message != NULL ?                                     \
        zt_core_error_clone(error) :                                            \
        zt_core_error_from_message("error", "error");                           \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message) {\
    zt_core_error error = zt_core_error_from_message("error", message);         \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_text(zt_text *message) {      \
    zt_core_error error = zt_core_error_from_text("error", message);            \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome) {         \
    return outcome.is_success;                                                  \
}                                                                                \
                                                                                 \
VAL_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome) {             \
    if (!outcome.is_success) {                                                  \
        zt_runtime_error(ZT_ERR_UNWRAP, "outcome_value on failure");            \
    }                                                                           \
    return outcome.value;                                                       \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(                            \
        zt_outcome_##SUFFIX outcome) {                                          \
    if (outcome.is_success) return outcome;                                     \
    return zt_outcome_##SUFFIX##_failure(outcome.error);                        \
}                                                                                \
                                                                                 \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome) {              \
    if (outcome == NULL) return;                                                \
    if (!outcome->is_success) zt_core_error_dispose(&outcome->error);           \
    memset(outcome, 0, sizeof(*outcome));                                       \
}

#define ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL(                                  \
        SUFFIX, VAL_TYPE, REQUIRE_FN, ERR_NAME_LITERAL)                         \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(VAL_TYPE value) {             \
    zt_outcome_##SUFFIX outcome;                                                \
    REQUIRE_FN(value, "zt_outcome_" #SUFFIX "_success requires value");         \
    memset(&outcome, 0, sizeof(outcome));                                       \
    outcome.is_success = true;                                                  \
    outcome.value = value;                                                      \
    zt_retain(value);                                                           \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(zt_core_error error) {        \
    zt_outcome_##SUFFIX outcome;                                                \
    memset(&outcome, 0, sizeof(outcome));                                       \
    outcome.is_success = false;                                                 \
    outcome.value = NULL;                                                       \
    outcome.error = error.message != NULL ?                                     \
        zt_core_error_clone(error) :                                            \
        zt_core_error_from_message("error", "error");                           \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message) {\
    zt_core_error error = zt_core_error_from_message("error", message);         \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_text(zt_text *message) {      \
    zt_core_error error = zt_core_error_from_text("error", message);            \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome) {         \
    return outcome.is_success;                                                  \
}                                                                                \
                                                                                 \
VAL_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome) {             \
    if (!outcome.is_success) {                                                  \
        zt_runtime_error(ZT_ERR_UNWRAP, "outcome_value on failure");            \
    }                                                                           \
    REQUIRE_FN(outcome.value,                                                   \
        "outcome<" ERR_NAME_LITERAL ",core.Error> success value cannot be null");\
    zt_retain(outcome.value);                                                   \
    return outcome.value;                                                       \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(                            \
        zt_outcome_##SUFFIX outcome) {                                          \
    if (outcome.is_success) return zt_outcome_##SUFFIX##_success(outcome.value);\
    return zt_outcome_##SUFFIX##_failure(outcome.error);                        \
}                                                                                \
                                                                                 \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome) {              \
    if (outcome == NULL) return;                                                \
    if (outcome->is_success) {                                                  \
        if (outcome->value != NULL) zt_release(outcome->value);                 \
    } else {                                                                    \
        zt_core_error_dispose(&outcome->error);                                 \
    }                                                                           \
    memset(outcome, 0, sizeof(*outcome));                                       \
}

#define ZT_DEFINE_OUTCOME_VOID_CORE_ERROR_IMPL(SUFFIX)                          \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(void) {                       \
    zt_outcome_##SUFFIX outcome;                                                \
    outcome.is_success = true;                                                  \
    outcome.error = (zt_core_error){0};                                         \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(zt_core_error error) {        \
    zt_outcome_##SUFFIX outcome;                                                \
    outcome.is_success = false;                                                 \
    outcome.error = error.message != NULL ?                                     \
        zt_core_error_clone(error) :                                            \
        zt_core_error_from_message("error", "error");                           \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message) {\
    zt_core_error error = zt_core_error_from_message("error", message);         \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_text(zt_text *message) {      \
    zt_core_error error = zt_core_error_from_text("error", message);            \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome) {         \
    return outcome.is_success;                                                  \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(                            \
        zt_outcome_##SUFFIX outcome) {                                          \
    if (outcome.is_success) return outcome;                                     \
    return zt_outcome_##SUFFIX##_failure(outcome.error);                        \
}                                                                                \
                                                                                 \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome) {              \
    if (outcome == NULL) return;                                                \
    if (!outcome->is_success) zt_core_error_dispose(&outcome->error);           \
    memset(outcome, 0, sizeof(*outcome));                                       \
}

#define ZT_DEFINE_OUTCOME_CORE_ERROR_OPTIONAL_PTR_IMPL(                         \
        SUFFIX, OPT_TYPE, INNER_REQUIRE_FN, ERR_NAME_LITERAL, EMPTY_FN)         \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_success(OPT_TYPE value) {             \
    zt_outcome_##SUFFIX outcome;                                                \
    outcome.is_success = true;                                                  \
    outcome.value = value;                                                      \
    outcome.error = (zt_core_error){0};                                         \
    if (value.is_present) {                                                     \
        INNER_REQUIRE_FN(value.value,                                           \
            "zt_outcome_" #SUFFIX "_success requires present value");           \
        zt_retain(value.value);                                                 \
    }                                                                           \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure(zt_core_error error) {        \
    zt_outcome_##SUFFIX outcome;                                                \
    outcome.is_success = false;                                                 \
    outcome.value = EMPTY_FN();                                                 \
    outcome.error = error.message != NULL ?                                     \
        zt_core_error_clone(error) :                                            \
        zt_core_error_from_message("error", "error");                           \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_message(const char *message) {\
    zt_core_error error = zt_core_error_from_message("error", message);         \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_failure_text(zt_text *message) {      \
    zt_core_error error = zt_core_error_from_text("error", message);            \
    zt_outcome_##SUFFIX outcome = zt_outcome_##SUFFIX##_failure(error);         \
    zt_core_error_dispose(&error);                                              \
    return outcome;                                                             \
}                                                                                \
                                                                                 \
zt_bool zt_outcome_##SUFFIX##_is_success(zt_outcome_##SUFFIX outcome) {         \
    return outcome.is_success;                                                  \
}                                                                                \
                                                                                 \
OPT_TYPE zt_outcome_##SUFFIX##_value(zt_outcome_##SUFFIX outcome) {             \
    OPT_TYPE value;                                                             \
    if (!outcome.is_success) {                                                  \
        zt_runtime_error(ZT_ERR_UNWRAP, "outcome_value on failure");            \
    }                                                                           \
    value = outcome.value;                                                      \
    if (value.is_present) {                                                     \
        INNER_REQUIRE_FN(value.value,                                           \
            "outcome<optional<" ERR_NAME_LITERAL ">,core.Error> present value cannot be null");\
        zt_retain(value.value);                                                 \
    }                                                                           \
    return value;                                                               \
}                                                                                \
                                                                                 \
zt_outcome_##SUFFIX zt_outcome_##SUFFIX##_propagate(                            \
        zt_outcome_##SUFFIX outcome) {                                          \
    if (outcome.is_success) return zt_outcome_##SUFFIX##_success(outcome.value);\
    return zt_outcome_##SUFFIX##_failure(outcome.error);                        \
}                                                                                \
                                                                                 \
void zt_outcome_##SUFFIX##_dispose(zt_outcome_##SUFFIX *outcome) {              \
    if (outcome == NULL) return;                                                \
    if (outcome->is_success) {                                                  \
        if (outcome->value.is_present && outcome->value.value != NULL) {        \
            zt_release(outcome->value.value);                                   \
        }                                                                       \
    } else {                                                                    \
        zt_core_error_dispose(&outcome->error);                                 \
    }                                                                           \
    memset(outcome, 0, sizeof(*outcome));                                       \
}

#endif /* ZENITH_RT_TEMPLATES_H */
