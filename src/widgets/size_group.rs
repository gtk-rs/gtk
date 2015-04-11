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

//! GtkSizeGroup — Grouping widgets so they request the same size

use ffi;
use glib::{to_bool, to_gboolean};

pub struct SizeGroup {
    pointer: *mut ffi::C_GtkSizeGroup
}

impl SizeGroup {
    pub fn new(mode: ::SizeGroupMode) -> Option<SizeGroup> {
        let tmp_pointer = unsafe { ffi::gtk_size_group_new(mode) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(SizeGroup {
                pointer: tmp_pointer
            })
        }
    }

    pub fn set_mode(&self, mode: ::SizeGroupMode) {
        unsafe { ffi::gtk_size_group_set_mode(self.pointer, mode) }
    }

    pub fn get_mode(&self) -> ::SizeGroupMode {
        unsafe { ffi::gtk_size_group_get_mode(self.pointer) }
    }

    pub fn set_ignore_hidden(&self, ignore_hidden: bool) {
        unsafe { ffi::gtk_size_group_set_ignore_hidden(self.pointer, to_gboolean(ignore_hidden)) }
    }

    pub fn get_ignore_hidden(&self) -> bool {
        unsafe { to_bool(ffi::gtk_size_group_get_ignore_hidden(self.pointer)) }
    }

    pub fn add_widget<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe { ffi::gtk_size_group_add_widget(self.pointer, widget.unwrap_widget()) }
    }

    pub fn remove_widget<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe { ffi::gtk_size_group_remove_widget(self.pointer, widget.unwrap_widget()) }
    }
}

impl_GObjectFunctions!(SizeGroup, C_GtkSizeGroup);
