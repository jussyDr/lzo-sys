use crate::{
    default_compress_impl, default_unsafe_decompress_impl,
    sys::{
        lzo1a_99_compress, lzo1a_compress, lzo1a_decompress, LZO1A_99_MEM_COMPRESS,
        LZO1A_MEM_COMPRESS,
    },
};

default_compress_impl!(compress, lzo1a_compress, LZO1A_MEM_COMPRESS);

default_unsafe_decompress_impl!(decompress, lzo1a_decompress);

default_compress_impl!(compress_99, lzo1a_99_compress, LZO1A_99_MEM_COMPRESS);
