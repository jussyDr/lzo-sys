use crate::{
    default_compress_impl, default_safe_decompress_impl, default_unsafe_decompress_impl,
    sys::{
        lzo1y_1_compress, lzo1y_999_compress, lzo1y_decompress, lzo1y_decompress_safe,
        lzo1y_optimize, LZO1Y_999_MEM_COMPRESS, LZO1Y_MEM_COMPRESS,
    },
};

default_unsafe_decompress_impl!(decompress, lzo1y_decompress);

default_safe_decompress_impl!(decompress_safe, lzo1y_decompress_safe);

default_compress_impl!(compress, lzo1y_1_compress, LZO1Y_MEM_COMPRESS);

default_compress_impl!(compress_999, lzo1y_999_compress, LZO1Y_999_MEM_COMPRESS);

default_unsafe_decompress_impl!(optimize, lzo1y_optimize);
