// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A button to launch a color selection dialog

use std::{ptr, str};
use std::num::cast;
use libc::{c_void};

use traits::{GtkWidget, GtkButton, GtkContainer, Signal};
use utils::cast::GTK_COLORBUTTON;
use ffi;
use std;
use std::owned;
use gdk;

/** 
* ColorButton — A button to launch a color selection dialog
*
* # Availables signals :
* * `color-set` : Run First
*/
pub struct ColorButton {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl ColorButton {
    pub fn new() -> Option<ColorButton> {
        let tmp_pointer = unsafe { ffi::gtk_color_button_new() };
        check_pointer!(tmp_pointer, ColorButton)
    }

    pub fn new_with_color(color: &gdk::Color) -> Option<ColorButton> {
        let tmp_pointer = unsafe { ffi::gtk_color_button_new_with_color(color) };
        check_pointer!(tmp_pointer, ColorButton)
    }

    pub fn new_with_rgba(rgba: &gdk::RGBA) -> Option<ColorButton> {
        let tmp_pointer = unsafe { ffi::gtk_color_button_new_with_rgba(rgba) };
        check_pointer!(tmp_pointer, ColorButton)
    }

    pub fn set_color(&mut self, color: &gdk::Color) -> () {
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

    pub fn set_alpha(&mut self, alpha: u16) -> () {
        unsafe {
            ffi::gtk_color_button_set_alpha(GTK_COLORBUTTON(self.pointer), alpha)
        }
    }

    pub fn get_alpha(&self) -> u16 {
        unsafe {
            ffi::gtk_color_button_get_alpha(GTK_COLORBUTTON(self.pointer))
        }
    }

    pub fn set_rgba(&mut self, rgba: &gdk::RGBA) -> () {
        unsafe {
            ffi::gtk_color_button_set_rgba(GTK_COLORBUTTON(self.pointer), rgba)
        }
    }

    pub fn get_rgba(&self) -> gdk::RGBA {
        let rgba = gdk::RGBA { red: 0., green: 0., blue: 0., alpha: 0. };
        unsafe {
            ffi::gtk_color_button_get_rgba(GTK_COLORBUTTON(self.pointer), &rgba);
        }
        rgba
    }

    pub fn set_use_alpha(&mut self, use_alpha: bool) -> () {
        match use_alpha {
            true    => unsafe { ffi::gtk_color_button_set_use_alpha(GTK_COLORBUTTON(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_color_button_set_use_alpha(GTK_COLORBUTTON(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_use_alpha(&self) -> bool {
        match unsafe { ffi::gtk_color_button_get_use_alpha(GTK_COLORBUTTON(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_title(&mut self, title: &str) -> () {
        unsafe {
            title.with_c_str(|c_str| {
                ffi::gtk_color_button_set_title(GTK_COLORBUTTON(self.pointer), c_str) 
            });
        }
    }

    pub fn get_title(&self) -> String {
        let c_str = unsafe { ffi::gtk_color_button_get_title(GTK_COLORBUTTON(self.pointer)) };
        unsafe { str::raw::from_c_str(c_str) }
    }
}

impl_GtkWidget!(ColorButton)
redirect_callback!(ColorButton)
redirect_callback_widget!(ColorButton)
struct_signal!(ColorButton)
impl_signals!(ColorButton)

impl GtkContainer for ColorButton {}
impl GtkButton for ColorButton {}

