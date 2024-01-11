use core::{ffi::c_short, mem::size_of};

use super::{lzo_999_func_decls, lzo_func_decl};

pub const LZO1Z_MEM_DECOMPRESS: u32 = 0;

pub const LZO1Z_999_MEM_COMPRESS: u32 = 14 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    lzo_func_decl!(lzo1z_decompress);

    lzo_func_decl!(lzo1z_decompress_safe);

    lzo_func_decl!(lzo1z_999_compress);

    lzo_999_func_decls!(
        lzo1z_999_compress_dict,
        lzo1z_999_compress_level,
        lzo1z_999_compress_dict_safe
    );
}
