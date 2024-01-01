use core::{ffi::c_short, mem::size_of};

use super::{
    default_compress_dict_prototypes, default_function_prototype, lzo_callback_t, lzo_sizeof_dict_t,
};

pub const LZO1X_1_MEM_COMPRESS: u32 = 16384 * lzo_sizeof_dict_t;
pub const LZO1X_1_11_MEM_COMPRESS: u32 = 2048 * lzo_sizeof_dict_t;
pub const LZO1X_1_12_MEM_COMPRESS: u32 = 4096 * lzo_sizeof_dict_t;
pub const LZO1X_1_15_MEM_COMPRESS: u32 = 32768 * lzo_sizeof_dict_t;
pub const LZO1X_999_MEM_COMPRESS: u32 = 14 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    default_function_prototype!(lzo1x_decompress);

    default_function_prototype!(lzo1x_decompress_safe);

    default_function_prototype!(lzo1x_1_compress);

    default_function_prototype!(lzo1x_1_11_compress);

    default_function_prototype!(lzo1x_1_12_compress);

    default_function_prototype!(lzo1x_1_15_compress);

    default_function_prototype!(lzo1x_999_compress);

    default_compress_dict_prototypes!(
        lzo1x_999_compress_dict,
        lzo1x_999_compress_level,
        lzo1x_999_compress_dict_safe
    );

    default_function_prototype!(lzo1x_optimize);
}
