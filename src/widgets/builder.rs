// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_imports))]

use ffi::{self, C_GtkBuilder};
use libc::{c_char, c_long};
use traits::GObjectTrait;
use glib::translate::ToGlibPtr;

#[repr(C)]
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

    #[cfg(feature = "gtk_3_10")]
    pub fn new_from_file(file_name: &str) -> Option<Builder> {
        let tmp = unsafe {
            ffi::gtk_builder_new_from_file(file_name.borrow_to_glib().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn new_from_resource(resource_path: &str) -> Option<Builder> {
        let tmp = unsafe {
            ffi::gtk_builder_new_from_resource(resource_path.borrow_to_glib().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    #[cfg(feature = "gtk_3_10")]
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
            ffi::gtk_builder_get_object(self.pointer, name.borrow_to_glib().0)
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