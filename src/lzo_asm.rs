use super::lzo_func_decl;

extern "C" {
    lzo_func_decl!(lzo1c_decompress_asm);

    lzo_func_decl!(lzo1c_decompress_asm_safe);

    lzo_func_decl!(lzo1f_decompress_asm_fast);

    lzo_func_decl!(lzo1f_decompress_asm_fast_safe);

    lzo_func_decl!(lzo1x_decompress_asm);

    lzo_func_decl!(lzo1x_decompress_asm_safe);

    lzo_func_decl!(lzo1x_decompress_asm_fast);

    lzo_func_decl!(lzo1x_decompress_asm_fast_safe);

    lzo_func_decl!(lzo1y_decompress_asm);

    lzo_func_decl!(lzo1y_decompress_asm_safe);

    lzo_func_decl!(lzo1y_decompress_asm_fast);

    lzo_func_decl!(lzo1y_decompress_asm_fast_safe);
}
