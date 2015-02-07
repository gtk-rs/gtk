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

use gtk::{self, ffi};
use gtk::ffi::to_bool;
use std::ffi::CString;

#[derive(Copy)]
pub struct RecentFilter {
    pointer: *mut ffi::C_GtkRecentFilter
}

impl RecentFilter {
    pub fn new() -> Option<RecentFilter> {
        let tmp_pointer = unsafe { ffi::gtk_recent_filter_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(RecentFilter{ pointer: tmp_pointer })
        }
    }

    pub fn add_application(&self, application: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(application.as_bytes());

            ffi::gtk_recent_filter_add_application(self.pointer, c_str.as_ptr())
        }
    }

    pub fn add_group(&self, group: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(group.as_bytes());

            ffi::gtk_recent_filter_add_group(self.pointer, c_str.as_ptr())
        }
    }

    pub fn add_age(&self, days: i32) -> () {
        unsafe { ffi::gtk_recent_filter_add_age(self.pointer, days) }
    }

    pub fn get_needed(&self) -> gtk::RecentFilterFlags {
        unsafe { ffi::gtk_recent_filter_get_needed(self.pointer) }
    }

    pub fn filter(&self, filter_info: &gtk::RecentFilterInfo) -> bool {
        unsafe { to_bool(ffi::gtk_recent_filter_filter(self.pointer, &filter_info.get_ffi())) }
    }

    pub fn wrap(pointer: *mut ffi::C_GtkRecentFilter) -> Option<RecentFilter> {
        if pointer.is_null() {
            None
        } else {
            Some(RecentFilter{ pointer: pointer })
        }
    }

    pub fn get_pointer(&self) -> *mut ffi::C_GtkRecentFilter {
        self.pointer
    }
}

// impl_drop!(RecentFilter, GTK_RECENT_FILTER)
