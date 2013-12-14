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

use std::str;

use traits::{GtkWidget, GtkContainer, GtkBin, GtkToolItem, GtkLabel};
use gtk;
use utils::cast::GTK_TOOLBUTTON;
use ffi;

pub trait GtkToolButton: GtkWidget + GtkContainer + GtkBin + GtkToolItem {
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

    fn set_icone_name(&mut self, icone_name: &str) -> () {
        unsafe {
            icone_name.with_c_str(|c_str| {
                ffi::gtk_tool_button_set_icon_name(GTK_TOOLBUTTON(self.get_widget()), c_str)
            });
        }
    }

    fn get_label(&self) -> Option<~str> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_label(GTK_TOOLBUTTON(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(c_str))
            }
        }
    }

    fn get_stock_id(&self) -> Option<~str> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_stock_id(GTK_TOOLBUTTON(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(c_str))
            }
        }
    }

    fn get_icon_name(&self) -> Option<~str> {
        unsafe {
            let c_str = ffi::gtk_tool_button_get_icon_name(GTK_TOOLBUTTON(self.get_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(c_str))
            }
        }
    }

    fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_tool_button_get_use_underline(GTK_TOOLBUTTON(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_use_underline(&mut self, set_underline: bool) -> () {
         match set_underline {
            true    => unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn set_label_widget<T: GtkLabel>(&mut self, label: &T) -> () {
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
                Some(GtkWidget::wrap_widget(tmp_pointer))
            }
        }
    }
}

