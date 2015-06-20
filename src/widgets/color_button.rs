// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button to launch a color selection dialog

use glib::translate::*;
use glib::types;
use ffi;
use gdk;
use gdk_ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/**
* ColorButton — A button to launch a color selection dialog
*/
pub type ColorButton = Object<ffi::GtkColorButton>;

unsafe impl Upcast<Widget> for ColorButton { }
unsafe impl Upcast<super::container::Container> for ColorButton { }
unsafe impl Upcast<super::bin::Bin> for ColorButton { }
unsafe impl Upcast<super::button::Button> for ColorButton { }

unsafe impl Upcast<super::actionable::Actionable> for ColorButton { }
unsafe impl Upcast<::builder::Buildable> for ColorButton { }
unsafe impl Upcast<::chooser::color::ColorChooser> for ColorButton {}

impl ColorButton {
    /// Creates a new color button.
    /// This returns a widget in the form of a small button
    /// containing a swatch representing the current selected color.
    /// When the button is clicked, a color-selection dialog will open,
    /// allowing the user to select a color.
    /// The swatch will be updated to reflect the new color when the user finishes.
    pub fn new() -> ColorButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new()).downcast_unchecked()
        }
    }
    /// Creates a new color button.
    /// Deprecated: since 3.4, use `new_with_rgba`
    pub fn new_with_color(color: &gdk::Color) -> ColorButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_color(color))
                .downcast_unchecked()
        }
    }
    /// Creates a new color button.
    pub fn new_with_rgba(rgba: &gdk_ffi::GdkRGBA) -> ColorButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_rgba(rgba))
                .downcast_unchecked()
        }
    }
    /// Sets the current color to be `color`.
    /// Deprecated: since 3.4, use `set_rgba`
    pub fn set_color(&self, color: &gdk::Color) {
        unsafe {
            ffi::gtk_color_button_set_color(self.to_glib_none().0, color)
        }
    }
    /// Sets `color` to be the current color in the ColorButton widget.
    /// Deprecated: since 3.4, use `get_rgba`
    pub fn get_color(&self) -> gdk::Color {
        let color = gdk::Color { pixel: 0, red: 0, green: 0, blue: 0 };
        unsafe {
            ffi::gtk_color_button_get_color(self.to_glib_none().0, &color);
        }
        color
    }
    /// Sets the current opacity to be `alpha`.
    /// Deprecated: since 3.4, use `set_rgba`
    pub fn set_alpha(&self, alpha: u16) {
        unsafe {
            ffi::gtk_color_button_set_alpha(self.to_glib_none().0, alpha)
        }
    }
    /// Returns the current alpha value.
    /// Deprecated: since 3.4, use `get_rgba`
    pub fn get_alpha(&self) -> u16 {
        unsafe {
            ffi::gtk_color_button_get_alpha(self.to_glib_none().0)
        }
    }
    /// Sets the title for the color selection dialog.
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_color_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }
    /// Gets the title of the color selection dialog.
    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_color_button_get_title(self.to_glib_none().0))
        }
    }
}

impl types::StaticType for ColorButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_color_button_get_type()) }
    }
}
