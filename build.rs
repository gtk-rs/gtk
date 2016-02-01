fn main() {
    manage_docs();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs () {
    extern crate lgpl_docs;
    const PATH: &'static str = "src";
    const IGNORES: &'static [&'static str] = &[
        "lib.rs",
        "rt.rs",
        "signal.rs",
    ];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::GTK_DOCS, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() { }
