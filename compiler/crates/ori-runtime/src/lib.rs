// ori-runtime implementation

use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::raw::{c_char, c_uchar};

// ── Non-atomic ARC ──────────────────────────────────────────────────────────
//
// Every heap-allocated managed object starts with an 8-byte ArcHeader:
//   [ref_count: u32][type_tag: u32][... payload ...]
//                                  ^── ptr passed to retain/release
//
// Type tags identify the object kind so the correct destructor is called.

use std::sync::atomic::{AtomicI64, Ordering};

#[repr(C)]
pub struct OriHeapHeader {
    pub refcount: AtomicI64,
    pub destructor: Option<unsafe extern "C" fn(*mut u8)>,
}

#[no_mangle]
pub unsafe extern "C" fn ori_alloc(size: usize, destructor: Option<unsafe extern "C" fn(*mut u8)>) -> *mut u8 {
    let total = size + std::mem::size_of::<OriHeapHeader>();
    let ptr = libc::malloc(total) as *mut u8;
    if !ptr.is_null() {
        let header = ptr as *mut OriHeapHeader;
        std::ptr::write(&mut (*header).refcount, AtomicI64::new(1));
        (*header).destructor = destructor;
        ptr.add(std::mem::size_of::<OriHeapHeader>())
    } else {
        ptr
    }
}

/// Increment the reference count of a managed object.
/// Silently ignores null pointers and non-managed values (e.g. static strings).
#[no_mangle]
pub unsafe extern "C" fn ori_arc_retain(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    let header = ptr.sub(std::mem::size_of::<OriHeapHeader>()) as *mut OriHeapHeader;
    (*header).refcount.fetch_add(1, Ordering::Relaxed);
}

/// Decrement the reference count. When it reaches zero, the object is freed.
#[no_mangle]
pub unsafe extern "C" fn ori_arc_release(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    let header = ptr.sub(std::mem::size_of::<OriHeapHeader>()) as *mut OriHeapHeader;
    if (*header).refcount.fetch_sub(1, Ordering::Release) == 1 {
        (*header).refcount.load(Ordering::Acquire); // synchronize
        if let Some(dtor) = (*header).destructor {
            dtor(ptr);
        }
        std::ptr::drop_in_place(&mut (*header).refcount);
        libc::free(header as *mut libc::c_void);
    }
}

/// Cycle collection stub — non-atomic single-threaded ARC does not detect cycles.
/// Returns number of objects collected (always 0 for now).
#[no_mangle]
pub unsafe extern "C" fn ori_arc_collect_cycles() -> i64 {
    0
}

// ── ori.io ────────────────────────────────────────────────────────────────────

/// Print `len` bytes from `ptr` to stdout, followed by a newline.
#[no_mangle]
pub unsafe extern "C" fn ori_io_print(ptr: *const u8, len: i64) {
    if ptr.is_null() || len <= 0 {
        println!();
        return;
    }
    let data = std::slice::from_raw_parts(ptr, len as usize);
    let _ = std::io::stdout().write_all(data);
    let _ = std::io::stdout().write_all(b"\n");
    let _ = std::io::stdout().flush();
}

/// Print `len` bytes from `ptr` to stderr, followed by a newline.
#[no_mangle]
pub unsafe extern "C" fn ori_io_eprint(ptr: *const u8, len: i64) {
    if ptr.is_null() || len <= 0 {
        eprintln!();
        return;
    }
    let data = std::slice::from_raw_parts(ptr, len as usize);
    let _ = std::io::stderr().write_all(data);
    let _ = std::io::stderr().write_all(b"\n");
    let _ = std::io::stderr().flush();
}

#[no_mangle]
pub unsafe extern "C" fn ori_io_read_line() -> *mut u8 {
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err() {
        return cstring_from_str("");
    }
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    cstring_from_str(&line)
}

// ── ori.string ────────────────────────────────────────────────────────────────

