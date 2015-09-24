/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate pkg_config;

use std::env;
use std::process::Command;

fn main() {
    if !env::var("TARGET").unwrap().contains("eabi") &&
        pkg_config::Config::new().atleast_version("16.0.10").find("freetype2").is_ok()
    {
        return;
    }

    let make = env::var("MAKE").unwrap_or("make".to_string());
    let result = Command::new(make)
        .args(&["-R", "-f", "makefile.cargo"])
        .status()
        .unwrap();
    assert!(result.success());
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:outdir={}", out_dir);
}
