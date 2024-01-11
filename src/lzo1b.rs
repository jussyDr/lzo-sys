use core::{
    ffi::{c_int, c_uchar},
    mem::size_of,
};

use super::{lzo_compress_level_func_decl, lzo_func_decl};

pub const LZO1B_MEM_COMPRESS: u32 = 16384 * size_of::<*mut c_uchar>() as u32;

pub const LZO1B_MEM_DECOMPRESS: u32 = 0;

pub const LZO1B_BEST_SPEED: c_int = 1;

pub const LZO1B_BEST_COMPRESSION: c_int = 8;

pub const LZO1B_DEFAULT_COMPRESSION: c_int = -1;

pub const LZO1B_99_MEM_COMPRESS: u32 = 65536 * size_of::<*mut c_uchar>() as u32;

pub const LZO1B_999_MEM_COMPRESS: u32 = 3 * 65536 * size_of::<*mut c_uchar>() as u32;

extern "C" {
    lzo_compress_level_func_decl!(lzo1b_compress);

    lzo_func_decl!(lzo1b_decompress);

    lzo_func_decl!(lzo1b_decompress_safe);

    lzo_func_decl!(lzo1b_1_compress);

    lzo_func_decl!(lzo1b_2_compress);

    lzo_func_decl!(lzo1b_3_compress);

    lzo_func_decl!(lzo1b_4_compress);

    lzo_func_decl!(lzo1b_5_compress);

    lzo_func_decl!(lzo1b_6_compress);

    lzo_func_decl!(lzo1b_7_compress);

    lzo_func_decl!(lzo1b_8_compress);

    lzo_func_decl!(lzo1b_9_compress);

    lzo_func_decl!(lzo1b_99_compress);

    lzo_func_decl!(lzo1b_999_compress);
}
