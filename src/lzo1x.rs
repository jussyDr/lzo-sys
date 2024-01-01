use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    sys::{
        lzo1x_1_11_compress, lzo1x_1_12_compress, lzo1x_1_15_compress, lzo1x_1_compress,
        lzo1x_999_compress, lzo1x_decompress, lzo1x_decompress_safe, lzo1x_optimize,
        LZO1X_1_11_MEM_COMPRESS, LZO1X_1_12_MEM_COMPRESS, LZO1X_1_15_MEM_COMPRESS,
        LZO1X_1_MEM_COMPRESS, LZO1X_999_MEM_COMPRESS,
    },
};

default_unsafe_decompress_impl!(decompress, lzo1x_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1x_decompress_safe);

default_compress_impl!(compress_1, lzo1x_1_compress, LZO1X_1_MEM_COMPRESS);

default_compress_impl!(compress_1_11, lzo1x_1_11_compress, LZO1X_1_11_MEM_COMPRESS);

default_compress_impl!(compress_1_12, lzo1x_1_12_compress, LZO1X_1_12_MEM_COMPRESS);

default_compress_impl!(compress_1_15, lzo1x_1_15_compress, LZO1X_1_15_MEM_COMPRESS);

default_compress_impl!(compress_999, lzo1x_999_compress, LZO1X_999_MEM_COMPRESS);

default_safe_decompress_impl!(optimize, lzo1x_optimize);
