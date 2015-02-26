extern crate "pkg-config" as pkg_config;

fn main() {
    match pkg_config::find_library("cairo") {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    };
}
