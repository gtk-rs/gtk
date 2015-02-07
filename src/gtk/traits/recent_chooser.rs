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

use std::ffi::CString;
use gtk::cast::{GTK_RECENT_CHOOSER};
use gtk::{self, ffi};
use gtk::ffi::{to_bool, to_gboolean};
use gtk::ffi::FFIWidget;
use glib;
use libc::c_char;

pub trait RecentChooserTrait: gtk::WidgetTrait + FFIWidget {
    fn set_show_private(&self, show_private: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_private(GTK_RECENT_CHOOSER(self.get_widget()), to_gboolean(show_private)) }
    }

    fn get_show_private(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_private(GTK_RECENT_CHOOSER(self.get_widget()))) }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_not_found(GTK_RECENT_CHOOSER(self.get_widget()), to_gboolean(show_not_found)) }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_not_found(GTK_RECENT_CHOOSER(self.get_widget()))) }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_icons(GTK_RECENT_CHOOSER(self.get_widget()), to_gboolean(show_icons)) }
    }

    fn get_show_icons(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_icons(GTK_RECENT_CHOOSER(self.get_widget()))) }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe { ffi::gtk_recent_chooser_set_select_multiple(GTK_RECENT_CHOOSER(self.get_widget()), to_gboolean(select_multiple)) }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_select_multiple(GTK_RECENT_CHOOSER(self.get_widget()))) }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe { ffi::gtk_recent_chooser_set_local_only(GTK_RECENT_CHOOSER(self.get_widget()), to_gboolean(local_only)) }
    }

    fn get_local_only(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_local_only(GTK_RECENT_CHOOSER(self.get_widget()))) }
    }

    fn set_limit(&self, limit: i32) {
        unsafe { ffi::gtk_recent_chooser_set_limit(GTK_RECENT_CHOOSER(self.get_widget()), limit) }
    }

    fn get_limit(&self) -> i32 {
        unsafe { ffi::gtk_recent_chooser_get_limit(GTK_RECENT_CHOOSER(self.get_widget())) }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_tips(GTK_RECENT_CHOOSER(self.get_widget()), to_gboolean(show_tips)) }
    }

    fn get_show_tips(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_tips(GTK_RECENT_CHOOSER(self.get_widget()))) }
    }

    fn set_sort_type(&self, sort_type: gtk::RecentSortType) {
        unsafe { ffi::gtk_recent_chooser_set_sort_type(GTK_RECENT_CHOOSER(self.get_widget()), sort_type) }
    }

    fn get_sort_type(&self) -> gtk::RecentSortType {
        unsafe { ffi::gtk_recent_chooser_get_sort_type(GTK_RECENT_CHOOSER(self.get_widget())) }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            let tmp = ffi::gtk_recent_chooser_get_current_uri(GTK_RECENT_CHOOSER(self.get_widget()));

            if tmp.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&(tmp as *const c_char))).to_string())
            }
        }
    }

    fn get_current_item(&self) -> Option<gtk::RecentInfo> {
        let tmp = unsafe { ffi::gtk_recent_chooser_get_current_item(GTK_RECENT_CHOOSER(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp as *mut ffi::C_GtkWidget))
        }
    }

    fn unselect_uri(&self, uri: &str) -> bool {
        unsafe {
            let c_str = CString::from_slice(uri.as_bytes());

            to_bool(ffi::gtk_recent_chooser_unselect_uri(GTK_RECENT_CHOOSER(self.get_widget()), c_str.as_ptr()))
        }
    }

    fn select_all(&self) {
        unsafe { ffi::gtk_recent_chooser_select_all(GTK_RECENT_CHOOSER(self.get_widget())) }
    }

    fn unselect_all(&self) {
        unsafe { ffi::gtk_recent_chooser_unselect_all(GTK_RECENT_CHOOSER(self.get_widget())) }
    }

    fn get_items(&self) -> glib::List<Box<gtk::RecentInfo>> {
        let tmp = unsafe { ffi::gtk_recent_chooser_get_items(GTK_RECENT_CHOOSER(self.get_widget())) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::C_GtkRecentInfo> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<gtk::RecentInfo>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(ffi::FFIWidget::wrap(*it as *mut gtk::ffi::C_GtkWidget)));
            }
            tmp_vec
        }
    }

    fn get_uris(&self) -> Option<Vec<String>> {
        let mut length = 0;
        let tmp = unsafe { ffi::gtk_recent_chooser_get_uris(GTK_RECENT_CHOOSER(self.get_widget()), &mut length) };

        if tmp.is_null() {
            None
        } else {
            unsafe {
                let mut ret = Vec::with_capacity(length as usize);

                for count in range(0, length) {
                    let t = tmp.offset(count as isize) as *const c_char;

                    ret.push(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&t)).to_string());
                }
                Some(ret)
            }
        }
    }

    fn add_filter(&self, filter: &gtk::RecentFilter) {
        unsafe { ffi::gtk_recent_chooser_add_filter(GTK_RECENT_CHOOSER(self.get_widget()), filter.get_pointer()) }
    }

    fn remove_filter(&self, filter: &gtk::RecentFilter) {
        unsafe { ffi::gtk_recent_chooser_remove_filter(GTK_RECENT_CHOOSER(self.get_widget()), filter.get_pointer()) }
    }

    // fn list_filters(&self) -> glib::SList<Box<gtk::RecentFilter>> {
    //     let tmp = unsafe { ffi::gtk_recent_chooser_list_filters(self.get_pointer()) };

    //     if tmp.is_null() {
    //         glib::SList::new()
    //     } else {
    //         let old_list : glib::SList<*mut ffi::C_GtkRecentFilter> = glib::GlibContainer::wrap(tmp);
    //         let mut tmp_vec : glib::SList<Box<gtk::RecentFilter>> = glib::SList::new();

    //         for it in old_list.iter() {
    //             match gtk::RecentFilter::wrap(*it) {
    //                 Some(r) => tmp_vec.append(Box::new(r)),
    //                 None => {}
    //             }
    //         }
    //         tmp_vec
    //     }
    // }

    fn set_filter(&self, filter: &gtk::RecentFilter) {
        unsafe { ffi::gtk_recent_chooser_set_filter(GTK_RECENT_CHOOSER(self.get_widget()), filter.get_pointer()) }
    }

    fn get_filter(&self) -> Option<gtk::RecentFilter> {
        let tmp = unsafe { ffi::gtk_recent_chooser_get_filter(GTK_RECENT_CHOOSER(self.get_widget())) };

        gtk::RecentFilter::wrap(tmp)
    }
}
