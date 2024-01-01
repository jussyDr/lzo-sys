use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    sys::{
        lzo1f_1_compress, lzo1f_999_compress, lzo1f_decompress, lzo1f_decompress_safe,
        LZO1F_999_MEM_COMPRESS, LZO1F_MEM_COMPRESS,
    },
};

default_unsafe_decompress_impl!(decompress, lzo1f_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1f_decompress_safe);

default_compress_impl!(compress_1, lzo1f_1_compress, LZO1F_MEM_COMPRESS);

default_compress_impl!(compress_999, lzo1f_999_compress, LZO1F_999_MEM_COMPRESS);
