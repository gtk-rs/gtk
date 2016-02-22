## v0.0.7 (master)

### Breaking changes

- Version features [have been reintroduced][gtk248] and autodetection is no
  more. The fallback version is 3.4.

    Select the minimal version required by your application like this:
    ```toml
    [dependencies.gtk]
    features = ["3.10"]
    ```

### Improvements

- [Building][sys21]:

 - Windows users no longer need to delete Rust's bundled gcc.

 - In the absence of `pkg-config` we try to link anyway assuming the libraries
   can be found in the default search path. There are no version checks in this
   case.

 - Setting `GTK_LIB_DIR` skips `pkg-config` altogether.

- TBD

[gtk248]: https://github.com/gtk-rs/gtk/pull/248
[sys21]: https://github.com/gtk-rs/sys/pull/21
