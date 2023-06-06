fn main() {
    let dst = cmake::Config::new("g2o")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("G2O_USE_LGPL_LIBS", "OFF")
        .define("G2O_BUILD_APPS", "OFF")
        .define("G2O_BUILD_EXAMPLES", "OFF")
        .define("G2O_USE_OPENGL", "OFF")
        .build()
        .join("lib");
    println!("cargo:rustc-link-search=native={}", dst.display());
    for lib in dst.read_dir().unwrap() {
        let libname = lib.unwrap().file_name().to_str().unwrap().to_owned();
        if libname.starts_with("lib") && libname.ends_with(".a") {
            println!(
                "cargo:rustc-link-lib=static={}",
                &libname[3..libname.len() - 2]
            );
        }
    }
}
