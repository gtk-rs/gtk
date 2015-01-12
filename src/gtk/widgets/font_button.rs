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

//! A button to launch a font chooser dialog

use std::ffi::CString;
use gtk::{self, ffi};
use gtk::cast::GTK_FONTBUTTON;

/// FontButton — A button to launch a font chooser dialog
/*
* # Availables signals :
* * `font-set` : Run First
*/
struct_Widget!(FontButton);

impl FontButton {
    pub fn new() -> Option<FontButton> {
        let tmp_pointer = unsafe { ffi::gtk_font_button_new() };
        check_pointer!(tmp_pointer, FontButton)
    }

    pub fn new_with_font(font_name: &str) -> Option<FontButton> {
        let tmp_pointer = unsafe {
            font_name.with_c_str(|c_str| {
                ffi::gtk_font_button_new_with_font(c_str)
            })
        };
        check_pointer!(tmp_pointer, FontButton)
    }

    pub fn set_font_name(&mut self, font_name: &str) -> bool {
        match unsafe { font_name.with_c_str(|c_str| { ffi::gtk_font_button_set_font_name(GTK_FONTBUTTON(self.pointer), c_str) }) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn get_font_name(&self) -> String {
        let c_str = unsafe { ffi::gtk_font_button_get_font_name(GTK_FONTBUTTON(self.pointer)) };
        unsafe { String::from_utf8(c_str as *const u8) }
    }

    pub fn set_show_style(&mut self, show_style: bool) -> () {
        match show_style {
            true    => unsafe { ffi::gtk_font_button_set_show_style(GTK_FONTBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_font_button_set_show_style(GTK_FONTBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_show_style(&self) -> bool {
        match unsafe { ffi::gtk_font_button_get_show_style(GTK_FONTBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_show_size(&mut self, show_size: bool) -> () {
        match show_size {
            true    => unsafe { ffi::gtk_font_button_set_show_size(GTK_FONTBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_font_button_set_show_size(GTK_FONTBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_show_size(&self) -> bool {
        match unsafe { ffi::gtk_font_button_get_show_size(GTK_FONTBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_use_font(&mut self, use_font: bool) -> () {
        match use_font {
            true    => unsafe { ffi::gtk_font_button_set_use_font(GTK_FONTBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_font_button_set_use_font(GTK_FONTBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_use_font(&self) -> bool {
        match unsafe { ffi::gtk_font_button_get_use_font(GTK_FONTBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_use_size(&mut self, use_size: bool) -> () {
        match use_size {
            true    => unsafe { ffi::gtk_font_button_set_use_size(GTK_FONTBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_font_button_set_use_font(GTK_FONTBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_use_size(&self) -> bool {
        match unsafe { ffi::gtk_font_button_get_use_size(GTK_FONTBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_title(&mut self, title: &str) -> () {
        unsafe {
            title.with_c_str(|c_str| {
                ffi::gtk_font_button_set_title(GTK_FONTBUTTON(self.pointer), c_str)
            });
        }
    }

    pub fn get_title(&self) -> String {
        let c_str = unsafe { ffi::gtk_font_button_get_title(GTK_FONTBUTTON(self.pointer)) };
        unsafe { String::from_utf8(c_str as *const u8) }
    }
}

impl_drop!(FontButton);
impl_TraitWidget!(FontButton);

impl gtk::ContainerTrait for FontButton {}
impl gtk::ButtonTrait for FontButton {}

impl_widget_events!(FontButton);
impl_button_events!(FontButton);
