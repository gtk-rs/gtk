// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use cast::GTK_CHECK_MENU_ITEM;
use glib::{to_bool, to_gboolean};

pub trait CheckMenuItemTrait: ::WidgetTrait +
                              ::ContainerTrait +
                              ::BinTrait +
                              ::MenuItemTrait {

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_active(GTK_CHECK_MENU_ITEM(self.unwrap_widget()),
                                                to_gboolean(is_active))
        }
    }

    fn active(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_check_menu_item_get_active(GTK_CHECK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_check_menu_item_toggled(GTK_CHECK_MENU_ITEM(self.unwrap_widget()))
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_inconsistent(GTK_CHECK_MENU_ITEM(self.unwrap_widget()),
                                                      to_gboolean(setting))
        }
    }

    fn inconsistent(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_check_menu_item_get_inconsistent(GTK_CHECK_MENU_ITEM(self.unwrap_widget())))
        }
    }

    fn set_draw_as_radio(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_draw_as_radio(GTK_CHECK_MENU_ITEM(self.unwrap_widget()),
                                                       to_gboolean(setting))
        }
    }

    fn draw_as_radio(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_check_menu_item_get_draw_as_radio(GTK_CHECK_MENU_ITEM(self.unwrap_widget())))
        }
    }
}
