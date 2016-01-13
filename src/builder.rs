// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(gtk_3_10), allow(unused_imports))]

use libc::{c_char, ssize_t};

use glib::Object;
use glib::object::{Downcast, IsA};
use glib::translate::*;
use ffi;


glib_wrapper! {
    pub struct Buildable(Object<ffi::GtkBuildable>);

    match fn {
        get_type => || ffi::gtk_buildable_get_type(),
    }
}

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

    #[cfg(gtk_3_10)]
    pub fn new_from_file(file_name: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new_from_file(file_name.to_glib_none().0)) }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_resource(resource_path: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_resource(resource_path.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_string(string: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            // Don't need a null-terminated string here
            from_glib_full(
                ffi::gtk_builder_new_from_string(string.as_ptr() as *const c_char,
                    string.len() as ssize_t))
        }
    }

    pub fn get_object<T>(&self, name: &str) -> Option<T>
    where T: IsA<Buildable> + IsA<Object> {
        unsafe {
            Option::<Object>::from_glib_none(
                ffi::gtk_builder_get_object(self.to_glib_none().0, name.to_glib_none().0))
                .and_then(|obj| obj.downcast().ok())
        }
    }
}
