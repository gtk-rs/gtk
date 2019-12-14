// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::mem;
use ColorButton;
use Widget;

pub trait ColorButtonExtManual: 'static {
    fn new_with_color(color: &gdk::Color) -> ColorButton;

    fn get_color(&self) -> gdk::Color;

    fn set_color(&self, color: &gdk::Color);
}

impl<O: IsA<ColorButton>> ColorButtonExtManual for O {
    fn new_with_color(color: &gdk::Color) -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_color_button_new_with_color(color)).unsafe_cast()
        }
    }

    fn get_color(&self) -> gdk::Color {
        unsafe {
            let mut color = mem::MaybeUninit::uninit();
            gtk_sys::gtk_color_button_get_color(self.as_ref().to_glib_none().0, color.as_mut_ptr());
            color.assume_init()
        }
    }

    fn set_color(&self, color: &gdk::Color) {
        unsafe { gtk_sys::gtk_color_button_set_color(self.as_ref().to_glib_none().0, color) }
    }
}
