use core::{
    ffi::{c_int, c_short, c_uchar, c_ulonglong, c_void},
    mem::size_of,
};

use super::{default_function_prototype, lzo_sizeof_dict_t};

pub const LZO1C_MEM_COMPRESS: u32 = 16384 * lzo_sizeof_dict_t;
pub const LZO1C_99_MEM_COMPRESS: u32 = 65536 * lzo_sizeof_dict_t;
pub const LZO1C_999_MEM_COMPRESS: u32 = 5 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    pub fn lzo1c_compress(
        src: *const c_uchar,
        src_len: c_ulonglong,
        dst: *mut c_uchar,
        dst_len: *mut c_ulonglong,
        wrkmem: *mut c_void,
        compression_level: c_int,
    ) -> c_int;

    default_function_prototype!(lzo1c_decompress);

    default_function_prototype!(lzo1c_decompress_safe);

    default_function_prototype!(lzo1c_1_compress);

    default_function_prototype!(lzo1c_2_compress);

    default_function_prototype!(lzo1c_3_compress);

    default_function_prototype!(lzo1c_4_compress);

    default_function_prototype!(lzo1c_5_compress);

    default_function_prototype!(lzo1c_6_compress);

    default_function_prototype!(lzo1c_7_compress);

    default_function_prototype!(lzo1c_8_compress);

    default_function_prototype!(lzo1c_9_compress);

    default_function_prototype!(lzo1c_99_compress);

    default_function_prototype!(lzo1c_999_compress);
}
