// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button to launch a color selection dialog

use glib::translate::*;
use glib::types;
use ffi;
use gdk_ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/**
* ColorButton — A button to launch a color selection dialog
*/
pub type ColorButton = Object<ffi::GtkColorButton>;

unsafe impl Upcast<Widget> for ColorButton { }
unsafe impl Upcast<::Container> for ColorButton { }
unsafe impl Upcast<::Bin> for ColorButton { }
unsafe impl Upcast<::Button> for ColorButton { }

unsafe impl Upcast<::Actionable> for ColorButton { }
unsafe impl Upcast<::Buildable> for ColorButton { }
unsafe impl Upcast<::ColorChooser> for ColorButton {}

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
    pub fn new_with_rgba(rgba: &gdk_ffi::GdkRGBA) -> ColorButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_rgba(rgba))
                .downcast_unchecked()
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
