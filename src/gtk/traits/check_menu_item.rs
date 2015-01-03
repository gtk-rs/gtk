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

use gtk::{self, ffi};
use gtk::cast::GTK_CHECK_MENU_ITEM;

/// The widget used for item in menus
pub trait CheckMenuItemTrait: gtk::WidgetTrait +
                              gtk::ContainerTrait +
                              gtk::BinTrait +
                              gtk::MenuItemTrait {

    fn set_active(&mut self, is_active: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_active(GTK_CHECK_MENU_ITEM(self.get_widget()),
                                                ffi::to_gboolean(is_active))
        }
    }

    fn active(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_check_menu_item_get_active(GTK_CHECK_MENU_ITEM(self.get_widget())))
        }
    }

    fn toggled(&mut self) {
        unsafe {
            ffi::gtk_check_menu_item_toggled(GTK_CHECK_MENU_ITEM(self.get_widget()))
        }
    }

    fn set_inconsistent(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_inconsistent(GTK_CHECK_MENU_ITEM(self.get_widget()),
                                                      ffi::to_gboolean(setting))
        }
    }

    fn inconsistent(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_check_menu_item_get_inconsistent(GTK_CHECK_MENU_ITEM(self.get_widget())))
        }
    }

    fn set_draw_as_radio(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_draw_as_radio(GTK_CHECK_MENU_ITEM(self.get_widget()),
                                                       ffi::to_gboolean(setting))
        }
    }

    fn draw_as_radio(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_check_menu_item_get_draw_as_radio(GTK_CHECK_MENU_ITEM(self.get_widget())))
        }
    }
}
