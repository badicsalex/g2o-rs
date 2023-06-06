use std::path::{Path, PathBuf};

fn build_g2o() -> PathBuf {
    let dst = cmake::Config::new("g2o")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("G2O_USE_LGPL_LIBS", "OFF")
        .define("G2O_BUILD_APPS", "OFF")
        .define("G2O_BUILD_EXAMPLES", "OFF")
        .define("G2O_USE_OPENGL", "OFF")
        .define("G2O_USE_LOGGING", "OFF")
        .build();
    let lib_path = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=fmt");
    println!("cargo:rustc-link-lib=cholmod");
    dst
}

fn run_autocxx(g2o_include_path: &Path) {
    let eigen_path = std::path::PathBuf::from("/usr/include/eigen3");
    autocxx_build::Builder::new("src/lib.rs", [&eigen_path, g2o_include_path])
        .extra_clang_args(&["-std=c++17"])
        .build()
        .unwrap()
        .compile("g2o-bridge");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn main() {
    let g2o_build_path = build_g2o();
    let g2o_include_path = g2o_build_path.join("include");
    run_autocxx(&g2o_include_path);
}
