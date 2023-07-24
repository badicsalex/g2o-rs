// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::path::{Path, PathBuf};

fn build_g2o() -> PathBuf {
    let mut dst = cmake::Config::new("g2o");
    dst.define("G2O_NO_IMPLICIT_OWNERSHIP_OF_OBJECTS", "ON");
    dst.define("BUILD_SHARED_LIBS", "OFF");
    dst.define("G2O_USE_LGPL_LIBS", "OFF");
    dst.define("G2O_USE_CHOLMOD", "OFF");
    dst.define("G2O_BUILD_APPS", "OFF");
    dst.define("G2O_BUILD_EXAMPLES", "OFF");
    dst.define("G2O_BUILD_SLAM2D_TYPES", "OFF");
    dst.define("G2O_BUILD_SLAM3D_ADDON_TYPES", "OFF");
    dst.define("G2O_BUILD_SBA_TYPES", "OFF");
    dst.define("G2O_BUILD_ICP_TYPES", "OFF");
    dst.define("G2O_BUILD_SIM3_TYPES", "OFF");
    dst.define("G2O_USE_OPENGL", "OFF");
    dst.define("G2O_USE_LOGGING", "OFF");
    dst.define(
        "EIGEN3_INCLUDE_DIR",
        std::fs::canonicalize("eigen").unwrap(),
    );

    println!("cargo:rerun-if-env-changed=ANDROID_NDK");
    if let Ok(ndk) = std::env::var("ANDROID_NDK") {
        dst.define("CMAKE_SYSTEM_NAME", "Android");
        dst.define("ANDROID_NDK", ndk);
    }

    let dst = dst.build();
    let lib_path = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=fmt");
    for lib in lib_path.read_dir().unwrap() {
        let libname = lib.unwrap().file_name().to_str().unwrap().to_owned();
        if libname.starts_with("lib") && libname.ends_with(".a") {
            println!(
                "cargo:rustc-link-lib=static={}",
                &libname[3..libname.len() - 2]
            );
        }
    }
    dst
}

fn build_cpp(g2o_include_path: &Path) {
    for entry in glob::glob("src/**/*.rs").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.as_path();
        // TODO: skip files that don't have a cpp! macro
        println!("cargo:rerun-if-changed={}", file_name.display());
    }
    cpp_build::Config::new()
        .include(g2o_include_path)
        .include("/usr/include/eigen3/")
        .flag("-std=c++17")
        .build("src/lib.rs");
}

fn main() {
    let g2o_build_path = build_g2o();
    let g2o_include_path = g2o_build_path.join("include");
    build_cpp(&g2o_include_path);
}
