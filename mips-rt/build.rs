use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = &PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("native_lib/libmips-rt.a", out_dir.join("libmips-rt.a")).unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=mips-rt");
}
