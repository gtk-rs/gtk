// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

use glib::object::Downcast;
use Widget;

glib_wrapper! {
    pub struct Socket(Object<ffi::GtkSocket>): Widget, ::Container;

    match fn {
        get_type => || ffi::gtk_socket_get_type(),
    }
}

impl Socket {
    pub fn new() -> Socket {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_socket_new()).downcast_unchecked() }
    }

    /*pub fn add_id(&self, window: Window) {
        unsafe { ffi::gtk_socket_add_id(self.to_glib_none().0, window) };
    }

    pub fn get_id(&self) -> Window {
        unsafe { ffi::gtk_socket_get_id(self.to_glib_none().0) };
    }

    pub fn get_plug_window(&self) -> GdkWindow {
        let tmp_pointer = unsafe { ffi::gtk_socket_get_plug_window(self.to_glib_none().0) };

        // add end of code
    }*/
}
