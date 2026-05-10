/*
 * zenith_collections_generic.h — Generic collection runtime for Zenith v1.0
 *
 * Provides void*-based generic collections (list, map, set) that work with
 * any element type via callback-based lifecycle management (zt_elem_ops).
 *
 * These are used for list<UserStruct>, map<K,V> where K or V is a user type,
 * and set<UserStruct>. Primitive specializations (zt_list_i64, etc.) remain
 * for hot-path performance.
 *
 * Architecture: Tier 2, Phase 1 (M21.F1)
 */

#ifndef ZENITH_COLLECTIONS_GENERIC_H
#define ZENITH_COLLECTIONS_GENERIC_H

#include "zenith_rt.h"
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ================================================================
 * Element Operations (zt_elem_ops)
 *
 * One static const instance per concrete element type. The emitter
 * generates these per monomorphized type.
 * ================================================================ */

typedef struct zt_elem_ops {
    size_t elem_size;                               /* sizeof(T) */
    void (*copy)(void *dst, const void *src);       /* deep copy element */
    void (*destroy)(void *elem);                    /* release/free element */
    uint64_t (*hash)(const void *elem);             /* hash for map keys / set elements; NULL if unused */
    int (*equals)(const void *a, const void *b);    /* equality for map keys / set elements; NULL if unused */
    zt_text *(*to_text)(const void *elem);          /* TextRepresentable; NULL if unused */
} zt_elem_ops;

/* ================================================================
 * Generic List
 *
 * Dynamic array of elements stored inline (value semantics).
 * Elements are contiguous: data[i] is at (char*)data + i * ops->elem_size.
 * ================================================================ */

typedef struct zt_list_generic {
    zt_header header;
    void *data;                     /* raw byte array */
    size_t count;
    size_t capacity;
    const zt_elem_ops *ops;
} zt_list_generic;

zt_list_generic *zt_list_generic_create(const zt_elem_ops *ops);
zt_list_generic *zt_list_generic_from_array(const zt_elem_ops *ops, const void *items, size_t count);
zt_list_generic *zt_list_generic_clone(const zt_list_generic *list);
void zt_list_generic_free(zt_list_generic *list);

void  zt_list_generic_append(zt_list_generic *list, const void *elem);
void *zt_list_generic_get(const zt_list_generic *list, zt_int index);
void  zt_list_generic_set(zt_list_generic *list, zt_int index, const void *elem);
zt_list_generic *zt_list_generic_set_owned(zt_list_generic *list, zt_int index, const void *elem);
void  zt_list_generic_remove(zt_list_generic *list, zt_int index);
void  zt_list_generic_insert(zt_list_generic *list, zt_int index, const void *elem);
zt_list_generic *zt_list_generic_slice(const zt_list_generic *list, zt_int start, zt_int end);
zt_int zt_list_generic_len(const zt_list_generic *list);
void  zt_list_generic_clear(zt_list_generic *list);

/* Iteration helper: returns pointer to element at index (no bounds check) */
void *zt_list_generic_raw_get(const zt_list_generic *list, size_t index);

/* ================================================================
 * Generic Map
 *
 * Open-addressing hash map with tombstone reuse.
 * Keys and values stored inline in parallel arrays.
 * ================================================================ */

typedef struct zt_map_generic_entry {
    uint8_t occupied;
    uint8_t tombstone;
    uint64_t hash;
} zt_map_generic_entry;

typedef struct zt_map_generic {
    zt_header header;
    zt_map_generic_entry *meta;     /* metadata array */
    void *keys;                     /* raw key array: key_ops->elem_size * capacity */
    void *values;                   /* raw value array: val_ops->elem_size * capacity */
    size_t count;
    size_t capacity;
    const zt_elem_ops *key_ops;
    const zt_elem_ops *val_ops;
} zt_map_generic;

zt_map_generic *zt_map_generic_create(const zt_elem_ops *key_ops, const zt_elem_ops *val_ops);
zt_map_generic *zt_map_generic_clone(const zt_map_generic *map);
void zt_map_generic_free(zt_map_generic *map);

void  zt_map_generic_put(zt_map_generic *map, const void *key, const void *value);
void *zt_map_generic_get(const zt_map_generic *map, const void *key);
int   zt_map_generic_has(const zt_map_generic *map, const void *key);
void  zt_map_generic_remove(zt_map_generic *map, const void *key);
zt_int zt_map_generic_len(const zt_map_generic *map);
void  zt_map_generic_clear(zt_map_generic *map);

/* ================================================================
 * Generic Set
 *
 * Hash set using open addressing.
 * Elements stored inline.
 * ================================================================ */

typedef struct zt_set_generic_entry {
    uint8_t occupied;
    uint8_t tombstone;
    uint64_t hash;
} zt_set_generic_entry;

typedef struct zt_set_generic {
    zt_header header;
    zt_set_generic_entry *meta;
    void *elements;                 /* raw element array */
    size_t count;
    size_t capacity;
    const zt_elem_ops *ops;
} zt_set_generic;

zt_set_generic *zt_set_generic_create(const zt_elem_ops *ops);
zt_set_generic *zt_set_generic_clone(const zt_set_generic *set);
void zt_set_generic_free(zt_set_generic *set);

void  zt_set_generic_add(zt_set_generic *set, const void *elem);
int   zt_set_generic_has(const zt_set_generic *set, const void *elem);
void  zt_set_generic_remove(zt_set_generic *set, const void *elem);
zt_int zt_set_generic_len(const zt_set_generic *set);
void  zt_set_generic_clear(zt_set_generic *set);

/* ================================================================
 * Built-in elem_ops for primitive types
 *
 * The emitter can reference these directly for primitive generics
 * like list<int> when falling through to the generic path.
 * ================================================================ */

extern const zt_elem_ops zt_elem_ops_i64;
extern const zt_elem_ops zt_elem_ops_f64;
extern const zt_elem_ops zt_elem_ops_bool;
extern const zt_elem_ops zt_elem_ops_text;

#ifdef __cplusplus
}
#endif

#endif /* ZENITH_COLLECTIONS_GENERIC_H */
