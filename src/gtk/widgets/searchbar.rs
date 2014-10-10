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

use gtk;
use gtk::cast::{GTK_SEARCHBAR, GTK_ENTRY};
use gtk::ffi;
use gtk::traits;
use gtk::ffi::FFIWidget;

/// Box — A container box
struct_Widget!(SearchBar)

impl SearchBar {
    pub fn new() -> Option<SearchBar> {
        let tmp_pointer = unsafe { ffi::gtk_search_bar_new() };
        check_pointer!(tmp_pointer, SearchBar)
    }

    pub fn connect_entry(&mut self, entry: &gtk::Entry) -> () {
        unsafe {
            ffi::gtk_search_bar_connect_entry(GTK_SEARCHBAR(self.pointer), GTK_ENTRY(entry.get_widget()));
        }
    }

    pub fn set_search_mode(&mut self, search_mode: bool) -> () {
        match search_mode {
            true    => unsafe { ffi::gtk_search_bar_set_search_mode(GTK_SEARCHBAR(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_search_bar_set_search_mode(GTK_SEARCHBAR(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_search_mode(&self) -> bool {
        match unsafe { ffi::gtk_search_bar_get_search_mode(GTK_SEARCHBAR(self.pointer)) } {
            0i32 => false,
            _ => true
        }
    }

    pub fn set_show_close_button(&mut self, visible: bool) -> () {
        match visible {
            true    => unsafe { ffi::gtk_search_bar_set_show_close_button(GTK_SEARCHBAR(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_search_bar_set_show_close_button(GTK_SEARCHBAR(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_show_close_button(&self) -> bool {
        match unsafe { ffi::gtk_search_bar_get_show_close_button(GTK_SEARCHBAR(self.pointer)) } {
            0i32 => false,
            _ => true
        }
    }
}

impl_drop!(SearchBar)
impl_TraitWidget!(SearchBar)

impl traits::Container for SearchBar {}
impl traits::Bin for SearchBar {}
