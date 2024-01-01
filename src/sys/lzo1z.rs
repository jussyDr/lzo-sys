use core::{ffi::c_short, mem::size_of};

use super::{default_compress_dict_prototypes, default_function_prototype, lzo_callback_t};

pub const LZO1Z_999_MEM_COMPRESS: u32 = 14 * 16384 * size_of::<c_short>() as u32;

extern "C" {
    default_function_prototype!(lzo1z_decompress);

    default_function_prototype!(lzo1z_decompress_safe);

    default_function_prototype!(lzo1z_999_compress);

    default_compress_dict_prototypes!(
        lzo1z_999_compress_dict,
        lzo1z_999_compress_level,
        lzo1z_999_compress_dict_safe
    );
}
