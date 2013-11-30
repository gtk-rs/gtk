rgtk
====

__Rust__ bindings and wrappers for __GTK+__


Installation
============

__rgtk__ use autoconf tools to build, so you should install them on your system.

You should install __GTK+__ developpement library before install __rgtk__.

__rgtk__ use the version 3.10 of __GTK+__, so it should be up to date or the library cannot build.

Then you can build __rgtk__ in two steps: 

* First build a little c-glue library to deal with some gtk macros by typing :

```Shell
> make glue
```

* next you can build __rgtk__ with the following command :

```Shell
> make 
```

* you can build an awful test main which display some widget :

```Shell
> make test
```

__rgtk__ should build / work on osx and Linux.


Use __rgtk__
============

To respect __GTK+__ inheritence, all the functionnalities of gtk widgets are dispatched between class implementation and trait default methods.

That's why all the widgets are in the module gtk, and all the main traits are reexport on the main module of __rgtk__.

To use __rgtk__, you can simply do :

```Rust
extern mod rgtk;
use rgtk::*;
```

You can now access easily all the widgets and all the traits methods:

```Rust
let button = gtk::Button:new(); // You have access to the methods of the button and all the method of the trait GtkButton.
```


Contribute
==========

Contributor you're welcome !

You probably know but __Gtk+__ use it own Object system : inherited class and interface.

To respect this design I follow a special design on __rgtk__ :

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Exemple for GtkOrientable, GtkBox, GtkButtonBox :

GtkOrientable is an interface with all the methods implemented as default method of the trait GtkOrientable.

GtkBox is a class with constructors implemented on the class gtk::Box, and the other method as default methods of the trait GtkBox. So gtk::Box implement GtkOrientable and GtkBox.

GtkButtonBox is a sub-class of GtkBox, the class gtk::ButtonBox implement all the methods of GtkButtonBox, the trait GtkOrientable and GtkBox.

Finally all the gtk widget implements the traits GtkWidget.

License
=======

__rgtk__ is available under the same license term than GTK+, the LGPL (Lesser General Public license). 