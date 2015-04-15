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

use libc::{c_char, c_int};
use cast::GTK_EDITABLE;
use ffi;
use glib::translate::{FromGlibPtr};
use glib::{to_bool, to_gboolean};

pub trait EditableTrait: ::WidgetTrait {
    fn select_region(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(GTK_EDITABLE(self.unwrap_widget()), start_pos, end_pos)
        }
    }
    fn get_selection_bounds (&self) -> Option<(i32, i32)> {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let res = unsafe {
            to_bool(ffi::gtk_editable_get_selection_bounds(GTK_EDITABLE(self.unwrap_widget()),
                                                                &mut i,
                                                                &mut j))
        };

        match res {
            true => Some((i, j)),
            false => None
        }
    }

    fn insert_text(&self, new_text: &str, position: &mut i32) {
        unsafe {
            // Don't need a null-terminated string here
            ffi::gtk_editable_insert_text(GTK_EDITABLE(self.unwrap_widget()),
                                              new_text.as_ptr() as *const c_char,
                                              new_text.len() as c_int,
                                              position)
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(GTK_EDITABLE(self.unwrap_widget()), start_pos, end_pos)
        }
    }

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_editable_get_chars(GTK_EDITABLE(self.unwrap_widget()), start_pos, end_pos))
        }
    }

    fn cut_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_cut_clipboard(GTK_EDITABLE(self.unwrap_widget()))
        }
    }

    fn copy_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_copy_clipboard(GTK_EDITABLE(self.unwrap_widget()))
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_paste_clipboard(GTK_EDITABLE(self.unwrap_widget()))
        }
    }

    fn delete_selection(&self) {
        unsafe {
            ffi::gtk_editable_delete_selection(GTK_EDITABLE(self.unwrap_widget()))
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_editable(GTK_EDITABLE(self.unwrap_widget()), position)
        }
    }

    fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_editable_get_position(GTK_EDITABLE(self.unwrap_widget()))
        }
    }

    fn set_editable(&self, editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(GTK_EDITABLE(self.unwrap_widget()), to_gboolean(editable))
        }
    }

    fn is_editable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_editable_get_editable(GTK_EDITABLE(self.unwrap_widget())))
        }
    }
}
