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

use std::ffi::CString;
use gtk::cast::GTK_EDITABLE;
use gtk::{self, ffi};

pub trait EditableTrait: gtk::WidgetTrait {
    fn select_region(&mut self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(GTK_EDITABLE(self.get_widget()), start_pos, end_pos)
        }
    }
    fn get_selection_bounds (&self) -> Option<(i32, i32)> {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let res = unsafe {
            ffi::to_bool(ffi::gtk_editable_get_selection_bounds(GTK_EDITABLE(self.get_widget()),
                                                                &mut i,
                                                                &mut j))
        };
        match res {
            true => Some((i, j)),
            false => None
        }
    }

    fn insert_text(&mut self, new_text: &str, new_text_length: i32, position: i32) {
        let c_str = CString::from_slice(new_text.as_bytes());
        unsafe {
                ffi::gtk_editable_insert_text(GTK_EDITABLE(self.get_widget()),
                                              c_str,
                                              new_text_length,
                                              position)
            }
    }

    fn delete_text(&mut self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(GTK_EDITABLE(self.get_widget()), start_pos, end_pos)
        }
    }

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> String {
        let chars = unsafe {
            ffi::gtk_editable_get_chars(GTK_EDITABLE(self.get_widget()), start_pos, end_pos)
        };
        unsafe { String::from_utf8(chars as *const u8) }
    }

    fn cut_clipboard(&mut self) {
        unsafe {
            ffi::gtk_editable_cut_clipboard(GTK_EDITABLE(self.get_widget()))
        }
    }

    fn copy_clipboard(&mut self) {
        unsafe {
            ffi::gtk_editable_copy_clipboard(GTK_EDITABLE(self.get_widget()))
        }
    }

    fn paste_clipboard(&mut self) {
        unsafe {
            ffi::gtk_editable_paste_clipboard(GTK_EDITABLE(self.get_widget()))
        }
    }

    fn delete_selection(&mut self) {
        unsafe {
            ffi::gtk_editable_delete_selection(GTK_EDITABLE(self.get_widget()))
        }
    }

    fn set_position(&mut self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_editable(GTK_EDITABLE(self.get_widget()), position)
        }
    }

    fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_editable_get_position(GTK_EDITABLE(self.get_widget()))
        }
    }

    fn set_editable(&mut self, editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(GTK_EDITABLE(self.get_widget()), ffi::to_gboolean(editable))
        }
    }

    fn is_editable(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_editable_get_editable(GTK_EDITABLE(self.get_widget())))
        }
    }
}