// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A container box

use std::ptr;
use libc::{c_void};


use traits::{GtkContainer, GtkWidget, GtkBin, Signal};
use gtk;
use utils::cast::{GTK_SEARCHBAR, GTK_ENTRY};
use ffi;
use std;

/// Box — A container box
pub struct SearchBar {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

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
            true    => unsafe { ffi::gtk_search_bar_set_search_mode(GTK_SEARCHBAR(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_search_bar_set_search_mode(GTK_SEARCHBAR(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_search_mode(&self) -> bool {
        match unsafe { ffi::gtk_search_bar_get_search_mode(GTK_SEARCHBAR(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_show_close_button(&mut self, visible: bool) -> () {
        match visible {
            true    => unsafe { ffi::gtk_search_bar_set_show_close_button(GTK_SEARCHBAR(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_search_bar_set_show_close_button(GTK_SEARCHBAR(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_show_close_button(&self) -> bool {
        match unsafe { ffi::gtk_search_bar_get_show_close_button(GTK_SEARCHBAR(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }
}

impl_GtkWidget!(SearchBar)
redirect_callback!(SearchBar)
redirect_callback_widget!(SearchBar)
struct_signal!(SearchBar)
impl_signals!(SearchBar)

impl GtkContainer for SearchBar {}
impl GtkBin for SearchBar {}
