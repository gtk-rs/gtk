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

//! Adjustment — A representation of an adjustable bounded value


#![allow(visible_private_types)]

use libc::c_double;
use gtk::ffi;
/**
* A representation of an adjustable bounded value
*
* # Availables signals:
* * `changed` : No Recursion
* * `value-changed` : No Recursion
*/
pub struct Adjustment {
    pointer:   *ffi::C_GtkAdjustment,
    can_drop:  bool
}

impl Adjustment {
    pub fn new(value: f64,
               lower: f64,
               upper: f64,
               step_increment: f64,
               page_increment: f64,
               page_size: f64) -> Option<Adjustment> {
        let tmp_pointer = unsafe { ffi::gtk_adjustment_new(value as c_double, lower as c_double,
                                                           upper as c_double, step_increment as c_double,
                                                           page_increment as c_double, page_size as c_double) };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(Adjustment {
                pointer:    tmp_pointer,
                can_drop:   true
            })
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_value(self.pointer) as f64
        }
    }

    pub fn set_value(&mut self, value: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_value(self.pointer, value as c_double)
        }
    }

    pub fn get_lower(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_lower(self.pointer) as f64
        }
    }

    pub fn set_lower(&mut self, lower: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_lower(self.pointer, lower as c_double)
        }
    }

    pub fn get_page_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_increment(self.pointer) as f64
        }
    }

    pub fn set_page_increment(&mut self, page_increment: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_page_increment(self.pointer, page_increment as c_double)
        }
    }

    pub fn get_page_size(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_size(self.pointer) as f64
        }
    }

    pub fn set_page_size(&mut self, page_size: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_page_size(self.pointer, page_size as c_double)
        }
    }

    pub fn get_step_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_step_increment(self.pointer) as f64
        }
    }

    pub fn set_step_increment(&mut self, step_increment: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_step_increment(self.pointer, step_increment as c_double)
        }
    }

    pub fn get_upper(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_upper(self.pointer) as f64
        }
    }

    pub fn set_upper(&mut self, upper: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_upper(self.pointer, upper as c_double)
        }
    }

    pub fn get_minimum_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_minimum_increment(self.pointer) as f64
        }
    }

    pub fn clamp_page(&mut self, lower: f64, upper: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_clamp_page(self.pointer, lower as c_double, upper as c_double);
        }
    }

    pub fn changed(&mut self) -> () {
        unsafe {
            ffi::gtk_adjustment_changed(self.pointer);
        }
    }

    pub fn value_changer(&mut self) -> () {
        unsafe {
            ffi::gtk_adjustment_value_changed(self.pointer)
        }
    }

    pub fn configure(&mut self,
                     value: f64,
                     lower: f64,
                     upper: f64,
                     step_increment: f64,
                     page_increment: f64,
                     page_size: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_configure(self.pointer, value as c_double, lower as c_double,
                                          upper as c_double, step_increment as c_double,
                                          page_increment as c_double, page_size as c_double);
        }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *ffi::C_GtkAdjustment {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_adjustment: *ffi::C_GtkAdjustment) -> Adjustment {
        Adjustment {
            pointer: c_adjustment,
            can_drop: false
        }
    }
}

