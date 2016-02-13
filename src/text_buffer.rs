// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_char, c_int};
use ffi;
use glib::translate::*;
use TextBuffer;
use TextIter;

impl TextBuffer {
    pub fn insert(&self, iter: &mut TextIter, text: &str) {
        unsafe {
            ffi::gtk_text_buffer_insert(self.to_glib_none().0, iter.to_glib_none_mut().0,
                text.as_ptr() as *const c_char, text.len() as c_int);
        }
    }

    pub fn insert_at_cursor(&self, text: &str) {
        unsafe {
            ffi::gtk_text_buffer_insert_at_cursor(self.to_glib_none().0,
                text.as_ptr() as *const c_char, text.len() as c_int);
        }
    }

    pub fn insert_interactive(&self, iter: &mut TextIter, text: &str, default_editable: bool)
            -> bool {
        unsafe {
            from_glib(
                ffi::gtk_text_buffer_insert_interactive(self.to_glib_none().0,
                    iter.to_glib_none_mut().0, text.as_ptr() as *const c_char, text.len() as c_int,
                    default_editable.to_glib()))
        }
    }

    pub fn insert_interactive_at_cursor(&self, text: &str, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_interactive_at_cursor(self.to_glib_none().0,
                text.as_ptr() as *const c_char, text.len() as c_int, default_editable.to_glib()))
        }
    }

    #[cfg(feature = "3.16")]
    pub fn insert_markup(&self, iter: &mut TextIter, markup: &str) {
        unsafe {
            ffi::gtk_text_buffer_insert_markup(self.to_glib_none().0, iter.to_glib_none_mut().0,
                markup.as_ptr() as *const c_char, markup.len() as c_int);
        }
    }

    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_text_buffer_set_text(self.to_glib_none().0, text.as_ptr() as *const c_char,
                text.len() as c_int);
        }
    }


}
