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

use gtk::ffi::{self, C_GtkBuilder};
use libc::{c_char, c_long};
use gtk::traits::GObjectTrait;
use glib::translate::{ToGlibPtr, ToTmp};

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
            let mut tmp_file_name = file_name.to_tmp_for_borrow();
            ffi::gtk_builder_new_from_file(tmp_file_name.to_glib_ptr())
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
            let mut tmp_resource_path = resource_path.to_tmp_for_borrow();
            ffi::gtk_builder_new_from_resource(tmp_resource_path.to_glib_ptr())
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
            // Don't need a null-terminated string here
            ffi::gtk_builder_new_from_string(string.as_ptr() as *const c_char,
                                             string.len() as c_long)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    pub fn get_object<T: GObjectTrait>(&self, name: &str) -> Option<T> {
        let tmp = unsafe {
            let mut tmp_name = name.to_tmp_for_borrow();
            ffi::gtk_builder_get_object(self.pointer, tmp_name.to_glib_ptr())
        };

        if tmp.is_null() {
            None
        } else {
            Some(::glib::traits::FFIGObject::wrap_object(tmp))
        }
    }
}

impl_GObjectFunctions!(Builder, C_GtkBuilder);
impl_TraitObject!(Builder, C_GtkBuilder);