// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{from_glib_none, ToGlibPtr};
use cast::GTK_TOOLBUTTON;
use ffi;
use glib::{to_bool, to_gboolean};

pub trait ToolButtonTrait: ::WidgetTrait + ::ContainerTrait + ::BinTrait + ::ToolItemTrait {
    fn set_label(&self, label: &str) -> () {
        unsafe {
            ffi::gtk_tool_button_set_label(GTK_TOOLBUTTON(self.unwrap_widget()), label.to_glib_none().0)
        }
    }

    fn set_stock_id(&self, stock_id: &str) -> () {
        unsafe {
            ffi::gtk_tool_button_set_stock_id(GTK_TOOLBUTTON(self.unwrap_widget()), stock_id.to_glib_none().0)
        }
    }

    fn set_icon_name(&self, icon_name: &str) -> () {
        unsafe {
            ffi::gtk_tool_button_set_icon_name(GTK_TOOLBUTTON(self.unwrap_widget()),
                                               icon_name.to_glib_none().0);
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_tool_button_get_label(GTK_TOOLBUTTON(self.unwrap_widget())))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_tool_button_get_stock_id(GTK_TOOLBUTTON(self.unwrap_widget())))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_tool_button_get_icon_name(GTK_TOOLBUTTON(self.unwrap_widget())))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_button_get_use_underline(GTK_TOOLBUTTON(self.unwrap_widget()))) }
    }

    fn set_use_underline(&self, set_underline: bool) -> () {
         unsafe { ffi::gtk_tool_button_set_use_underline(GTK_TOOLBUTTON(self.unwrap_widget()), to_gboolean(set_underline)); }
    }

    fn set_label_widget<T: ::LabelTrait>(&self, label: &T) -> () {
        unsafe {
            ffi::gtk_tool_button_set_label_widget(GTK_TOOLBUTTON(self.unwrap_widget()), label.unwrap_widget())
        }
    }

    fn get_label_widget(&self) -> Option<::Label> {
        unsafe {
            let tmp_pointer = ffi::gtk_tool_button_get_label_widget(GTK_TOOLBUTTON(self.unwrap_widget()));
            if tmp_pointer.is_null() {
                None
            } else {
                Some(::FFIWidget::wrap_widget(tmp_pointer))
            }
        }
    }
}
