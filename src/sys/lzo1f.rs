use core::{ffi::c_short, mem::size_of};

use super::{default_function_prototype, lzo_sizeof_dict_t};

pub const LZO1F_MEM_COMPRESS: u32 = 16384 * lzo_sizeof_dict_t;
pub const LZO1F_999_MEM_COMPRESS: u32 = 5 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    default_function_prototype!(lzo1f_decompress);

    default_function_prototype!(lzo1f_decompress_safe);

    default_function_prototype!(lzo1f_1_compress);

    default_function_prototype!(lzo1f_999_compress);
}
