use cc;
use std::env;
use std::path::Path;


fn main() {
    let dst = Path::new(".");

    let mut cfg = cc::Build::new();
    cfg.cuda(false);
    cfg.include("./include")
        .file("./src/ffi/trans.c")
        .out_dir(dst.join("lib"))
        .flag("-O3")
        .compile("libwindvir.a");

    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst.join("include").display());
    println!(
        "cargo:rerun-if-changed={}",
        env::current_dir().unwrap().to_string_lossy()
    );
}
