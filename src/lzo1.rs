use crate::{
    default_compress_impl, default_unsafe_decompress_impl, default_worst_compress_size_impl,
    sys::{
        lzo1_99_compress, lzo1_compress, lzo1_decompress, LZO1_99_MEM_COMPRESS, LZO1_MEM_COMPRESS,
    },
};

default_worst_compress_size_impl!(worst_compress_size);

default_compress_impl!(compress, lzo1_compress, LZO1_MEM_COMPRESS);

default_unsafe_decompress_impl!(decompress, lzo1_decompress);

default_compress_impl!(compress_99, lzo1_99_compress, LZO1_99_MEM_COMPRESS);
