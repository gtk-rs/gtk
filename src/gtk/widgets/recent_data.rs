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
use glib::{to_bool, to_gboolean};
use std::default::Default;
use libc::c_char;
use std::ffi::CString;

pub struct RecentData {
    display_name: String,
    description: String,
    mime_type: String,
    app_name: String,
    app_exec: String,
    groups: Vec<String>,
    is_private: bool
}

impl RecentData {
    #[doc(hidden)]
    pub fn from_c(ptr: *mut ffi::C_GtkRecentData) -> RecentData {
        if ptr.is_null() {
            Default::default()
        } else {
            let mut tmp_groups = Vec::new();
            let mut count = 0;

            unsafe {
                loop {
                    let tmp = (*ptr).groups.offset(count) as *const c_char;

                    if tmp.is_null() {
                        break;
                    }
                    count = count + 1;
                    tmp_groups.push(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string());
                }
                RecentData {
                    display_name: String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&((*ptr).display_name as *const c_char))).to_string(),
                    description: String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&((*ptr).description as *const c_char))).to_string(),
                    mime_type: String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&((*ptr).mime_type as *const c_char))).to_string(),
                    app_name: String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&((*ptr).app_name as *const c_char))).to_string(),
                    app_exec: String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&((*ptr).app_exec as *const c_char))).to_string(),
                    groups: tmp_groups,
                    is_private: to_bool((*ptr).is_private),
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn get_ffi(&self) -> ffi::C_GtkRecentData {
        let mut t_groups = Vec::with_capacity(self.groups.len());

        for tmp in self.groups.iter() {
            let c = CString::from_slice(tmp.as_bytes());

            t_groups.push(c.as_ptr());
        }
        let c_display_name = CString::from_slice(self.display_name.as_bytes());
        let c_description = CString::from_slice(self.description.as_bytes());
        let c_mime_type = CString::from_slice(self.mime_type.as_bytes());
        let c_app_name = CString::from_slice(self.app_name.as_bytes());
        let c_app_exec = CString::from_slice(self.app_exec.as_bytes());

        ffi::C_GtkRecentData {
            display_name: c_display_name.as_ptr() as *mut c_char,
            description: c_description.as_ptr() as *mut c_char,
            mime_type: c_mime_type.as_ptr() as *mut c_char,
            app_name: c_app_name.as_ptr() as *mut c_char,
            app_exec: c_app_exec.as_ptr() as *mut c_char,
            groups: t_groups.as_ptr() as *mut *mut c_char,
            is_private: to_gboolean(self.is_private)
        }
    }
}

impl Default for RecentData {
    fn default() -> RecentData {
        RecentData {
            display_name: String::new(),
            description: String::new(),
            mime_type: String::new(),
            app_name: String::new(),
            app_exec: String::new(),
            groups: Vec::new(),
            is_private: false
        }
    }
}
