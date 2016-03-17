// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use glib::translate::*;
use ffi;

glib_wrapper! {
    pub struct RecentInfo(Shared<ffi::GtkRecentInfo>);

    match fn {
        ref => |ptr| ffi::gtk_recent_info_ref(ptr),
        unref => |ptr| ffi::gtk_recent_info_unref(ptr),
    }
}

impl RecentInfo {
    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_display_name(self.to_glib_none().0))
        }
    }

    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_description(self.to_glib_none().0))
        }
    }

    pub fn get_mime_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_mime_type(self.to_glib_none().0))
        }
    }

    pub fn get_added(&self) -> Option<u64> {
        match unsafe { ffi::gtk_recent_info_get_added(self.to_glib_none().0) } {
            x if x >= 0 => Some(x as u64),
            _ => None
        }
    }

    pub fn get_modified(&self) -> Option<u64> {
        match unsafe { ffi::gtk_recent_info_get_modified(self.to_glib_none().0) } {
            x if x >= 0 => Some(x as u64),
            _ => None
        }
    }

    pub fn get_visited(&self) -> Option<u64> {
        match unsafe { ffi::gtk_recent_info_get_visited(self.to_glib_none().0) } {
            x if x >= 0 => Some(x as u64),
            _ => None
        }
    }

    pub fn get_private_hint(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_get_private_hint(self.to_glib_none().0)) }
    }

    pub fn get_application_info(&self, app_name: &str) -> Option<(String, u32, u64)> {
        unsafe {
            let mut app_exec = ptr::null();
            let mut count = 0;
            let mut time_ = 0;

            if from_glib(ffi::gtk_recent_info_get_application_info(
                    self.to_glib_none().0, app_name.to_glib_none().0,
                    &mut app_exec, &mut count, &mut time_)) {
                Some((from_glib_none(app_exec), count, time_ as u64))
            } else {
                None
            }
        }
    }

    pub fn get_applications(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gtk_recent_info_get_applications(self.to_glib_none().0, &mut length);
            Vec::from_glib_full_num(ptr, length as usize)
        }
    }

    pub fn last_application(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_last_application(self.to_glib_none().0))
        }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_recent_info_has_application(self.to_glib_none().0,
                    app_name.to_glib_none().0))
        }
    }

    pub fn get_groups(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gtk_recent_info_get_groups(self.to_glib_none().0, &mut length);
            Vec::from_glib_full_num(ptr, length as usize)
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_recent_info_has_group(self.to_glib_none().0, group_name.to_glib_none().0))
        }
    }

    pub fn get_short_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_short_name(self.to_glib_none().0))
        }
    }

    pub fn get_uri_display(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_uri_display(self.to_glib_none().0))
        }
    }

    pub fn get_age(&self) -> i32 {
        unsafe { ffi::gtk_recent_info_get_age(self.to_glib_none().0) }
    }

    pub fn is_local(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_is_local(self.to_glib_none().0)) }
    }

    pub fn exists(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_exists(self.to_glib_none().0)) }
    }

    pub fn match_(&self, other: &RecentInfo) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_match(self.to_glib_none().0, other.to_glib_none().0))
        }
    }
}
