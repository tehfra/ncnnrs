use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ncnn_include_dir = env::var("NCNN_INCLUDE_DIR")
        .map(|dir| PathBuf::from(dir))
        .expect("ERROR: please set NCNN_INCLUDE_DIR,e.g. export NCNN_INCLUDE_DIR=/path/to/ncnn/include");
    if !ncnn_include_dir
        .join("c_api.h")
        .exists()
    {
        panic!("ERROR: please set NCNN_INCLUDE_DIR,e.g2. export NCNN_INCLUDE_DIR=/path/to/ncnn/include");
    }

    // Rerun if our dummy headers change
    println!("cargo:rerun-if-changed=dummy_includes/x86intrin.h");
    println!("cargo:rerun-if-env-changed=NCNN_INCLUDE_DIR");
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++") // Use C++ since NCNN headers contain C++ code
        .clang_arg(format!("-I{}", ncnn_include_dir.display()))
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        .opaque_type("std::vector.*");

    // Add MinGW include paths and configuration for cross-compilation to Windows
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    if target_arch == "x86_64" && target_os == "windows" && target_env == "gnu" {
        // Find MinGW paths
        builder = builder
            .clang_arg("-target")
            .clang_arg("x86_64-w64-mingw32")
            // Prevent loading host standard includes
            .clang_arg("-nostdinc")
            .clang_arg("-nostdinc++")
            .clang_arg("-fno-builtin")
            // Add dummy includes first to override problematic headers
            .clang_arg("-Idummy_includes")
            // Add MinGW include paths for Windows headers
            .clang_arg(format!("-I{}", "/usr/lib/gcc/x86_64-w64-mingw32/10-posix/include/c++"))
            .clang_arg(format!("-I{}", "/usr/lib/gcc/x86_64-w64-mingw32/10-posix/include/c++/x86_64-w64-mingw32"))
            // Add system include paths
            .clang_arg("-isystem")
            .clang_arg("/usr/share/mingw-w64/include")
            // Basic Windows defines
            .clang_arg("-D_WIN32")
            .clang_arg("-D_WIN64")
            .clang_arg("-D__MINGW64__")
            .clang_arg("-D_WIN32_WINNT=0x0602") // Windows 8
            .clang_arg("-DWINVER=0x0602")
            // Disable CPU features to avoid target-specific builtin conflicts
            .clang_arg("-mno-sse")
            .clang_arg("-mno-sse2")
            .clang_arg("-mno-sse3")
            .clang_arg("-mno-ssse3")
            .clang_arg("-mno-sse4.1")
            .clang_arg("-mno-sse4.2")
            .clang_arg("-mno-avx")
            .clang_arg("-mno-avx2")
            .clang_arg("-mno-mmx")
            .clang_arg("-mno-3dnow")
            .clang_arg("-mno-fma")
            .clang_arg("-mno-fma4")
            .clang_arg("-mno-xop")
            .clang_arg("-mno-bmi")
            .clang_arg("-mno-bmi2")
            .clang_arg("-mno-lzcnt")
            .clang_arg("-mno-popcnt")
            .clang_arg("-mno-rdseed")
            .clang_arg("-mno-prfchw")
            // Suppress warnings for cross-compilation compatibility
            .clang_arg("-Wno-error")
            .clang_arg("-w");
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
