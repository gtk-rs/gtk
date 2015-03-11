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
use libc::c_char;
use std::ffi::CString;
use glib::translate::{ToGlib, ToGlibPtr, ToTmp, ToArray, StackBox, PtrArray};

pub struct RecentData {
    display_name: String,
    description: String,
    mime_type: String,
    app_name: String,
    app_exec: String,
    groups: Vec<String>,
    is_private: bool
}

impl ToTmp for RecentData {
    type Tmp = StackBox<ffi::C_GtkRecentData, ([CString; 5], PtrArray<String>)>;

    fn to_tmp_for_borrow(&self)
        -> StackBox<ffi::C_GtkRecentData, ([CString; 5], PtrArray<String>)> {
        let mut tmp_display_name = self.display_name.to_tmp_for_borrow();
        let mut tmp_description = self.description.to_tmp_for_borrow();
        let mut tmp_mime_type = self.mime_type.to_tmp_for_borrow();
        let mut tmp_app_name = self.app_name.to_tmp_for_borrow();
        let mut tmp_app_exec = self.app_exec.to_tmp_for_borrow();
        let mut tmp_groups = (*self.groups).to_array_for_borrow();

        let data = ffi::C_GtkRecentData {
            display_name: tmp_display_name.to_glib_ptr() as *mut c_char,
            description: tmp_description.to_glib_ptr() as *mut c_char,
            mime_type: tmp_mime_type.to_glib_ptr() as *mut c_char,
            app_name: tmp_app_name.to_glib_ptr() as *mut c_char,
            app_exec: tmp_app_exec.to_glib_ptr() as *mut c_char,
            groups: tmp_groups.to_glib_ptr() as *mut *mut c_char,
            is_private: self.is_private.to_glib(),
        };

        StackBox(data, ([tmp_display_name, tmp_description, tmp_mime_type,
                         tmp_app_name, tmp_app_exec], tmp_groups))
    }
}
