use std::{env, path::PathBuf, error::Error, fs::read_dir};

fn main()->Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let deps_dir = out_dir.parent().unwrap().parent().unwrap().parent().unwrap().join("deps");

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

    let dyn_libs = get_all_dynlib(out_dir);
    for lib in &dyn_libs{
        let dst = deps_dir.join(lib.file_name().unwrap());
        println!("uhd-sys copy {} -> {}", lib.display(), dst.display());
        std::fs::copy(lib, dst).unwrap();
    }

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
fn get_all_dynlib(out_dir: PathBuf)->Vec<PathBuf>{
    let patten = if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        "dll"
    }else{
        "so"
    };

    read_dir(out_dir).unwrap().into_iter().filter(|file|{
        file.as_ref().is_ok_and(|file|{
            if let Some(ext ) = file.path().extension(){
                if let Some(e)=ext.to_str(){
                    return  e == patten;
                };
            }
            false
        })        
    }).map(|f|{
        f.unwrap().path()
    }).collect()
}


struct UHDInfo{
    lib: PathBuf,
    bin: PathBuf,
}