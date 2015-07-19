// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate gcc;
extern crate pkg_config;

use std::ascii::AsciiExt;
use std::process::Command;
use gcc::Config;
use std::env;
use std::path::Path;

const MIN_MAJOR: u16 = 3;
const MIN_MINOR: u16 = 4;
const MINOR_STEP: u16 = 2;

fn main() {
    let lib = pkg_config::find_library("gtk+-3.0")
        .unwrap_or_else(|e| panic!("{}", e));
    let mut parts = lib.version.splitn(3, '.')
        .map(|s| s.parse())
        .take_while(|r| r.is_ok())
        .map(|r| r.unwrap());
    let version: (u16, u16) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    let mut cfgs = Vec::new();
    if version.0 == MIN_MAJOR && version.1 > MIN_MINOR {
        let major = version.0;
        let mut minor = MIN_MINOR + MINOR_STEP;
        while minor <= version.1 {
            cfgs.push(format!("gtk_{}_{}", major, minor));
            minor += MINOR_STEP;
        }
    }
    for cfg in &cfgs {
        println!("cargo:rustc-cfg={}", cfg);
    }
    println!("cargo:cfg={}", cfgs.connect(" "));

    env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");

    // call native pkg-config, there is no way to do this with pkg-config for now
    let cmd = Command::new("pkg-config").arg("--cflags").arg("gtk+-3.0")
                .output().unwrap();
    if !cmd.status.success() {
        panic!("{}", String::from_utf8_lossy(&cmd.stderr));
    }

    // make the vector of path to set to gcc::Config
    let output = String::from_utf8(cmd.stdout).unwrap();

    // build include path
    let mut gcc_conf = Config::new();
    for s in output.split(' ') {
        if s.starts_with("-I") {
            let path: &Path = s[2..].as_ref();
            gcc_conf.include(path);
        }
    }
    gcc_conf.file("src/gtk_glue.c");

    // pass the GTK feature flags
    for cfg in &cfgs {
        gcc_conf.flag(&format!("-D{}", cfg.to_ascii_uppercase()));
    }

    // build library
    gcc_conf.compile("librgtk_glue.a");
}
