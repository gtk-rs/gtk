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

//! Retrieve an integer or floating-point number from the user

use libc::{c_double, c_uint};

use gtk::{mod, ffi};
use gtk::{SpinType, SpinButtonUpdatePolicy};
use gtk::cast::{GTK_SPINBUTTON};

/**
* SpinButton — Retrieve an integer or floating-point number from the user
*
* # Available signals:
* * `change-value` : Action
* * `input` : Run Last
* * `output` : Run Last
* * `value-changed` : Run Last
* * `wrapped` : Run Last
*
*/
struct_Widget!(SpinButton)

impl SpinButton {
    pub fn new(adjustment: &gtk::Adjustment,
               climb_rate: f64,
               digits: u32) -> Option<SpinButton> {
        let tmp_pointer = unsafe { ffi::gtk_spin_button_new(adjustment.get_pointer(), climb_rate as c_double, digits as c_uint) };
        check_pointer!(tmp_pointer, SpinButton)
    }

    pub fn new_with_range(min: f64, max: f64, step: f64) -> Option<SpinButton> {
        let tmp_pointer = unsafe { ffi::gtk_spin_button_new_with_range(min as c_double, max as c_double, step as c_double) };
        check_pointer!(tmp_pointer, SpinButton)
    }

    pub fn configure(&mut self, adjustment: &gtk::Adjustment, climb_rate: f64, digits: u32) -> () {
        unsafe {
            ffi::gtk_spin_button_configure(GTK_SPINBUTTON(self.pointer), adjustment.get_pointer(), climb_rate as c_double, digits as c_uint);
        }
    }

    pub fn set_adjustment(&mut self, adjustment: &gtk::Adjustment) -> () {
        unsafe {
            ffi:: gtk_spin_button_set_adjustment(GTK_SPINBUTTON(self.pointer), adjustment.get_pointer());
        }
    }

    pub fn get_adjustment(&self) -> gtk::Adjustment {
        unsafe {
            gtk::Adjustment::wrap_pointer(ffi::gtk_spin_button_get_adjustment(GTK_SPINBUTTON(self.pointer)))
        }
    }

    pub fn set_digits(&mut self, digits: u32) -> () {
        unsafe {
            ffi::gtk_spin_button_set_digits(GTK_SPINBUTTON(self.pointer), digits as c_uint);
        }
    }

    pub fn set_increments(&mut self, step: f64, page: f64) -> () {
        unsafe {
            ffi::gtk_spin_button_set_increments(GTK_SPINBUTTON(self.pointer), step as c_double, page as c_double);
        }
    }

    pub fn set_range(&mut self, min: f64, max: f64) -> () {
        unsafe {
            ffi::gtk_spin_button_set_range(GTK_SPINBUTTON(self.pointer), min as c_double, max as c_double);
        }
    }

    pub fn get_value_as_int(&self) -> i32 {
        unsafe {
            ffi::gtk_spin_button_get_value_as_int(GTK_SPINBUTTON(self.pointer)) as i32
        }
    }

    pub fn set_value(&mut self, value: f64) -> () {
        unsafe {
            ffi::gtk_spin_button_set_value(GTK_SPINBUTTON(self.pointer), value as c_double);
        }
    }

    pub fn set_update_policy(&mut self, policy: SpinButtonUpdatePolicy) -> () {
        unsafe {
            ffi::gtk_spin_button_set_update_policy(GTK_SPINBUTTON(self.pointer), policy);
        }
    }

    pub fn set_numeric(&mut self, numeric: bool) -> () {
        match numeric {
            true    => unsafe { ffi::gtk_spin_button_set_numeric(GTK_SPINBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_spin_button_set_numeric(GTK_SPINBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_numeric(&self) -> bool {
        match unsafe { ffi::gtk_spin_button_get_numeric(GTK_SPINBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_wrap(&mut self, wrap: bool) -> () {
        match wrap {
            true    => unsafe { ffi::gtk_spin_button_set_wrap(GTK_SPINBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_spin_button_set_wrap(GTK_SPINBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_wrap(&self) -> bool {
        match unsafe { ffi::gtk_spin_button_get_wrap(GTK_SPINBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_snap_to_ticks(&mut self, snap_to_ticks: bool) -> () {
        match snap_to_ticks {
            true    => unsafe { ffi::gtk_spin_button_set_snap_to_ticks(GTK_SPINBUTTON(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_spin_button_set_snap_to_ticks(GTK_SPINBUTTON(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_snap_to_ticks(&self) -> bool {
        match unsafe { ffi::gtk_spin_button_get_snap_to_ticks(GTK_SPINBUTTON(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn spin(&mut self, direction: SpinType, increment: f64) -> () {
        unsafe {
            ffi::gtk_spin_button_spin(GTK_SPINBUTTON(self.pointer), direction, increment as c_double);
        }
    }

    pub fn update(&self) -> () {
        unsafe {
            ffi::gtk_spin_button_update(GTK_SPINBUTTON(self.pointer));
        }
    }

    pub fn get_digits(&self) -> u32 {
        unsafe {
            ffi::gtk_spin_button_get_digits(GTK_SPINBUTTON(self.pointer)) as u32
        }
    }

    pub fn get_increments(&self) -> (f64, f64) {
        let mut step = 0.;
        let mut page = 0.;
        unsafe {
            ffi::gtk_spin_button_get_increments(GTK_SPINBUTTON(self.pointer), &mut step, &mut page);
        }
        (step, page)
    }

    pub fn get_range(&self) -> (f64, f64) {
        let mut min = 0.;
        let mut max = 0.;
        unsafe {
            ffi::gtk_spin_button_get_range(GTK_SPINBUTTON(self.pointer), &mut min, &mut max);
        }
        (min, max)
    }

    pub fn get_update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            ffi::gtk_spin_button_get_update_policy(GTK_SPINBUTTON(self.pointer))
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_spin_button_get_value(GTK_SPINBUTTON(self.pointer)) as f64
        }
    }
}

// pub fn gtk_spin_button_get_update_policy   (spin_button: *C_GtkSpinButton) -> SpinButtonUpdatePolicy;
// pub fn gtk_spin_button_get_value           (spin_button: *C_GtkSpinButton) -> c_double;

impl_drop!(SpinButton)
impl_TraitWidget!(SpinButton)

impl gtk::EntryTrait for SpinButton {}
impl gtk::EditableTrait for SpinButton {}
impl gtk::OrientableTrait for SpinButton {}

impl_widget_events!(SpinButton)
impl_connect!(SpinButton -> ChangedValue, ValueChanged, Wrapped) // Input, Output
