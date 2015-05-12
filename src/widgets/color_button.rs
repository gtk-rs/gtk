// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button to launch a color selection dialog

use glib::translate::{from_glib_none, ToGlibPtr};
use cast::GTK_COLORBUTTON;
use ffi;
use glib::{to_bool, to_gboolean};
use gdk;
use gdk_ffi;

/**
* ColorButton — A button to launch a color selection dialog
*
* # Availables signals :
* * `color-set` : Run First
*/
struct_Widget!(ColorButton);

impl ColorButton {
    pub fn new() -> Option<ColorButton> {
        let tmp_pointer = unsafe { ffi::gtk_color_button_new() };
        check_pointer!(tmp_pointer, ColorButton)
    }

    pub fn new_with_color(color: &gdk::Color) -> Option<ColorButton> {
        let tmp_pointer = unsafe { ffi::gtk_color_button_new_with_color(color) };
        check_pointer!(tmp_pointer, ColorButton)
    }

    pub fn new_with_rgba(rgba: &gdk_ffi::GdkRGBA) -> Option<ColorButton> {
        let tmp_pointer = unsafe { ffi::gtk_color_button_new_with_rgba(rgba) };
        check_pointer!(tmp_pointer, ColorButton)
    }

    pub fn set_color(&self, color: &gdk::Color) -> () {
        unsafe {
            ffi::gtk_color_button_set_color(GTK_COLORBUTTON(self.pointer), color)
        }
    }

    pub fn get_color(&self) -> gdk::Color {
        let color = gdk::Color { pixel: 0, red: 0, green: 0, blue: 0 };
        unsafe {
            ffi::gtk_color_button_get_color(GTK_COLORBUTTON(self.pointer), &color);
        }
        color
    }

    pub fn set_alpha(&self, alpha: u16) -> () {
        unsafe {
            ffi::gtk_color_button_set_alpha(GTK_COLORBUTTON(self.pointer), alpha)
        }
    }

    pub fn get_alpha(&self) -> u16 {
        unsafe {
            ffi::gtk_color_button_get_alpha(GTK_COLORBUTTON(self.pointer))
        }
    }

    pub fn set_rgba(&self, rgba: &gdk_ffi::GdkRGBA) -> () {
        unsafe {
            ffi::gtk_color_button_set_rgba(GTK_COLORBUTTON(self.pointer), rgba)
        }
    }

    pub fn get_rgba(&self) -> gdk_ffi::GdkRGBA {
        let rgba = gdk_ffi::GdkRGBA { red: 0., green: 0., blue: 0., alpha: 0. };
        unsafe {
            ffi::gtk_color_button_get_rgba(GTK_COLORBUTTON(self.pointer), &rgba);
        }
        rgba
    }

    pub fn set_use_alpha(&self, use_alpha: bool) -> () {
        unsafe { ffi::gtk_color_button_set_use_alpha(GTK_COLORBUTTON(self.pointer), to_gboolean(use_alpha)); }
    }

    pub fn get_use_alpha(&self) -> bool {
        unsafe { to_bool(ffi::gtk_color_button_get_use_alpha(GTK_COLORBUTTON(self.pointer))) }
    }

    pub fn set_title(&self, title: &str) -> () {
        unsafe {
            ffi::gtk_color_button_set_title(GTK_COLORBUTTON(self.pointer), title.to_glib_none().0);
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_color_button_get_title(GTK_COLORBUTTON(self.pointer)))
        }
    }
}

impl_drop!(ColorButton);
impl_TraitWidget!(ColorButton);

impl ::ContainerTrait for ColorButton {}
impl ::ButtonTrait for ColorButton {}
