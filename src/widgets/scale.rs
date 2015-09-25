// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A slider widget for selecting a value from a range

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;
use Adjustment;
use Orientation;
use PositionType;

/// Scale — A slider widget for selecting a value from a range
pub type Scale = Object<ffi::GtkScale>;

unsafe impl Upcast<Widget> for Scale { }
unsafe impl Upcast<::Range> for Scale { }

unsafe impl Upcast<::Buildable> for Scale { }
unsafe impl Upcast<::Orientable> for Scale { }

impl Scale {
    /// Creates a new `Scale`.
    pub fn new(orientation: Orientation, adjustment: Option<&Adjustment>) -> Scale {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_scale_new(orientation, adjustment.to_glib_none().0))
                .downcast_unchecked()
        }
    }
    /// Creates a new scale widget with the given orientation
    /// that lets the user input a number between `min` and `max` (including `min` and `max`)
    /// with the increment `step` . `step` must be nonzero;
    /// it’s the distance the slider moves when using the arrow keys to adjust the scale value.
    /// Note that the way in which the precision is derived works best if step is a power of ten.
    /// If the resulting precision is not suitable for your needs,
    /// use `set_digits()` to correct it.
    pub fn new_with_range(orientation: Orientation,
                          min: f64,
                          max: f64,
                          step: f64) -> Scale {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_scale_new_with_range(orientation, min, max, step))
                .downcast_unchecked()
        }
    }
    /// Sets the number of decimal places that are displayed in the value.
    /// Also causes the value of the adjustment to be rounded off to this number of digits,
    /// so the retrieved value matches the value the user saw.
    pub fn set_digits(&self, digits: i32) {
        unsafe {
            ffi::gtk_scale_set_digits(self.to_glib_none().0, digits)
        }
    }
    /// Specifies whether the current value is displayed as a string next to the slider.
    pub fn set_draw_value(&self, draw_value: bool) {
        unsafe { ffi::gtk_scale_set_draw_value(self.to_glib_none().0, draw_value.to_glib()) }
    }
    /// Returns whether the current value is displayed as a string next to the slider.
    pub fn get_draw_value(&self) -> bool {
        unsafe { from_glib(ffi::gtk_scale_get_draw_value(self.to_glib_none().0)) }
    }
    /// If `has_origin` is set to `TRUE` (the default),
    /// the scale will highlight the part of the scale between the origin (bottom or left side) of the scale and the current value.
    pub fn set_has_origin(&self, has_origin: bool) {
        unsafe { ffi::gtk_scale_set_has_origin(self.to_glib_none().0, has_origin.to_glib()) }
    }
    /// Returns whether the scale has an origin.
    pub fn get_has_origin(&self) -> bool {
        unsafe { from_glib(ffi::gtk_scale_get_has_origin(self.to_glib_none().0)) }
    }
    /// Sets the position in which the current value is displayed.
    pub fn set_value_pos(&self, position: PositionType) {
        unsafe {
            ffi::gtk_scale_set_value_pos(self.to_glib_none().0, position)
        }
    }
    /// Gets the number of decimal places that are displayed in the value.
    pub fn get_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_scale_get_digits(self.to_glib_none().0)
        }
    }
    /// Gets the position in which the current value is displayed.
    pub fn get_value_pos(&self) -> PositionType {
        unsafe {
            ffi::gtk_scale_get_value_pos(self.to_glib_none().0)
        }
    }
    /// Obtains the coordinates where the scale will
    /// draw the `PangoLayout` representing the text in the scale.
    /// If the `draw-value` property is `FALSE`, the return values are undefined.
    pub fn get_layout_offsets(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        unsafe {
            ffi::gtk_scale_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y)
        }
        (x, y)
    }
    /// Adds a mark at value.
    /// A mark is indicated visually by drawing a tick mark next to the scale,
    /// and GTK+ makes it easy for the user to position the scale exactly at the marks value.
    /// If `markup` is not `NULL`, text is shown next to the tick mark.
    /// To remove marks from a scale, use `clear_marks()`.
    pub fn add_mark(&self, value: f64, position: PositionType, markup: &str) {
        unsafe {
            ffi::gtk_scale_add_mark(self.to_glib_none().0,
                value, position, markup.to_glib_none().0)
        }
    }
    /// Removes any marks that have been added with `add_mark()`.
    pub fn clear_marks(&self) {
        unsafe {
            ffi::gtk_scale_clear_marks(self.to_glib_none().0)
        }
    }
}

impl types::StaticType for Scale {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_scale_get_type()) }
    }
}
