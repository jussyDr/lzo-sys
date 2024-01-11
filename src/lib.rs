#![no_std]

//! Raw bindings to the [LZO compression library](http://www.oberhumer.com/opensource/lzo/).
//!
//! Building requires `cmake`.

pub mod lzo1;
pub mod lzo1a;
pub mod lzo1b;
pub mod lzo1c;
pub mod lzo1f;
pub mod lzo1x;
pub mod lzo1y;
pub mod lzo1z;
pub mod lzo2a;
pub mod lzo_asm;
pub mod lzoconf;

macro_rules! lzo_func_decl {
    ($name:ident) => {
        pub fn $name(
            src: *const core::ffi::c_uchar,
            src_len: usize,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut usize,
            wrkmem: *mut core::ffi::c_void,
        ) -> core::ffi::c_int;
    };
}

use lzo_func_decl;

macro_rules! lzo_compress_level_func_decl {
    ($name:ident) => {
        pub fn $name(
            src: *const core::ffi::c_uchar,
            src_len: usize,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut usize,
            wrkmem: *mut core::ffi::c_void,
            compression_level: core::ffi::c_int,
        ) -> core::ffi::c_int;
    };
}

use lzo_compress_level_func_decl;

macro_rules! lzo_999_func_decls {
    ($unsafe_name:ident, $level_name:ident, $safe_name:ident) => {
        pub fn $unsafe_name(
            src: *const core::ffi::c_uchar,
            src_len: usize,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut usize,
            wrkmem: *mut core::ffi::c_void,
            dict: *const core::ffi::c_uchar,
            dict_len: usize,
        ) -> core::ffi::c_int;

        pub fn $level_name(
            src: *const core::ffi::c_uchar,
            src_len: usize,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut usize,
            wrkmem: *mut core::ffi::c_void,
            dict: *const core::ffi::c_uchar,
            dict_len: usize,
            cb: *mut crate::lzoconf::lzo_callback_t,
            compression_level: core::ffi::c_int,
        ) -> core::ffi::c_int;

        pub fn $safe_name(
            src: *const core::ffi::c_uchar,
            src_len: usize,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut usize,
            wrkmem: *mut core::ffi::c_void,
            dict: *const core::ffi::c_uchar,
            dict_len: usize,
        ) -> core::ffi::c_int;
    };
}

use lzo_999_func_decls;
