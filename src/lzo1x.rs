use core::{
    ffi::{c_short, c_uchar},
    mem::size_of,
};

use super::{lzo_999_func_decls, lzo_func_decl};

pub const LZO1X_MEM_COMPRESS: u32 = LZO1X_1_MEM_COMPRESS;

pub const LZO1X_MEM_DECOMPRESS: u32 = 0;

pub const LZO1X_MEM_OPTIMIZE: u32 = 0;

pub const LZO1X_1_MEM_COMPRESS: u32 = 16384 * size_of::<*mut c_uchar>() as u32;

pub const LZO1X_1_11_MEM_COMPRESS: u32 = 2048 * size_of::<*mut c_uchar>() as u32;

pub const LZO1X_1_12_MEM_COMPRESS: u32 = 4096 * size_of::<*mut c_uchar>() as u32;

pub const LZO1X_1_15_MEM_COMPRESS: u32 = 32768 * size_of::<*mut c_uchar>() as u32;

pub const LZO1X_999_MEM_COMPRESS: u32 = 14 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    lzo_func_decl!(lzo1x_decompress);

    lzo_func_decl!(lzo1x_decompress_safe);

    lzo_func_decl!(lzo1x_1_compress);

    lzo_func_decl!(lzo1x_1_11_compress);

    lzo_func_decl!(lzo1x_1_12_compress);

    lzo_func_decl!(lzo1x_1_15_compress);

    lzo_func_decl!(lzo1x_999_compress);

    lzo_999_func_decls!(
        lzo1x_999_compress_dict,
        lzo1x_999_compress_level,
        lzo1x_999_compress_dict_safe
    );

    lzo_func_decl!(lzo1x_optimize);
}
