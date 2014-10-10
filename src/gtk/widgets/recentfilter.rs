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
use gtk::cast::GTK_RECENT_FILTER;
use gtk;

struct_Widget!(RecentFilter)

impl RecentFilter {
    pub fn new() -> Option<RecentFilter> {
        let tmp_pointer = unsafe { ffi::gtk_recent_filter_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn add_application(&self, application: &str) -> () {
        unsafe {
            application.with_c_str(|c_str| {
                ffi::gtk_recent_filter_add_application(GTK_RECENT_FILTER(self.get_widget()), c_str)
            })
        }
    }

    pub fn add_group(&self, group: &str) -> () {
        unsafe {
            group.with_c_str(|c_str| {
                ffi::gtk_recent_filter_add_group(GTK_RECENT_FILTER(self.get_widget()), c_str)
            })
        }
    }

    pub fn add_age(&self, days: i32) -> () {
        unsafe { ffi::gtk_recent_filter_add_age(GTK_RECENT_FILTER(self.get_widget()), days) }
    }

    pub fn get_needed(&self) -> gtk::RecentFilterFlags {
        unsafe { ffi::gtk_recent_filter_get_needed(GTK_RECENT_FILTER(self.get_widget())) }
    }

    pub fn filter(&self, filter_info: &gtk::RecentFilterInfo) -> bool {
        match unsafe { ffi::gtk_recent_filter_filter(GTK_RECENT_FILTER(self.get_widget()), &filter_info.get_ffi()) } {
            0i32 => false,
            _ => true
        }
    }
}

impl_drop!(RecentFilter)
impl_TraitWidget!(RecentFilter)
