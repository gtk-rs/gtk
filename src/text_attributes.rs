// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TextAttributes(Refcounted<ffi::GtkTextAttributes>);

    match fn {
        ref => |ptr| ffi::gtk_text_attributes_ref(ptr),
        unref => |ptr| ffi::gtk_text_attributes_unref(ptr),
    }
}

impl TextAttributes {
    pub fn new() -> TextAttributes {
        unsafe { from_glib_full(ffi::gtk_text_attributes_new()) }
    }

    pub fn copy(&self) -> TextAttributes {
        unsafe {
            from_glib_full(ffi::gtk_text_attributes_copy(mut_override(self.to_glib_none().0)))
        }
    }
}
