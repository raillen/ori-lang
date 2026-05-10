/*
 * zenith_collections_generic.c — Generic collection runtime implementation
 *
 * Tier 2, Phase 1 (M21.F1)
 */

#include "zenith_collections_generic.h"

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

/* ================================================================
 * Internal helpers
 * ================================================================ */

#define ZT_GENERIC_INITIAL_CAPACITY 8
#define ZT_MAP_LOAD_FACTOR_NUM 3
#define ZT_MAP_LOAD_FACTOR_DEN 4

static void *zt_generic_elem_at(void *base, size_t index, size_t elem_size) {
    return (char *)base + index * elem_size;
}

static const void *zt_generic_elem_at_const(const void *base, size_t index, size_t elem_size) {
    return (const char *)base + index * elem_size;
}

static void zt_generic_elem_copy(const zt_elem_ops *ops, void *dst, const void *src) {
    if (ops->copy != NULL) {
        ops->copy(dst, src);
    } else {
        memcpy(dst, src, ops->elem_size);
    }
}

static void zt_generic_elem_destroy(const zt_elem_ops *ops, void *elem) {
    if (ops->destroy != NULL) {
        ops->destroy(elem);
    }
}

/* ================================================================
 * Generic List Implementation
 * ================================================================ */

zt_list_generic *zt_list_generic_create(const zt_elem_ops *ops) {
    zt_list_generic *list;
    if (ops == NULL || ops->elem_size == 0) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_list_generic_create requires element ops");
    }
    list = (zt_list_generic *)calloc(1, sizeof(zt_list_generic));
    if (list == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate generic list");
    }
    list->header.rc = 1;
    list->header.kind = ZT_HEAP_LIST_GENERIC;
    list->ops = ops;
    return list;
}

zt_list_generic *zt_list_generic_clone(const zt_list_generic *list) {
    zt_list_generic *clone;
    size_t i;
    if (list == NULL) return NULL;

    clone = zt_list_generic_create(list->ops);
    if (clone == NULL) return NULL;

    if (list->count > 0) {
        clone->data = calloc(list->count, list->ops->elem_size);
        if (clone->data == NULL) {
            free(clone);
            zt_runtime_error(ZT_ERR_MEMORY, "failed to clone generic list");
        }
        clone->capacity = list->count;
        clone->count = list->count;

        for (i = 0; i < list->count; i++) {
            void *dst = zt_generic_elem_at(clone->data, i, list->ops->elem_size);
            const void *src = zt_generic_elem_at_const(list->data, i, list->ops->elem_size);
            zt_generic_elem_copy(list->ops, dst, src);
        }
    }

    return clone;
}

void zt_list_generic_free(zt_list_generic *list) {
    size_t i;
    if (list == NULL) return;
    for (i = 0; i < list->count; i++) {
        zt_generic_elem_destroy(list->ops, zt_generic_elem_at(list->data, i, list->ops->elem_size));
    }
    free(list->data);
    free(list);
}

static void zt_list_generic_grow(zt_list_generic *list) {
    size_t new_capacity = list->capacity == 0 ? ZT_GENERIC_INITIAL_CAPACITY : list->capacity * 2;
    void *new_data = realloc(list->data, new_capacity * list->ops->elem_size);
    if (new_data == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to grow generic list");
    }
    list->data = new_data;
    list->capacity = new_capacity;
}

zt_list_generic *zt_list_generic_from_array(const zt_elem_ops *ops, const void *items, size_t count) {
    zt_list_generic *list = zt_list_generic_create(ops);
    size_t index;

    if (count == 0) {
        return list;
    }
    if (items == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_list_generic_from_array requires items");
    }

    while (list->capacity < count) {
        zt_list_generic_grow(list);
    }

    for (index = 0; index < count; index += 1) {
        zt_generic_elem_copy(
            ops,
            zt_generic_elem_at(list->data, index, ops->elem_size),
            zt_generic_elem_at_const(items, index, ops->elem_size));
    }

    list->count = count;
    return list;
}

