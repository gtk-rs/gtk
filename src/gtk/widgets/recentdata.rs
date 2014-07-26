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
use std::str;
use std::default::Default;
use libc::c_char;

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
            let mut count = 0i;

            unsafe {
                loop {
                    let tmp = (*ptr).groups.offset(count);

                    if tmp.is_null() {
                        break;
                    }
                    count = count + 1;
                    tmp_groups.push(str::raw::from_c_str(*tmp as *const c_char));
                }
                RecentData {
                    display_name: str::raw::from_c_str((*ptr).display_name as *const c_char),
                    description: str::raw::from_c_str((*ptr).description as *const c_char),
                    mime_type: str::raw::from_c_str((*ptr).mime_type as *const c_char),
                    app_name: str::raw::from_c_str((*ptr).app_name as *const c_char),
                    app_exec: str::raw::from_c_str((*ptr).app_exec as *const c_char),
                    groups: tmp_groups,
                    is_private: match (*ptr).is_private {
                    	ffi::Gtrue => true,
                    	_ => false
                    }
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn get_ffi(&self) -> ffi::C_GtkRecentData {
        let mut t_groups = Vec::with_capacity(self.groups.len());

        for tmp in self.groups.iter() {
            t_groups.push(tmp.with_c_str(|c| {c as *mut c_char}));
        }
        self.display_name.with_c_str(|c_display_name| {
            self.description.with_c_str(|c_description| {
                self.mime_type.with_c_str(|c_mime_type| {
                	self.app_name.with_c_str(|c_app_name| {
                		self.app_exec.with_c_str(|c_app_exec| {
                			ffi::C_GtkRecentData {
		                        display_name: c_display_name as *mut c_char,
		                        description: c_description as *mut c_char,
		                        mime_type: c_mime_type as *mut c_char,
		                        app_name: c_app_name as *mut c_char,
		                        app_exec: c_app_exec as *mut c_char,
		                        groups: t_groups.as_mut_ptr(),
		                        is_private: match self.is_private {
		                        	true => ffi::Gtrue,
		                        	false => ffi::Gfalse
		                        }
		                    }
                		})
                	})
                })
            })
        })
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