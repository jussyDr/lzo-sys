use core::{ffi::c_short, mem::size_of};

use super::lzo_func_decl;

pub const LZO2A_MEM_COMPRESS: u32 = 0;

pub const LZO2A_999_MEM_COMPRESS: u32 = 8 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    lzo_func_decl!(lzo2a_decompress);

    lzo_func_decl!(lzo2a_decompress_safe);

    lzo_func_decl!(lzo2a_999_compress);
}
