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
use TextChildAnchor;
use TextIter;
use TextTag;

pub trait TextBufferExtManual: 'static {
    fn connect_apply_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_delete_range<F: Fn(&Self, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_insert_child_anchor<F: Fn(&Self, &mut TextIter, &TextChildAnchor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_insert_pixbuf<F: Fn(&Self, &mut TextIter, &gdk_pixbuf::Pixbuf) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_remove_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TextBuffer>> TextBufferExtManual for O {
    fn connect_apply_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn apply_tag_trampoline<
            P,
            F: Fn(&P, &TextTag, &mut TextIter, &mut TextIter) + 'static,
        >(
            this: *mut gtk_sys::GtkTextBuffer,
            tag: *mut gtk_sys::GtkTextTag,
            start: *mut gtk_sys::GtkTextIter,
            end: *mut gtk_sys::GtkTextIter,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut start_copy = from_glib_none(start);
            let mut end_copy = from_glib_none(end);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tag),
                &mut start_copy,
                &mut end_copy,
            );

            *start = *start_copy.to_glib_none().0;
            *end = *end_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply-tag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    apply_tag_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_delete_range<F: Fn(&Self, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn delete_range_trampoline<
            P,
            F: Fn(&P, &mut TextIter, &mut TextIter) + 'static,
        >(
            this: *mut gtk_sys::GtkTextBuffer,
            start: *mut gtk_sys::GtkTextIter,
            end: *mut gtk_sys::GtkTextIter,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut start_copy = from_glib_none(start);
            let mut end_copy = from_glib_none(end);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut start_copy,
                &mut end_copy,
            );

            *start = *start_copy.to_glib_none().0;
            *end = *end_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-range\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    delete_range_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert_child_anchor<F: Fn(&Self, &mut TextIter, &TextChildAnchor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_child_anchor_trampoline<
            P,
            F: Fn(&P, &mut TextIter, &TextChildAnchor) + 'static,
        >(
            this: *mut gtk_sys::GtkTextBuffer,
            location: *mut gtk_sys::GtkTextIter,
            anchor: *mut gtk_sys::GtkTextChildAnchor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut location_copy = from_glib_none(location);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut location_copy,
                &from_glib_borrow(anchor),
            );

            *location = *location_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-child-anchor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_child_anchor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert_pixbuf<F: Fn(&Self, &mut TextIter, &gdk_pixbuf::Pixbuf) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_pixbuf_trampoline<
            P,
            F: Fn(&P, &mut TextIter, &gdk_pixbuf::Pixbuf) + 'static,
        >(
            this: *mut gtk_sys::GtkTextBuffer,
            location: *mut gtk_sys::GtkTextIter,
            pixbuf: *mut gdk_pixbuf_sys::GdkPixbuf,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut location_copy = from_glib_none(location);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut location_copy,
                &from_glib_borrow(pixbuf),
            );

            *location = *location_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_pixbuf_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_text_trampoline<T, F: Fn(&T, &mut TextIter, &str) + 'static>(
            this: *mut gtk_sys::GtkTextBuffer,
            location: *mut gtk_sys::GtkTextIter,
            text: *mut c_char,
            len: c_int,
            f: glib_sys::gpointer,
        ) where
            T: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut location_copy = from_glib_none(location);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut location_copy,
                str::from_utf8(slice::from_raw_parts(text as *const u8, len as usize)).unwrap(),
            );

            *location = *location_copy.to_glib_none().0;
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

    fn connect_remove_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn remove_tag_trampoline<
            P,
            F: Fn(&P, &TextTag, &mut TextIter, &mut TextIter) + 'static,
        >(
            this: *mut gtk_sys::GtkTextBuffer,
            tag: *mut gtk_sys::GtkTextTag,
            start: *mut gtk_sys::GtkTextIter,
            end: *mut gtk_sys::GtkTextIter,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut start_copy = from_glib_none(start);
            let mut end_copy = from_glib_none(end);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tag),
                &mut start_copy,
                &mut end_copy,
            );

            *start = *start_copy.to_glib_none().0;
            *end = *end_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-tag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    remove_tag_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
