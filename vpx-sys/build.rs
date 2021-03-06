extern crate bindgen;
extern crate metadeps;

use std::path::PathBuf;
use std::io::Write;
use std::env;
use std::fs::File;

fn format_write(builder: bindgen::Builder) -> String {
    builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*")
}

fn main() {
    let libs = metadeps::probe().unwrap();
    let headers = libs.get("vpx").unwrap().include_paths.clone();

    let mut builder = bindgen::builder().header("data/vpx.h")
        .default_enum_style(bindgen::EnumVariation::Rust);

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    // Manually fix the comment so rustdoc won't try to pick them
    let s = format_write(builder);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut file = File::create(out_path.join("vpx.rs")).unwrap();

    let _ = file.write(s.as_bytes());
}
