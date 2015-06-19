// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Create buttons which retain their state

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// ToggleButton — Create buttons which retain their state
pub type ToggleButton = Object<ffi::GtkToggleButton>;

impl ToggleButton {
    /// Creates a new toggle button. A widget should be packed into the button,
    /// as in `Button::new()`.
    pub fn new() -> ToggleButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new()).downcast_unchecked()
        }
    }
    /// Creates a new toggle button with a text label.
    pub fn new_with_label(label: &str) -> ToggleButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_toggle_button_new_with_label(label.to_glib_none().0))
                    .downcast_unchecked()
        }
    }
    /// Creates a new `ToggleButton` containing a label.
    /// The label will be created using `Label::new_with_mnemonic()`,
    /// so underscores in label indicate the mnemonic for the button.
    pub fn new_with_mnemonic(mnemonic: &str) -> ToggleButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_toggle_button_new_with_mnemonic(mnemonic.to_glib_none().0))
                    .downcast_unchecked()
        }
    }
}

impl types::StaticType for ToggleButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_toggle_button_get_type()) }
    }
}

unsafe impl Upcast<Widget> for ToggleButton { }
unsafe impl Upcast<super::container::Container> for ToggleButton { }
unsafe impl Upcast<super::button::Button> for ToggleButton { }
unsafe impl Upcast<super::bin::Bin> for ToggleButton { }
unsafe impl Upcast<::builder::Buildable> for ToggleButton { }
unsafe impl Upcast<super::actionable::Actionable> for ToggleButton { }

pub trait ToggleButtonExt{
    /// Sets whether the button is displayed as a separate indicator and label.
    /// You can call this function on a checkbutton or a radiobutton
    /// with `draw_indicator` = `FALSE` to make the button look like a normal button
    /// This function only affects instances of classes like `CheckButton` and `RadioButton`
    /// that derive from `ToggleButton`, not instances of `ToggleButton` itself.
    fn set_mode(&self, draw_indicate: bool);
    /// Retrieves whether the button is displayed as a separate indicator and label.
    /// See `set_mode()`.
    fn get_mode(&self) -> bool;
    /// Emits the `toggled` signal on the `ToggleButton`.
    /// There is no good reason for an application ever to call this function.
    fn toggled(&self);
    /// Sets the status of the toggle button.
    /// Set to `TRUE` if you want the GtkToggleButton to be “pressed in”,
    /// and `FALSE` to raise it.
    /// This action causes the `toggled` signal and the `clicked` signal to be emitted.
    fn set_active(&self, is_active: bool);
    /// Queries a `ToggleButton` and returns its current state.
    /// Returns `TRUE` if the toggle button is pressed in and `FALSE` if it is raised.
    fn get_active(&self) -> bool;
    /// If the user has selected a range of elements (such as some text or spreadsheet cells)
    /// that are affected by a toggle button, and the current values in that range are inconsistent,
    /// you may want to display the toggle in an “in between” state.
    /// This function turns on “in between” display.
    /// Normally you would turn off the inconsistent state again
    /// if the user toggles the toggle button.
    /// This has to be done manually, `set_inconsistent()` only affects visual appearance,
    /// it doesn’t affect the semantics of the button.
    fn set_inconsistent(&self, setting: bool);
    /// Gets the value set by `set_inconsistent()`.
    fn get_inconsistent(&self) -> bool;
}

impl<O: Upcast<ToggleButton>> ToggleButtonExt for O {
    fn set_mode(&self, draw_indicate: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_mode(self.upcast().to_glib_none().0,
                draw_indicate.to_glib());
        }
    }

    fn get_mode(&self) -> bool {
        unsafe { from_glib(ffi::gtk_toggle_button_get_mode(self.upcast().to_glib_none().0)) }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_toggle_button_toggled(self.upcast().to_glib_none().0)
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_active(self.upcast().to_glib_none().0,
                is_active.to_glib());
        }
    }

    fn get_active(&self) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_toggle_button_get_active(self.upcast().to_glib_none().0))
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_inconsistent(self.upcast().to_glib_none().0,
                setting.to_glib());
        }
    }

    fn get_inconsistent(&self) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_toggle_button_get_inconsistent(self.upcast().to_glib_none().0))
        }
    }
}
