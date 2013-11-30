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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.

//! A widget that shows a menu when clicked on

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkWidget, GtkButton, GtkContainer, GtkToggleButton, Signal};
use ffi;
use utils::cast::GTK_MENUBUTTON;
use gtk::enums::GtkArrowType;

/// MenuButton — A widget that shows a menu when clicked on
pub struct MenuButton {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl MenuButton {
    pub fn new() -> Option<MenuButton> {
        let tmp_pointer = unsafe { ffi::gtk_menu_button_new() };
        check_pointer!(tmp_pointer, MenuButton)
    }

    pub fn set_popup<T: GtkWidget>(&mut self, popup: &T) -> () {
        unsafe {
            ffi::gtk_menu_button_set_popup(GTK_MENUBUTTON(self.pointer), popup.get_widget());
        }
    }

    pub fn set_direction(&mut self, direction: GtkArrowType) -> () {
        unsafe {
            ffi::gtk_menu_button_set_direction(GTK_MENUBUTTON(self.pointer), direction);
        }
    }

    pub fn get_direction(&self) -> GtkArrowType {
        unsafe {
            ffi::gtk_menu_button_get_direction(GTK_MENUBUTTON(self.pointer))
        }
    }

    pub fn set_align_widget<T: GtkWidget>(&mut self, align_widget: &T) -> () {
        unsafe {
            ffi::gtk_menu_button_set_align_widget(GTK_MENUBUTTON(self.pointer), align_widget.get_widget())
        }
    }
}

impl_GtkWidget!(MenuButton)
redirect_callback!(MenuButton)
redirect_callback_widget!(MenuButton)
struct_signal!(MenuButton)
impl_signals!(MenuButton)

impl GtkContainer for MenuButton {}
impl GtkButton for MenuButton {}
impl GtkToggleButton for MenuButton {}

