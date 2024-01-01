use core::ffi::{c_int, c_ulonglong, c_void};

use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    default_worst_compress_size_impl,
    sys::{
        lzo1c_1_compress, lzo1c_2_compress, lzo1c_3_compress, lzo1c_4_compress, lzo1c_5_compress,
        lzo1c_6_compress, lzo1c_7_compress, lzo1c_8_compress, lzo1c_999_compress,
        lzo1c_99_compress, lzo1c_9_compress, lzo1c_compress, lzo1c_decompress,
        lzo1c_decompress_safe, LZO1C_999_MEM_COMPRESS, LZO1C_99_MEM_COMPRESS, LZO1C_MEM_COMPRESS,
    },
    Error,
};

default_worst_compress_size_impl!(worst_compress_size);

pub fn compress<'a>(src: &[u8], dst: &'a mut [u8], level: c_int) -> Result<&'a mut [u8], Error> {
    let mut dst_len = dst.len() as c_ulonglong;
    let mut wrkmem = vec![0; LZO1C_MEM_COMPRESS as usize];

    let result = unsafe {
        lzo1c_compress(
            src.as_ptr(),
            src.len() as c_ulonglong,
            dst.as_mut_ptr(),
            &mut dst_len,
            wrkmem.as_mut_ptr() as *mut c_void,
            level,
        )
    };

    if result.is_negative() {
        return Err(Error);
    }

    Ok(&mut dst[..dst_len as usize])
}

default_unsafe_decompress_impl!(decompress, lzo1c_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1c_decompress_safe);

default_compress_impl!(compress_1, lzo1c_1_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_2, lzo1c_2_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_3, lzo1c_3_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_4, lzo1c_4_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_5, lzo1c_5_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_6, lzo1c_6_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_7, lzo1c_7_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_8, lzo1c_8_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_9, lzo1c_9_compress, LZO1C_MEM_COMPRESS);

default_compress_impl!(compress_99, lzo1c_99_compress, LZO1C_99_MEM_COMPRESS);

default_compress_impl!(compress_999, lzo1c_999_compress, LZO1C_999_MEM_COMPRESS);
