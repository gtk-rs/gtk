// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

    pub fn connect_entry(&self, entry: &::Entry) -> () {
        unsafe {
            ffi::gtk_search_bar_connect_entry(GTK_SEARCHBAR(self.pointer), GTK_ENTRY(entry.unwrap_widget()));
        }
    }

    pub fn set_search_mode(&self, search_mode: bool) -> () {
        unsafe { ffi::gtk_search_bar_set_search_mode(GTK_SEARCHBAR(self.pointer), to_gboolean(search_mode)); }
    }

    pub fn get_search_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_search_bar_get_search_mode(GTK_SEARCHBAR(self.pointer))) }
    }

    pub fn set_show_close_button(&self, visible: bool) -> () {
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
