#![allow(non_camel_case_types)]

use core::{
    ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_void},
    mem::size_of,
};

pub const LZO_VERSION: c_uint = 0x20a0;
pub const LZO_VERSION_STRING: *const c_char = "2.10\0".as_ptr() as *const c_char;
pub const LZO_VERSION_DATE: *const c_char = "Mar 01 2017\0".as_ptr() as *const c_char;

pub type lzo_compress_t = unsafe extern "C" fn(
    src: *const core::ffi::c_uchar,
    src_len: core::ffi::c_ulonglong,
    dst: *mut core::ffi::c_uchar,
    dst_len: *mut core::ffi::c_ulonglong,
    wrkmem: *mut core::ffi::c_void,
) -> core::ffi::c_int;

pub type lzo_decompress_t = unsafe extern "C" fn(
    src: *const core::ffi::c_uchar,
    src_len: core::ffi::c_ulonglong,
    dst: *mut core::ffi::c_uchar,
    dst_len: *mut core::ffi::c_ulonglong,
    wrkmem: *mut core::ffi::c_void,
) -> core::ffi::c_int;

pub type lzo_optimize_t = unsafe extern "C" fn(
    src: *const core::ffi::c_uchar,
    src_len: core::ffi::c_ulonglong,
    dst: *mut core::ffi::c_uchar,
    dst_len: *mut core::ffi::c_ulonglong,
    wrkmem: *mut core::ffi::c_void,
) -> core::ffi::c_int;

pub type lzo_compress_dict_t = unsafe extern "C" fn(
    src: *const core::ffi::c_uchar,
    src_len: core::ffi::c_ulonglong,
    dst: *mut core::ffi::c_uchar,
    dst_len: *mut core::ffi::c_ulonglong,
    wrkmem: *mut core::ffi::c_void,
    dict: *const core::ffi::c_uchar,
    dict_len: usize,
) -> core::ffi::c_int;

pub type lzo_decompress_dict_t = unsafe extern "C" fn(
    src: *const core::ffi::c_uchar,
    src_len: core::ffi::c_ulonglong,
    dst: *mut core::ffi::c_uchar,
    dst_len: *mut core::ffi::c_ulonglong,
    wrkmem: *mut core::ffi::c_void,
    dict: *const core::ffi::c_uchar,
    dict_len: usize,
) -> core::ffi::c_int;

pub type lzo_alloc_func_t =
    unsafe extern "C" fn(cb: *mut lzo_callback_t, items: usize, size: usize) -> *mut c_void;

pub type lzo_free_func_t = unsafe extern "C" fn(cb: *mut lzo_callback_t, ptr: *mut c_void);

pub type lzo_progress_func_t = unsafe extern "C" fn(*mut lzo_callback_t, usize, usize, c_int);

#[repr(C)]
pub struct lzo_callback_t {
    pub nalloc: lzo_alloc_func_t,
    pub nfree: lzo_free_func_t,
    pub nprogress: lzo_progress_func_t,
    pub user1: *mut c_void,
    pub user2: usize,
    pub user3: usize,
}

pub const LZO_E_OK: c_int = 0;
pub const LZO_E_ERROR: c_int = -1;
pub const LZO_E_OUT_OF_MEMORY: c_int = -2;
pub const LZO_E_NOT_COMPRESSIBLE: c_int = -3;
pub const LZO_E_INPUT_OVERRUN: c_int = -4;
pub const LZO_E_OUTPUT_OVERRUN: c_int = -5;
pub const LZO_E_LOOKBEHIND_OVERRUN: c_int = -6;
pub const LZO_E_EOF_NOT_FOUND: c_int = -7;
pub const LZO_E_INPUT_NOT_CONSUMED: c_int = -8;
pub const LZO_E_NOT_YET_IMPLEMENTED: c_int = -9;
pub const LZO_E_INVALID_ARGUMENT: c_int = -10;
pub const LZO_E_INVALID_ALIGNMENT: c_int = -11;
pub const LZO_E_OUTPUT_NOT_CONSUMED: c_int = -12;
pub const LZO_E_INTERNAL_ERROR: c_int = -99;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn lzo_init() -> c_int {
    __lzo_init_v2(
        LZO_VERSION,
        size_of::<c_short>() as c_int,
        size_of::<c_int>() as c_int,
        size_of::<c_long>() as c_int,
        size_of::<u32>() as c_int,
        size_of::<usize>() as c_int,
        size_of::<*mut c_uchar>() as c_int,
        size_of::<*mut c_char>() as c_int,
        size_of::<*mut c_void>() as c_int,
        size_of::<lzo_callback_t>() as c_int,
    )
}

extern "C" {
    fn __lzo_init_v2(
        v: c_uint,
        s1: c_int,
        s2: c_int,
        s3: c_int,
        s4: c_int,
        s5: c_int,
        s6: c_int,
        s7: c_int,
        s8: c_int,
        s9: c_int,
    ) -> c_int;

    pub fn lzo_version() -> c_uint;

    pub fn lzo_version_string() -> *const c_char;

    pub fn lzo_version_date() -> *const c_char;

    pub fn lzo_memcmp(a: *const c_void, b: *const c_void, len: usize) -> c_int;

    pub fn lzo_memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> c_int;

    pub fn lzo_memmove(dst: *mut c_void, src: *const c_void, len: usize) -> c_int;

    pub fn lzo_memset(buf: *mut c_void, c: c_int, len: usize) -> c_int;

    pub fn lzo_adler32(c: u32, buf: *const c_uchar, len: usize) -> u32;

    pub fn lzo_crc32(c: u32, buf: *const c_uchar, len: usize) -> u32;

    pub fn lzo_get_crc32_table() -> *const u32;
}
