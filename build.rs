extern crate cmake;

use std::env;

fn find_and_link_tgl(cmake_cfg: &mut cmake::Config) {
    let dst = cmake_cfg.build();
    let path = dst.join("build")
        .join("tgl");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}", path.join("Release").display());
    } else {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=static=tplgy_tgl-static");
}

fn main() {
    let mut cmake_cfg = cmake::Config::new("./src/3rdparty");
    if let Ok(gen) = env::var("CMAKE_GENERATOR") {
        cmake_cfg.generator(gen);
    }

    println!("cargo:rustc-link-search=native={}", cmake_cfg.build().join("lib").display());
    find_and_link_tgl(&mut cmake_cfg);
}
 
