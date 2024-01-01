use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    sys::{lzo2a_999_compress, lzo2a_decompress, lzo2a_decompress_safe, LZO2A_999_MEM_COMPRESS},
};

pub const fn worst_compress_size(src_len: usize) -> usize {
    src_len + (src_len / 8) + 128 + 3
}

default_unsafe_decompress_impl!(decompress, lzo2a_decompress);

default_safe_decompress_impl!(decompress_safe, lzo2a_decompress_safe);

default_compress_impl!(compress_999, lzo2a_999_compress, LZO2A_999_MEM_COMPRESS);