void zt_list_generic_append(zt_list_generic *list, const void *elem) {
    if (list == NULL || elem == NULL) return;
    if (list->count >= list->capacity) {
        zt_list_generic_grow(list);
    }
    zt_generic_elem_copy(list->ops,
        zt_generic_elem_at(list->data, list->count, list->ops->elem_size),
        elem);
    list->count++;
}

void *zt_list_generic_get(const zt_list_generic *list, zt_int index) {
    if (list == NULL || index < 0 || (size_t)index >= list->count) {
        zt_runtime_error(ZT_ERR_INDEX, "generic list index out of bounds");
    }
    return zt_generic_elem_at(list->data, (size_t)index, list->ops->elem_size);
}

void zt_list_generic_set(zt_list_generic *list, zt_int index, const void *elem) {
    void *slot;
    if (list == NULL || index < 0 || (size_t)index >= list->count) {
        zt_runtime_error(ZT_ERR_INDEX, "generic list index out of bounds");
    }
    slot = zt_generic_elem_at(list->data, (size_t)index, list->ops->elem_size);
    zt_generic_elem_destroy(list->ops, slot);
    zt_generic_elem_copy(list->ops, slot, elem);
}

zt_list_generic *zt_list_generic_set_owned(zt_list_generic *list, zt_int index, const void *elem) {
    if (list == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_list_generic_set_owned requires list");
    }

    if (list->header.rc > 1u) {
        list = zt_list_generic_clone(list);
    } else {
        zt_retain(list);
    }

    zt_list_generic_set(list, index, elem);
    return list;
}

void zt_list_generic_remove(zt_list_generic *list, zt_int index) {
    void *slot;
    size_t idx;
    if (list == NULL || index < 0 || (size_t)index >= list->count) {
        zt_runtime_error(ZT_ERR_INDEX, "generic list index out of bounds");
    }
    idx = (size_t)index;
    slot = zt_generic_elem_at(list->data, idx, list->ops->elem_size);
    zt_generic_elem_destroy(list->ops, slot);

    /* Shift remaining elements */
    if (idx + 1 < list->count) {
        memmove(slot,
                zt_generic_elem_at(list->data, idx + 1, list->ops->elem_size),
                (list->count - idx - 1) * list->ops->elem_size);
    }
    list->count--;
}

void zt_list_generic_insert(zt_list_generic *list, zt_int index, const void *elem) {
    size_t idx;
    void *slot;
    if (list == NULL || elem == NULL) return;
    if (index < 0 || (size_t)index > list->count) {
        zt_runtime_error(ZT_ERR_INDEX, "generic list insert index out of bounds");
    }
    idx = (size_t)index;
    if (list->count >= list->capacity) {
        zt_list_generic_grow(list);
    }
    /* Shift elements right */
    if (idx < list->count) {
        memmove(zt_generic_elem_at(list->data, idx + 1, list->ops->elem_size),
                zt_generic_elem_at(list->data, idx, list->ops->elem_size),
                (list->count - idx) * list->ops->elem_size);
    }
    slot = zt_generic_elem_at(list->data, idx, list->ops->elem_size);
    zt_generic_elem_copy(list->ops, slot, elem);
    list->count++;
}