/// Convert an i64 to a null-terminated C string allocated with malloc.
/// Caller is responsible for freeing the result.
#[no_mangle]
pub unsafe extern "C" fn ori_int_to_cstr(n: i64) -> *mut u8 {
    let mut ptr = std::ptr::null_mut();
    let mut len = 0;
    ori_to_string_parts(n, &mut ptr, &mut len);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn ori_to_string(n: i64) -> *mut u8 {
    ori_int_to_cstr(n)
}

#[no_mangle]
pub unsafe extern "C" fn ori_to_string_parts(n: i64, out_ptr: *mut *mut u8, out_len: *mut i64) {
    let body = n.to_string();
    let ptr = cstring_from_str(&body);
    if !out_ptr.is_null() {
        *out_ptr = ptr;
    }
    if !out_len.is_null() {
        *out_len = body.len() as i64;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_len(ptr: *const u8) -> i64 {
    if ptr.is_null() {
        return 0;
    }
    CStr::from_ptr(ptr as *const c_char).to_bytes().len() as i64
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_len(ptr: *const u8) -> i64 {
    ori_len(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_concat(a: *const u8, b: *const u8) -> *mut u8 {
    ori_string_concat_parts(a, ori_len(a), b, ori_len(b))
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_concat_parts(
    a: *const u8,
    a_len: i64,
    b: *const u8,
    b_len: i64,
) -> *mut u8 {
    let a = bounded_cstr_bytes(a, a_len);
    let b = bounded_cstr_bytes(b, b_len);
    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend_from_slice(a);
    out.extend_from_slice(b);
    cstring_from_bytes(out)
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_slice(s: *const u8, start: i64, end: i64) -> *mut u8 {
    let s = cstr_bytes(s);
    let start = start.max(0).min(s.len() as i64) as usize;
    let end = end.max(start as i64).min(s.len() as i64) as usize;
    cstring_from_bytes(s[start..end].to_vec())
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_contains(s: *const u8, sub: *const u8) -> c_uchar {
    let s = cstr_str(s);
    let sub = cstr_str(sub);
    u8::from(s.contains(sub)) as c_uchar
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_starts_with(s: *const u8, prefix: *const u8) -> c_uchar {
    let s = cstr_str(s);
    let prefix = cstr_str(prefix);
    u8::from(s.starts_with(prefix)) as c_uchar
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_ends_with(s: *const u8, suffix: *const u8) -> c_uchar {
    let s = cstr_str(s);
    let suffix = cstr_str(suffix);
    u8::from(s.ends_with(suffix)) as c_uchar
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_trim(s: *const u8) -> *mut u8 {
    cstring_from_str(cstr_str(s).trim())
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_to_upper(s: *const u8) -> *mut u8 {
    cstring_from_str(&cstr_str(s).to_uppercase())
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_to_lower(s: *const u8) -> *mut u8 {
    cstring_from_str(&cstr_str(s).to_lowercase())
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_replace(
    s: *const u8,
    from: *const u8,
    to: *const u8,
) -> *mut u8 {
    cstring_from_str(&cstr_str(s).replace(cstr_str(from), cstr_str(to)))
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_index_of(s: *const u8, sub: *const u8) -> i64 {
    let s = cstr_str(s);
    let sub = cstr_str(sub);
    s.find(sub).map(|index| index as i64).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_join(list: *mut OriList, sep: *const u8) -> *mut u8 {
    if list.is_null() {
        return cstring_from_str("");
    }
    let sep = cstr_str(sep);
    let mut out = String::new();
    for i in 0..(*list).len {
        if i > 0 {
            out.push_str(sep);
        }
        let item = *(*list).data.add(i as usize) as *const u8;
        out.push_str(cstr_str(item));
    }
    cstring_from_str(&out)
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_repeat(s: *const u8, count: i64) -> *mut u8 {
    if count <= 0 {
        return cstring_from_str("");
    }
    cstring_from_str(&cstr_str(s).repeat(count as usize))
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_pad_left(
    s: *const u8,
    target_len: i64,
    fill: *const u8,
) -> *mut u8 {
    pad_string(cstr_str(s), target_len, cstr_str(fill), true)
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_pad_right(
    s: *const u8,
    target_len: i64,
    fill: *const u8,
) -> *mut u8 {
    pad_string(cstr_str(s), target_len, cstr_str(fill), false)
}

unsafe fn cstr_bytes<'a>(ptr: *const u8) -> &'a [u8] {
    if ptr.is_null() {
        &[]
    } else {
        CStr::from_ptr(ptr as *const c_char).to_bytes()
    }
}

unsafe fn bounded_cstr_bytes<'a>(ptr: *const u8, len: i64) -> &'a [u8] {
    if ptr.is_null() || len <= 0 {
        &[]
    } else {
        std::slice::from_raw_parts(ptr, len as usize)
    }
}

unsafe fn cstr_str<'a>(ptr: *const u8) -> &'a str {
    if ptr.is_null() {
        ""
    } else {
        CStr::from_ptr(ptr as *const c_char).to_str().unwrap_or("")
    }
}

fn cstring_from_str(s: &str) -> *mut u8 {
    cstring_from_bytes(s.as_bytes().to_vec())
}

fn cstring_from_bytes(bytes: Vec<u8>) -> *mut u8 {
    CString::new(bytes)
        .unwrap_or_else(|_| CString::new("").unwrap())
        .into_raw() as *mut u8
}

fn pad_string(s: &str, target_len: i64, fill: &str, left: bool) -> *mut u8 {
    let target_len = target_len.max(0) as usize;
    let current_len = s.chars().count();
    if current_len >= target_len {
        return cstring_from_str(s);
    }
    let fill = if fill.is_empty() { " " } else { fill };
    let pad_len = target_len - current_len;
    let padding: String = fill.chars().cycle().take(pad_len).collect();
    if left {
        cstring_from_str(&(padding + s))
    } else {
        cstring_from_str(&(s.to_owned() + &padding))
    }
}

#[repr(C)]
pub struct OriList {
    pub data: *mut i64,
    pub len: i64,
    pub cap: i64,
}

unsafe extern "C" fn ori_list_dtor(ptr: *mut u8) {
    let list = ptr as *mut OriList;
    if !(*list).data.is_null() {
        libc::free((*list).data as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_new() -> *mut OriList {
    let cap = 8_i64;
    let bytes = cap as usize * std::mem::size_of::<i64>();
    let data = libc::malloc(bytes) as *mut i64;
    let list = ori_alloc(std::mem::size_of::<OriList>(), Some(ori_list_dtor)) as *mut OriList;
    if !list.is_null() {
        (*list).data = data;
        (*list).len = 0;
        (*list).cap = cap;
    }
    list
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_push(list: *mut OriList, value: i64) {
    if list.is_null() {
        return;
    }
    if (*list).len >= (*list).cap {
        let next_cap = ((*list).cap * 2).max(1);
        let bytes = next_cap as usize * std::mem::size_of::<i64>();
        (*list).data = libc::realloc((*list).data as *mut libc::c_void, bytes) as *mut i64;
        (*list).cap = next_cap;
    }
    *(*list).data.add((*list).len as usize) = value;
    (*list).len += 1;
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_get(list: *mut OriList, index: i64) -> i64 {
    if list.is_null() || index < 0 || index >= (*list).len {
        return 0;
    }
    *(*list).data.add(index as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_set(list: *mut OriList, index: i64, value: i64) {
    if list.is_null() || index < 0 || index >= (*list).len {
        return;
    }
    *(*list).data.add(index as usize) = value;
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_len(list: *mut OriList) -> i64 {
    if list.is_null() {
        0
    } else {
        (*list).len
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_pop(list: *mut OriList) -> i64 {
    if list.is_null() || (*list).len <= 0 {
        return 0;
    }
    (*list).len -= 1;
    *(*list).data.add((*list).len as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_remove(list: *mut OriList, index: i64) {
    if list.is_null() || index < 0 || index >= (*list).len {
        return;
    }
    for i in index..((*list).len - 1) {
        let next = *(*list).data.add((i + 1) as usize);
        *(*list).data.add(i as usize) = next;
    }
    (*list).len -= 1;
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_insert(list: *mut OriList, index: i64, value: i64) {
    if list.is_null() {
        return;
    }
    if (*list).len >= (*list).cap {
        let next_cap = ((*list).cap * 2).max(1);
        let bytes = next_cap as usize * std::mem::size_of::<i64>();
        (*list).data = libc::realloc((*list).data as *mut libc::c_void, bytes) as *mut i64;
        (*list).cap = next_cap;
    }
    let index = index.max(0).min((*list).len) as usize;
    for i in (index..(*list).len as usize).rev() {
        let current = *(*list).data.add(i);
        *(*list).data.add(i + 1) = current;
    }
    *(*list).data.add(index) = value;
    (*list).len += 1;
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_contains(list: *mut OriList, value: i64) -> c_uchar {
    u8::from(ori_list_index_of(list, value) >= 0) as c_uchar
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_index_of(list: *mut OriList, value: i64) -> i64 {
    if list.is_null() {
        return -1;
    }
    for i in 0..(*list).len {
        if *(*list).data.add(i as usize) == value {
            return i;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_sort(list: *mut OriList) {
    if list.is_null() || (*list).len <= 1 {
        return;
    }
    let data = std::slice::from_raw_parts_mut((*list).data, (*list).len as usize);
    data.sort_unstable();
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_reverse(list: *mut OriList) {
    if list.is_null() || (*list).len <= 1 {
        return;
    }
    let data = std::slice::from_raw_parts_mut((*list).data, (*list).len as usize);
    data.reverse();
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_slice(list: *mut OriList, start: i64, end: i64) -> *mut OriList {
    let out = ori_list_new();
    if list.is_null() {
        return out;
    }
    let start = start.max(0).min((*list).len);
    let end = end.max(start).min((*list).len);
    for i in start..end {
        ori_list_push(out, *(*list).data.add(i as usize));
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn ori_list_free(list: *mut OriList) {
    ori_arc_release(list as *mut u8);
}

/// Map: applies fn_ptr(env_ptr, elem) to each element and returns a new list.
/// fn_ptr must be compatible with `fn(*const c_void, i64) -> i64`.
#[no_mangle]
pub unsafe extern "C" fn ori_list_map(
    list: *mut OriList,
    fn_ptr: *const std::ffi::c_void,
    env_ptr: *const std::ffi::c_void,
) -> *mut OriList {
    let out = ori_list_new();
    if list.is_null() || fn_ptr.is_null() {
        return out;
    }
    let f: unsafe extern "C" fn(*const std::ffi::c_void, i64) -> i64 =
        std::mem::transmute(fn_ptr);
    for i in 0..(*list).len {
        let elem = *(*list).data.add(i as usize);
        ori_list_push(out, f(env_ptr, elem));
    }
    out
}

/// Filter: keeps elements for which fn_ptr(env_ptr, elem) returns non-zero.
#[no_mangle]
pub unsafe extern "C" fn ori_list_filter(
    list: *mut OriList,
    fn_ptr: *const std::ffi::c_void,
    env_ptr: *const std::ffi::c_void,
) -> *mut OriList {
    let out = ori_list_new();
    if list.is_null() || fn_ptr.is_null() {
        return out;
    }
    let f: unsafe extern "C" fn(*const std::ffi::c_void, i64) -> i64 =
        std::mem::transmute(fn_ptr);
    for i in 0..(*list).len {
        let elem = *(*list).data.add(i as usize);
        if f(env_ptr, elem) != 0 {
            ori_list_push(out, elem);
        }
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_split(s: *const u8, sep: *const u8) -> *mut OriList {
    let list = ori_list_new();
    let text = cstr_str(s);
    let sep = cstr_str(sep);
    if sep.is_empty() {
        for ch in text.chars() {
            ori_list_push(list, cstring_from_str(&ch.to_string()) as i64);
        }
    } else {
        for part in text.split(sep) {
            ori_list_push(list, cstring_from_str(part) as i64);
        }
    }
    list
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_chars(s: *const u8) -> *mut OriList {
    let list = ori_list_new();
    for ch in cstr_str(s).chars() {
        ori_list_push(list, cstring_from_str(&ch.to_string()) as i64);
    }
    list
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_new() -> *mut OriList {
    ori_list_new()
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_add(set: *mut OriList, value: i64) {
    if ori_set_contains(set, value) == 0 {
        ori_list_push(set, value);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_contains(set: *mut OriList, value: i64) -> c_uchar {
    if set.is_null() {
        return 0;
    }
    for i in 0..(*set).len {
        if *(*set).data.add(i as usize) == value {
            return 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_remove(set: *mut OriList, value: i64) {
    if set.is_null() {
        return;
    }
    let index = ori_list_index_of(set, value);
    if index >= 0 {
        ori_list_remove(set, index);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_len(set: *mut OriList) -> i64 {
    ori_list_len(set)
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_free(set: *mut OriList) {
    ori_list_free(set);
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_union(a: *mut OriList, b: *mut OriList) -> *mut OriList {
    let out = ori_set_new();
    if !a.is_null() {
        for i in 0..(*a).len {
            ori_set_add(out, *(*a).data.add(i as usize));
        }
    }
    if !b.is_null() {
        for i in 0..(*b).len {
            ori_set_add(out, *(*b).data.add(i as usize));
        }
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_intersection(a: *mut OriList, b: *mut OriList) -> *mut OriList {
    let out = ori_set_new();
    if a.is_null() || b.is_null() {
        return out;
    }
    for i in 0..(*a).len {
        let v = *(*a).data.add(i as usize);
        if ori_set_contains(b, v) != 0 {
            ori_set_add(out, v);
        }
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn ori_set_difference(a: *mut OriList, b: *mut OriList) -> *mut OriList {
    let out = ori_set_new();
    if a.is_null() {
        return out;
    }
    for i in 0..(*a).len {
        let v = *(*a).data.add(i as usize);
        if b.is_null() || ori_set_contains(b, v) == 0 {
            ori_set_add(out, v);
        }
    }
    out
}

#[repr(C)]
pub struct OriMap {
    pub keys: *mut i64,
    pub values: *mut i64,
    pub len: i64,
    pub cap: i64,
}

unsafe extern "C" fn ori_map_dtor(ptr: *mut u8) {
    let map = ptr as *mut OriMap;
    if !(*map).keys.is_null() {
        libc::free((*map).keys as *mut libc::c_void);
    }
    if !(*map).values.is_null() {
        libc::free((*map).values as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_new() -> *mut OriMap {
    let cap = 8_i64;
    let bytes = cap as usize * std::mem::size_of::<i64>();
    let map = ori_alloc(std::mem::size_of::<OriMap>(), Some(ori_map_dtor)) as *mut OriMap;
    if !map.is_null() {
        (*map).keys = libc::malloc(bytes) as *mut i64;
        (*map).values = libc::malloc(bytes) as *mut i64;
        (*map).len = 0;
        (*map).cap = cap;
    }
    map
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_set(map: *mut OriMap, key: i64, value: i64) {
    if map.is_null() {
        return;
    }
    for i in 0..(*map).len {
        if *(*map).keys.add(i as usize) == key {
            *(*map).values.add(i as usize) = value;
            return;
        }
    }
    if (*map).len >= (*map).cap {
        let next_cap = ((*map).cap * 2).max(1);
        let bytes = next_cap as usize * std::mem::size_of::<i64>();
        (*map).keys = libc::realloc((*map).keys as *mut libc::c_void, bytes) as *mut i64;
        (*map).values = libc::realloc((*map).values as *mut libc::c_void, bytes) as *mut i64;
        (*map).cap = next_cap;
    }
    *(*map).keys.add((*map).len as usize) = key;
    *(*map).values.add((*map).len as usize) = value;
    (*map).len += 1;
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_get(map: *mut OriMap, key: i64) -> i64 {
    if map.is_null() {
        return 0;
    }
    for i in 0..(*map).len {
        if *(*map).keys.add(i as usize) == key {
            return *(*map).values.add(i as usize);
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_contains(map: *mut OriMap, key: i64) -> c_uchar {
    if map.is_null() {
        return 0;
    }
    for i in 0..(*map).len {
        if *(*map).keys.add(i as usize) == key {
            return 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_remove(map: *mut OriMap, key: i64) {
    if map.is_null() {
        return;
    }
    for i in 0..(*map).len {
        if *(*map).keys.add(i as usize) == key {
            for j in i..((*map).len - 1) {
                *(*map).keys.add(j as usize) = *(*map).keys.add((j + 1) as usize);
                *(*map).values.add(j as usize) = *(*map).values.add((j + 1) as usize);
            }
            (*map).len -= 1;
            return;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_keys(map: *mut OriMap) -> *mut OriList {
    let out = ori_list_new();
    if map.is_null() {
        return out;
    }
    for i in 0..(*map).len {
        ori_list_push(out, *(*map).keys.add(i as usize));
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_values(map: *mut OriMap) -> *mut OriList {
    let out = ori_list_new();
    if map.is_null() {
        return out;
    }
    for i in 0..(*map).len {
        ori_list_push(out, *(*map).values.add(i as usize));
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_len(map: *mut OriMap) -> i64 {
    if map.is_null() {
        0
    } else {
        (*map).len
    }
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_key_at(map: *mut OriMap, index: i64) -> i64 {
    if map.is_null() || index < 0 || index >= (*map).len {
        return 0;
    }
    *(*map).keys.add(index as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_value_at(map: *mut OriMap, index: i64) -> i64 {
    if map.is_null() || index < 0 || index >= (*map).len {
        return 0;
    }
    *(*map).values.add(index as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ori_map_free(map: *mut OriMap) {
    ori_arc_release(map as *mut u8);
}

#[no_mangle]
pub extern "C" fn ori_math_sqrt(value: f64) -> f64 {
    value.sqrt()
}

#[no_mangle]
pub extern "C" fn ori_math_pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

#[no_mangle]
pub extern "C" fn ori_math_floor(value: f64) -> i64 {
    value.floor() as i64
}

#[no_mangle]
pub extern "C" fn ori_math_ceil(value: f64) -> i64 {
    value.ceil() as i64
}

#[no_mangle]
pub extern "C" fn ori_math_round(value: f64) -> i64 {
    value.round() as i64
}

#[no_mangle]
pub extern "C" fn ori_math_log(value: f64) -> f64 {
    value.ln()
}

#[no_mangle]
pub extern "C" fn ori_math_sin(value: f64) -> f64 {
    value.sin()
}

#[no_mangle]
pub extern "C" fn ori_math_cos(value: f64) -> f64 {
    value.cos()
}

#[no_mangle]
pub extern "C" fn ori_math_tan(value: f64) -> f64 {
    value.tan()
}

#[no_mangle]
pub extern "C" fn ori_math_abs(value: i64) -> i64 {
    value.abs()
}

#[no_mangle]
pub extern "C" fn ori_math_min(a: i64, b: i64) -> i64 {
    a.min(b)
}

#[no_mangle]
pub extern "C" fn ori_math_max(a: i64, b: i64) -> i64 {
    a.max(b)
}

#[repr(C)]
pub struct OriOptionalInt {
    has_value: c_uchar,
    value: i64,
}

#[repr(C)]
pub struct OriOptionalFloat {
    has_value: c_uchar,
    value: f64,
}

#[no_mangle]
pub extern "C" fn ori_float_to_string(value: f64) -> *mut u8 {
    cstring_from_str(&value.to_string())
}

#[no_mangle]
pub extern "C" fn ori_bool_to_string(value: c_uchar) -> *mut u8 {
    cstring_from_str(if value != 0 { "true" } else { "false" })
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_to_int(s: *const u8) -> *mut OriOptionalInt {
    let parsed = cstr_str(s).trim().parse::<i64>().ok();
    Box::into_raw(Box::new(match parsed {
        Some(value) => OriOptionalInt {
            has_value: 1,
            value,
        },
        None => OriOptionalInt {
            has_value: 0,
            value: 0,
        },
    }))
}

#[no_mangle]
pub unsafe extern "C" fn ori_string_to_float(s: *const u8) -> *mut OriOptionalFloat {
    let parsed = cstr_str(s).trim().parse::<f64>().ok();
    Box::into_raw(Box::new(match parsed {
        Some(value) => OriOptionalFloat {
            has_value: 1,
            value,
        },
        None => OriOptionalFloat {
            has_value: 0,
            value: 0.0,
        },
    }))
}
