fn main() {
    let mut install_dir = cmake::build("lzo");
    install_dir.push("lib");

    println!("cargo:rustc-link-search=native={}", install_dir.display());
    println!("cargo:rustc-link-lib=static=lzo2");
}
