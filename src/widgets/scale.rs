// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A slider widget for selecting a value from a range

use libc::{c_double, c_int};
use glib::translate::ToGlibPtr;

use {Orientation, PositionType};
use cast::GTK_SCALE;
use ffi;
use glib::{to_bool, to_gboolean};

/// Scale — A slider widget for selecting a value from a range
/*
* # Signal availables:
* * `format-value` : Run Last
*/
struct_Widget!(Scale);

impl Scale {
    pub fn new(orientation: Orientation,
               adjustment: &::Adjustment) -> Option<Scale> {
        let tmp_pointer = unsafe { ffi::gtk_scale_new(orientation, adjustment.unwrap_pointer()) };
        check_pointer!(tmp_pointer, Scale)
    }

    pub fn new_with_range(orientation: Orientation,
                          min: f64,
                          max: f64,
                          step: f64) -> Option<Scale> {
        let tmp_pointer = unsafe { ffi::gtk_scale_new_with_range(orientation, min as c_double, max as c_double, step as c_double) };
        check_pointer!(tmp_pointer, Scale)
    }

    pub fn set_digits(&self, digits: i32) -> () {
        unsafe {
            ffi::gtk_scale_set_digits(GTK_SCALE(self.pointer), digits as c_int);
        }
    }

    pub fn set_draw_value(&self, draw_value: bool) -> () {
        unsafe { ffi::gtk_scale_set_draw_value(GTK_SCALE(self.pointer), to_gboolean(draw_value)); }
    }

    pub fn get_draw_value(&self) -> bool {
        unsafe { to_bool(ffi::gtk_scale_get_draw_value(GTK_SCALE(self.pointer))) }
    }

    pub fn set_has_origin(&self, has_origin: bool) -> () {
        unsafe { ffi::gtk_scale_set_has_origin(GTK_SCALE(self.pointer), to_gboolean(has_origin)); }
    }

    pub fn get_has_origin(&self) -> bool {
        unsafe { to_bool(ffi::gtk_scale_get_has_origin(GTK_SCALE(self.pointer))) }
    }

    pub fn set_value_pos(&self, position: PositionType) -> () {
        unsafe {
            ffi::gtk_scale_set_value_pos(GTK_SCALE(self.pointer), position);
        }
    }

    pub fn get_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_scale_get_digits(GTK_SCALE(self.pointer)) as i32
        }
    }

    pub fn get_value_pos(&self) ->PositionType {
        unsafe {
            ffi::gtk_scale_get_value_pos(GTK_SCALE(self.pointer))
        }
    }

    pub fn get_layout_offsets(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;

        unsafe {
            ffi::gtk_scale_get_layout_offsets(GTK_SCALE(self.pointer), &x, &y);
        }
        (x, y)
    }

    pub fn add_mark(&self, value: f64, position: PositionType, markup: &str) -> () {
        unsafe {
            ffi::gtk_scale_add_mark(GTK_SCALE(self.pointer), value as c_double, position, markup.to_glib_none().0);
        }
    }

    pub fn clear_marks(&self) -> () {
        unsafe {
            ffi::gtk_scale_clear_marks(GTK_SCALE(self.pointer))
        }
    }
}

impl_drop!(Scale);
impl_TraitWidget!(Scale);

impl ::OrientableTrait for Scale {}
impl ::RangeTrait for Scale {}
