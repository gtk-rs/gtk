// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! The widget used for item in menus

use glib::translate::{from_glib_none, ToGlibPtr};
use ffi;
use cast::GTK_MENU_ITEM;
use glib::{to_bool, to_gboolean};

/// The widget used for item in menus
pub trait MenuItemTrait: ::WidgetTrait + ::ContainerTrait + ::BinTrait {
    fn set_submenu<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe {
            ffi::gtk_menu_item_set_submenu(GTK_MENU_ITEM(self.unwrap_widget()),
                                           widget.unwrap_widget())
        }
    }

    fn get_submenu<T: ::WidgetTrait>(&self) -> T {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_menu_item_get_submenu(GTK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn select(&self) {
        unsafe {
            ffi::gtk_menu_item_select(GTK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_item_deselect(GTK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn activate(&self) {
        unsafe {
            ffi::gtk_menu_item_activate(GTK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn set_accel_path(&self, accel_path: &str) {
        unsafe {
            ffi::gtk_menu_item_set_accel_path(GTK_MENU_ITEM(self.unwrap_widget()), accel_path.to_glib_none().0)
        }
    }

    fn get_accel_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_menu_item_get_accel_path(GTK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_menu_item_set_label(GTK_MENU_ITEM(self.unwrap_widget()),
                                         label.to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_menu_item_get_label(GTK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn set_use_underline(&self, setting: bool) {
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

    fn set_reserve_indicator(&self, setting: bool) {
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
