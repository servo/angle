/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate bindgen;
extern crate cmake;
use bindgen::Builder;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Generate required Angle bindings
    let includes_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("include");

    let bindings = Builder::default()
                .no_unstable_rust()
                .header("include/wrapper.hpp")
                .clang_arg(format!("-I{}", includes_path.to_str().unwrap()))
                .opaque_type("std.*")
                .whitelisted_type("Sh.*")
                .generate()
                .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");

    // Compile bridge using cmake
    let dst = cmake::Config::new(".").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=angle");
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else if !target.contains("windows-msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
