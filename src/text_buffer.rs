// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_char, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{slice, str};
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::signal::connect;
use glib_ffi;
use TextBuffer;
use TextIter;

pub trait TextBufferExtManual {
    fn connect_insert_text<F: Fn(&TextBuffer, &TextIter, &str) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<TextBuffer> + IsA<glib::object::Object>> TextBufferExtManual for O {
    fn connect_insert_text<F: Fn(&TextBuffer, &TextIter, &str) + 'static>(&self, f: F) -> u64 {
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
