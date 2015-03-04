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
use gtk::cast::GTK_RECENT_MANAGER;
use glib;
use std::ffi::CString;
use glib::translate::{ToGlibPtr, ToTmp};

struct_Widget!(RecentManager);

impl RecentManager {
    pub fn new() -> Option<RecentManager> {
        let tmp_pointer = unsafe { ffi::gtk_recent_manager_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn get_default(&self) -> Option<RecentManager> {
        let tmp_pointer = unsafe { ffi::gtk_recent_manager_get_default() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn add_item(&self, uri: &str) -> bool {
        let c_str = CString::from_slice(uri.as_bytes());

        unsafe {
            to_bool(ffi::gtk_recent_manager_add_item(GTK_RECENT_MANAGER(self.unwrap_widget()), c_str.as_ptr()))
        }
    }

    pub fn add_full(&self, uri: &str, recent_data: &gtk::RecentData) -> bool {
        unsafe {
            let mut tmp_uri = uri.to_tmp_for_borrow();
            to_bool(ffi::gtk_recent_manager_add_full(GTK_RECENT_MANAGER(self.unwrap_widget()), tmp_uri.to_glib_ptr(), &recent_data.get_ffi()))
        }
    }

    pub fn has_item(&self, uri: &str) -> bool {
        let c_str = CString::from_slice(uri.as_bytes());

        unsafe {
            to_bool(ffi::gtk_recent_manager_has_item(GTK_RECENT_MANAGER(self.unwrap_widget()), c_str.as_ptr()))
        }
    }

    pub fn get_items(&self) -> glib::List<Box<gtk::RecentInfo>> {
        let tmp = unsafe { ffi::gtk_recent_manager_get_items(GTK_RECENT_MANAGER(self.unwrap_widget())) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::C_GtkRecentInfo> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<gtk::RecentInfo>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(gtk::FFIWidget::wrap_widget(*it as *mut ffi::C_GtkWidget)));
            }
            tmp_vec
        }
    }
}

impl_drop!(RecentManager);
impl_TraitWidget!(RecentManager);

impl_widget_events!(RecentManager);
