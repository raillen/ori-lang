/*
 * R2.M1 (T2.9): Specialized collections runtime impl.
 *
 * This file is part of the Zenith unity-build runtime; it is `#include`d into
 * `runtime/c/zenith_rt.c` after `zt_optional_*` and the `ZT_DEFINE_*_IMPL`
 * macros from `zenith_rt_templates.h` are visible. Including via the unity
 * pattern keeps a single translation unit while letting us split logical
 * groups across files for navigation.
 *
 * Provides the monomorphized implementations for:
 *   - grid2d<int>, grid2d<text>
 *   - pqueue<int>, pqueue<text>
 *   - circbuf<int>, circbuf<text>
 *   - btreemap<text,text>, btreeset<text>
 *   - grid3d<int>, grid3d<text>
 *
 * Each macro expands to the full set of constructor / accessor / mutator /
 * disposal functions defined by the template macros in
 * `runtime/c/zenith_rt_templates.h`.
 */

ZT_DEFINE_GRID2D_IMPL(i64, zt_int, ZT_HEAP_GRID2D_I64, 0, "int", 0)
ZT_DEFINE_GRID2D_IMPL(text, zt_text *, ZT_HEAP_GRID2D_TEXT, 1, "text", zt_text_from_utf8_literal(""))

ZT_DEFINE_PQUEUE_IMPL(i64, zt_int, ZT_HEAP_PQUEUE_I64, zt_optional_i64, zt_optional_i64_present, zt_optional_i64_empty, 0, "int", (_lhs < _rhs))
ZT_DEFINE_PQUEUE_IMPL(text, zt_text *, ZT_HEAP_PQUEUE_TEXT, zt_optional_text, zt_optional_text_present, zt_optional_text_empty, 1, "text", (_lhs != NULL && _rhs != NULL && strcmp(_lhs->data, _rhs->data) < 0))

ZT_DEFINE_CIRCBUF_IMPL(i64, zt_int, ZT_HEAP_CIRCBUF_I64, zt_optional_i64, zt_optional_i64_present, zt_optional_i64_empty, 0, "int")
ZT_DEFINE_CIRCBUF_IMPL(text, zt_text *, ZT_HEAP_CIRCBUF_TEXT, zt_optional_text, zt_optional_text_present, zt_optional_text_empty, 1, "text")

ZT_DEFINE_BTREEMAP_IMPL(text_text, zt_text *, zt_text *, ZT_HEAP_BTREEMAP_TEXT_TEXT, zt_optional_text_present, zt_optional_text_empty, 1, 1, "text,text", strcmp(_lhs->data, key->data), NULL)
ZT_DEFINE_BTREESET_IMPL(text, zt_text *, ZT_HEAP_BTREESET_TEXT, 1, "text", strcmp(_lhs->data, value->data))

zt_list_text *zt_btreemap_text_text_keys(const zt_btreemap_text_text *map) {
    zt_runtime_require_btreemap_text_text(
        map,
        "zt_btreemap_text_text_keys requires map");
    return zt_list_text_from_array((zt_text *const *)map->keys, map->len);
}

zt_list_text *zt_btreemap_text_text_values(const zt_btreemap_text_text *map) {
    zt_runtime_require_btreemap_text_text(
        map,
        "zt_btreemap_text_text_values requires map");
    return zt_list_text_from_array((zt_text *const *)map->values, map->len);
}

zt_list_text *zt_btreeset_text_values(const zt_btreeset_text *set) {
    zt_runtime_require_btreeset_text(
        set,
        "zt_btreeset_text_values requires set");
    return zt_list_text_from_array((zt_text *const *)set->data, set->len);
}

ZT_DEFINE_GRID3D_IMPL(i64, zt_int, ZT_HEAP_GRID3D_I64, 0, "int", 0)
ZT_DEFINE_GRID3D_IMPL(text, zt_text *, ZT_HEAP_GRID3D_TEXT, 1, "text", zt_text_from_utf8_literal(""))
