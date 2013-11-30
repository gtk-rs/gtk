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

//! A container for arranging buttons

use std::{ptr, cast};
use std::libc::c_void;

use gtk::enums::{GtkOrientation, GtkButtonBoxStyle};
use traits::{GtkContainer, GtkWidget, GtkBox, GtkOrientable, GtkButton, Signal};
use utils::cast::GTK_BUTTONBOX;
use ffi;

/// ButtonBox — A container for arranging buttons
pub struct ButtonBox {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl ButtonBox {
    pub fn new(orientation: GtkOrientation) -> Option<ButtonBox> {
        let tmp_pointer = unsafe { ffi::gtk_button_box_new(orientation) };
        check_pointer!(tmp_pointer, ButtonBox)
    }

    pub fn get_layout(&self) -> GtkButtonBoxStyle {
        unsafe {
            ffi::gtk_button_box_get_layout(GTK_BUTTONBOX(self.pointer))
        }
    }

    pub fn get_child_secondary<T: GtkWidget + GtkButton>(&self, child: &T) -> bool {
        match unsafe { ffi::gtk_button_box_get_child_secondary(GTK_BUTTONBOX(self.pointer), child.get_widget()) } {
            ffi::Gfalse => false,
            _           => true
        }
    }

    pub fn get_child_non_homogeneous<T: GtkWidget + GtkButton>(&self, child: &T) -> bool {
        match unsafe { ffi::gtk_button_box_get_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.get_widget()) } {
            ffi::Gfalse => false,
            _           => true
        }
    }

    pub fn set_layout(&mut self, layout_style: GtkButtonBoxStyle) -> () {
        unsafe {
            ffi::gtk_button_box_set_layout(GTK_BUTTONBOX(self.pointer), layout_style)
        }
    }

    pub fn set_child_secondary<T: GtkWidget + GtkButton>(&mut self, child: &T, is_secondary: bool) -> () {
        match is_secondary {
            false   => unsafe { ffi::gtk_button_box_set_child_secondary(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::Gfalse) },
            true    => unsafe { ffi::gtk_button_box_set_child_secondary(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::Gtrue) }
        }
    }

    pub fn set_child_non_homogeneous<T: GtkWidget + GtkButton>(&mut self, child: &T, non_homogeneous: bool) -> () {
        match non_homogeneous {
            false   => unsafe { ffi::gtk_button_box_set_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::Gfalse) },
            true    => unsafe { ffi::gtk_button_box_set_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::Gtrue) }
        }
    }
}

impl_GtkWidget!(ButtonBox)
redirect_callback!(ButtonBox)
redirect_callback_widget!(ButtonBox)
struct_signal!(ButtonBox)
impl_signals!(ButtonBox)

impl GtkContainer for ButtonBox {}
impl GtkBox for ButtonBox {}
impl GtkOrientable for ButtonBox {}


