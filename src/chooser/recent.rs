// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::c_char;

use glib::translate::*;
use ffi;

//////////////////////////////////////////////////////////////////////////////

pub struct RecentData {
    display_name: String,
    description: String,
    mime_type: String,
    app_name: String,
    app_exec: String,
    groups: Vec<String>,
    is_private: bool
}

impl <'a> ToGlibPtr<'a, *mut ffi::GtkRecentData> for RecentData {
    type Storage = (Box<ffi::GtkRecentData>,
                    [Stash<'a, *const c_char, String>; 5],
                    Stash<'a, *mut *const c_char, [String]>);

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GtkRecentData, Self> {
        let display_name = self.display_name.to_glib_none();
        let description = self.description.to_glib_none();
        let mime_type = self.mime_type.to_glib_none();
        let app_name = self.app_name.to_glib_none();
        let app_exec = self.app_exec.to_glib_none();
        let groups = self.groups.to_glib_none();

        let mut data = Box::new(ffi::GtkRecentData {
            display_name: display_name.0 as *mut c_char,
            description: description.0 as *mut c_char,
            mime_type: mime_type.0 as *mut c_char,
            app_name: app_name.0 as *mut c_char,
            app_exec: app_exec.0 as *mut c_char,
            groups: groups.0 as *mut *mut c_char,
            is_private: self.is_private.to_glib(),
        });

        Stash(&mut *data, (data, [display_name, description, mime_type,
                                  app_name, app_exec], groups))
    }
}

//////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct RecentInfo(Refcounted<ffi::GtkRecentInfo>);

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

            match from_glib(ffi::gtk_recent_info_get_application_info(
                    self.to_glib_none().0, app_name.to_glib_none().0,
                    &mut app_exec, &mut count, &mut time_)) {
                true => Some((from_glib_none(app_exec), count, time_ as u64)),
                _ => None
            }
        }
    }

    pub fn get_applications(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gtk_recent_info_get_applications(self.to_glib_none().0, &mut length);
            Vec::from_glib_full_num(ptr as *const *const c_char, length as usize)
        }
    }

    pub fn last_application(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_last_application(self.to_glib_none().0))
        }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_application(self.to_glib_none().0, app_name.to_glib_none().0))
        }
    }

    pub fn get_groups(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gtk_recent_info_get_groups(self.to_glib_none().0, &mut length);
            Vec::from_glib_full_num(ptr as *const *const c_char, length as usize)
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_group(self.to_glib_none().0, group_name.to_glib_none().0))
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
