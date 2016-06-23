/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::process::Command;
use std::env;


fn find_make<'a>() -> &'a str {
    match Command::new("gmake").status() {
        Ok(_) => "gmake",
        Err(_) => "make",
    }
}

fn main() {
    assert!(Command::new(env::var("MAKE")
            .unwrap_or_else(|_| find_make().to_owned()))
        .args(&["-R", "-f", "makefile.cargo", &format!("-j{}", env::var("NUM_JOBS").unwrap())])
        .status()
        .unwrap()
        .success());
    println!("cargo:rustc-flags=-L native={}",
             env::var("OUT_DIR").unwrap());
}