zt_list_generic *zt_list_generic_slice(const zt_list_generic *list, zt_int start, zt_int end) {
    zt_list_generic *sliced;
    size_t i, s, e, count;
    void *src_slot;
    void *dst_slot;

    if (list == NULL) return NULL;

    if (start < 0 || (size_t)start > list->count || end < 0 || (size_t)end > list->count || start > end) {
        zt_runtime_error(ZT_ERR_INDEX, "generic list slice indices out of bounds");
    }

    s = (size_t)start;
    e = (size_t)end;
    count = e - s;

    sliced = zt_list_generic_create(list->ops);
    if (count > 0) {
        if (sliced->capacity < count) {
            sliced->capacity = count;
            sliced->data = realloc(sliced->data, sliced->capacity * sliced->ops->elem_size);
            if (sliced->data == NULL) {
                zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate generic list slice");
            }
        }

        for (i = 0; i < count; i++) {
            src_slot = zt_generic_elem_at(list->data, s + i, list->ops->elem_size);
            dst_slot = zt_generic_elem_at(sliced->data, i, sliced->ops->elem_size);
            zt_generic_elem_copy(list->ops, dst_slot, src_slot);
        }
        sliced->count = count;
    }

    return sliced;
}

zt_int zt_list_generic_len(const zt_list_generic *list) {
    return list != NULL ? (zt_int)list->count : 0;
}

void zt_list_generic_clear(zt_list_generic *list) {
    size_t i;
    if (list == NULL) return;
    for (i = 0; i < list->count; i++) {
        zt_generic_elem_destroy(list->ops, zt_generic_elem_at(list->data, i, list->ops->elem_size));
    }
    list->count = 0;
}

void *zt_list_generic_raw_get(const zt_list_generic *list, size_t index) {
    return zt_generic_elem_at(list->data, index, list->ops->elem_size);
}

/* ================================================================
 * Generic Map Implementation
 * ================================================================ */

static size_t zt_map_generic_probe(const zt_map_generic *map, const void *key, uint64_t hash) {
    size_t idx = (size_t)(hash % (uint64_t)map->capacity);
    size_t first_tombstone = SIZE_MAX;
    size_t i;
    for (i = 0; i < map->capacity; i++) {
        size_t probe = (idx + i) % map->capacity;
        if (!map->meta[probe].occupied && !map->meta[probe].tombstone) {
            return first_tombstone != SIZE_MAX ? first_tombstone : probe;
        }
        if (!map->meta[probe].occupied && map->meta[probe].tombstone && first_tombstone == SIZE_MAX) {
            first_tombstone = probe;
            continue;
        }
        if (map->meta[probe].occupied &&
            map->meta[probe].hash == hash &&
            map->key_ops->equals(
                zt_generic_elem_at_const(map->keys, probe, map->key_ops->elem_size),
                key)) {
            return probe; /* found key */
        }
    }
    return first_tombstone != SIZE_MAX ? first_tombstone : map->capacity;
}

static void zt_map_generic_rehash(zt_map_generic *map) {
    size_t old_capacity = map->capacity;
    zt_map_generic_entry *old_meta = map->meta;
    void *old_keys = map->keys;
    void *old_values = map->values;
    size_t new_capacity = old_capacity == 0 ? ZT_GENERIC_INITIAL_CAPACITY : old_capacity * 2;
    size_t i;

    map->meta = (zt_map_generic_entry *)calloc(new_capacity, sizeof(zt_map_generic_entry));
    map->keys = calloc(new_capacity, map->key_ops->elem_size);
    map->values = calloc(new_capacity, map->val_ops->elem_size);
    map->capacity = new_capacity;
    map->count = 0;

    if (map->meta == NULL || map->keys == NULL || map->values == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate generic map");
    }

    for (i = 0; i < old_capacity; i++) {
        if (old_meta[i].occupied) {
            zt_map_generic_put(map,
                zt_generic_elem_at_const(old_keys, i, map->key_ops->elem_size),
                zt_generic_elem_at_const(old_values, i, map->val_ops->elem_size));
            zt_generic_elem_destroy(map->key_ops, zt_generic_elem_at(old_keys, i, map->key_ops->elem_size));
            zt_generic_elem_destroy(map->val_ops, zt_generic_elem_at(old_values, i, map->val_ops->elem_size));
        }
    }

    free(old_meta);
    free(old_keys);
    free(old_values);
}

