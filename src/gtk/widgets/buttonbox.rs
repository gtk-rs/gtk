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

//! A container for arranging buttons

use gtk::enums::{Orientation, ButtonBoxStyle};
use gtk::cast::GTK_BUTTONBOX;
use gtk::ffi;
use gtk::traits;

/// ButtonBox — A container for arranging buttons
struct_Widget!(ButtonBox)

impl ButtonBox {
    pub fn new(orientation: Orientation) -> Option<ButtonBox> {
        let tmp_pointer = unsafe { ffi::gtk_button_box_new(orientation) };
        check_pointer!(tmp_pointer, ButtonBox)
    }

    pub fn get_layout(&self) -> ButtonBoxStyle {
        unsafe {
            ffi::gtk_button_box_get_layout(GTK_BUTTONBOX(self.pointer))
        }
    }

    pub fn get_child_secondary<T: traits::Widget + traits::Button>(&self, child: &T) -> bool {
        match unsafe { ffi::gtk_button_box_get_child_secondary(GTK_BUTTONBOX(self.pointer), child.get_widget()) } {
            ffi::GFALSE => false,
            _           => true
        }
    }

    pub fn get_child_non_homogeneous<T: traits::Widget + traits::Button>(&self, child: &T) -> bool {
        match unsafe { ffi::gtk_button_box_get_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.get_widget()) } {
            ffi::GFALSE => false,
            _           => true
        }
    }

    pub fn set_layout(&mut self, layout_style: ButtonBoxStyle) -> () {
        unsafe {
            ffi::gtk_button_box_set_layout(GTK_BUTTONBOX(self.pointer), layout_style)
        }
    }

    pub fn set_child_secondary<T: traits::Widget + traits::Button>(&mut self, child: &T, is_secondary: bool) -> () {
        match is_secondary {
            false   => unsafe { ffi::gtk_button_box_set_child_secondary(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::GFALSE) },
            true    => unsafe { ffi::gtk_button_box_set_child_secondary(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::GTRUE) }
        }
    }

    pub fn set_child_non_homogeneous<T: traits::Widget + traits::Button>(&mut self, child: &T, non_homogeneous: bool) -> () {
        match non_homogeneous {
            false   => unsafe { ffi::gtk_button_box_set_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::GFALSE) },
            true    => unsafe { ffi::gtk_button_box_set_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.get_widget(), ffi::GTRUE) }
        }
    }
}

impl_drop!(ButtonBox)
impl_TraitWidget!(ButtonBox)

impl traits::Container for ButtonBox {}
impl traits::Box for ButtonBox {}
impl traits::Orientable for ButtonBox {}
