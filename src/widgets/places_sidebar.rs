// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::GTK_PLACES_SIDEBAR;
use glib::{to_bool, to_gboolean};

struct_Widget!(PlacesSidebar);

impl PlacesSidebar {
    pub fn new() -> Option<PlacesSidebar> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_places_sidebar_new() };
        check_pointer!(tmp_pointer, PlacesSidebar)
    }

    pub fn set_open_flags(&self, flags: ::PlacesOpenFlags) {
        unsafe { ffi::gtk_places_sidebar_set_open_flags(GTK_PLACES_SIDEBAR(self.unwrap_widget()), flags) }
    }

    pub fn get_open_flags(&self) -> ::PlacesOpenFlags {
        unsafe { ffi::gtk_places_sidebar_get_open_flags(GTK_PLACES_SIDEBAR(self.unwrap_widget())) }
    }

    pub fn set_show_desktop(&self, show_desktop: bool) {
        unsafe { ffi::gtk_places_sidebar_set_show_desktop(GTK_PLACES_SIDEBAR(self.unwrap_widget()), to_gboolean(show_desktop)) }
    }

    pub fn get_show_desktop(&self) -> bool {
        unsafe { to_bool(ffi::gtk_places_sidebar_get_show_desktop(GTK_PLACES_SIDEBAR(self.unwrap_widget()))) }
    }

    pub fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe { ffi::gtk_places_sidebar_set_show_connect_to_server(GTK_PLACES_SIDEBAR(self.unwrap_widget()),
            to_gboolean(show_connect_to_server)) }
    }

    pub fn get_show_connect_to_server(&self) -> bool {
        unsafe { to_bool(ffi::gtk_places_sidebar_get_show_connect_to_server(GTK_PLACES_SIDEBAR(self.unwrap_widget()))) }
    }

    #[cfg(gtk_3_12)]
    pub fn set_local_only(&self, local_only: bool) {
        unsafe { ffi::gtk_places_sidebar_set_local_only(GTK_PLACES_SIDEBAR(self.unwrap_widget()), to_gboolean(local_only)) }
    }

    #[cfg(gtk_3_12)]
    pub fn get_local_only(&self) -> bool {
        unsafe { to_bool(ffi::gtk_places_sidebar_get_local_only(GTK_PLACES_SIDEBAR(self.unwrap_widget()))) }
    }

    #[cfg(gtk_3_14)]
    pub fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe { ffi::gtk_places_sidebar_set_show_enter_location(GTK_PLACES_SIDEBAR(self.unwrap_widget()),
            to_gboolean(show_enter_location)) }
    }

    #[cfg(gtk_3_14)]
    pub fn get_show_enter_location(&self) -> bool {
        unsafe { to_bool(ffi::gtk_places_sidebar_get_show_enter_location(GTK_PLACES_SIDEBAR(self.unwrap_widget()))) }
    }
}

impl_drop!(PlacesSidebar);
impl_TraitWidget!(PlacesSidebar);

impl ::ContainerTrait for PlacesSidebar {}
impl ::BinTrait for PlacesSidebar {}
impl ::ScrolledWindowTrait for PlacesSidebar {}
