// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A choice from multiple check buttons

use std::ptr;
use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// A choice from multiple check buttons
pub type RadioButton = Object<ffi::GtkRadioButton>;

unsafe impl Upcast<Widget> for RadioButton { }
unsafe impl Upcast<::Container> for RadioButton { }
unsafe impl Upcast<::Bin> for RadioButton { }
unsafe impl Upcast<::Button> for RadioButton { }
unsafe impl Upcast<::ToggleButton> for RadioButton { }

unsafe impl Upcast<::Actionable> for RadioButton { }
unsafe impl Upcast<::Buildable> for RadioButton { }

impl RadioButton {
    /// Creates a new `RadioButton`.
    pub fn new() -> RadioButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new(ptr::null_mut()))
                .downcast_unchecked()
        }
    }
    /// Creates a new `RadioButton` with a text label.
    pub fn new_with_label(label: &str) -> RadioButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_radio_button_new_with_label(ptr::null_mut(), label.to_glib_none().0))
                .downcast_unchecked()
        }
    }
    /// Creates a new `RadioButton` containing a label.
    /// The label will be created using `Label::new_with_mnemonic()`,
    /// so underscores in `label` indicate the mnemonic for the button.
    pub fn new_with_mnemonic(label: &str) -> RadioButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_check_button_new_with_mnemonic(label.to_glib_none().0))
                .downcast_unchecked()
        }
    }
    /// Joins a `RadioButton` object to the group of another `RadioButton` object
    pub fn join_group(&self, group_source: &RadioButton) {
        unsafe {
            ffi::gtk_radio_button_join_group(self.to_glib_none().0,
                group_source.to_glib_none().0)
        }
    }
}

impl types::StaticType for RadioButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_radio_button_get_type()) }
    }
}
