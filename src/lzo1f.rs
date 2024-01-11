use core::{
    ffi::{c_short, c_uchar},
    mem::size_of,
};

use super::lzo_func_decl;

pub const LZO1F_MEM_COMPRESS: u32 = 16384 * size_of::<*mut c_uchar>() as u32;

pub const LZO1F_MEM_DECOMPRESS: u32 = 0;

pub const LZO1F_999_MEM_COMPRESS: u32 = 5 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    lzo_func_decl!(lzo1f_decompress);

    lzo_func_decl!(lzo1f_decompress_safe);

    lzo_func_decl!(lzo1f_1_compress);

    lzo_func_decl!(lzo1f_999_compress);
}
