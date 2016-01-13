# gtk [![Build Status](https://travis-ci.org/gtk-rs/gtk.png?branch=master)](https://travis-ci.org/gtk-rs/gtk) [![Build status](https://ci.appveyor.com/api/projects/status/5mot32ipr12iocw0?svg=true)](https://ci.appveyor.com/project/GuillaumeGomez/gtk) [![Gitter](https://badges.gitter.im/Join Chat.svg)](https://gitter.im/gtk-rs/gtk)

__Rust__ bindings and wrappers for __GLib__, __GDK 3__, __GTK+ 3__  and __Cairo__.
[Online documentation](http://gtk-rs.org/docs/gtk/).

## Building

__gtk__ expects __GTK+__, __GLib__ and __Cairo__ development files to be installed on your system. Optionally, it is recommended to install the debug packages containing helpful debug symbols.

### Debian and Ubuntu

```Shell
> sudo apt-get install libgtk-3-dev
> sudo apt-get install libgtk-3-0-dbg libglib2.0-0-dbg libcairo2-dbg
```

### Fedora

```Shell
> sudo dnf install gtk3-devel glib2-devel

# Fedora 21 and earlier
> sudo yum install gtk3-devel glib2-devel
```

### OS X

```Shell
> brew install gtk+3
```

### Windows

Make sure you have the [GNU
ABI](https://www.rust-lang.org/downloads.html#win-foot) version of the rust
compiler installed. Since we need our own toolchain make sure to uncheck
"Linker and platform libraries" in the Rust setup.
![Screenshot](rust_setup.png)

If you already have installed Rust, check if your Rust installation has
`gcc.exe` and `ld.exe` in its `bin` directory. In that case remove those
executables, they will be provided by mingw instead. If you won't you'll
probably get a linking error `ld: cannot find -limm32`.

For the toolchain one has two possibilities:
 1. Install msys2 with a mingw toolchain and the gtk3 package
 2. Install the mingw-w64 toolchain separately and then install a GTK+ SDK

#### 1. msys2 toolchain

This method is recommended according to the [GTK+ project]
(http://www.gtk.org/download/windows.php). You can follow [this guide]
(https://blogs.gnome.org/nacho/2014/08/01/how-to-build-your-gtk-application-on-windows/)
or follow these steps:

 - Install and update [msys2](https://msys2.github.io/)
 - Install the mingw toolchain:

 ```Shell
 $ pacman -S mingw-w64-x86_64-toolchain
 ```

 - Install the gtk3 package:

 ```Shell
 $ pacman -S mingw-w64-x86_64-gtk3
 ```
 
Make sure that either `<your msys installation folder>\mingw32\bin` or `<your msys installation folder>\mingw64\bin` is in your `PATH` e.g. (if you have msys32 installed at `c:\msys32`, add `c:\msys32\mingw32\bin` or `c:\msys32\mingw64\bin` to your `PATH`).

#### 2. separate mingw toolchain

Install [mingw-w64](http://mingw-w64.yaxm.org/) (select the win32 threading model) and download a __GTK+__ SDK:
 * The GNOME project hosts [distributions](http://win32builder.gnome.org/) of GTK+ 3.4 upto 3.10
 * [GTK+ for Windows Runtime Environment Installer: 64-bit](https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer) supports GTK+ 3.14, its SDK download links can currently be found [here](http://lvserver.ugent.be/gtk-win64/sdk/).

Make sure both mingw's and the sdk's `bin` directories are in your `PATH` e.g. (assuming mingw is installed in `C:\mingw-w64` and the SDK unpacked into `C:\gtk`)

```
C:\> set PATH="C:\mingw-w64\bin;C:\gtk\bin;%PATH%"
```

## Versions

The build script will query the installed library versions from `pkg-config`
and instruct `rustc` via `cfg` arguments to compile the appropriate set of APIs.

All the APIs available in the installed library will just work but if you
attempt to use newer ones, the build will fail. Presently, Rust doesn't allow
to generate custom error messages so there doesn't appear to be a way to make
such errors more friendly.

Examples are providing in the [gtk-rs/examples](https://github.com/gtk-rs/examples) repository, you can find some tests showing off the functionality, these can be built and run as follows:

```Shell
> cargo build --release
> ./target/release/gtktest
> ./target/release/cairotest
```

## Documentation

Browse the documentation [at our website](http://gtk-rs.org/docs/gtk/).

### Examples

Examples of gtk applications can be found in the [examples repository](https://github.com/gtk-rs/examples).

## Including gtk as a cargo dependency

To include gtk as a cargo dependency you have to add it to your Cargo.toml:
```Toml
[dependencies]
gtk = "0.0"
```

## Use __gtk__

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits located in `gtk::gtk::traits::*`. The various widgets implement these traits and live in `gtk::gtk::*`.

For your convenience the various traits are reexported in the `gtk::*` namespace as `Gtk{trait_name}Trait` so you can just use...

```Rust
extern crate gtk;

use gtk::*;
```

...to easily access all the gtk widgets and all traits methods:

```Rust
let button = gtk::Button:new(); // You have access to the struct methods of gtk::Button aswell
                                // as the trait methods from gtk::traits::Button as GtkButtonTrait.
```

## Projects using gtk
* [SolidOak](https://github.com/oakes/SolidOak)
* [rrun](https://github.com/buster/rrun)
* [process-viewer](https://github.com/GuillaumeGomez/process-viewer)

If you want yours to be added to this list, please create a Pull Request for it!

##Screenshots

![screenshot](http://guillaume-gomez.fr/image/gtk.png)
![screenshot](http://guillaume-gomez.fr/image/gtk2.png)

## Contribute

Contributor you're welcome!

You probably know but __Gtk+__ uses its own GObject system: inherited class and interface.

To respect this design, I follow a special design on __gtk__:

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Example for GtkOrientable, GtkBox, GtkButtonBox:

GtkOrientable is an interface with all methods implemented as default method of the trait gtk::traits::Orientable.

GtkBox is a class with constructors implemented on the struct `gtk::Box`, and the other method as default methods of the trait `gtk::traits::Box`. So `gtk::Box` implements `gtk::traits::Orientable` and `gtk::traits::Box`.

GtkButtonBox is a sub-class of GtkBox, the struct `gtk::ButtonBox` implements all the methods of GtkButtonBox and the traits `gtk::traits::Orientable` and `gtk::traits::Box`.

Finally, all the gtk widgets implement the trait gtk::traits::Widget.

## License

__gtk__ is available under the MIT License, please refer to it.
