use core::{ffi::c_uchar, mem::size_of};

use super::lzo_func_decl;

pub const LZO1A_MEM_COMPRESS: u32 = 8192 * size_of::<*mut c_uchar>() as u32;

pub const LZO1A_MEM_DECOMPRESS: u32 = 0;

pub const LZO1A_99_MEM_COMPRESS: u32 = 65536 * size_of::<*mut c_uchar>() as u32;

extern "C" {
    lzo_func_decl!(lzo1a_compress);

    lzo_func_decl!(lzo1a_decompress);

    lzo_func_decl!(lzo1a_99_compress);
}
