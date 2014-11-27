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

use std::io::Command;
use std::os;
use std::str;

fn main() {
    Command::new("./configure").status().unwrap();
    if os::getenv(format!("{}_NO_PKG_CONFIG", envify("gtk+-3.0")).as_slice()).is_some() {
        panic!(format!("pkg-config requested to be aborted for {}", "gtk+-3.0"))
    }
    let mut cmd = Command::new("pkg-config");
    
    cmd.arg("--modversion").arg("gtk+-3.0");

    let out = match cmd.output() {
        Ok(s) => s,
        Err(e) => panic!(e)
    };

    let stdout : Vec<&str> = str::from_utf8(out.output.as_slice()).unwrap().split_str(".").collect();

    let gtk_version = format!("GTK_{}_{}", stdout[0], stdout[1]);
    os::setenv("GTK_VERSION", gtk_version);

    match Command::new("make").arg("glue").spawn() {
        Ok(_) => {}
        Err(e) => panic!(e)
    }
    let current = os::getcwd().unwrap();
    println!("cargo:rustc-flags=-l rgtk_glue:static -L {}{}", current.display(), "/target/deps");
}

fn envify(name: &str) -> String {
    name.chars().map(|c| c.to_uppercase()).map(|c| if c == '-' {'_'} else {c})
        .collect()
}