use std::env;

fn main() {
    let mut libpath = env::var("CARGO_MANIFEST_DIR").unwrap().to_string();
    libpath.push_str("/native_lib");
    println!("cargo:rustc-link-search=native={}", libpath);
    println!("cargo:rustc-link-lib=static=native");
}

