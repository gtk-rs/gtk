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

//! A bar that can used as a level indicator

#![cfg_attr(not(feature = "gtk_3_8"), allow(unused_imports))]

use libc::c_double;
use glib::translate::ToGlibPtr;

use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::{LevelBarMode};
use gtk::cast::GTK_LEVELBAR;

/// LevelBar — A bar that can used as a level indicator
/*
* # Signal availables:
* * `offset-changed` : Has Details
*/
struct_Widget!(LevelBar);

impl LevelBar {
    pub fn new() -> Option<LevelBar> {
        let tmp_pointer = unsafe { ffi::gtk_level_bar_new() };
        check_pointer!(tmp_pointer, LevelBar)
    }

    pub fn new_for_interval(min: f64, max: f64) -> Option<LevelBar> {
        let tmp_pointer = unsafe { ffi::gtk_level_bar_new_for_interval(min as c_double, max as c_double) };
        check_pointer!(tmp_pointer, LevelBar)
    }

    pub fn set_value(&mut self, value: f64) -> () {
        unsafe {
            ffi::gtk_level_bar_set_value(GTK_LEVELBAR(self.pointer), value as c_double);
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_value(GTK_LEVELBAR(self.pointer)) as f64
        }
    }

    pub fn set_mode(&mut self, mode: LevelBarMode) -> () {
        unsafe {
            ffi::gtk_level_bar_set_mode(GTK_LEVELBAR(self.pointer), mode);
        }
    }

    pub fn get_mode(&self) -> LevelBarMode {
        unsafe {
            ffi::gtk_level_bar_get_mode(GTK_LEVELBAR(self.pointer))
        }
    }

    pub fn set_min_value(&mut self, value: f64) -> () {
        unsafe {
            ffi::gtk_level_bar_set_min_value(GTK_LEVELBAR(self.pointer), value as c_double);
        }
    }

    pub fn get_min_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_min_value(GTK_LEVELBAR(self.pointer)) as c_double
        }
    }

    pub fn set_max_value(&mut self, value: f64) -> () {
        unsafe {
            ffi::gtk_level_bar_set_max_value(GTK_LEVELBAR(self.pointer), value as c_double);
        }
    }

    pub fn get_max_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_max_value(GTK_LEVELBAR(self.pointer)) as c_double
        }
    }

    #[cfg(feature = "gtk_3_8")]
    pub fn set_inverted(&mut self, inverted: bool) -> () {
        unsafe { ffi::gtk_level_bar_set_inverted(GTK_LEVELBAR(self.pointer), to_gboolean(inverted)); }
    }

    #[cfg(feature = "gtk_3_8")]
    pub fn get_inverted(&self) -> bool {
        unsafe { to_bool(ffi::gtk_level_bar_get_inverted(GTK_LEVELBAR(self.pointer))) }
    }

    pub fn add_offset_value(&mut self, name: &str, value: f64) -> () {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(
                GTK_LEVELBAR(self.pointer),
                name.borrow_to_glib().0,
                value as c_double)
        }
    }

    pub fn remove_offset_value(&mut self, name: &str) -> () {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(
                GTK_LEVELBAR(self.pointer),
                name.borrow_to_glib().0);
        }
    }

    pub fn get_offset_value(&self, name: &str) -> Option<f64> {
        unsafe {
            let mut value = 0.;

            let res = to_bool(
                ffi::gtk_level_bar_get_offset_value(
                    GTK_LEVELBAR(self.pointer),
                    name.borrow_to_glib().0,
                    &mut value));

            if res {
                Some(value)
            }
            else {
                None
            }
        }
    }
}

impl_drop!(LevelBar);
impl_TraitWidget!(LevelBar);

impl gtk::OrientableTrait for LevelBar {}

impl_widget_events!(LevelBar);
