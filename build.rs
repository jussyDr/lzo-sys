fn main() {
    let mut install_dir = cmake::Config::new("lzo")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .build();

    install_dir.push("lib");

    println!("cargo:rustc-link-search=native={}", install_dir.display());
    println!("cargo:rustc-link-lib=static=lzo2");
}
