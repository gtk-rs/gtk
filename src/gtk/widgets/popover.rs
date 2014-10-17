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

//! GtkPopover — Context dependent bubbles

use gtk::ffi;
use gtk::traits;
use gtk::cast::GTK_POPOVER;
use std::string;
use gtk;

struct_Widget!(Popover)

impl Popover {
    pub fn new<T: traits::Widget>(relative_to: &T) -> Option<Popover> {
        let tmp_pointer = unsafe { ffi::gtk_popover_new(relative_to.get_widget()) };
        check_pointer!(tmp_pointer, Popover)
    }

    pub fn set_relative_to<T: traits::Widget>(&self, relative_to: &T) {
        unsafe { ffi::gtk_popover_set_relative_to(GTK_POPOVER(self.pointer), relative_to.get_widget()) }
    }

    pub fn get_relative_to<T: traits::Widget>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_popover_get_relative_to(GTK_POPOVER(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn set_position(&self, position: gtk::PositionType) {
        unsafe { ffi::gtk_popover_set_position(GTK_POPOVER(self.pointer), position) }
    }

    pub fn get_position(&self) -> gtk::PositionType {
        unsafe { ffi::gtk_popover_get_position(GTK_POPOVER(self.pointer)) }
    }

    pub fn set_modal(&self, modal: bool) {
        unsafe { ffi::gtk_popover_set_position(GTK_POPOVER(self.pointer), ffi::to_gboolean(modal)) }
    }

    pub fn get_modal(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_popover_get_position(GTK_POPOVER(self.pointer))) }
    }
}

impl_drop!(Popover)
impl_TraitWidget!(Popover)

impl traits::Container for Popover {}
impl traits::Bin for Popover {}