// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::to_bool;
use FFIWidget;
use cast::GTK_RECENT_MANAGER;
use glib;
use glib::translate::ToGlibPtr;

struct_Widget!(RecentManager);

impl RecentManager {
    pub fn new() -> Option<RecentManager> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_recent_manager_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn get_default(&self) -> Option<RecentManager> {
        let tmp_pointer = unsafe { ffi::gtk_recent_manager_get_default() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn add_item(&self, uri: &str) -> bool {
        unsafe {
            to_bool(
                ffi::gtk_recent_manager_add_item(
                    GTK_RECENT_MANAGER(self.unwrap_widget()),
                    uri.to_glib_none().0))
        }
    }

    pub fn add_full(&self, uri: &str, recent_data: &::RecentData) -> bool {
        unsafe {
            to_bool(
                ffi::gtk_recent_manager_add_full(
                    GTK_RECENT_MANAGER(self.unwrap_widget()),
                    uri.to_glib_none().0,
                    recent_data.to_glib_none().0))
        }
    }

    pub fn has_item(&self, uri: &str) -> bool {
        unsafe {
            to_bool(
                ffi::gtk_recent_manager_has_item(
                    GTK_RECENT_MANAGER(self.unwrap_widget()),
                    uri.to_glib_none().0))
        }
    }

    pub fn get_items(&self) -> glib::List<Box<::RecentInfo>> {
        let tmp = unsafe { ffi::gtk_recent_manager_get_items(GTK_RECENT_MANAGER(self.unwrap_widget())) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::GtkRecentInfo> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<::RecentInfo>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(::FFIWidget::wrap_widget(*it as *mut ffi::GtkWidget)));
            }
            tmp_vec
        }
    }
}

impl_drop!(RecentManager);
impl_TraitWidget!(RecentManager);
