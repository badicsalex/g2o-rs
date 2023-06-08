// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::path::{Path, PathBuf};

fn build_g2o() -> PathBuf {
    let dst = cmake::Config::new("g2o")
        .define("G2O_NO_IMPLICIT_OWNERSHIP_OF_OBJECTS", "ON")
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
        cpp_build::Config::new()
            .include(g2o_include_path)
            .include("/usr/include/eigen3/")
            .flag("-std=c++17")
            .build(file_name);
    }
}

fn main() {
    let g2o_build_path = build_g2o();
    let g2o_include_path = g2o_build_path.join("include");
    build_cpp(&g2o_include_path);
}
