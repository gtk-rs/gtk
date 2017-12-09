// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Border(Boxed<ffi::GtkBorder>);

    match fn {
        copy => |ptr| ffi::gtk_border_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_border_free(ptr),
        get_type => || ffi::gtk_border_get_type(),
    }
}

impl Border {
    pub fn new() -> Border {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_border_new())
        }
    }

    pub fn right(&mut self) -> &mut i16 {
        &mut (*self.0).right
    }

    pub fn left(&mut self) -> &mut i16 {
        &mut (*self.0).left
    }

    pub fn top(&mut self) -> &mut i16 {
        &mut (*self.0).top
    }

    pub fn bottom(&mut self) -> &mut i16 {
        &mut (*self.0).bottom
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}
