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

use gtk::traits::{Widget, Container, Bin, ToolItem, Label};
use gtk;
use gtk::cast::GTK_TOOLBUTTON;
use gtk::ffi;
use std::string;

pub trait ToolButton: Widget + Container + Bin + ToolItem {
    fn set_label(&mut self, label: &str) -> () {
        unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_tool_button_set_label(GTK_TOOLBUTTON(self.get_widget()), c_str)
            });
        }
    }

    fn set_stock_id(&mut self, stock_id: &str) -> () {
        unsafe {
            stock_id.with_c_str(|c_str| {
                ffi::gtk_tool_button_set_stock_id(GTK_TOOLBUTTON(self.get_widget()), c_str)
            });
        }
    }

    fn set_icon_name(&mut self, icon_name: &str) -> () {
        unsafe {
            icon_name.with_c_str(|c_str| {
                ffi::gtk_tool_button_set_icon_name(GTK_TOOLBUTTON(self.get_widget()), c_str)
            });
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_label(GTK_TOOLBUTTON(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(string::raw::from_buf(c_str as *const u8))
            }
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_stock_id(GTK_TOOLBUTTON(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(string::raw::from_buf(c_str as *const u8))
            }
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_icon_name(GTK_TOOLBUTTON(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(string::raw::from_buf(c_str as *const u8))
            }
        }
    }

    fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_tool_button_get_use_underline(GTK_TOOLBUTTON(self.get_widget())) } {
            0i32 => false,
            _ => true
        }
    }

    fn set_use_underline(&mut self, set_underline: bool) -> () {
         match set_underline {
            true    => unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn set_label_widget<T: Label>(&mut self, label: &T) -> () {
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

