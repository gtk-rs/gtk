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

use libc::{c_void, c_double, c_int};
use std::{ptr, mem};

use traits::{GtkWidget, GtkOrientable, Signal};
use gtk::enums::{GtkOrientation, GtkPositionType};
use utils::cast::GTK_SCALE;
use gtk;
use ffi;

// TODO : implements GtkRange

/**
* Scale — A slider widget for selecting a value from a range
*
* # Signal availables:
* * `format-value` : Run Last
*/
pub struct Scale {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Scale {
    pub fn new(orientation: GtkOrientation,
               adjustment: &gtk::Adjustment) -> Option<Scale> {
        let tmp_pointer = unsafe { ffi::gtk_scale_new(orientation, adjustment.get_pointer()) };
        check_pointer!(tmp_pointer, Scale)
    }

    pub fn new_with_range(orientation: GtkOrientation,
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

    pub fn set_value_pos(&mut self, position: GtkPositionType) -> () {
        unsafe {
            ffi::gtk_scale_set_value_pos(GTK_SCALE(self.pointer), position);
        }
    }

    pub fn get_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_scale_get_digits(GTK_SCALE(self.pointer)) as i32
        }
    }

    pub fn get_value_pos(&self) ->GtkPositionType {
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

    pub fn add_mark(&mut self, value: f64, position: GtkPositionType, markup: &str) -> () {
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

impl_GtkWidget!(Scale)
redirect_callback!(Scale)
redirect_callback_widget!(Scale)
struct_signal!(Scale)
impl_signals!(Scale)

impl GtkOrientable for Scale {}
