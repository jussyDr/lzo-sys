use core::ffi::{c_int, c_ulonglong, c_void};

use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    default_worst_compress_size_impl,
    sys::{
        lzo1b_1_compress, lzo1b_2_compress, lzo1b_3_compress, lzo1b_4_compress, lzo1b_5_compress,
        lzo1b_6_compress, lzo1b_7_compress, lzo1b_8_compress, lzo1b_999_compress,
        lzo1b_99_compress, lzo1b_9_compress, lzo1b_compress, lzo1b_decompress,
        lzo1b_decompress_safe, LZO1B_999_MEM_COMPRESS, LZO1B_99_MEM_COMPRESS, LZO1B_MEM_COMPRESS,
    },
    Error,
};

default_worst_compress_size_impl!(worst_compress_size);

pub fn compress<'a>(src: &[u8], dst: &'a mut [u8], level: c_int) -> Result<&'a mut [u8], Error> {
    let mut dst_len = dst.len() as c_ulonglong;
    let mut wrkmem = vec![0; LZO1B_MEM_COMPRESS as usize];

    let result = unsafe {
        lzo1b_compress(
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

default_unsafe_decompress_impl!(decompress, lzo1b_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1b_decompress_safe);

default_compress_impl!(compress_1, lzo1b_1_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_2, lzo1b_2_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_3, lzo1b_3_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_4, lzo1b_4_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_5, lzo1b_5_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_6, lzo1b_6_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_7, lzo1b_7_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_8, lzo1b_8_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_9, lzo1b_9_compress, LZO1B_MEM_COMPRESS);

default_compress_impl!(compress_99, lzo1b_99_compress, LZO1B_99_MEM_COMPRESS);

default_compress_impl!(compress_999, lzo1b_999_compress, LZO1B_999_MEM_COMPRESS);
