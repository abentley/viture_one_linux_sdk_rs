use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=usb-1.0");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=../viture_one_linux_sdk/libs");
    println!("cargo:rustc-link-lib=static=viture_one_sdk");
    println!("cargo:rustc-link-lib=dylib=udev");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=viture_wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("viture_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to gen bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}
