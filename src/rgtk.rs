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

For your conveniance the various traits of `rgtk` are reexported in the `rgtk::*`
namespace as `{Gtk/Gdk/Glib/Cairo}{trait_name}Trait` so you can just use...

```Rust
extern mod rgtk;
use rgtk::*;
```

...to easily access all the traits methods:

```Rust
let button = gtk::Button:new(); // trait gtk::traits::Button reexported as GtkButtonTrait,
                                // its trait methods can be accessed here.
```
*/

#![crate_name = "rgtk"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![feature(macro_rules)]
#![allow(dead_code)] // TODO: drop this
#![feature(unsafe_destructor)]
#![feature(associated_types)]

extern crate libc;

pub use glib::traits::GObject         as GObjectTrait;
pub use gtk::traits::Widget           as GtkWidgetTrait;
pub use gtk::traits::Container        as GtkContainerTrait;
pub use gtk::traits::Window           as GtkWindowTrait;
pub use gtk::traits::Misc             as GtkMiscTrait;
pub use gtk::traits::Button           as GtkButtonTrait;
pub use gtk::traits::Label            as GtkLabelTrait;
pub use gtk::traits::Box              as GtkBoxTrait;
pub use gtk::traits::Orientable       as GtkOrientableTrait;
pub use gtk::traits::Frame            as GtkFrameTrait;
pub use gtk::traits::ToggleButton     as GtkToggleButtonTrait;
pub use gtk::traits::ScaleButton      as GtkScaleButtonTrait;
pub use gtk::traits::Entry            as GtkEntryTrait;
pub use gtk::traits::Bin              as GtkBinTrait;
pub use gtk::traits::ToolShell        as GtkToolShellTrait;
pub use gtk::traits::ToolItem         as GtkToolItemTrait;
pub use gtk::traits::ToolButton       as GtkToolButtonTrait;
pub use gtk::traits::ToggleToolButton as GtkToggleToolButtonTrait;
pub use gtk::traits::Dialog           as GtkDialogTrait;
pub use gtk::traits::ColorChooser     as GtkColorChooserTrait;
pub use gtk::traits::Scrollable       as GtkScrollableTrait;
pub use gtk::traits::FileChooser      as GtkFileChooserTrait;
pub use gtk::traits::FontChooser      as GtkFontChooserTrait;
pub use gtk::traits::AppChooser       as GtkAppChooserTrait;
pub use gtk::traits::Range            as GtkRangeTrait;
pub use gtk::traits::Editable         as GtkEditableTrait;
pub use gtk::traits::MenuShell        as GtkMenuShellTrait;
pub use gtk::traits::MenuItem         as GtkMenuItemTrait;
pub use gtk::traits::CheckMenuItem    as GtkCheckMenuItemTrait;
pub use gtk::traits::CellEditable     as GtkCellEditableTrait;
pub use gtk::traits::CellRenderer     as GtkCellRendererTrait;
pub use gtk::traits::Actionable       as GtkActionableTrait;
pub use gtk::traits::ComboBox         as GtkComboBoxTrait;

#[doc(hidden)]
#[cfg(target_os="macos")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3.0")]
    #[link(name = "gio-2.0")]
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
    #[link(name = "gio-2.0")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3")]
    #[link(name = "rgtk_glue", kind = "static")]
    extern{}
}

#[doc(hidden)]
#[cfg(target_os="windows")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3")]
    #[link(name = "gio-2.0")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3")]
    #[link(name = "rgtk_glue", kind = "static")]
    extern{}
}

pub mod gtk;
pub mod cairo;
pub mod gdk;
pub mod glib;
