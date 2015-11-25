// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::c_char;

use glib::translate::*;
use ffi;

use glib::object::Upcast;

use {
    RecentFilterFlags,
    RecentSortType,
};

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

impl <'a> ToGlibPtr<'a, *mut ffi::GtkRecentData> for &'a RecentData {
    type Storage = (Box<ffi::GtkRecentData>,
                    [Stash<'a, *const c_char, String>; 5],
                    Stash<'a, *mut *const c_char, &'a [String]>);

    fn to_glib_none(&self)
        -> Stash<'a, *mut ffi::GtkRecentData, &'a RecentData> {
        let display_name = self.display_name.to_glib_none();
        let description = self.description.to_glib_none();
        let mime_type = self.mime_type.to_glib_none();
        let app_name = self.app_name.to_glib_none();
        let app_exec = self.app_exec.to_glib_none();
        let groups = (&*self.groups).to_glib_none();

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
    pub struct RecentFilter(Object<ffi::GtkRecentFilter>): ::Buildable;

    match fn {
        get_type => || ffi::gtk_recent_filter_get_type(),
    }
}

impl RecentFilter {
    pub fn new() -> RecentFilter {
        unsafe { from_glib_full(ffi::gtk_recent_filter_new()) }
    }

    pub fn add_application(&self, application: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_application(self.to_glib_none().0, application.to_glib_none().0)
        }
    }

    pub fn add_group(&self, group: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_group(self.to_glib_none().0, group.to_glib_none().0)
        }
    }

    pub fn add_age(&self, days: i32) {
        unsafe { ffi::gtk_recent_filter_add_age(self.to_glib_none().0, days) }
    }

    pub fn get_needed(&self) -> RecentFilterFlags {
        unsafe { ffi::gtk_recent_filter_get_needed(self.to_glib_none().0) }
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

//////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct RecentManager(Object<ffi::GtkRecentManager>);

    match fn {
        get_type => || ffi::gtk_recent_manager_get_type(),
    }
}

impl RecentManager {
    /// Creates a new recent manager object.
    ///
    /// Recent manager objects are used to handle the list of recently used
    /// resources. A `RecentManager` object monitors the recently used
    /// resources list, and emits the `changed` signal each time something
    /// inside the list changes.
    ///
    /// `RecentManager` objects are expensive: be sure to create them only
    /// when needed.
    pub fn new() -> RecentManager {
        unsafe { from_glib_full(ffi::gtk_recent_manager_new()) }
    }

    pub fn add_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_recent_manager_add_item(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    pub fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_recent_manager_add_full(self.to_glib_none().0, uri.to_glib_none().0,
                    recent_data.to_glib_none().0))
        }
    }

    pub fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_recent_manager_has_item(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    pub fn get_items(&self) -> Vec<RecentInfo> {
        unsafe { Vec::from_glib_full(ffi::gtk_recent_manager_get_items(self.to_glib_none().0)) }
    }
}

//////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct RecentChooser(Object<ffi::GtkRecentChooser>);

    match fn {
        get_type => || ffi::gtk_recent_chooser_get_type(),
    }
}

pub trait RecentChooserExt {
    fn set_show_private(&self, show_private: bool);
    fn get_show_private(&self) -> bool;
    fn set_show_not_found(&self, show_not_found: bool);
    fn get_show_not_found(&self) -> bool;
    fn set_show_icons(&self, show_icons: bool);
    fn get_show_icons(&self) -> bool;
    fn set_select_multiple(&self, select_multiple: bool);
    fn get_select_multiple(&self) -> bool;
    fn set_local_only(&self, local_only: bool);
    fn get_local_only(&self) -> bool;
    fn set_limit(&self, limit: i32);
    fn get_limit(&self) -> i32;
    fn set_show_tips(&self, show_tips: bool);
    fn get_show_tips(&self) -> bool;
    fn set_sort_type(&self, sort_type: RecentSortType);
    fn get_sort_type(&self) -> RecentSortType;
    fn get_current_uri(&self) -> Option<String>;
    fn get_current_item(&self) -> Option<RecentInfo>;
    fn unselect_uri(&self, uri: &str);
    fn select_all(&self);
    fn unselect_all(&self);
    fn get_items(&self) -> Vec<RecentInfo>;
    fn get_uris(&self) -> Vec<String>;
    fn add_filter(&self, filter: &RecentFilter);
    fn remove_filter(&self, filter: &RecentFilter);
    fn list_filters(&self) -> Vec<RecentFilter>;
    fn set_filter(&self, filter: &RecentFilter);
    fn get_filter(&self) -> Option<RecentFilter>;
}

