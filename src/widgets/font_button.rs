// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button to launch a font chooser dialog

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// FontButton — A button to launch a font chooser dialog
pub type FontButton = Object<ffi::GtkFontButton>;

unsafe impl Upcast<Widget> for FontButton { }
unsafe impl Upcast<::Container> for FontButton { }
unsafe impl Upcast<::Bin> for FontButton { }
unsafe impl Upcast<::Button> for FontButton { }

unsafe impl Upcast<::Actionable> for FontButton { }
unsafe impl Upcast<::Buildable> for FontButton { }
unsafe impl Upcast<::FontChooser> for FontButton {}

impl FontButton {
    /// Creates a new font picker widget.
    pub fn new() -> FontButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_button_new()).downcast_unchecked()
        }
    }
    /// Creates a new font picker widget.
    pub fn new_with_font(font_name: &str) -> FontButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_font_button_new_with_font(font_name.to_glib_none().0))
                .downcast_unchecked()
        }
    }
    /// Sets or updates the currently-displayed font in font picker dialog.
    pub fn set_font_name(&self, font_name: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_font_button_set_font_name(self.to_glib_none().0,
                    font_name.to_glib_none().0))
        }
    }
    /// Retrieves the name of the currently selected font.
    /// This name includes style and size information as well.
    /// If you want to render something with the font,
    /// use this string with `pango_font_description_from_string()`.
    /// If you’re interested in peeking certain values (family name, style, size, weight)
    /// just query these properties from the PangoFontDescription object.
    pub fn get_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_font_button_get_font_name(self.to_glib_none().0))
        }
    }
    /// If `show_style` is `TRUE`,
    /// the font style will be displayed along with name of the selected font.
    pub fn set_show_style(&self, show_style: bool) {
        unsafe {
            ffi::gtk_font_button_set_show_style(self.to_glib_none().0, show_style.to_glib());
        }
    }
    /// Returns whether the name of the font style will be shown in the label.
    pub fn get_show_style(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_show_style(self.to_glib_none().0))
        }
    }
    /// If `show_size` is `TRUE`, the font size will be displayed along
    /// with the name of the selected font.
    pub fn set_show_size(&self, show_size: bool) {
        unsafe {
            ffi::gtk_font_button_set_show_size(self.to_glib_none().0, show_size.to_glib());
        }
    }
    /// Returns whether the font size will be shown in the label.
    pub fn get_show_size(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_button_get_show_size(self.to_glib_none().0)) }
    }
    /// If `use_font` is `TRUE`, the font name will be written using the selected font.
    pub fn set_use_font(&self, use_font: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_font(self.to_glib_none().0, use_font.to_glib());
        }
    }
    /// Returns whether the selected font is used in the label.
    pub fn get_use_font(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_button_get_use_font(self.to_glib_none().0)) }
    }
    /// If `use_size` is `TRUE`, the font name will be written using the selected size.
    pub fn set_use_size(&self, use_size: bool) {
        unsafe { ffi::gtk_font_button_set_use_size(self.to_glib_none().0, use_size.to_glib()); }
    }
    /// Returns whether the selected size is used in the label.
    pub fn get_use_size(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_button_get_use_size(self.to_glib_none().0)) }
    }
    /// Sets the title for the font chooser dialog.
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_font_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }
    /// Retrieves the title of the font chooser dialog.
    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_font_button_get_title(self.to_glib_none().0))
        }
    }
}

impl types::StaticType for FontButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_font_button_get_type()) }
    }
}
