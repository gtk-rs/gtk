// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(gtk_3_10), allow(unused_imports))]

use ffi::{self, GtkBuilder};
use libc::{c_char, ssize_t};
use traits::GObjectTrait;
use glib::translate::ToGlibPtr;

#[repr(C)]
pub struct Builder {
    pointer: *mut GtkBuilder
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

    #[cfg(gtk_3_10)]
    pub fn new_from_file(file_name: &str) -> Option<Builder> {
        let tmp = unsafe {
            ffi::gtk_builder_new_from_file(file_name.to_glib_none().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_resource(resource_path: &str) -> Option<Builder> {
        let tmp = unsafe {
            ffi::gtk_builder_new_from_resource(resource_path.to_glib_none().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Builder {
                pointer: tmp
            })
        }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_string(string: &str) -> Option<Builder> {
        let tmp = unsafe {
            // Don't need a null-terminated string here
            ffi::gtk_builder_new_from_string(string.as_ptr() as *const c_char,
                                             string.len() as ssize_t)
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
            ffi::gtk_builder_get_object(self.pointer, name.to_glib_none().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(::glib::traits::FFIGObject::wrap_object(tmp))
        }
    }
}

impl_GObjectFunctions!(Builder, GtkBuilder);
impl_TraitObject!(Builder, GtkBuilder);