zt_map_generic *zt_map_generic_create(const zt_elem_ops *key_ops, const zt_elem_ops *val_ops) {
    zt_map_generic *map;
    if (key_ops == NULL || val_ops == NULL ||
            key_ops->elem_size == 0 || val_ops->elem_size == 0 ||
            key_ops->hash == NULL || key_ops->equals == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_map_generic_create requires key/value ops");
    }
    map = (zt_map_generic *)malloc(sizeof(zt_map_generic));
    if (map == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate generic map");
    }
    map->header.rc = 1;
    map->header.kind = ZT_HEAP_MAP_GENERIC;
    map->meta = NULL;
    map->keys = NULL;
    map->values = NULL;
    map->count = 0;
    map->capacity = 0;
    map->key_ops = key_ops;
    map->val_ops = val_ops;
    return map;
}

zt_map_generic *zt_map_generic_clone(const zt_map_generic *map) {
    zt_map_generic *clone;
    size_t i;
    if (map == NULL) return NULL;

    clone = zt_map_generic_create(map->key_ops, map->val_ops);
    if (clone == NULL) return NULL;

    for (i = 0; i < map->capacity; i++) {
        if (map->meta[i].occupied) {
            zt_map_generic_put(clone,
                zt_generic_elem_at_const(map->keys, i, map->key_ops->elem_size),
                zt_generic_elem_at_const(map->values, i, map->val_ops->elem_size));
        }
    }

    return clone;
}

void zt_map_generic_free(zt_map_generic *map) {
    size_t i;
    if (map == NULL) return;
    for (i = 0; i < map->capacity; i++) {
        if (map->meta[i].occupied) {
            zt_generic_elem_destroy(map->key_ops, zt_generic_elem_at(map->keys, i, map->key_ops->elem_size));
            zt_generic_elem_destroy(map->val_ops, zt_generic_elem_at(map->values, i, map->val_ops->elem_size));
        }
    }
    free(map->meta);
    free(map->keys);
    free(map->values);
    free(map);
}

void zt_map_generic_put(zt_map_generic *map, const void *key, const void *value) {
    size_t idx;
    uint64_t hash;

    if (map == NULL || key == NULL) return;

    /* Rehash if needed */
    if (map->capacity == 0 || map->count * ZT_MAP_LOAD_FACTOR_DEN >= map->capacity * ZT_MAP_LOAD_FACTOR_NUM) {
        zt_map_generic_rehash(map);
    }

    hash = map->key_ops->hash(key);
    idx = zt_map_generic_probe(map, key, hash);

    if (idx >= map->capacity) {
        zt_runtime_error(ZT_ERR_PLATFORM, "generic map probe failed");
    }

    if (map->meta[idx].occupied) {
        /* Update existing value */
        void *val_slot = zt_generic_elem_at(map->values, idx, map->val_ops->elem_size);
        zt_generic_elem_destroy(map->val_ops, val_slot);
        zt_generic_elem_copy(map->val_ops, val_slot, value);
    } else {
        /* Insert new entry */
        zt_generic_elem_copy(map->key_ops,
            zt_generic_elem_at(map->keys, idx, map->key_ops->elem_size), key);
        zt_generic_elem_copy(map->val_ops,
            zt_generic_elem_at(map->values, idx, map->val_ops->elem_size), value);
        map->meta[idx].occupied = 1;
        map->meta[idx].tombstone = 0;
        map->meta[idx].hash = hash;
        map->count++;
    }
}

void *zt_map_generic_get(const zt_map_generic *map, const void *key) {
    uint64_t hash;
    size_t idx;
    if (map == NULL || key == NULL || map->capacity == 0) return NULL;

    hash = map->key_ops->hash(key);
    idx = zt_map_generic_probe(map, key, hash);

    if (idx < map->capacity && map->meta[idx].occupied) {
        return zt_generic_elem_at(map->values, idx, map->val_ops->elem_size);
    }
    return NULL;
}

