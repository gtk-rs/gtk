// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

#![feature(collections)]

extern crate gcc;
extern crate "pkg-config" as pkg_config;

use std::process::Command;
use gcc::Config;
use std::env;
use std::path::PathBuf;

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
    let res: Vec<&str> = output.split(' ').collect();
    let paths: Vec<PathBuf> = res.iter().filter_map(|s| {
        if s.len() > 1 && s.char_at(1) == 'I' { Some(PathBuf::new(&s[2..])) }
        else { None }
    }).collect();

    // build include path
    let mut gcc_conf = Config::new();
    for path in paths {
        gcc_conf.include(&path);
    }
    gcc_conf.file("src/gtk_glue.c");

    // pass the GTK feature flags
    for (key, _) in env::vars() {
        if key.starts_with("CARGO_FEATURE_") {
            let feature = key.trim_left_matches("CARGO_FEATURE_");
            if feature.starts_with("GTK_") {
                let mut flag = String::from_str("-D");
                flag.push_str(feature);
                gcc_conf.flag(&flag);
            }
        }
    }

    // build library
    gcc_conf.compile("librgtk_glue.a");
}
