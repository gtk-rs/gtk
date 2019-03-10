// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::Cast;
use glib::translate::*;
use gtk_sys;
use Buildable;
use Container;
use Widget;

glib_wrapper! {
    pub struct Socket(Object<gtk_sys::GtkSocket, gtk_sys::GtkSocketClass, SocketClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_socket_get_type(),
    }
}

impl Socket {
    pub fn new() -> Socket {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_socket_new()).unsafe_cast() }
    }

    /*pub fn add_id(&self, window: Window) {
        unsafe { gtk_sys::gtk_socket_add_id(self.to_glib_none().0, window) };
    }

    pub fn get_id(&self) -> Window {
        unsafe { gtk_sys::gtk_socket_get_id(self.to_glib_none().0) };
    }

    pub fn get_plug_window(&self) -> GdkWindow {
        let tmp_pointer = unsafe { gtk_sys::gtk_socket_get_plug_window(self.to_glib_none().0) };

        // add end of code
    }*/
}
