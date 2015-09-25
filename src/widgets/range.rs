// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/// Base class for widgets which visualize an adjustment

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast};
use super::widget::Widget;
use adjustment::Adjustment;

/// GtkRange — Base class for widgets which visualize an adjustment
pub type Range = Object<ffi::GtkRange>;

unsafe impl Upcast<Widget> for Range { }

unsafe impl Upcast<::Buildable> for Range { }
unsafe impl Upcast<::Orientable> for Range { }

impl types::StaticType for Range {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_range_get_type()) }
    }
}

pub trait RangeExt {
    /// Sets the adjustment to be used as the “model” object for this range widget.
    /// The adjustment indicates the current range value,
    /// the minimum and maximum range values,
    /// the step/page increments used for keybindings and scrolling, and the page size.
    /// The page size is normally 0 for `Scale` and nonzero for `Scrollbar`,
    /// and indicates the size of the visible area of the widget being scrolled.
    /// The page size affects the size of the scrollbar slider.
    fn set_adjustment(&self, adjustment: &Adjustment);
    /// Get the `Adjustment` which is the “model” object for `Range`.
    /// See `set_adjustment()` for details.
    fn get_adjustment(&self) -> Adjustment;
    /// Gets the current value of the range.
    fn get_value(&self) -> f64;
    /// Sets the current value of the range;
    /// if the value is outside the minimum or maximum range values,
    /// it will be clamped to fit inside them.
    /// The range emits the `value-changed` signal if the value changes.
    fn set_value(&self, value: f64);
}

impl<O: Upcast<Range>> RangeExt for O {
    fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_range_set_adjustment(self.upcast().to_glib_none().0,
                adjustment.to_glib_none().0);
        }
    }

    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_range_get_adjustment(self.upcast().to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_range_get_value(self.upcast().to_glib_none().0)
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_range_set_value(self.upcast().to_glib_none().0, value)
        }
    }
}
