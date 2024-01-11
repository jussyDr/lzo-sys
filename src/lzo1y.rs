use core::{
    ffi::{c_short, c_uchar},
    mem::size_of,
};

use super::{lzo_999_func_decls, lzo_func_decl, lzo_optimize_func_decl};

pub const LZO1Y_MEM_COMPRESS: u32 = 16384 * size_of::<*mut c_uchar>() as u32;

pub const LZO1Y_MEM_DECOMPRESS: u32 = 0;

pub const LZO1Y_MEM_OPTIMIZE: u32 = 0;

pub const LZO1Y_999_MEM_COMPRESS: u32 = 14 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    lzo_func_decl!(lzo1y_decompress);

    lzo_func_decl!(lzo1y_decompress_safe);

    lzo_func_decl!(lzo1y_1_compress);

    lzo_func_decl!(lzo1y_999_compress);

    lzo_999_func_decls!(
        lzo1y_999_compress_dict,
        lzo1y_999_compress_level,
        lzo1y_999_compress_dict_safe
    );

    lzo_optimize_func_decl!(lzo1y_optimize);
}
