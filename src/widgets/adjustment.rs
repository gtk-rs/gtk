// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Adjustment — A representation of an adjustable bounded value

use libc::c_double;
use ffi;

/**
* A representation of an adjustable bounded value
*
* # Availables signals:
* * `changed` : No Recursion
* * `value-changed` : No Recursion
*/
pub struct Adjustment {
    pointer:   *mut ffi::GtkAdjustment
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
                pointer:    tmp_pointer
            })
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_value(self.pointer) as f64
        }
    }

    pub fn set_value(&self, value: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_value(self.pointer, value as c_double)
        }
    }

    pub fn get_lower(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_lower(self.pointer) as f64
        }
    }

    pub fn set_lower(&self, lower: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_lower(self.pointer, lower as c_double)
        }
    }

    pub fn get_page_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_increment(self.pointer) as f64
        }
    }

    pub fn set_page_increment(&self, page_increment: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_page_increment(self.pointer, page_increment as c_double)
        }
    }

    pub fn get_page_size(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_size(self.pointer) as f64
        }
    }

    pub fn set_page_size(&self, page_size: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_page_size(self.pointer, page_size as c_double)
        }
    }

    pub fn get_step_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_step_increment(self.pointer) as f64
        }
    }

    pub fn set_step_increment(&self, step_increment: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_step_increment(self.pointer, step_increment as c_double)
        }
    }

    pub fn get_upper(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_upper(self.pointer) as f64
        }
    }

    pub fn set_upper(&self, upper: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_set_upper(self.pointer, upper as c_double)
        }
    }

    pub fn get_minimum_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_minimum_increment(self.pointer) as f64
        }
    }

    pub fn clamp_page(&self, lower: f64, upper: f64) -> () {
        unsafe {
            ffi::gtk_adjustment_clamp_page(self.pointer, lower as c_double, upper as c_double);
        }
    }

    pub fn changed(&self) -> () {
        unsafe {
            ffi::gtk_adjustment_changed(self.pointer);
        }
    }

    pub fn value_changed(&self) -> () {
        unsafe {
            ffi::gtk_adjustment_value_changed(self.pointer)
        }
    }

    pub fn configure(&self,
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
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkAdjustment {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_adjustment: *mut ffi::GtkAdjustment) -> Adjustment {
        unsafe { ::glib_ffi::g_object_ref(c_adjustment as *mut _); }
        Adjustment {
            pointer: c_adjustment
        }
    }
}

impl_drop!(Adjustment, GTK_ADJUSTMENT);
