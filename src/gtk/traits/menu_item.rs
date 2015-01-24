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
use gtk::{self, ffi};
use gtk::cast::GTK_MENU_ITEM;

/// The widget used for item in menus
pub trait MenuItemTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait {
    fn set_submenu(&mut self, widget: &mut gtk::WidgetTrait) {
        unsafe {
            ffi::gtk_menu_item_set_submenu(GTK_MENU_ITEM(self.get_widget()),
                                           widget.get_widget())
        }
    }

    fn get_submenu<T: gtk::WidgetTrait>(&self) -> T {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_menu_item_get_submenu(GTK_MENU_ITEM(self.get_widget())))
        }
    }

    fn select(&mut self) {
        unsafe {
            ffi::gtk_menu_item_select(GTK_MENU_ITEM(self.get_widget()))
        }
    }

    fn deselect(&mut self) {
        unsafe {
            ffi::gtk_menu_item_deselect(GTK_MENU_ITEM(self.get_widget()))
        }
    }

    fn activate(&mut self) {
        unsafe {
            ffi::gtk_menu_item_activate(GTK_MENU_ITEM(self.get_widget()))
        }
    }

    fn set_accel_path(&mut self, accel_path: &str) {
        unsafe {
            let c_str = CString::from_slice(accel_path.as_bytes());

            ffi::gtk_menu_item_set_accel_path(GTK_MENU_ITEM(self.get_widget()), c_str.as_ptr())
        }
    }

    fn get_accel_path(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_menu_item_get_accel_path(GTK_MENU_ITEM(self.get_widget()));

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
            ffi::gtk_menu_item_set_label(GTK_MENU_ITEM(self.get_widget()), c_str.as_ptr())
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_menu_item_get_label(GTK_MENU_ITEM(self.get_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn set_use_underline(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_menu_item_set_use_underline(GTK_MENU_ITEM(self.get_widget()),
                                                 ffi::to_gboolean(setting))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_menu_item_get_use_underline(GTK_MENU_ITEM(self.get_widget())))
        }
    }

    fn set_reserve_indicator(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_menu_item_set_reserve_indicator(GTK_MENU_ITEM(self.get_widget()),
                                                     ffi::to_gboolean(setting))
        }
    }

    fn get_reserve_indicator(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_menu_item_get_reserve_indicator(GTK_MENU_ITEM(self.get_widget())))
        }
    }
}
