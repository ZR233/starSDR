use std::{env, path::PathBuf, error::Error};

fn main()->Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let uhd = from_env().unwrap_or_else(|| {
        println!("env[UHD_ROOT] not found");
        from_sys().expect("Can't find uhd, you can set env [UHD_ROOT]")
    });

    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        env::set_var("VCPKGRS_DYNAMIC", "1");
        if let Err(e) = vcpkg::find_package("libusb") {
            panic!("Can't find libusb: {}", e);
        }
        let src = uhd.bin.join("uhd.dll");
        let dst = out_dir.join("uhd.dll");
        println!("uhd-sys copy {} -> {}", src.display(), dst.display());

        std::fs::copy(src, dst).unwrap();
    }

    println!("cargo:rustc-link-search=native={}", uhd.lib.display());
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=uhd");

    Ok(())
}

fn from_env() -> Option<UHDInfo> {
    if let Ok(uhd_root) = env::var("UHD_ROOT") {
        let uhd_root = PathBuf::from(uhd_root);
        let uhd_lib = uhd_root.join("lib");
        let uhd_bin = uhd_root.join("bin");
        return Some(UHDInfo {
            lib: uhd_lib,
            bin: uhd_bin,
        });
    }
    None
}

fn from_sys() -> Option<UHDInfo> {
    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        let root = PathBuf::from("C:\\Program Files\\UHD");
        if root.exists() {
            return Some(UHDInfo {
                lib: root.join("lib"),
                bin: root.join("bin"),
            });
        }
    }

    None
}

struct UHDInfo {
    lib: PathBuf,
    bin: PathBuf,
}
