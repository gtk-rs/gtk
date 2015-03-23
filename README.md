rgtk [![Build Status](https://travis-ci.org/jeremyletang/rgtk.svg?branch=master)](https://travis-ci.org/jeremyletang/rgtk) [![Gitter chat](https://badges.gitter.im/jeremyletang/rgtk.png)](https://gitter.im/jeremyletang/rgtk)
====

__Rust__ bindings and wrappers for __GLib__, __GDK 3__, __GTK+ 3__  and __Cairo__.

Installation
============

__rgtk__ expects __GTK+__, __GLib__ and __Cairo__ development files to be installed on your system. Optionally, it is recommended to install the debug packages containing helpful debug symbols.

For Debian based system:
```Shell
> apt-get install libgtk-3-dev   libglib2.0-dev   libcairo2-dev
> apt-get install libgtk-3-0-dbg libglib2.0-0-dbg libcairo2-dbg
```

For Fedora:
```Shell
> yum install gtk3-devel glib2-devel
```

For OSX, install [XQuartz](http://xquartz.macosforge.org/landing/), then:
```Shell
> brew install gtk+3 --without-x11
> export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig:/opt/X11/lib/pkgconfig
```

__rgtk__ targets __GTK+__ 3.6 by default, newer verions support up to 3.14 is enabled by requesting a corresponding feature e.g.
```Shell
> cargo build --features GTK_3_10
```

We are currently targetting rust master compiler to build __rgtk__, make sure you have the latest version before submitting any bugs.

In `examples` you can find some tests showing off the functionality, these can be built and run as follows:

```Shell
> cd examples/gtktest
> cargo run

> cd examples/cairotest
> cargo run
```

__rgtk__ should build and work on both OSX and GNU/Linux. We plan on adding windows support in the future.


When building documentation don't forget to specify the feature set you're using:

```Shell
> cargo doc --feature GTK_3_12
```

Your local copy can be accessed using your browser at

file:///{rgtk_location}/target/doc/rgtk/index.html

You can also access a daily build of the docs via the internet:

http://rust-ci.org/jeremyletang/rgtk/doc/rgtk/

Including rgtk as a cargo dependency
====================================

To include rgtk as a cargo dependency you have to add it to your Cargo.toml and specify the GTK version you want using Cargo features
```Toml
[dependencies.rgtk]
git = "https://github.com/jeremyletang/rgtk.git"
features = ["GTK_3_12"]
```
Currently available GTK versions are 3.14, 3.12, 3.10, 3.8 and 3.6

Use __rgtk__
============

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits located in `rgtk::gtk::traits::*`. The various widgets implement these traits and live in `rgtk::gtk::*`.

For your convenience the various traits are reexported in the `rgtk::*` namespace as `Gtk{trait_name}Trait` so you can just use...

```Rust
extern mod rgtk;
use rgtk::*;
```

...to easily access all the gtk widgets and all traits methods:

```Rust
let button = gtk::Button:new(); // You have access to the struct methods of gtk::Button aswell
                                // as the trait methods from gtk::traits::Button as GtkButtonTrait.
```

Projects using rgtk
===================
* [SolidOak](https://github.com/oakes/SolidOak)
* [rrun](https://github.com/buster/rrun)

If you want yours to be added to this list, please a Pull Request for it!

Contribute
==========

Contributor you're welcome!

You probably know but __Gtk+__ uses its own GObject system: inherited class and interface.

To respect this design, I follow a special design on __rgtk__:

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Example for GtkOrientable, GtkBox, GtkButtonBox:

GtkOrientable is an interface with all methods implemented as default method of the trait gtk::traits::Orientable.

GtkBox is a class with constructors implemented on the struct `gtk::Box`, and the other method as default methods of the trait `gtk::traits::Box`. So `gtk::Box` implements `gtk::traits::Orientable` and `gtk::traits::Box`.

GtkButtonBox is a sub-class of GtkBox, the struct `gtk::ButtonBox` implements all the methods of GtkButtonBox and the traits `gtk::traits::Orientable` and `gtk::traits::Box`.

Finally, all the gtk widgets implement the trait gtk::traits::Widget.

License
=======

__rgtk__ is available under the same license term as GTK+: the LGPL (Lesser General Public license).
