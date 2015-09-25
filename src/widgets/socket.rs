// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// GtkSocket — Container for widgets from other processes.
pub type Socket = Object<ffi::GtkSocket>;

unsafe impl Upcast<Widget> for Socket { }
unsafe impl Upcast<::Container> for Socket { }
unsafe impl Upcast<::Buildable> for Socket { }

impl Socket {
    pub fn new() -> Socket {
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

impl types::StaticType for Socket {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_socket_get_type()) }
    }
}
