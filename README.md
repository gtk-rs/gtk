rgtk [![Build Status](https://travis-ci.org/jeremyletang/rgtk.svg?branch=master)](https://travis-ci.org/jeremyletang/rgtk) [![Gitter chat](https://badges.gitter.im/jeremyletang/rgtk.png)](https://gitter.im/jeremyletang/rgtk)
====

__Rust__ bindings and wrappers for __GTK+__, __GLib__ and __Cairo__.

Installation
============

__rgtk__ uses autoconf tools to build, so you should install them on your system.


For debian based systems:
```Shell
> apt-get install autoconf
```

For OSX:
```Shell
> brew install autoconf
```

You should also install __GTK+__, __GLib__ and __Cairo__ development files before installing __rgtk__. Optionally, it is recommended to install the debug packages containing helpful debug symbols.

For debian based system:
```Shell
> apt-get install libgtk-3-dev   libglib2.0-dev   libcairo2-dev
> apt-get install libgtk-3-0-dbg libglib2.0-0-dbg libcairo2-dbg
```

For OSX:
```Shell
> apt-get install gtk+3
```

__rgtk__ targets __GTK+__ 3.12 but can also compile older versions 3.6, 3.8 and 3.10. Setting the environment variable `GTK_VERSION` to any of `GTK_3_6`, `GTK_3_8`, `GTK_3_10`, `GTK_3_12` allows pick a version. The default is GTK_3_12.

We are currently targetting rust master compiler to build __rgtk__, make sure you have the latest version before submitting any bugs.

Then you can build __rgtk__ by generating the make file and then running `make`.

```Shell
> ./configure
> make
```

On OSX build may fail by not finding `GTK`, just add this var to your env:
```Shell
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/opt/X11/lib/pkgconfig
```

In src/bin you can find some tests showing of some functionality, these can be build and run as follows:

```Shell
> make gtktest
> ./target/gtktest

> make cairotest
> ./target/cairotest
```

__rgtk__ should build and work on both OSX and GNU/Linux. We plan on adding windows support in the future.


Finally build documentation using:

```Shell
> make doc
```

Your local copy can be accessed using your browser at

file:///{rgtk_location}/doc/rgtk/index.html

You can also access a daily build of the docs via the internet:

http://rust-ci.org/jeremyletang/rgtk/doc/rgtk/

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
