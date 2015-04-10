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

//! A container box

use ffi;
use glib::{to_bool, to_gboolean};
use cast::{GTK_SEARCHBAR, GTK_ENTRY};
use FFIWidget;

/// Box — A container box
struct_Widget!(SearchBar);

impl SearchBar {
    pub fn new() -> Option<SearchBar> {
        let tmp_pointer = unsafe { ffi::gtk_search_bar_new() };
        check_pointer!(tmp_pointer, SearchBar)
    }

    pub fn connect_entry(&mut self, entry: &::Entry) -> () {
        unsafe {
            ffi::gtk_search_bar_connect_entry(GTK_SEARCHBAR(self.pointer), GTK_ENTRY(entry.unwrap_widget()));
        }
    }

    pub fn set_search_mode(&mut self, search_mode: bool) -> () {
        unsafe { ffi::gtk_search_bar_set_search_mode(GTK_SEARCHBAR(self.pointer), to_gboolean(search_mode)); }
    }

    pub fn get_search_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_search_bar_get_search_mode(GTK_SEARCHBAR(self.pointer))) }
    }

    pub fn set_show_close_button(&mut self, visible: bool) -> () {
        unsafe { ffi::gtk_search_bar_set_show_close_button(GTK_SEARCHBAR(self.pointer), to_gboolean(visible)); }
    }

    pub fn get_show_close_button(&self) -> bool {
        unsafe { to_bool(ffi::gtk_search_bar_get_show_close_button(GTK_SEARCHBAR(self.pointer))) }
    }
}

impl_drop!(SearchBar);
impl_TraitWidget!(SearchBar);

impl ::ContainerTrait for SearchBar {}
impl ::BinTrait for SearchBar {}

impl_widget_events!(SearchBar);
