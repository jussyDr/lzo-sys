use core::{
    ffi::{c_int, c_short, c_uchar},
    mem::size_of,
};

use super::{lzo_compress_level_func_decl, lzo_func_decl};

pub const LZO1C_MEM_COMPRESS: u32 = 16384 * size_of::<*mut c_uchar>() as u32;

pub const LZO1C_MEM_DECOMPRESS: u32 = 0;

pub const LZO1C_BEST_SPEED: c_int = 1;

pub const LZO1C_BEST_COMPRESSION: c_int = 9;

pub const LZO1C_DEFAULT_COMPRESSION: c_int = -1;

pub const LZO1C_99_MEM_COMPRESS: u32 = 65536 * size_of::<*mut c_uchar>() as u32;

pub const LZO1C_999_MEM_COMPRESS: u32 = 5 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    lzo_compress_level_func_decl!(lzo1c_compress);

    lzo_func_decl!(lzo1c_decompress);

    lzo_func_decl!(lzo1c_decompress_safe);

    lzo_func_decl!(lzo1c_1_compress);

    lzo_func_decl!(lzo1c_2_compress);

    lzo_func_decl!(lzo1c_3_compress);

    lzo_func_decl!(lzo1c_4_compress);

    lzo_func_decl!(lzo1c_5_compress);

    lzo_func_decl!(lzo1c_6_compress);

    lzo_func_decl!(lzo1c_7_compress);

    lzo_func_decl!(lzo1c_8_compress);

    lzo_func_decl!(lzo1c_9_compress);

    lzo_func_decl!(lzo1c_99_compress);

    lzo_func_decl!(lzo1c_999_compress);
}
