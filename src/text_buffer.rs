// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc::{c_char, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{slice, str};
use TextBuffer;
use TextIter;

pub trait TextBufferExtManual: 'static {
    fn connect_insert_text<F: Fn(&Self, TextIter, &str) -> TextIter + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TextBuffer>> TextBufferExtManual for O {
    fn connect_insert_text<F: Fn(&Self, TextIter, &str) -> TextIter + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_text_trampoline<
            T,
            F: Fn(&T, TextIter, &str) -> TextIter + 'static,
        >(
            this: *mut gtk_sys::GtkTextBuffer,
            location: *mut gtk_sys::GtkTextIter,
            text: *mut c_char,
            len: c_int,
            f: glib_sys::gpointer,
        ) where
            T: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let iter = f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib_full(location),
                str::from_utf8(slice::from_raw_parts(text as *const u8, len as usize)).unwrap(),
            );

            gtk_sys::gtk_text_iter_assign(location, iter.to_glib_none().0);
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"insert-text\0".as_ptr() as *mut _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
