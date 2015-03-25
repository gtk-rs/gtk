extern crate pkg_config;

fn main() {
    match pkg_config::find_library("gdk-3.0") {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    };
}