impl<O: Upcast<RecentChooser>> RecentChooserExt for O {
    fn set_show_private(&self, show_private: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_private(self.upcast().to_glib_none().0,
                show_private.to_glib())
        }
    }

    fn get_show_private(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_private(self.upcast().to_glib_none().0))
        }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_not_found(self.upcast().to_glib_none().0,
                show_not_found.to_glib())
        }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_not_found(self.upcast().to_glib_none().0))
        }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_icons(self.upcast().to_glib_none().0,
                show_icons.to_glib())
        }
    }

    fn get_show_icons(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_chooser_get_show_icons(self.upcast().to_glib_none().0)) }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_select_multiple(self.upcast().to_glib_none().0,
                select_multiple.to_glib())
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_select_multiple(self.upcast().to_glib_none().0))
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_local_only(self.upcast().to_glib_none().0,
                local_only.to_glib())
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_chooser_get_local_only(self.upcast().to_glib_none().0)) }
    }

    fn set_limit(&self, limit: i32) {
        unsafe { ffi::gtk_recent_chooser_set_limit(self.upcast().to_glib_none().0, limit) }
    }

    fn get_limit(&self) -> i32 {
        unsafe { ffi::gtk_recent_chooser_get_limit(self.upcast().to_glib_none().0) }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_tips(self.upcast().to_glib_none().0,
                show_tips.to_glib())
        }
    }

    fn get_show_tips(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_chooser_get_show_tips(self.upcast().to_glib_none().0)) }
    }

    fn set_sort_type(&self, sort_type: RecentSortType) {
        unsafe { ffi::gtk_recent_chooser_set_sort_type(self.upcast().to_glib_none().0, sort_type) }
    }

    fn get_sort_type(&self) -> RecentSortType {
        unsafe { ffi::gtk_recent_chooser_get_sort_type(self.upcast().to_glib_none().0) }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_recent_chooser_get_current_uri(self.upcast().to_glib_none().0))
        }
    }

    fn get_current_item(&self) -> Option<RecentInfo> {
        unsafe { from_glib_full(ffi::gtk_recent_chooser_get_current_item(self.upcast().to_glib_none().0)) }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_uri(self.upcast().to_glib_none().0,
                uri.to_glib_none().0)
        }
    }

    fn select_all(&self) {
        unsafe { ffi::gtk_recent_chooser_select_all(self.upcast().to_glib_none().0) }
    }

    fn unselect_all(&self) {
        unsafe { ffi::gtk_recent_chooser_unselect_all(self.upcast().to_glib_none().0) }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            Vec::from_glib_full(ffi::gtk_recent_chooser_get_items(self.upcast().to_glib_none().0))
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gtk_recent_chooser_get_uris(self.upcast().to_glib_none().0, &mut length);
            Vec::from_glib_none_num(ptr as *const *const c_char, length as usize)
        }
    }

    fn add_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_add_filter(self.upcast().to_glib_none().0, filter.to_glib_none().0)
        }
    }

    fn remove_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_remove_filter(self.upcast().to_glib_none().0, filter.to_glib_none().0)
        }
    }

    fn list_filters(&self) -> Vec<RecentFilter> {
        unsafe {
            Vec::from_glib_full(
                ffi::gtk_recent_chooser_list_filters(self.upcast().to_glib_none().0))
        }
    }

    fn set_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_set_filter(self.upcast().to_glib_none().0, filter.to_glib_none().0)
        }
    }

    fn get_filter(&self) -> Option<RecentFilter> {
        unsafe {
            from_glib_none(ffi::gtk_recent_chooser_get_filter(self.upcast().to_glib_none().0))
        }
    }
}
