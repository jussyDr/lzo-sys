use super::{default_function_prototype, lzo_sizeof_dict_t};

pub const LZO1_MEM_COMPRESS: u32 = 8192 * lzo_sizeof_dict_t;
pub const LZO1_99_MEM_COMPRESS: u32 = 65536 * lzo_sizeof_dict_t;

extern "C" {
    default_function_prototype!(lzo1_compress);

    default_function_prototype!(lzo1_decompress);

    default_function_prototype!(lzo1_99_compress);
}
