// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate gcc;
extern crate pkg_config;

use std::process::Command;
use gcc::Config;
use std::env;
use std::path::Path;

fn main() {
    env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");

    // try to find gtk+-3.0 library
    match pkg_config::find_library("gtk+-3.0") {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    };

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
    for (key, _) in env::vars() {
        if key.starts_with("CARGO_FEATURE_") {
            let feature = key.trim_left_matches("CARGO_FEATURE_");
            if feature.starts_with("GTK_") {
                let mut flag = String::from("-D");
                flag.push_str(feature);
                gcc_conf.flag(&flag);
            }
        }
    }

    // build library
    gcc_conf.compile("librgtk_glue.a");
}
