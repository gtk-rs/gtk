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

use glib::translate::{FromGlibPtr, ToGlibPtr};
use gtk::cast::GTK_TOOLBUTTON;
use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};

pub trait ToolButtonTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait + gtk::ToolItemTrait {
    fn set_label(&mut self, label: &str) -> () {
        unsafe {
            ffi::gtk_tool_button_set_label(GTK_TOOLBUTTON(self.unwrap_widget()), label.borrow_to_glib().0)
        }
    }

    fn set_stock_id(&mut self, stock_id: &str) -> () {
        unsafe {
            ffi::gtk_tool_button_set_stock_id(GTK_TOOLBUTTON(self.unwrap_widget()), stock_id.borrow_to_glib().0)
        }
    }

    fn set_icon_name(&mut self, icon_name: &str) -> () {
        unsafe {
            ffi::gtk_tool_button_set_icon_name(GTK_TOOLBUTTON(self.unwrap_widget()),
                                               icon_name.borrow_to_glib().0);
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_tool_button_get_label(GTK_TOOLBUTTON(self.unwrap_widget())))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_tool_button_get_stock_id(GTK_TOOLBUTTON(self.unwrap_widget())))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_tool_button_get_icon_name(GTK_TOOLBUTTON(self.unwrap_widget())))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_button_get_use_underline(GTK_TOOLBUTTON(self.unwrap_widget()))) }
    }

    fn set_use_underline(&mut self, set_underline: bool) -> () {
         unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.unwrap_widget()), to_gboolean(set_underline)); }
    }

    fn set_label_widget<T: gtk::LabelTrait>(&mut self, label: &T) -> () {
        unsafe {
            ffi::gtk_tool_button_set_label_widget(GTK_TOOLBUTTON(self.unwrap_widget()), label.unwrap_widget())
        }
    }

    fn get_label_widget(&self) -> Option<gtk::Label> {
        unsafe {
            let tmp_pointer = ffi::gtk_tool_button_get_label_widget(GTK_TOOLBUTTON(self.unwrap_widget()));
            if tmp_pointer.is_null() {
                None
            } else {
                Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
            }
        }
    }
}
