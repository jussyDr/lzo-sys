#![allow(clippy::missing_safety_doc)]

mod sys;

pub mod lzo1;
pub mod lzo1a;
pub mod lzo1b;
pub mod lzo1c;
pub mod lzo1f;
pub mod lzo1x;
pub mod lzo1y;
pub mod lzo1z;
pub mod lzo2a;

#[derive(Debug)]
pub struct Error;

macro_rules! default_worst_compress_size_impl {
    ($name:ident) => {
        pub const fn $name(src_len: usize) -> usize {
            src_len + (src_len / 16) + 64 + 3
        }
    };
}

use default_worst_compress_size_impl;

macro_rules! default_compress_impl {
    ($name:ident, $sys_name:ident, $mem_size:expr) => {
        pub fn $name<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a mut [u8], crate::Error> {
            let mut dst_len = dst.len() as core::ffi::c_ulonglong;
            let mut wrkmem = vec![0; $mem_size as usize];

            let result = unsafe {
                $sys_name(
                    src.as_ptr(),
                    src.len() as core::ffi::c_ulonglong,
                    dst.as_mut_ptr(),
                    &mut dst_len,
                    wrkmem.as_mut_ptr() as *mut core::ffi::c_void,
                )
            };

            if result.is_negative() {
                return Err(crate::Error);
            }

            Ok(&mut dst[..dst_len as usize])
        }
    };
}

use default_compress_impl;

macro_rules! default_unsafe_decompress_impl {
    ($name:ident, $sys_name:ident) => {
        pub unsafe fn $name<'a>(
            src: &[u8],
            dst: &'a mut [u8],
        ) -> Result<&'a mut [u8], crate::Error> {
            let mut dst_len = core::mem::MaybeUninit::uninit();

            let result = $sys_name(
                src.as_ptr(),
                src.len() as core::ffi::c_ulonglong,
                dst.as_mut_ptr(),
                dst_len.as_mut_ptr(),
                core::ptr::null_mut(),
            );

            if result.is_negative() {
                return Err(crate::Error);
            }

            let dst_len = dst_len.assume_init();

            Ok(&mut dst[..dst_len as usize])
        }
    };
}

use default_unsafe_decompress_impl;

macro_rules! default_safe_decompress_impl {
    ($name:ident, $sys_name:ident) => {
        pub fn $name<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a mut [u8], crate::Error> {
            let mut dst_len = dst.len() as core::ffi::c_ulonglong;

            let result = unsafe {
                $sys_name(
                    src.as_ptr(),
                    src.len() as core::ffi::c_ulonglong,
                    dst.as_mut_ptr(),
                    &mut dst_len,
                    core::ptr::null_mut(),
                )
            };

            if result.is_negative() {
                return Err(crate::Error);
            }

            Ok(&mut dst[..dst_len as usize])
        }
    };
}

use default_safe_decompress_impl;
