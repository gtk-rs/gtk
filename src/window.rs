// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


use glib::translate::*;
use gdk;
use ffi;

use glib::object::{Downcast, Upcast};
use {
    Widget,
    WindowPosition,
    WindowType,
};

glib_wrapper! {
    pub struct Window(Object<ffi::GtkWindow>): ::Widget, ::Container, ::Bin, ::Buildable;

    match fn {
        get_type => || ffi::gtk_window_get_type(),
    }
}

impl Window {
    pub fn new(window_type: WindowType) -> Window {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_window_new(window_type)).downcast_unchecked() }
    }
}

pub trait WindowExt {
    fn move_(&self, x: i32, y: i32);
    fn set_type_hint(&self, hint: gdk::WindowTypeHint);
    fn set_title(&self, title: &str);
    fn set_decorated(&self, setting: bool);
    fn get_title(&self) -> Option<String>;
    fn set_default_size(&self, width: i32, height: i32);
    fn set_window_position(&self, window_position: WindowPosition);
    #[cfg(gtk_3_10)]
    fn set_titlebar<T: Upcast<Widget>>(&self, titlebar: &T);
    fn set_transient_for<T: Upcast<Window> = Window>(&self, parent: Option<&T>);
}

impl<O: Upcast<Window>> WindowExt for O {
    fn move_(&self, x: i32, y: i32) {
        unsafe {
            ffi::gtk_window_move(self.to_glib_none().0, x, y);
        }
    }

    fn set_type_hint(&self, hint: gdk::WindowTypeHint) {
        unsafe {
            ffi::gtk_window_set_type_hint(self.to_glib_none().0, hint);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_window_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_decorated(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_decorated(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_title(self.to_glib_none().0))
        }
    }

    fn set_default_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.to_glib_none().0, width, height)
        }
    }

    fn set_window_position(&self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.to_glib_none().0, window_position);
        }
    }

    #[cfg(gtk_3_10)]
    fn set_titlebar<T: Upcast<Widget>>(&self, titlebar: &T) {
        unsafe {
            ffi::gtk_window_set_titlebar(self.to_glib_none().0,
                titlebar.to_glib_none().0);
        }
    }

    fn set_transient_for<T: Upcast<Window>>(&self, parent: Option<&T>) {
        unsafe {
            ffi::gtk_window_set_transient_for(self.to_glib_none().0, parent.to_glib_none().0);
        }
    }
}
