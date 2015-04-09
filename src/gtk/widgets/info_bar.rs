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

//! Report important messages to the user

#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_imports))]

use libc::c_int;
use glib::translate::ToGlibPtr;

use gtk::MessageType;
use gtk::cast::GTK_INFOBAR;
use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};

/// InfoBar — Report important messages to the user
struct_Widget!(InfoBar);

impl InfoBar {
    pub fn new() -> Option<InfoBar> {
        let tmp_pointer = unsafe { ffi::gtk_info_bar_new() };
        check_pointer!(tmp_pointer, InfoBar)
    }

    pub fn add_action_widget<T: gtk::WidgetTrait>(&mut self, child: &T, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(GTK_INFOBAR(self.pointer), child.unwrap_widget(), response_id as c_int)
        }
    }

    pub fn add_button(&mut self, button_text: &str, response_id: i32) -> gtk::Button {
        let button = unsafe {
            ffi::gtk_info_bar_add_button(GTK_INFOBAR(self.pointer), button_text.borrow_to_glib().0, response_id as c_int)
        };
        gtk::FFIWidget::wrap_widget(button)
    }

    pub fn set_response_sensitive(&mut self, response_id: i32, setting: bool) -> () {
        unsafe { ffi::gtk_info_bar_set_response_sensitive(GTK_INFOBAR(self.pointer), response_id as c_int, to_gboolean(setting)); }
    }

    pub fn set_default_response(&mut self, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_set_default_response(GTK_INFOBAR(self.pointer), response_id as c_int)
        }
    }

    pub fn response(&mut self, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_response(GTK_INFOBAR(self.pointer), response_id as c_int)
        }
    }

    pub fn set_message_type(&mut self, message_type: MessageType) -> () {
        unsafe {
            ffi::gtk_info_bar_set_message_type(GTK_INFOBAR(self.pointer), message_type);
        }
    }

    pub fn get_message_type(&mut self) -> MessageType {
        unsafe {
            ffi::gtk_info_bar_get_message_type(GTK_INFOBAR(self.pointer))
        }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn show_close_button(&mut self, show: bool) -> () {
         unsafe { ffi::gtk_info_bar_set_show_close_button(GTK_INFOBAR(self.pointer), to_gboolean(show)); }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn get_show_close_button(&self) -> bool {
        unsafe { to_bool(ffi::gtk_info_bar_get_show_close_button(GTK_INFOBAR(self.pointer))) }
    }
}

impl_drop!(InfoBar);
impl_TraitWidget!(InfoBar);

impl gtk::ContainerTrait for InfoBar {}
impl gtk::BoxTrait for InfoBar {}
impl gtk::OrientableTrait for InfoBar {}

impl_widget_events!(InfoBar);
