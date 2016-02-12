## v0.0.7 (master)

### Breaking changes

- Version features [have been reintroduced][gtk248] and autodetection is no more. The fallback version is 3.4.

    Select the minimal version required by your application like this:
    ```toml
    [dependencies.gtk]
    features = ["3.10"]
    ```

    Windows users should set `GTK_LIB_DIR` and use Rust's bundled gcc.

- TBD

[gtk248]: https://github.com/gtk-rs/gtk/pull/248
