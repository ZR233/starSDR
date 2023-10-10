use std::path::PathBuf;

fn main() {
    let mut cfg = cmake::Config::new(PathBuf::from("uhd").join("host"));

    cfg.define("CMAKE_BUILD_TYPE", "Release").target("uhd");

    if cfg!(target_os = "windows") {
        let vcpkg = std::env::var("VCPKG_ROOT").expect("evn [VCPKG_ROOT] not set");
        let vcpkg = PathBuf::from(vcpkg);
        let cmake_toolchain = vcpkg
            .join("scripts")
            .join("buildsystems")
            .join("vcpkg.cmake");
        cfg.define("CMAKE_TOOLCHAIN_FILE", cmake_toolchain)
            .define("CMAKE_CXX_FLAGS", "/DWIN32 /D_WINDOWS /W3 /GR /EHsc")
            .define("CMAKE_C_FLAGS", "/DWIN32 /D_WINDOWS /W3")
            .define("DVCPKG_TARGET_TRIPLET", "x64-windows-static");
    }

    let dst = cfg.build();

    println!("cargo:rustc-dynamic-link-search=native={}", dst.join("bin").display());
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=uhd");
}
