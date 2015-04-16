// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr};

pub struct FileFilter {
    pointer : *mut ffi::C_GtkFileFilter
}

impl FileFilter {
    pub fn new() -> Option<FileFilter> {
        let tmp_pointer = unsafe { ffi::gtk_file_filter_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FileFilter { pointer: tmp_pointer })
        }
    }

    pub fn set_name(&self, name: &str) -> () {
        unsafe {
            ffi::gtk_file_filter_set_name(self.pointer, name.borrow_to_glib().0)
        };
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_file_filter_get_name(self.pointer))
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) -> () {
        unsafe {
            ffi::gtk_file_filter_add_mime_type(self.pointer, mime_type.borrow_to_glib().0)
        };
    }

    pub fn add_pattern(&self, pattern: &str) -> () {
        unsafe {
            ffi::gtk_file_filter_add_pattern(self.pointer, pattern.borrow_to_glib().0)
        };
    }

    pub fn add_pixbuf_formats(&self) -> () {
        unsafe { ffi::gtk_file_filter_add_pixbuf_formats(self.pointer) }
    }

    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkFileFilter {
        self.pointer
    }

    pub fn wrap(pointer: *mut ffi::C_GtkFileFilter) -> Option<FileFilter> {
        if pointer.is_null() {
            None
        } else {
            Some(FileFilter { pointer: pointer })
        }
    }
}

impl_drop!(FileFilter, GTK_FILE_FILTER);
