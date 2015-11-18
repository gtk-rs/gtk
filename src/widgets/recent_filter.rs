// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::to_bool;
use glib::translate::ToGlibPtr;

pub struct RecentFilter {
    pointer: *mut ffi::GtkRecentFilter
}

impl RecentFilter {
    pub fn new() -> Option<RecentFilter> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_recent_filter_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(RecentFilter{ pointer: tmp_pointer })
        }
    }

    pub fn add_application(&self, application: &str) -> () {
        unsafe {
            ffi::gtk_recent_filter_add_application(self.pointer, application.to_glib_none().0)
        }
    }

    pub fn add_group(&self, group: &str) -> () {
        unsafe {
            ffi::gtk_recent_filter_add_group(self.pointer, group.to_glib_none().0)
        }
    }

    pub fn add_age(&self, days: i32) -> () {
        unsafe { ffi::gtk_recent_filter_add_age(self.pointer, days) }
    }

    pub fn get_needed(&self) -> ::RecentFilterFlags {
        unsafe { ffi::gtk_recent_filter_get_needed(self.pointer) }
    }

    pub fn filter(&self, filter_info: &::RecentFilterInfo) -> bool {
        unsafe { to_bool(ffi::gtk_recent_filter_filter(self.pointer, &filter_info.get_ffi())) }
    }

    pub unsafe fn wrap(pointer: *mut ffi::GtkRecentFilter) -> Option<RecentFilter> {
        if pointer.is_null() {
            None
        } else {
            Some(RecentFilter{ pointer: pointer })
        }
    }

    pub fn unwrap_pointer(&self) -> *mut ffi::GtkRecentFilter {
        self.pointer
    }
}

// impl_drop!(RecentFilter, GTK_RECENT_FILTER)
