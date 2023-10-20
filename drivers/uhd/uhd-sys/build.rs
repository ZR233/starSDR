use std::{env, path::PathBuf};

fn main() {
    let uhd_root = PathBuf::from(env::var("UHD_ROOT").unwrap());
    let uhd_lib = uhd_root.join("lib");

    println!("cargo:rustc-link-search=native={}", uhd_lib.display());
    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        let uhd_bin = uhd_root.join("bin");
        env::set_var("VCPKGRS_DYNAMIC", "1");
        if let Err(e) = vcpkg::find_package("libusb"){
           println!("cargo:warning={}", e);   
        }

        println!("cargo:rustc-link-search=native={}", uhd_bin.display());
    }

    println!("cargo:rustc-link-lib=uhd");
}
