// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button to launch a font chooser dialog

use glib::translate::{FromGlibPtr, ToGlibPtr};
use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_FONTBUTTON;

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
            ffi::gtk_font_button_new_with_font(font_name.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, FontButton)
    }

    pub fn set_font_name(&self, font_name: &str) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_set_font_name(GTK_FONTBUTTON(self.pointer), font_name.borrow_to_glib().0)) }
    }

    pub fn get_font_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_font_button_get_font_name(GTK_FONTBUTTON(self.pointer)))
        }
    }

    pub fn set_show_style(&self, show_style: bool) -> () {
        unsafe { ffi::gtk_font_button_set_show_style(GTK_FONTBUTTON(self.pointer), to_gboolean(show_style)); }
    }

    pub fn get_show_style(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_show_style(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_show_size(&self, show_size: bool) -> () {
        unsafe { ffi::gtk_font_button_set_show_size(GTK_FONTBUTTON(self.pointer), to_gboolean(show_size)); }
    }

    pub fn get_show_size(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_show_size(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_use_font(&self, use_font: bool) -> () {
        unsafe { ffi::gtk_font_button_set_use_font(GTK_FONTBUTTON(self.pointer), to_gboolean(use_font)); }
    }

    pub fn get_use_font(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_use_font(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_use_size(&self, use_size: bool) -> () {
        unsafe { ffi::gtk_font_button_set_use_size(GTK_FONTBUTTON(self.pointer), to_gboolean(use_size)); }
    }

    pub fn get_use_size(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_use_size(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_title(&self, title: &str) -> () {
        unsafe {
            ffi::gtk_font_button_set_title(GTK_FONTBUTTON(self.pointer), title.borrow_to_glib().0);
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_font_button_get_title(GTK_FONTBUTTON(self.pointer)))
        }
    }
}

impl_drop!(FontButton);
impl_TraitWidget!(FontButton);

impl ::ContainerTrait for FontButton {}
impl ::ButtonTrait for FontButton {}

impl_widget_events!(FontButton);
impl_button_events!(FontButton);
