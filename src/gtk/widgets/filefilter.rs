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
use gtk::traits;
use gtk::traits::Widget;
use std::str;
use gtk::cast::GTK_FILE_FILTER;
use gtk::enums;

struct_Widget!(FileFilter)

impl FileFilter {
    pub fn new() -> Option<FileFilter> {
        let tmp_pointer = unsafe { ffi::gtk_file_filter_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(traits::Widget::wrap(tmp_pointer))
        }
    }

    pub fn set_name(&self, name: &str) -> () {
        unsafe {
            name.with_c_str(|c_str| {
                ffi::gtk_file_filter_set_name(GTK_FILE_FILTER(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_name(&self) -> Option<String> {
        let name = unsafe { ffi::gtk_file_filter_get_name(GTK_FILE_FILTER(self.get_widget())) };

        if name.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(name) })
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) -> () {
        unsafe {
            mime_type.with_c_str(|c_str| {
                ffi::gtk_file_filter_add_mime_type(GTK_FILE_FILTER(self.get_widget()), c_str)
            })
        };
    }

    pub fn add_pattern(&self, pattern: &str) -> () {
        unsafe {
            pattern.with_c_str(|c_str| {
                ffi::gtk_file_filter_add_pattern(GTK_FILE_FILTER(self.get_widget()), c_str)
            })
        };
    }

    pub fn add_pixbuf_formats(&self) -> () {
        unsafe { ffi::gtk_file_filter_add_pixbuf_formats(GTK_FILE_FILTER(self.get_widget())) }
    }
}

impl_drop!(FileFilter)
impl_TraitWidget!(FileFilter)