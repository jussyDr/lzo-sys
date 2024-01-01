use core::{ffi::c_short, mem::size_of};

use super::default_function_prototype;

pub const LZO2A_999_MEM_COMPRESS: u32 = 8 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    default_function_prototype!(lzo2a_decompress);

    default_function_prototype!(lzo2a_decompress_safe);

    default_function_prototype!(lzo2a_999_compress);
}
