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

//! GtkPlacesSidebar — Sidebar that displays frequently-used places in the file system

use gtk::{mod, ffi};
use gtk::ffi::FFIWidget;
use gtk::cast::GTK_PLACES_SIDEBAR;

struct_Widget!(PlacesSidebar);

impl PlacesSidebar {
    pub fn new() -> Option<PlacesSidebar> {
        let tmp_pointer = unsafe { ffi::gtk_places_sidebar_new() };
        check_pointer!(tmp_pointer, PlacesSidebar)
    }

    pub fn set_open_flags(&self, flags: gtk::PlacesOpenFlags) {
        unsafe { ffi::gtk_places_sidebar_set_open_flags(GTK_PLACES_SIDEBAR(self.get_widget()), flags) }
    }

    pub fn get_open_flags(&self) -> gtk::PlacesOpenFlags {
        unsafe { ffi::gtk_places_sidebar_get_open_flags(GTK_PLACES_SIDEBAR(self.get_widget())) }
    }

    pub fn set_show_desktop(&self, show_desktop: bool) {
        unsafe { ffi::gtk_places_sidebar_set_show_desktop(GTK_PLACES_SIDEBAR(self.get_widget()), ffi::to_gboolean(show_desktop)) }
    }

    pub fn get_show_desktop(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_places_sidebar_get_show_desktop(GTK_PLACES_SIDEBAR(self.get_widget()))) }
    }

    pub fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe { ffi::gtk_places_sidebar_set_show_connect_to_server(GTK_PLACES_SIDEBAR(self.get_widget()),
            ffi::to_gboolean(show_connect_to_server)) }
    }

    pub fn get_show_connect_to_server(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_places_sidebar_get_show_connect_to_server(GTK_PLACES_SIDEBAR(self.get_widget()))) }
    }

    pub fn set_local_only(&self, local_only: bool) {
        unsafe { ffi::gtk_places_sidebar_set_local_only(GTK_PLACES_SIDEBAR(self.get_widget()), ffi::to_gboolean(local_only)) }
    }

    pub fn get_local_only(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_places_sidebar_get_local_only(GTK_PLACES_SIDEBAR(self.get_widget()))) }
    }

    pub fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe { ffi::gtk_places_sidebar_set_show_enter_location(GTK_PLACES_SIDEBAR(self.get_widget()),
            ffi::to_gboolean(show_enter_location)) }
    }

    pub fn get_show_enter_location(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_places_sidebar_get_show_enter_location(GTK_PLACES_SIDEBAR(self.get_widget()))) }
    }
}

impl_drop!(PlacesSidebar);
impl_TraitWidget!(PlacesSidebar);

impl gtk::ContainerTrait for PlacesSidebar {}
impl gtk::BinTrait for PlacesSidebar {}
impl gtk::ScrolledWindowTrait for PlacesSidebar {}

impl_widget_events!(PlacesSidebar);