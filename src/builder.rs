// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(feature = "v3_10"), allow(unused_imports))]

use libc::{c_char, ssize_t};

use glib::Object;
use glib::object::{Downcast, IsA};
use glib::translate::*;
use ffi;

use std::path::Path;

use Error;

glib_wrapper! {
    pub struct Builder(Object<ffi::GtkBuilder>);

    match fn {
        get_type => || ffi::gtk_builder_get_type(),
    }
}

impl Builder {
    pub fn new() -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new()) }
    }

    #[cfg(feature = "v3_10")]
    pub fn new_from_file<T: AsRef<Path>>(file_path: T) -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new_from_file(file_path.as_ref().to_glib_none().0)) }
    }

    #[cfg(feature = "v3_10")]
    pub fn new_from_resource(resource_path: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_resource(resource_path.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn new_from_string(string: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            // Don't need a null-terminated string here
            from_glib_full(
                ffi::gtk_builder_new_from_string(string.as_ptr() as *const c_char,
                    string.len() as ssize_t))
        }
    }

    pub fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T> {
        unsafe {
            Option::<Object>::from_glib_none(
                ffi::gtk_builder_get_object(self.to_glib_none().0, name.to_glib_none().0))
                .and_then(|obj| obj.downcast().ok())
        }
    }

    pub fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_builder_add_from_file(self.to_glib_none().0,
                                           file_path.as_ref().to_glib_none().0,
                                           &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn add_from_resource(&self, resource_path: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_builder_add_from_resource(self.to_glib_none().0,
                                               resource_path.to_glib_none().0,
                                               &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn add_from_string(&self, string: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_builder_add_from_string(self.to_glib_none().0,
                                             string.as_ptr() as *const c_char,
                                             string.len(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
    
    pub fn set_translation_domain(&self, domain: Option<&str>) {
        unsafe {
            ffi::gtk_builder_set_translation_domain(self.to_glib_none().0, domain.to_glib_none().0);
        }
    }

    pub fn get_translation_domain(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_builder_get_translation_domain(self.to_glib_none().0))
        }
    }
}
