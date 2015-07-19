use std::env;

fn main() {
    let cfgs = env::var("OVERRIDE_GTK_CFG")
        .or_else(|_| env::var("DEP_GTK_CFG"))
        .unwrap_or_else(|e| panic!("Failed to read `DEP_GTK_CFG`: {}", e));
    for cfg in cfgs.split(' ') {
        println!("cargo:rustc-cfg={}", cfg);
    }
}
