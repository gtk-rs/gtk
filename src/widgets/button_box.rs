// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container for arranging buttons

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast, Downcast};
use super::widget::Widget;
use {
    Orientation,
    ButtonBoxStyle,
};

/// ButtonBox — A container for arranging buttons
pub type ButtonBox = Object<ffi::GtkButtonBox>;

unsafe impl Upcast<Widget> for ButtonBox { }
unsafe impl Upcast<super::container::Container> for ButtonBox { }
unsafe impl Upcast<::builder::Buildable> for ButtonBox { }
unsafe impl Upcast<super::orientable::Orientable> for ButtonBox { }

impl types::StaticType for ButtonBox {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_button_box_get_type()) }
    }
}

impl ButtonBox {
    /// Creates a new ButtonBox.
    pub fn new(orientation: Orientation) -> ButtonBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_box_new(orientation)).downcast_unchecked()
        }
    }
    /// Retrieves the method being used to arrange the buttons in a button box.
    pub fn get_layout(&self) -> ButtonBoxStyle {
        unsafe { ffi::gtk_button_box_get_layout(self.to_glib_none().0) }
    }
    /// Returns whether `child` should appear in a secondary group of children.
    pub fn get_child_secondary<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_button_box_get_child_secondary(self.to_glib_none().0,
                    child.upcast().to_glib_none().0))
        }
    }
    /// Returns whether the `child` is exempted from homogenous sizing.
    pub fn get_child_non_homogeneous<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_button_box_get_child_non_homogeneous(self.to_glib_none().0,
                    child.upcast().to_glib_none().0))
        }
    }
    /// Changes the way buttons are arranged in their container.
    pub fn set_layout(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            ffi::gtk_button_box_set_layout(self.to_glib_none().0, layout_style)
        }
    }
    /// Sets whether child should appear in a secondary group of children.
    /// A typical use of a secondary child is the help button in a dialog.
    /// This group appears after the other children
    /// if the style is `ButtonBoxStyle::Start`, `ButtonBoxStyle::Spread` or `ButtonBoxStyle::Edge`,
    /// and before the other children if the style is `ButtonBoxStyle::End`.
    /// For horizontal button boxes, the definition of before/after depends
    /// on direction of the widget (see gtk_widget_set_direction()).
    /// If the style is `ButtonBoxStyle::Start` or `ButtonBoxStyle::End`,
    /// then the secondary children are aligned at the other end of the button box
    /// from the main children.
    /// For the other styles, they appear immediately next to the main children.
    pub fn set_child_secondary<T: Upcast<Widget>>(&self, child: &T, is_secondary: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_secondary(self.to_glib_none().0,
                child.upcast().to_glib_none().0, is_secondary.to_glib());
        }
    }
    /// Sets whether the child is exempted from homogeous sizing.
    pub fn set_child_non_homogeneous<T: Upcast<Widget>>(&self, child: &T, non_homogeneous: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_non_homogeneous(self.to_glib_none().0,
                child.upcast().to_glib_none().0, non_homogeneous.to_glib());
        }
    }
}
