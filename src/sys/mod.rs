mod lzo1;
mod lzo1a;
mod lzo1b;
mod lzo1c;
mod lzo1f;
mod lzo1x;
mod lzo1y;
mod lzo1z;
mod lzo2a;

pub use lzo1::*;
pub use lzo1a::*;
pub use lzo1b::*;
pub use lzo1c::*;
pub use lzo1f::*;
pub use lzo1x::*;
pub use lzo1y::*;
pub use lzo1z::*;
pub use lzo2a::*;

use core::{
    ffi::{c_uchar, c_uint},
    mem::size_of,
};

const lzo_sizeof_dict_t: c_uint = size_of::<*mut c_uchar>() as u32;

pub enum lzo_callback_t {}

macro_rules! default_function_prototype {
    ($name:ident) => {
        pub fn $name(
            src: *const core::ffi::c_uchar,
            src_len: core::ffi::c_ulonglong,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut core::ffi::c_ulonglong,
            wrkmem: *mut core::ffi::c_void,
        ) -> core::ffi::c_int;
    };
}

use default_function_prototype;

macro_rules! default_compress_dict_prototypes {
    ($unsafe_name:ident, $level_name:ident, $safe_name:ident) => {
        pub fn $unsafe_name(
            src: *const core::ffi::c_uchar,
            src_len: core::ffi::c_ulonglong,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut core::ffi::c_ulonglong,
            wrkmem: *mut core::ffi::c_void,
            dict: *const core::ffi::c_uchar,
            dict_len: core::ffi::c_ulonglong,
        ) -> core::ffi::c_int;

        pub fn $level_name(
            src: *const core::ffi::c_uchar,
            src_len: core::ffi::c_ulonglong,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut core::ffi::c_ulonglong,
            wrkmem: *mut core::ffi::c_void,
            dict: *const core::ffi::c_uchar,
            dict_len: core::ffi::c_ulonglong,
            cb: *mut lzo_callback_t,
            compression_level: core::ffi::c_int,
        ) -> core::ffi::c_int;

        pub fn $safe_name(
            src: *const core::ffi::c_uchar,
            src_len: core::ffi::c_ulonglong,
            dst: *mut core::ffi::c_uchar,
            dst_len: *mut core::ffi::c_ulonglong,
            wrkmem: *mut core::ffi::c_void,
            dict: *const core::ffi::c_uchar,
            dict_len: core::ffi::c_ulonglong,
        ) -> core::ffi::c_int;
    };
}

use default_compress_dict_prototypes;
