// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_char, c_int};

use glib::translate::*;
use ffi;

use glib::object::Upcast;

glib_wrapper! {
    pub struct Editable(Object<ffi::GtkEditable>);

    match fn {
        get_type => || ffi::gtk_editable_get_type(),
    }
}

pub trait EditableExt {
    fn select_region(&self, start_pos: i32, end_pos: i32);
    fn get_selection_bounds(&self) -> Option<(i32, i32)>;
    fn insert_text(&self, new_text: &str, mut position: i32) -> i32;
    fn delete_text(&self, start_pos: i32, end_pos: i32);
    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<String>;
    fn cut_clipboard(&self);
    fn copy_clipboard(&self);
    fn paste_clipboard(&self);
    fn delete_selection(&self);
    fn set_position(&self, position: i32);
    fn get_position(&self) -> i32;
    fn set_editable(&self, editable: bool);
    fn is_editable(&self) -> bool;
}

impl<O: Upcast<Editable>> EditableExt for O {
    fn select_region(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(self.to_glib_none().0, start_pos, end_pos)
        }
    }

    fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        let mut i = 0;
        let mut j = 0;
        unsafe {
            if from_glib(ffi::gtk_editable_get_selection_bounds(self.to_glib_none().0,
                                                                &mut i,
                                                                &mut j)) {
                Some((i, j))
            }
            else {
                None
            }
        }
    }

    fn insert_text(&self, new_text: &str, mut position: i32) -> i32 {
        unsafe {
            // Don't need a null-terminated string here
            ffi::gtk_editable_insert_text(self.to_glib_none().0,
                                              new_text.as_ptr() as *const c_char,
                                              new_text.len() as c_int,
                                              &mut position);
            position
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(self.to_glib_none().0, start_pos, end_pos)
        }
    }

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_editable_get_chars(self.to_glib_none().0, start_pos, end_pos))
        }
    }

    fn cut_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_cut_clipboard(self.to_glib_none().0)
        }
    }

    fn copy_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_copy_clipboard(self.to_glib_none().0)
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_paste_clipboard(self.to_glib_none().0)
        }
    }

    fn delete_selection(&self) {
        unsafe {
            ffi::gtk_editable_delete_selection(self.to_glib_none().0)
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_editable(self.to_glib_none().0, position)
        }
    }

    fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_editable_get_position(self.to_glib_none().0)
        }
    }

    fn set_editable(&self, editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(self.to_glib_none().0, editable.to_glib())
        }
    }

    fn is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_get_editable(self.to_glib_none().0))
        }
    }
}
