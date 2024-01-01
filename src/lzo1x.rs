use core::ffi::c_int;
use std::ptr::null_mut;

use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    default_worst_compress_size_impl,
    sys::{
        lzo1x_1_11_compress, lzo1x_1_12_compress, lzo1x_1_15_compress, lzo1x_1_compress,
        lzo1x_999_compress, lzo1x_999_compress_level, lzo1x_decompress, lzo1x_decompress_safe,
        lzo1x_optimize, LZO1X_1_11_MEM_COMPRESS, LZO1X_1_12_MEM_COMPRESS, LZO1X_1_15_MEM_COMPRESS,
        LZO1X_1_MEM_COMPRESS, LZO1X_999_MEM_COMPRESS,
    },
};

default_worst_compress_size_impl!(worst_compress_size);

default_unsafe_decompress_impl!(decompress, lzo1x_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1x_decompress_safe);

default_compress_impl!(compress_1, lzo1x_1_compress, LZO1X_1_MEM_COMPRESS);

default_compress_impl!(compress_1_11, lzo1x_1_11_compress, LZO1X_1_11_MEM_COMPRESS);

default_compress_impl!(compress_1_12, lzo1x_1_12_compress, LZO1X_1_12_MEM_COMPRESS);

default_compress_impl!(compress_1_15, lzo1x_1_15_compress, LZO1X_1_15_MEM_COMPRESS);

default_compress_impl!(compress_999, lzo1x_999_compress, LZO1X_999_MEM_COMPRESS);

pub fn compress_level<'a>(
    src: &[u8],
    dst: &'a mut [u8],
    dict: &mut [u8],
    level: c_int,
) -> Result<&'a mut [u8], crate::Error> {
    let mut dst_len = dst.len() as core::ffi::c_ulonglong;
    let mut wrkmem = vec![0; LZO1X_999_MEM_COMPRESS as usize];

    let result = unsafe {
        lzo1x_999_compress_level(
            src.as_ptr(),
            src.len() as core::ffi::c_ulonglong,
            dst.as_mut_ptr(),
            &mut dst_len,
            wrkmem.as_mut_ptr() as *mut core::ffi::c_void,
            dict.as_mut_ptr(),
            dict.len() as core::ffi::c_ulonglong,
            null_mut(),
            level,
        )
    };

    if result.is_negative() {
        return Err(crate::Error);
    }

    Ok(&mut dst[..dst_len as usize])
}

default_unsafe_decompress_impl!(optimize, lzo1x_optimize);
