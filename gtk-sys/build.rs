// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate pkg_config;

fn main() {
    // try to find gtk+-3.0 library
    match pkg_config::find_library("gtk+-3.0") {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    };
}
