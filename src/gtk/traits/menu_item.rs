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

//! The widget used for item in menus

use std::ffi::CString;
use glib::translate::{ToGlibPtr, ToTmp};
use gtk::{self, ffi};
use gtk::cast::GTK_MENU_ITEM;
use glib::{to_bool, to_gboolean};

/// The widget used for item in menus
pub trait MenuItemTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait {
    fn set_submenu<T: gtk::WidgetTrait>(&mut self, widget: &mut T) {
        unsafe {
            ffi::gtk_menu_item_set_submenu(GTK_MENU_ITEM(self.unwrap_widget()),
                                           widget.unwrap_widget())
        }
    }

    fn get_submenu<T: gtk::WidgetTrait>(&self) -> T {
        unsafe {
            gtk::FFIWidget::wrap_widget(ffi::gtk_menu_item_get_submenu(GTK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn select(&mut self) {
        unsafe {
            ffi::gtk_menu_item_select(GTK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn deselect(&mut self) {
        unsafe {
            ffi::gtk_menu_item_deselect(GTK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn activate(&mut self) {
        unsafe {
            ffi::gtk_menu_item_activate(GTK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn set_accel_path(&mut self, accel_path: &str) {
        unsafe {
            let mut tmp_accel_path = accel_path.to_tmp_for_borrow();
            ffi::gtk_menu_item_set_accel_path(GTK_MENU_ITEM(self.unwrap_widget()), tmp_accel_path.to_glib_ptr())
        }
    }

    fn get_accel_path(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_menu_item_get_accel_path(GTK_MENU_ITEM(self.unwrap_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn set_label(&mut self, label: &str) {
        let c_str = CString::from_slice(label.as_bytes());

        unsafe {
            ffi::gtk_menu_item_set_label(GTK_MENU_ITEM(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_menu_item_get_label(GTK_MENU_ITEM(self.unwrap_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn set_use_underline(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_menu_item_set_use_underline(GTK_MENU_ITEM(self.unwrap_widget()),
                                                 to_gboolean(setting))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_menu_item_get_use_underline(GTK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn set_reserve_indicator(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_menu_item_set_reserve_indicator(GTK_MENU_ITEM(self.unwrap_widget()),
                                                     to_gboolean(setting))
        }
    }

    fn get_reserve_indicator(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_menu_item_get_reserve_indicator(GTK_MENU_ITEM(self.unwrap_widget())))
        }
    }
}
