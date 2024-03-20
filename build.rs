use cc;
use std::env;
use std::path::Path;


fn main() {
    let dst = Path::new(".");

    let mut cfg = cc::Build::new();
    cfg.cuda(false);
    cfg.include("./include")
        .include("./src")
        .file("./src/kernel.cu")
        .out_dir(dst.join("lib"))
        .flag("-O3")
        .compile("libkernel.a");

    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst.join("include").display());
    println!(
        "cargo:rerun-if-changed={}",
        env::current_dir().unwrap().to_string_lossy()
    );

    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=cufft");
}
