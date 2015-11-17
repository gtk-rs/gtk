// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Report important messages to the user

#![cfg_attr(not(gtk_3_10), allow(unused_imports))]

use libc::c_int;
use glib::translate::ToGlibPtr;

use Button;
use MessageType;
use cast::GTK_INFOBAR;
use ffi;
use glib::{to_bool, to_gboolean};

/// InfoBar — Report important messages to the user
struct_Widget!(InfoBar);

impl InfoBar {
    pub fn new() -> Option<InfoBar> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_info_bar_new() };
        check_pointer!(tmp_pointer, InfoBar)
    }

    pub fn add_action_widget<T: ::WidgetTrait>(&self, child: &T, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(GTK_INFOBAR(self.pointer), child.unwrap_widget(), response_id as c_int)
        }
    }

    pub fn add_button(&self, button_text: &str, response_id: i32) -> ::Button {
        let button = unsafe {
            ffi::gtk_info_bar_add_button(GTK_INFOBAR(self.pointer), button_text.to_glib_none().0, response_id as c_int)
        };
        check_pointer!(button, Button).unwrap()
    }

    pub fn set_response_sensitive(&self, response_id: i32, setting: bool) -> () {
        unsafe { ffi::gtk_info_bar_set_response_sensitive(GTK_INFOBAR(self.pointer), response_id as c_int, to_gboolean(setting)); }
    }

    pub fn set_default_response(&self, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_set_default_response(GTK_INFOBAR(self.pointer), response_id as c_int)
        }
    }

    pub fn response(&self, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_response(GTK_INFOBAR(self.pointer), response_id as c_int)
        }
    }

    pub fn set_message_type(&self, message_type: MessageType) -> () {
        unsafe {
            ffi::gtk_info_bar_set_message_type(GTK_INFOBAR(self.pointer), message_type);
        }
    }

    pub fn get_message_type(&self) -> MessageType {
        unsafe {
            ffi::gtk_info_bar_get_message_type(GTK_INFOBAR(self.pointer))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn show_close_button(&self, show: bool) -> () {
         unsafe { ffi::gtk_info_bar_set_show_close_button(GTK_INFOBAR(self.pointer), to_gboolean(show)); }
    }

    #[cfg(gtk_3_10)]
    pub fn get_show_close_button(&self) -> bool {
        unsafe { to_bool(ffi::gtk_info_bar_get_show_close_button(GTK_INFOBAR(self.pointer))) }
    }
}

impl_drop!(InfoBar);
impl_TraitWidget!(InfoBar);

impl ::ContainerTrait for InfoBar {}
impl ::BoxTrait for InfoBar {}
impl ::OrientableTrait for InfoBar {}