int zt_map_generic_has(const zt_map_generic *map, const void *key) {
    uint64_t hash;
    size_t idx;
    if (map == NULL || key == NULL || map->capacity == 0) return 0;

    hash = map->key_ops->hash(key);
    idx = zt_map_generic_probe(map, key, hash);

    return idx < map->capacity && map->meta[idx].occupied;
}

void zt_map_generic_remove(zt_map_generic *map, const void *key) {
    uint64_t hash;
    size_t idx;
    if (map == NULL || key == NULL || map->capacity == 0) return;

    hash = map->key_ops->hash(key);
    idx = zt_map_generic_probe(map, key, hash);

    if (idx < map->capacity && map->meta[idx].occupied) {
        zt_generic_elem_destroy(map->key_ops, zt_generic_elem_at(map->keys, idx, map->key_ops->elem_size));
        zt_generic_elem_destroy(map->val_ops, zt_generic_elem_at(map->values, idx, map->val_ops->elem_size));
        map->meta[idx].occupied = 0;
        map->meta[idx].tombstone = 1;
        map->meta[idx].hash = 0;
        map->count--;
    }
}

zt_int zt_map_generic_len(const zt_map_generic *map) {
    return map != NULL ? (zt_int)map->count : 0;
}

void zt_map_generic_clear(zt_map_generic *map) {
    size_t i;
    if (map == NULL) return;
    for (i = 0; i < map->capacity; i++) {
        if (map->meta[i].occupied) {
            zt_generic_elem_destroy(map->key_ops, zt_generic_elem_at(map->keys, i, map->key_ops->elem_size));
            zt_generic_elem_destroy(map->val_ops, zt_generic_elem_at(map->values, i, map->val_ops->elem_size));
            map->meta[i].occupied = 0;
            map->meta[i].tombstone = 0;
            map->meta[i].hash = 0;
        }
    }
    map->count = 0;
}

/* ================================================================
 * Generic Set Implementation
 * ================================================================ */

static size_t zt_set_generic_probe(const zt_set_generic *set, const void *elem, uint64_t hash) {
    size_t idx = (size_t)(hash % (uint64_t)set->capacity);
    size_t first_tombstone = SIZE_MAX;
    size_t i;
    for (i = 0; i < set->capacity; i++) {
        size_t probe = (idx + i) % set->capacity;
        if (!set->meta[probe].occupied && !set->meta[probe].tombstone) {
            return first_tombstone != SIZE_MAX ? first_tombstone : probe;
        }
        if (!set->meta[probe].occupied && set->meta[probe].tombstone && first_tombstone == SIZE_MAX) {
            first_tombstone = probe;
            continue;
        }
        if (set->meta[probe].occupied &&
            set->meta[probe].hash == hash &&
            set->ops->equals(
                zt_generic_elem_at_const(set->elements, probe, set->ops->elem_size),
                elem)) {
            return probe;
        }
    }
    return first_tombstone != SIZE_MAX ? first_tombstone : set->capacity;
}

static void zt_set_generic_rehash(zt_set_generic *set) {
    size_t old_capacity = set->capacity;
    zt_set_generic_entry *old_meta = set->meta;
    void *old_elements = set->elements;
    size_t new_capacity = old_capacity == 0 ? ZT_GENERIC_INITIAL_CAPACITY : old_capacity * 2;
    size_t i;

    set->meta = (zt_set_generic_entry *)calloc(new_capacity, sizeof(zt_set_generic_entry));
    set->elements = calloc(new_capacity, set->ops->elem_size);
    set->capacity = new_capacity;
    set->count = 0;

    if (set->meta == NULL || set->elements == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate generic set");
    }

    for (i = 0; i < old_capacity; i++) {
        if (old_meta[i].occupied) {
            zt_set_generic_add(set, zt_generic_elem_at_const(old_elements, i, set->ops->elem_size));
            zt_generic_elem_destroy(set->ops, zt_generic_elem_at(old_elements, i, set->ops->elem_size));
        }
    }

    free(old_meta);
    free(old_elements);
}

