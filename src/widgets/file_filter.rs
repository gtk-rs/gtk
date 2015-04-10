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
