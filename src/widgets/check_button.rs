// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Create widgets with a discrete toggle button

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// CheckButton — Create widgets with a discrete toggle button
pub type CheckButton = Object<ffi::GtkCheckButton>;

impl CheckButton {
    /// Creates a new `CheckButton`.
    pub fn new() -> CheckButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new()).downcast_unchecked()
        }
    }
    /// Creates a new `CheckButton` with a `Label` to the right of it.
    pub fn new_with_label(label: &str) -> CheckButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_check_button_new_with_label(label.to_glib_none().0))
                    .downcast_unchecked()
        }
    }
    /// Creates a new `CheckButton` containing a label.
    /// The label will be created using `Label::new_with_mnemonic()`,
    /// so underscores in `label` indicate the mnemonic for the check button.
    pub fn new_with_mnemonic(mnemonic: &str) -> CheckButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_check_button_new_with_mnemonic(mnemonic.to_glib_none().0))
                    .downcast_unchecked()
        }
    }
}

impl types::StaticType for CheckButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_check_button_get_type()) }
    }
}

unsafe impl Upcast<Widget> for CheckButton { }
unsafe impl Upcast<super::container::Container> for CheckButton { }
unsafe impl Upcast<super::bin::Bin> for CheckButton { }
unsafe impl Upcast<super::button::Button> for CheckButton { }
unsafe impl Upcast<super::toggle_button::ToggleButton> for CheckButton { }
unsafe impl Upcast<::builder::Buildable> for CheckButton { }
unsafe impl Upcast<super::actionable::Actionable> for CheckButton { }
