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
use gtk::ffi::FFIWidget;
use gtk::cast::GTK_RECENT_INFO;

struct_Widget!(RecentInfo)

impl RecentInfo {
    pub fn _ref(&self) -> Option<RecentInfo> {
        let tmp_pointer = unsafe { ffi::gtk_recent_info_ref(GTK_RECENT_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn unref(&self) {
        unsafe { ffi::gtk_recent_info_unref(GTK_RECENT_INFO(self.get_widget())) }
    }

    pub fn get_uri(&self) -> Option<String> {
        let uri = unsafe { ffi::gtk_recent_info_get_uri(GTK_RECENT_INFO(self.get_widget())) };

        if uri.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(uri as *const u8) })
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        let display_name = unsafe { ffi::gtk_recent_info_get_display_name(GTK_RECENT_INFO(self.get_widget())) };

        if display_name.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(display_name as *const u8) })
        }
    }

    pub fn get_description(&self) -> Option<String> {
        let description = unsafe { ffi::gtk_recent_info_get_description(GTK_RECENT_INFO(self.get_widget())) };

        if description.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(description as *const u8) })
        }
    }

    pub fn get_mime_type(&self) -> Option<String> {
        let mime_type = unsafe { ffi::gtk_recent_info_get_mime_type(GTK_RECENT_INFO(self.get_widget())) };

        if mime_type.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(mime_type as *const u8) })
        }
    }

    pub fn get_added(&self) -> i64 {
        unsafe { ffi::gtk_recent_info_get_added(GTK_RECENT_INFO(self.get_widget())) }
    }

    pub fn get_modified(&self) -> i64 {
        unsafe { ffi::gtk_recent_info_get_modified(GTK_RECENT_INFO(self.get_widget())) }
    }

    pub fn get_visited(&self) -> i64 {
        unsafe { ffi::gtk_recent_info_get_visited(GTK_RECENT_INFO(self.get_widget())) }
    }

    pub fn get_private_hint(&self) -> bool {
        match unsafe { ffi::gtk_recent_info_get_private_hint(GTK_RECENT_INFO(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_name(&self, app_name: &str) -> (bool, String, u32, i64) {
        let app_exec = ::std::ptr::null_mut();
        let mut count = 0u32;
        let mut time_ = 0i64;

        let ret = match unsafe {
            app_name.with_c_str(|c_str| {
                ffi::gtk_recent_info_get_application_info(GTK_RECENT_INFO(self.get_widget()), c_str, &app_exec, &mut count, &mut time_)
            })
        } {
            ffi::GFALSE => false,
            _ => true
        };
        if app_exec.is_null() {
            (ret, String::new(), count, time_)
        } else {
            (ret, unsafe { String::from_raw_buf(app_exec as *const u8)}, count, time_)
        }
    }

    pub fn get_applications(&self) -> Option<Vec<String>> {
        let mut length = 0;
        let tmp = unsafe { ffi::gtk_recent_info_get_applications(GTK_RECENT_INFO(self.get_widget()), &mut length) };

        if tmp.is_null() {
            None
        } else {
            let mut ret = Vec::with_capacity(length as uint);

            for count in range(0, length) {
                ret.push(unsafe { String::from_raw_buf(*tmp.offset(count as int) as *const u8) });
            }
            Some(ret)
        }
    }

    pub fn last_application(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_recent_info_last_application(GTK_RECENT_INFO(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(tmp as *const u8) })
        }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        match unsafe { app_name.with_c_str(|c_str| {
            ffi::gtk_recent_info_has_application(GTK_RECENT_INFO(self.get_widget()), c_str)
        })} {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn get_groups(&self) -> Option<Vec<String>> {
        let mut length = 0;
        let tmp = unsafe { ffi::gtk_recent_info_get_groups(GTK_RECENT_INFO(self.get_widget()), &mut length) };

        if tmp.is_null() {
            None
        } else {
            let mut ret = Vec::with_capacity(length as uint);

            for count in range(0, length) {
                ret.push(unsafe { String::from_raw_buf(*tmp.offset(count as int) as *const u8) });
            }
            Some(ret)
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        match unsafe { group_name.with_c_str(|c_str| {
            ffi::gtk_recent_info_has_group(GTK_RECENT_INFO(self.get_widget()), c_str)
        })} {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn get_short_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_recent_info_get_short_name(GTK_RECENT_INFO(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(tmp as *const u8) })
        }
    }

    pub fn get_uri_display(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_recent_info_get_uri_display(GTK_RECENT_INFO(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(tmp as *const u8) })
        }
    }

    pub fn get_age(&self) -> i32 {
        unsafe { ffi::gtk_recent_info_get_age(GTK_RECENT_INFO(self.get_widget())) }
    }

    pub fn is_local(&self) -> bool {
        match unsafe { ffi::gtk_recent_info_is_local(GTK_RECENT_INFO(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn exists(&self) -> bool {
        match unsafe { ffi::gtk_recent_info_exists(GTK_RECENT_INFO(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn _match(&self, other: &RecentInfo) -> bool {
        match unsafe { ffi::gtk_recent_info_match(GTK_RECENT_INFO(self.get_widget()), GTK_RECENT_INFO(other.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }
}

impl_drop!(RecentInfo)
impl_TraitWidget!(RecentInfo)

impl_widget_events!(RecentInfo)
