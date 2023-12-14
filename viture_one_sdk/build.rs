fn main() {
    println!("cargo:rustc-link-lib=static=usb-1.0");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=../viture_one_linux_sdk/libs");
    println!("cargo:rustc-link-lib=static=viture_one_sdk");
    println!("cargo:rustc-link-lib=dylib=udev");
}
