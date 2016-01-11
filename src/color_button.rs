// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ColorButton;
use Widget;
use ffi;
use gdk;
use glib::object::Downcast;
use glib::translate::*;
use std::mem;

impl ColorButton {
    pub fn new_with_color(color: &gdk::Color) -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_color(color)).downcast_unchecked()
        }
    }

    pub fn new_with_rgba(rgba: &gdk::RGBA) -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_rgba(rgba)).downcast_unchecked()
        }
    }

    pub fn get_color(&self) -> gdk::Color {
        unsafe {
            let mut color = mem::uninitialized();
            ffi::gtk_color_button_get_color(self.to_glib_none().0, &mut color);
            color
        }
    }

    pub fn get_rgba(&self) -> gdk::RGBA {
        unsafe { 
            let mut rgba = mem::uninitialized();
            ffi::gtk_color_button_get_rgba(self.to_glib_none().0, &mut rgba);
            rgba
        }
    }

    pub fn set_color(&self, color: &gdk::Color) {
        unsafe { ffi::gtk_color_button_set_color(self.to_glib_none().0, color) }
    }

    pub fn set_rgba(&self, rgba: &gdk::RGBA) {
        unsafe { ffi::gtk_color_button_set_rgba(self.to_glib_none().0, rgba) }
    }
}
