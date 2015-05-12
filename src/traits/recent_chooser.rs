// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{from_glib_none, FromGlibPtrContainer, ToGlibPtr};
use cast::{GTK_RECENT_CHOOSER};
use ffi;
use glib::{to_bool, to_gboolean};
use FFIWidget;
use glib;
use libc::c_char;

pub trait RecentChooserTrait: ::WidgetTrait + FFIWidget {
    fn set_show_private(&self, show_private: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_private(GTK_RECENT_CHOOSER(self.unwrap_widget()), to_gboolean(show_private)) }
    }

    fn get_show_private(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_private(GTK_RECENT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_not_found(GTK_RECENT_CHOOSER(self.unwrap_widget()), to_gboolean(show_not_found)) }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_not_found(GTK_RECENT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_icons(GTK_RECENT_CHOOSER(self.unwrap_widget()), to_gboolean(show_icons)) }
    }

    fn get_show_icons(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_icons(GTK_RECENT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe { ffi::gtk_recent_chooser_set_select_multiple(GTK_RECENT_CHOOSER(self.unwrap_widget()), to_gboolean(select_multiple)) }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_select_multiple(GTK_RECENT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe { ffi::gtk_recent_chooser_set_local_only(GTK_RECENT_CHOOSER(self.unwrap_widget()), to_gboolean(local_only)) }
    }

    fn get_local_only(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_local_only(GTK_RECENT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_limit(&self, limit: i32) {
        unsafe { ffi::gtk_recent_chooser_set_limit(GTK_RECENT_CHOOSER(self.unwrap_widget()), limit) }
    }

    fn get_limit(&self) -> i32 {
        unsafe { ffi::gtk_recent_chooser_get_limit(GTK_RECENT_CHOOSER(self.unwrap_widget())) }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe { ffi::gtk_recent_chooser_set_show_tips(GTK_RECENT_CHOOSER(self.unwrap_widget()), to_gboolean(show_tips)) }
    }

    fn get_show_tips(&self) -> bool {
        unsafe { to_bool(ffi::gtk_recent_chooser_get_show_tips(GTK_RECENT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_sort_type(&self, sort_type: ::RecentSortType) {
        unsafe { ffi::gtk_recent_chooser_set_sort_type(GTK_RECENT_CHOOSER(self.unwrap_widget()), sort_type) }
    }

    fn get_sort_type(&self) -> ::RecentSortType {
        unsafe { ffi::gtk_recent_chooser_get_sort_type(GTK_RECENT_CHOOSER(self.unwrap_widget())) }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_recent_chooser_get_current_uri(GTK_RECENT_CHOOSER(self.unwrap_widget())))
        }
    }

    fn get_current_item(&self) -> Option<::RecentInfo> {
        let tmp = unsafe { ffi::gtk_recent_chooser_get_current_item(GTK_RECENT_CHOOSER(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp as *mut ffi::GtkWidget))
        }
    }

    fn unselect_uri(&self, uri: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_recent_chooser_unselect_uri(GTK_RECENT_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0))
        }
    }

    fn select_all(&self) {
        unsafe { ffi::gtk_recent_chooser_select_all(GTK_RECENT_CHOOSER(self.unwrap_widget())) }
    }

    fn unselect_all(&self) {
        unsafe { ffi::gtk_recent_chooser_unselect_all(GTK_RECENT_CHOOSER(self.unwrap_widget())) }
    }

    fn get_items(&self) -> glib::List<Box<::RecentInfo>> {
        let tmp = unsafe { ffi::gtk_recent_chooser_get_items(GTK_RECENT_CHOOSER(self.unwrap_widget())) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::GtkRecentInfo> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<::RecentInfo>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(::FFIWidget::wrap_widget(*it as *mut ::ffi::GtkWidget)));
            }
            tmp_vec
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gtk_recent_chooser_get_uris(
                GTK_RECENT_CHOOSER(self.unwrap_widget()),
                &mut length) as *const *const c_char;
            Vec::from_glib_none_num(ptr, length as usize)
        }
    }

    fn add_filter(&self, filter: &::RecentFilter) {
        unsafe { ffi::gtk_recent_chooser_add_filter(GTK_RECENT_CHOOSER(self.unwrap_widget()), filter.unwrap_pointer()) }
    }

    fn remove_filter(&self, filter: &::RecentFilter) {
        unsafe { ffi::gtk_recent_chooser_remove_filter(GTK_RECENT_CHOOSER(self.unwrap_widget()), filter.unwrap_pointer()) }
    }

    // fn list_filters(&self) -> glib::SList<Box<::RecentFilter>> {
    //     let tmp = unsafe { ffi::gtk_recent_chooser_list_filters(self.unwrap_pointer()) };

    //     if tmp.is_null() {
    //         glib::SList::new()
    //     } else {
    //         let old_list : glib::SList<*mut ffi::GtkRecentFilter> = glib::GlibContainer::wrap(tmp);
    //         let mut tmp_vec : glib::SList<Box<::RecentFilter>> = glib::SList::new();

    //         for it in old_list.iter() {
    //             match ::RecentFilter::wrap(*it) {
    //                 Some(r) => tmp_vec.append(Box::new(r)),
    //                 None => {}
    //             }
    //         }
    //         tmp_vec
    //     }
    // }

    fn set_filter(&self, filter: &::RecentFilter) {
        unsafe { ffi::gtk_recent_chooser_set_filter(GTK_RECENT_CHOOSER(self.unwrap_widget()), filter.unwrap_pointer()) }
    }

    fn get_filter(&self) -> Option<::RecentFilter> {
        let tmp = unsafe { ffi::gtk_recent_chooser_get_filter(GTK_RECENT_CHOOSER(self.unwrap_widget())) };

        ::RecentFilter::wrap(tmp)
    }
}
