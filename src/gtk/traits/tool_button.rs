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

use std::ffi::CString;
use gtk::cast::GTK_TOOLBUTTON;
use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};

pub trait ToolButtonTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait + gtk::ToolItemTrait {
    fn set_label(&mut self, label: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(label.as_bytes());

            ffi::gtk_tool_button_set_label(GTK_TOOLBUTTON(self.get_widget()), c_str.as_ptr())
        }
    }

    fn set_stock_id(&mut self, stock_id: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(stock_id.as_bytes());

            ffi::gtk_tool_button_set_stock_id(GTK_TOOLBUTTON(self.get_widget()), c_str.as_ptr())
        }
    }

    fn set_icon_name(&mut self, icon_name: &str) -> () {
        let c_str = CString::from_slice(icon_name.as_bytes());

        unsafe {
            ffi::gtk_tool_button_set_icon_name(GTK_TOOLBUTTON(self.get_widget()), c_str.as_ptr());
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_label(GTK_TOOLBUTTON(self.get_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_stock_id(GTK_TOOLBUTTON(self.get_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_icon_name(GTK_TOOLBUTTON(self.get_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_button_get_use_underline(GTK_TOOLBUTTON(self.get_widget()))) }
    }

    fn set_use_underline(&mut self, set_underline: bool) -> () {
         unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.get_widget()), to_gboolean(set_underline)); }
    }

    fn set_label_widget<T: gtk::LabelTrait>(&mut self, label: &T) -> () {
        unsafe {
            ffi::gtk_tool_button_set_label_widget(GTK_TOOLBUTTON(self.get_widget()), label.get_widget())
        }
    }

    fn get_label_widget(&self) -> Option<gtk::Label> {
        unsafe {
            let tmp_pointer = ffi::gtk_tool_button_get_label_widget(GTK_TOOLBUTTON(self.get_widget()));
            if tmp_pointer.is_null() {
                None
            } else {
                Some(ffi::FFIWidget::wrap(tmp_pointer))
            }
        }
    }
}
