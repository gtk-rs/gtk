// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A slider widget for selecting a value from a range

use libc::{c_double, c_int};

use gtk::enums::{Orientation, PositionType};
use gtk::cast::GTK_SCALE;
use gtk;
use gtk::ffi;
use gtk::traits;
// TODO : implements Range

/**
* Scale — A slider widget for selecting a value from a range
*
* # Signal availables:
* * `format-value` : Run Last
*/
struct_Widget!(Scale)

impl Scale {
    pub fn new(orientation: Orientation,
               adjustment: &gtk::Adjustment) -> Option<Scale> {
        let tmp_pointer = unsafe { ffi::gtk_scale_new(orientation, adjustment.get_pointer()) };
        check_pointer!(tmp_pointer, Scale)
    }

    pub fn new_with_range(orientation: Orientation,
                          min: f64,
                          max: f64,
                          step: f64) -> Option<Scale> {
        let tmp_pointer = unsafe { ffi::gtk_scale_new_with_range(orientation, min as c_double, max as c_double, step as c_double) };
        check_pointer!(tmp_pointer, Scale)
    }

    pub fn set_digits(&mut self, digits: i32) -> () {
        unsafe {
            ffi::gtk_scale_set_digits(GTK_SCALE(self.pointer), digits as c_int);
        }
    }

    pub fn set_draw_value(&mut self, draw_value: bool) -> () {
        match draw_value {
            true    => unsafe { ffi::gtk_scale_set_draw_value(GTK_SCALE(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_scale_set_draw_value(GTK_SCALE(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_draw_value(&self) -> bool {
        match unsafe { ffi::gtk_scale_get_draw_value(GTK_SCALE(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_has_origin(&mut self, has_origin: bool) -> () {
        match has_origin {
            true    => unsafe { ffi::gtk_scale_set_has_origin(GTK_SCALE(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_scale_set_has_origin(GTK_SCALE(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_has_origin(&self) -> bool {
        match unsafe { ffi::gtk_scale_get_has_origin(GTK_SCALE(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_value_pos(&mut self, position: PositionType) -> () {
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

    pub fn add_mark(&mut self, value: f64, position: PositionType, markup: &str) -> () {
        unsafe {
            markup.with_c_str(|c_str| {
                ffi::gtk_scale_add_mark(GTK_SCALE(self.pointer), value as c_double, position, c_str)
            });
        }
    }

    pub fn clear_marks(&mut self) -> () {
        unsafe {
            ffi::gtk_scale_clear_marks(GTK_SCALE(self.pointer))
        }
    }
}

impl_TraitWidget!(Scale)


impl traits::Orientable for Scale {}
