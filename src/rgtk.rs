// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

/*!

Bindings and wrappers for __GTK+__, __GLib__ and __Cairo__.

The various parts of __rgtk__ can be found in the submodules __gtk__, __gdk__, __glib__ and __cairo__.

Trait reexports
===============

For you're conveniance the various traits of `rgtk` are reexported in the `rgtk::*`
namespace as `{Gtk/Gdk/Glib/Cairo}{trait_name}Trait` so you can just use...

```Rust
extern mod rgtk;
use rgtk::*;
```

...to easily access all the traits methods:

```Rust
let button = gtk::Button:new(); // trait gtk::traits::Button reexported as GtkButtonTrait,
                                // it's trait methods can be accessed here.
```
*/

#![crate_name = "rgtk"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![feature(macro_rules)]
#![allow(dead_code)] // TODO: drop this
#![feature(unsafe_destructor)]

extern crate libc;

pub use GtkWidgetTrait          = gtk::traits::Widget;
pub use GtkContainerTrait       = gtk::traits::Container;
pub use GtkWindowTrait          = gtk::traits::Window;
pub use GtkMiscTrait            = gtk::traits::Misc;
pub use GtkButtonTrait          = gtk::traits::Button;
pub use GtkLabelTrait           = gtk::traits::Label;
pub use GtkBoxTrait             = gtk::traits::Box;
pub use GtkOrientableTrait      = gtk::traits::Orientable;
pub use GtkFrameTrait           = gtk::traits::Frame;
pub use GtkToggleButtonTrait    = gtk::traits::ToggleButton;
pub use GtkScaleButtonTrait     = gtk::traits::ScaleButton;
pub use GtkEntryTrait           = gtk::traits::Entry;
pub use GtkBinTrait             = gtk::traits::Bin;
pub use GtkToolShellTrait       = gtk::traits::ToolShell;
pub use GtkToolItemTrait        = gtk::traits::ToolItem;
pub use GtkToolButtonTrait      = gtk::traits::ToolButton;
pub use GtkToggleToolButtonTrait= gtk::traits::ToggleToolButton;
pub use GtkDialogTrait          = gtk::traits::Dialog;
pub use GtkColorChooserTrait    = gtk::traits::ColorChooser;
pub use GtkScrollableTrait      = gtk::traits::Scrollable;
pub use GtkFileChooserTrait     = gtk::traits::FileChooser;
pub use GtkFontChooserTrait     = gtk::traits::FontChooser;
pub use GtkAppChooserTrait      = gtk::traits::AppChooser;

#[doc(hidden)]
#[cfg(target_os="macos")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3.0")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3.0")]
    #[link(name = "rgtk_glue", kind = "static")]
    extern{}
}

#[doc(hidden)]
#[cfg(target_os="linux")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3")]
    #[link(name = "rgtk_glue", kind = "static")]
    extern{}
}

pub mod gtk;
pub mod cairo;
pub mod gdk;
pub mod glib;
