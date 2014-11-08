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

use gtk::{mod, ffi};
use std::default::Default;
use std::string;

pub struct RecentFilterInfo {
    contains: gtk::RecentFilterFlags,
    uri: String,
    display_name: String,
    mime_type: String,
    applications: Vec<String>,
    groups: Vec<String>,
    age: i32
}

impl RecentFilterInfo {
    #[doc(hidden)]
    pub fn from_c(ptr: *mut ffi::C_GtkRecentFilterInfo) -> RecentFilterInfo {
        if ptr.is_null() {
            Default::default()
        } else {
            let mut tmp_app = Vec::new();
            let mut tmp_groups = Vec::new();
            let mut count = 0i;

            unsafe {
                loop {
                    let tmp = (*ptr).applications.offset(count);

                    if tmp.is_null() {
                        break;
                    }
                    count = count + 1;
                    tmp_app.push(string::raw::from_buf(*tmp as *const u8));
                }
                count = 0;
                loop {
                    let tmp = (*ptr).groups.offset(count);

                    if tmp.is_null() {
                        break;
                    }
                    count = count + 1;
                    tmp_groups.push(string::raw::from_buf(*tmp as *const u8));
                }
                RecentFilterInfo {
                    contains: (*ptr).contains,
                    uri: string::raw::from_buf((*ptr).uri as *const u8),
                    display_name: string::raw::from_buf((*ptr).display_name as *const u8),
                    mime_type: string::raw::from_buf((*ptr).mime_type as *const u8),
                    applications: tmp_app,
                    groups: tmp_groups,
                    age: (*ptr).age
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn get_ffi(&self) -> ffi::C_GtkRecentFilterInfo {
        let mut t_app = Vec::with_capacity(self.applications.len());
        let mut t_groups = Vec::with_capacity(self.groups.len());

        for tmp in self.applications.iter() {
            t_app.push(tmp.with_c_str(|c| {c}));
        }
        for tmp in self.groups.iter() {
            t_groups.push(tmp.with_c_str(|c| {c}));
        }
        self.uri.with_c_str(|c_uri| {
            self.display_name.with_c_str(|c_display_name| {
                self.mime_type.with_c_str(|c_mime_type| {
                    ffi::C_GtkRecentFilterInfo {
                        contains: self.contains,
                        uri: c_uri,
                        display_name: c_display_name,
                        mime_type: c_mime_type,
                        applications: t_app.as_ptr(),
                        groups: t_groups.as_ptr(),
                        age: self.age
                    }
                })
            })
        })
    }
}

impl Default for RecentFilterInfo {
    fn default() -> RecentFilterInfo {
        RecentFilterInfo {
            contains: gtk::RecentFilterFlags::URI,
            uri: String::new(),
            display_name: String::new(),
            mime_type: String::new(),
            applications: Vec::new(),
            groups: Vec::new(),
            age: 0i32
        }
    }
}