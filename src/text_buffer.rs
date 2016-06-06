// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_char, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{slice, str};
use ffi;
use glib::translate::*;
use glib::signal::connect;
use glib_ffi;
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

    #[cfg(feature = "v3_16")]
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

    pub fn connect_insert_text<F: Fn(&TextBuffer, &TextIter, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextIter, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-text",
                transmute(insert_text_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn insert_text_trampoline(this: *mut ffi::GtkTextBuffer,
                                            location: *mut ffi::GtkTextIter,
                                            text: *mut c_char,
                                            len: c_int,
                                            f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextIter, &str) + 'static> = transmute(f);
    f(&from_glib_none(this),
      &from_glib_none(location),
      &str::from_utf8(slice::from_raw_parts(text as *const u8, len as usize)).unwrap())
}