zt_set_generic *zt_set_generic_create(const zt_elem_ops *ops) {
    zt_set_generic *set;
    if (ops == NULL || ops->elem_size == 0 || ops->hash == NULL || ops->equals == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, "zt_set_generic_create requires element ops");
    }
    set = (zt_set_generic *)malloc(sizeof(zt_set_generic));
    if (set == NULL) {
        zt_runtime_error(ZT_ERR_MEMORY, "failed to allocate generic set");
    }
    set->header.rc = 1;
    set->header.kind = ZT_HEAP_SET_GENERIC;
    set->meta = NULL;
    set->elements = NULL;
    set->count = 0;
    set->capacity = 0;
    set->ops = ops;
    return set;
}

zt_set_generic *zt_set_generic_clone(const zt_set_generic *set) {
    zt_set_generic *clone;
    size_t i;
    if (set == NULL) return NULL;

    clone = zt_set_generic_create(set->ops);
    if (clone == NULL) return NULL;

    for (i = 0; i < set->capacity; i++) {
        if (set->meta[i].occupied) {
            zt_set_generic_add(clone, zt_generic_elem_at_const(set->elements, i, set->ops->elem_size));
        }
    }

    return clone;
}

void zt_set_generic_free(zt_set_generic *set) {
    size_t i;
    if (set == NULL) return;
    for (i = 0; i < set->capacity; i++) {
        if (set->meta[i].occupied) {
            zt_generic_elem_destroy(set->ops, zt_generic_elem_at(set->elements, i, set->ops->elem_size));
        }
    }
    free(set->meta);
    free(set->elements);
    free(set);
}

void zt_set_generic_add(zt_set_generic *set, const void *elem) {
    uint64_t hash;
    size_t idx;

    if (set == NULL || elem == NULL) return;

    if (set->capacity == 0 || set->count * ZT_MAP_LOAD_FACTOR_DEN >= set->capacity * ZT_MAP_LOAD_FACTOR_NUM) {
        zt_set_generic_rehash(set);
    }

    hash = set->ops->hash(elem);
    idx = zt_set_generic_probe(set, elem, hash);

    if (idx >= set->capacity) {
        zt_runtime_error(ZT_ERR_PLATFORM, "generic set probe failed");
    }

    if (!set->meta[idx].occupied) {
        zt_generic_elem_copy(set->ops,
            zt_generic_elem_at(set->elements, idx, set->ops->elem_size), elem);
        set->meta[idx].occupied = 1;
        set->meta[idx].tombstone = 0;
        set->meta[idx].hash = hash;
        set->count++;
    }
    /* If already present, do nothing (set semantics) */
}

int zt_set_generic_has(const zt_set_generic *set, const void *elem) {
    uint64_t hash;
    size_t idx;
    if (set == NULL || elem == NULL || set->capacity == 0) return 0;

    hash = set->ops->hash(elem);
    idx = zt_set_generic_probe(set, elem, hash);

    return idx < set->capacity && set->meta[idx].occupied;
}

void zt_set_generic_remove(zt_set_generic *set, const void *elem) {
    uint64_t hash;
    size_t idx;
    if (set == NULL || elem == NULL || set->capacity == 0) return;

    hash = set->ops->hash(elem);
    idx = zt_set_generic_probe(set, elem, hash);

    if (idx < set->capacity && set->meta[idx].occupied) {
        zt_generic_elem_destroy(set->ops, zt_generic_elem_at(set->elements, idx, set->ops->elem_size));
        set->meta[idx].occupied = 0;
        set->meta[idx].tombstone = 1;
        set->meta[idx].hash = 0;
        set->count--;
    }
}

zt_int zt_set_generic_len(const zt_set_generic *set) {
    return set != NULL ? (zt_int)set->count : 0;
}

