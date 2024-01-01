use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    sys::{lzo1z_999_compress, lzo1z_decompress, lzo1z_decompress_safe, LZO1Z_999_MEM_COMPRESS},
};

default_unsafe_decompress_impl!(decompress, lzo1z_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1z_decompress_safe);

default_compress_impl!(compress_999, lzo1z_999_compress, LZO1Z_999_MEM_COMPRESS);
