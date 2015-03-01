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
let button = gtk::Button:new(); // trait gtk::Button reexported as GtkButtonTrait,
                                // its trait methods can be accessed here.
```
*/

#![crate_name = "rgtk"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![allow(dead_code)] // TODO: drop this
#![allow(raw_pointer_derive)]

#![feature(unsafe_destructor)]
#![feature(core)]
#![feature(collections)]
#![feature(std_misc)]

extern crate libc;

extern crate "glib-sys" as glib_ffi;
extern crate "gdk3-sys" as gdk_ffi;
extern crate "gtk3-sys" as gtk_ffi;
extern crate "cairo-sys" as cairo_ffi;

extern crate glib;

pub use glib::traits::Connect;
pub use gtk::widgets::GValuePublic;

pub use gtk::BoxTrait as GtkBoxTrait;
pub use gtk::ActionableTrait as GtkActionableTrait;
pub use gtk::AppChooserTrait as GtkAppChooserTrait;
pub use gtk::BinTrait as GtkBinTrait;
pub use gtk::ButtonTrait as GtkButtonTrait;
pub use gtk::CellEditableTrait as GtkCellEditableTrait;
pub use gtk::CellLayoutTrait as GtkCellLayoutTrait;
pub use gtk::CellRendererTrait as GtkCellRendererTrait;
pub use gtk::CheckMenuItemTrait as GtkCheckMenuItemTrait;
pub use gtk::ColorChooserTrait as GtkColorChooserTrait;
pub use gtk::ComboBoxTrait as GtkComboBoxTrait;
pub use gtk::ContainerTrait as GtkContainerTrait;
pub use gtk::DialogTrait as GtkDialogTrait;
pub use gtk::EditableTrait as GtkEditableTrait;
pub use gtk::EntryTrait as GtkEntryTrait;
pub use gtk::FileChooserTrait as GtkFileChooserTrait;
pub use gtk::FontChooserTrait as GtkFontChooserTrait;
pub use gtk::FrameTrait as GtkFrameTrait;
pub use gtk::GObjectTrait as GObjectTrait;
pub use gtk::LabelTrait as GtkLabelTrait;
pub use gtk::MenuItemTrait as GtkMenuItemTrait;
pub use gtk::MenuShellTrait as GtkMenuShellTrait;
pub use gtk::MiscTrait as GtkMiscTrait;
pub use gtk::OrientableTrait as GtkOrientableTrait;
pub use gtk::RangeTrait as GtkRangeTrait;
pub use gtk::RecentChooserTrait as GtkRecentChooserTrait;
pub use gtk::ScaleButtonTrait as GtkScaleButtonTrait;
pub use gtk::ScrollableTrait as GtkScrollableTrait;
pub use gtk::ScrolledWindowTrait as GtkScrolledWindowTrait;
pub use gtk::TextBufferTrait as GtkTextBufferTrait;
pub use gtk::ToggleButtonTrait as GtkToggleButtonTrait;
pub use gtk::ToggleToolButtonTrait as GtkToggleToolButtonTrait;
pub use gtk::ToolButtonTrait as GtkToolButtonTrait;
pub use gtk::ToolItemTrait as GtkToolItemTrait;
pub use gtk::ToolShellTrait as GtkToolShellTrait;
pub use gtk::WidgetTrait as GtkWidgetTrait;
pub use gtk::WindowTrait as GtkWindowTrait;

pub mod gtk;
pub mod cairo;
pub mod gdk;

pub mod ffi {
    pub mod glib {
        pub use glib_ffi::*;
    }
}
