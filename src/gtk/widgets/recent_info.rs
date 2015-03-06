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
use glib::to_bool;
use gtk::FFIWidget;
use gtk::cast::GTK_RECENT_INFO;
use std::ptr;
use std::ffi::CString;
use glib::translate::{FromGlibPtr, FromGlibPtrNotNull, ToGlibPtr, ToTmp};
use libc::c_char;

struct_Widget!(RecentInfo);

impl RecentInfo {
    pub fn _ref(&self) -> Option<RecentInfo> {
        let tmp_pointer = unsafe { ffi::gtk_recent_info_ref(GTK_RECENT_INFO(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn unref(&self) {
        unsafe { ffi::gtk_recent_info_unref(GTK_RECENT_INFO(self.unwrap_widget())) }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_get_uri(GTK_RECENT_INFO(self.unwrap_widget())))
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_get_display_name(GTK_RECENT_INFO(self.unwrap_widget())))
        }
    }

    pub fn get_description(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_get_description(GTK_RECENT_INFO(self.unwrap_widget())))
        }
    }

    pub fn get_mime_type(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_get_mime_type(GTK_RECENT_INFO(self.unwrap_widget())))
        }
    }

    pub fn get_added(&self) -> i64 {
        unsafe { ffi::gtk_recent_info_get_added(GTK_RECENT_INFO(self.unwrap_widget())) }
    }

    pub fn get_modified(&self) -> i64 {
        unsafe { ffi::gtk_recent_info_get_modified(GTK_RECENT_INFO(self.unwrap_widget())) }
    }

    pub fn get_visited(&self) -> i64 {
        unsafe { ffi::gtk_recent_info_get_visited(GTK_RECENT_INFO(self.unwrap_widget())) }
    }

    pub fn get_private_hint(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_info_get_private_hint(GTK_RECENT_INFO(self.unwrap_widget()))) }
    }

    pub fn get_application_info(&self, app_name: &str) -> Option<(String, u32, i64)> {
        unsafe {
            let mut app_exec = ptr::null();
            let mut count = 0u32;
            let mut time_ = 0i64;
            let mut tmp_app_name = app_name.to_tmp_for_borrow();

            let ret = to_bool(
                ffi::gtk_recent_info_get_application_info(
                    GTK_RECENT_INFO(self.unwrap_widget()),
                    tmp_app_name.to_glib_ptr(),
                    &mut app_exec,
                    &mut count,
                    &mut time_));

            if ret {
                let app_exec = FromGlibPtrNotNull::borrow(app_exec);
                Some((app_exec, count, time_))
            }
            else {
                None
            }
        }
    }

    pub fn get_applications(&self) -> Option<Vec<String>> {
        let mut length = 0;
        let tmp = unsafe { ffi::gtk_recent_info_get_applications(GTK_RECENT_INFO(self.unwrap_widget()), &mut length) };

        if tmp.is_null() {
            None
        } else {
            let mut ret = Vec::with_capacity(length as usize);

            for count in range(0, length) {
                ret.push(unsafe { String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&(*tmp.offset(count as isize) as *const c_char))).to_string() });
            }
            Some(ret)
        }
    }

    pub fn last_application(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_last_application(GTK_RECENT_INFO(self.unwrap_widget())) as *const c_char)
        }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            let mut tmp_app_name = app_name.to_tmp_for_borrow();
            to_bool(ffi::gtk_recent_info_has_application(GTK_RECENT_INFO(self.unwrap_widget()), tmp_app_name.to_glib_ptr()))
        }
    }

    pub fn get_groups(&self) -> Option<Vec<String>> {
        let mut length = 0;
        let tmp = unsafe { ffi::gtk_recent_info_get_groups(GTK_RECENT_INFO(self.unwrap_widget()), &mut length) };

        if tmp.is_null() {
            None
        } else {
            let mut ret = Vec::with_capacity(length as usize);

            for count in range(0, length) {
                ret.push(unsafe { String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&(*tmp.offset(count as isize) as *const c_char))).to_string() });
            }
            Some(ret)
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        let c_str = CString::from_slice(group_name.as_bytes());

        unsafe {
            to_bool(ffi::gtk_recent_info_has_group(GTK_RECENT_INFO(self.unwrap_widget()), c_str.as_ptr()))
        }
    }

    pub fn get_short_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_get_short_name(GTK_RECENT_INFO(self.unwrap_widget())) as *const c_char)
        }
    }

    pub fn get_uri_display(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_recent_info_get_uri_display(GTK_RECENT_INFO(self.unwrap_widget())) as *const c_char)
        }
    }

    pub fn get_age(&self) -> i32 {
        unsafe { ffi::gtk_recent_info_get_age(GTK_RECENT_INFO(self.unwrap_widget())) }
    }

    pub fn is_local(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_info_is_local(GTK_RECENT_INFO(self.unwrap_widget()))) }
    }

    pub fn exists(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_info_exists(GTK_RECENT_INFO(self.unwrap_widget()))) }
    }

    pub fn _match(&self, other: &RecentInfo) -> bool {
        unsafe { to_bool(ffi::gtk_recent_info_match(GTK_RECENT_INFO(self.unwrap_widget()), GTK_RECENT_INFO(other.unwrap_widget()))) }
    }
}

impl_drop!(RecentInfo);
impl_TraitWidget!(RecentInfo);

impl_widget_events!(RecentInfo);
