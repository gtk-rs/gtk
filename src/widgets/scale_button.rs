// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button which pops up a scale

use std::ptr;
use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;
use Adjustment;

/// ScaleButton — A button which pops up a scale
pub type ScaleButton = Object<ffi::GtkScaleButton>;

unsafe impl Upcast<Widget> for ScaleButton { }
unsafe impl Upcast<super::container::Container> for ScaleButton { }
unsafe impl Upcast<super::bin::Bin> for ScaleButton { }
unsafe impl Upcast<super::button::Button> for ScaleButton { }

unsafe impl Upcast<super::actionable::Actionable> for ScaleButton { }
unsafe impl Upcast<::builder::Buildable> for ScaleButton { }
unsafe impl Upcast<super::orientable::Orientable> for ScaleButton { }

impl ScaleButton {
    /// Creates a GtkScaleButton, with a range between `min` and `max`,
    /// with a stepping of `step`.
    // FIXME: icons -> last parameter
    pub fn new(size: i32, min: f64, max: f64, step: f64) -> ScaleButton {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_scale_button_new(size, min, max, step, ptr::null_mut()))
                .downcast_unchecked()
        }
    }
}

impl types::StaticType for ScaleButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_scale_button_get_type()) }
    }
}

pub trait ScaleButtonExt {
    /// Sets the `Adjustment` to be used as a model for the ScaleButton’s scale.
    /// See `Range::set_adjustment()` for details.
    fn set_adjustment(&self, adjustment: &Adjustment);
    /// Sets the current value of the scale;
    /// if the value is outside the minimum or maximum range values,
    /// it will be clamped to fit inside them.
    /// The scale button emits the `value-changed` signal if the value changes.
    fn set_value(&self, value: f64);
    /// Gets the current value of the scale button.
    fn get_value(&self) -> f64;
    /// Gets the `Adjustment` associated with the ScaleButton’s scale.
    /// See `Range::get_adjustment()` for details.
    fn get_adjustment(&self) -> Adjustment;
}

impl<O: Upcast<ScaleButton>> ScaleButtonExt for O {
    fn set_adjustment(&self, adjustment: &Adjustment) -> () {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(self.upcast().to_glib_none().0,
                adjustment.to_glib_none().0);
        }
    }

    fn set_value(&self, value: f64) -> () {
        unsafe {
            ffi::gtk_scale_button_set_value(self.upcast().to_glib_none().0, value);
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(self.upcast().to_glib_none().0)
        }
    }

    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_adjustment(self.upcast().to_glib_none().0))
        }
    }
}
