// ori-runtime implementation

use std::io::Write;

// ── ori.io ────────────────────────────────────────────────────────────────────

/// Print `len` bytes from `ptr` to stdout, followed by a newline.
#[no_mangle]
pub unsafe extern "C" fn ori_io_print(ptr: *const u8, len: i64) {
    if ptr.is_null() || len <= 0 { println!(); return; }
    let data = std::slice::from_raw_parts(ptr, len as usize);
    let _ = std::io::stdout().write_all(data);
    let _ = std::io::stdout().write_all(b"\n");
    let _ = std::io::stdout().flush();
}

/// Print `len` bytes from `ptr` to stderr, followed by a newline.
#[no_mangle]
pub unsafe extern "C" fn ori_io_eprint(ptr: *const u8, len: i64) {
    if ptr.is_null() || len <= 0 { eprintln!(); return; }
    let data = std::slice::from_raw_parts(ptr, len as usize);
    let _ = std::io::stderr().write_all(data);
    let _ = std::io::stderr().write_all(b"\n");
    let _ = std::io::stderr().flush();
}

// ── ori.string ────────────────────────────────────────────────────────────────

/// Convert an i64 to a null-terminated C string allocated with malloc.
/// Caller is responsible for freeing the result.
#[no_mangle]
pub unsafe extern "C" fn ori_int_to_cstr(n: i64) -> *mut u8 {
    let s = format!("{}\0", n);
    let ptr = libc::malloc(s.len()) as *mut u8;
    std::ptr::copy_nonoverlapping(s.as_ptr(), ptr, s.len());
    ptr
}
