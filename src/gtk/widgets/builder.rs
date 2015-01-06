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

use std::c_str::ToCStr;
use gtk::ffi::{self, C_GtkBuilder};
use libc::c_long;
use gtk::traits::GObjectTrait;

#[repr(C)]
#[derive(Copy)]
pub struct Builder {
    pointer: *mut C_GtkBuilder
}

impl Builder {
    pub fn new() -> Option<Builder> {
        let tmp = unsafe { ffi::gtk_builder_new() };
        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    pub fn new_from_file(file_name: &str) -> Option<Builder> {
        let tmp = unsafe {
            file_name.with_c_str(|c_str| {
                ffi::gtk_builder_new_from_file(c_str)
            })
        };
        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Option<Builder> {
        let tmp = unsafe {
            resource_path.with_c_str(|c_str| {
                ffi::gtk_builder_new_from_resource(c_str)
            })
        };
        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    pub fn new_from_string(string: &str) -> Option<Builder> {
        let tmp = unsafe {
            string.with_c_str(|c_str| {
                ffi::gtk_builder_new_from_string(c_str, string.len() as c_long)
            })
        };
        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    pub fn get_object<T: GObjectTrait>(&self, name: &str) -> T {
        let tmp = unsafe {
            name.with_c_str(|c_str| {
                ffi::gtk_builder_get_object(self.pointer, c_str)
            })
        };
        unsafe {
            ::std::mem::transmute(Builder {
                pointer: tmp
            })
        }
    }
}

impl_GObjectFunctions!(Builder, C_GtkBuilder);
