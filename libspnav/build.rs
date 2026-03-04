extern crate cc;

use std::{env, fs};
use std::path::PathBuf;

fn main() {

    configure_libspnav();
    compile_libspnav();
    bindgen();
}

fn configure_libspnav() {
    fs::write(
        &"libspnav/spnav_config.h",
        r#"
#ifndef SPNAV_CONFIG_H_
#define SPNAV_CONFIG_H_
//#define SPNAV_USE_X11
#endif /* SPNAV_CONFIG_H_ */
        "#.trim()
    ).unwrap();
}

fn compile_libspnav() {
    cc::Build::new()
        .file("libspnav/src/spnav.c")
        .file("libspnav/src/proto.c")
        .file("libspnav/src/util.c")
        .flag("-fno-strict-aliasing")
        .include("libspnav/")
        .include("libspnav/src")
        .compile("spnav");
}

fn bindgen() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("libspnav/src/spnav.h")
        .clang_arg("-Ilibspnav/")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("libspnav.rs"));

    bindings.expect("Couldn't write bindings!");
}