void zt_set_generic_clear(zt_set_generic *set) {
    size_t i;
    if (set == NULL) return;
    for (i = 0; i < set->capacity; i++) {
        if (set->meta[i].occupied) {
            zt_generic_elem_destroy(set->ops, zt_generic_elem_at(set->elements, i, set->ops->elem_size));
            set->meta[i].occupied = 0;
            set->meta[i].tombstone = 0;
            set->meta[i].hash = 0;
        }
    }
    set->count = 0;
}

/* ================================================================
 * Built-in elem_ops for primitive types
 * ================================================================ */

static void zt_ops_i64_copy(void *dst, const void *src) { *(int64_t *)dst = *(const int64_t *)src; }
static uint64_t zt_ops_i64_hash(const void *elem) {
    uint64_t v = (uint64_t)(*(const int64_t *)elem);
    v = (~v) + (v << 21);
    v = v ^ (v >> 24);
    v = (v + (v << 3)) + (v << 8);
    v = v ^ (v >> 14);
    v = (v + (v << 2)) + (v << 4);
    v = v ^ (v >> 28);
    v = v + (v << 31);
    return v;
}
static int zt_ops_i64_equals(const void *a, const void *b) { return *(const int64_t *)a == *(const int64_t *)b; }

const zt_elem_ops zt_elem_ops_i64 = {
    sizeof(int64_t), zt_ops_i64_copy, NULL, zt_ops_i64_hash, zt_ops_i64_equals, NULL
};

static void zt_ops_f64_copy(void *dst, const void *src) { *(double *)dst = *(const double *)src; }
static uint64_t zt_ops_f64_hash(const void *elem) {
    double value = *(const double *)elem;
    uint64_t bits;
    if (value == 0.0) {
        bits = 0;
    } else {
        memcpy(&bits, &value, sizeof(bits));
    }
    bits ^= bits >> 33;
    bits *= UINT64_C(0xff51afd7ed558ccd);
    bits ^= bits >> 33;
    bits *= UINT64_C(0xc4ceb9fe1a85ec53);
    bits ^= bits >> 33;
    return bits;
}
static int zt_ops_f64_equals(const void *a, const void *b) {
    return *(const double *)a == *(const double *)b;
}

const zt_elem_ops zt_elem_ops_f64 = {
    sizeof(double), zt_ops_f64_copy, NULL, zt_ops_f64_hash, zt_ops_f64_equals, NULL
};

static void zt_ops_bool_copy(void *dst, const void *src) { *(zt_bool *)dst = *(const zt_bool *)src; }
static uint64_t zt_ops_bool_hash(const void *elem) { return *(const zt_bool *)elem ? UINT64_C(0x9e3779b97f4a7c15) : 0; }
static int zt_ops_bool_equals(const void *a, const void *b) { return *(const zt_bool *)a == *(const zt_bool *)b; }

const zt_elem_ops zt_elem_ops_bool = {
    sizeof(zt_bool), zt_ops_bool_copy, NULL, zt_ops_bool_hash, zt_ops_bool_equals, NULL
};

static void zt_ops_text_copy(void *dst, const void *src) {
    zt_text *s = *(zt_text *const *)src;
    if (s != NULL) zt_retain(s);
    *(zt_text **)dst = s;
}
static void zt_ops_text_destroy(void *elem) {
    zt_text *s = *(zt_text **)elem;
    if (s != NULL) zt_release(s);
}
static uint64_t zt_ops_text_hash(const void *elem) {
    const zt_text *s = *(const zt_text *const *)elem;
    return s != NULL ? zt_text_hash(s) : 0;
}
static int zt_ops_text_equals(const void *a, const void *b) {
    const zt_text *sa = *(const zt_text *const *)a;
    const zt_text *sb = *(const zt_text *const *)b;
    return zt_text_eq(sa, sb);
}

const zt_elem_ops zt_elem_ops_text = {
    sizeof(zt_text *), zt_ops_text_copy, zt_ops_text_destroy, zt_ops_text_hash, zt_ops_text_equals, NULL
};
