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

use gtk::ffi;
use std::ffi::CString;

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
            name.with_c_str(|c_str| {
                ffi::gtk_file_filter_set_name(self.pointer, c_str)
            })
        };
    }

    pub fn get_name(&self) -> Option<String> {
        let name = unsafe { ffi::gtk_file_filter_get_name(self.pointer) };

        if name.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(name as *const u8) })
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) -> () {
        unsafe {
            mime_type.with_c_str(|c_str| {
                ffi::gtk_file_filter_add_mime_type(self.pointer, c_str)
            })
        };
    }

    pub fn add_pattern(&self, pattern: &str) -> () {
        unsafe {
            pattern.with_c_str(|c_str| {
                ffi::gtk_file_filter_add_pattern(self.pointer, c_str)
            })
        };
    }

    pub fn add_pixbuf_formats(&self) -> () {
        unsafe { ffi::gtk_file_filter_add_pixbuf_formats(self.pointer) }
    }

    pub fn get_pointer(&self) -> *mut ffi::C_GtkFileFilter {
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
