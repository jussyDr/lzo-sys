use std::{
    ffi::{c_int, c_uchar, c_ulonglong, c_void},
    mem::size_of,
};

use super::{default_function_prototype, lzo_sizeof_dict_t};

pub const LZO1B_MEM_COMPRESS: u32 = 16384 * lzo_sizeof_dict_t;
pub const LZO1B_99_MEM_COMPRESS: u32 = 65536 * lzo_sizeof_dict_t;
pub const LZO1B_999_MEM_COMPRESS: u32 = 3 * 65536 * size_of::<c_ulonglong>() as u32;

extern "C" {
    pub fn lzo1b_compress(
        src: *const c_uchar,
        src_len: c_ulonglong,
        dst: *mut c_uchar,
        dst_len: *mut c_ulonglong,
        wrkmem: *mut c_void,
        compression_level: c_int,
    ) -> c_int;

    default_function_prototype!(lzo1b_decompress);

    default_function_prototype!(lzo1b_decompress_safe);

    default_function_prototype!(lzo1b_1_compress);

    default_function_prototype!(lzo1b_2_compress);

    default_function_prototype!(lzo1b_3_compress);

    default_function_prototype!(lzo1b_4_compress);

    default_function_prototype!(lzo1b_5_compress);

    default_function_prototype!(lzo1b_6_compress);

    default_function_prototype!(lzo1b_7_compress);

    default_function_prototype!(lzo1b_8_compress);

    default_function_prototype!(lzo1b_9_compress);

    default_function_prototype!(lzo1b_99_compress);

    default_function_prototype!(lzo1b_999_compress);
}
