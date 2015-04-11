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

//! A base class for menu objects

use ffi;
use cast::GTK_MENU_SHELL;
use glib::{to_bool, to_gboolean};

/// A base class for menu objects
pub trait MenuShellTrait: ::WidgetTrait + ::ContainerTrait {
    fn append<T: ::WidgetTrait>(&mut self, widget: &T) {
        unsafe {
            ffi::gtk_menu_shell_append(GTK_MENU_SHELL(self.unwrap_widget()), widget.unwrap_widget())
        }
    }

    fn prepend<T: ::WidgetTrait>(&mut self, widget: &T) {
        unsafe {
            ffi::gtk_menu_shell_prepend(GTK_MENU_SHELL(self.unwrap_widget()), widget.unwrap_widget())
        }
    }

    fn insert<T: ::WidgetTrait>(&mut self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_shell_insert(GTK_MENU_SHELL(self.unwrap_widget()),
                                       widget.unwrap_widget(),
                                       position)
        }
    }

    fn deactivate(&mut self) {
        unsafe {
            ffi::gtk_menu_shell_deactivate(GTK_MENU_SHELL(self.unwrap_widget()))
        }
    }

    fn select_item<T: ::MenuItemTrait>(&mut self, menu_item: &T) {
        unsafe {
            ffi::gtk_menu_shell_select_item(GTK_MENU_SHELL(self.unwrap_widget()),
                                            menu_item.unwrap_widget())
        }
    }

    fn deselect(&mut self) {
        unsafe {
            ffi::gtk_menu_shell_deselect(GTK_MENU_SHELL(self.unwrap_widget()))
        }
    }

    fn activate_item<T: ::MenuItemTrait>(&mut self, menu_item: &T, force_deactivate: bool) {
        unsafe {
            ffi::gtk_menu_shell_activate_item(GTK_MENU_SHELL(self.unwrap_widget()),
                                              menu_item.unwrap_widget(),
                                              to_gboolean(force_deactivate))
        }
    }

    fn select_first(&mut self, search_sensitive: bool) {
        unsafe {
            ffi::gtk_menu_shell_select_first(GTK_MENU_SHELL(self.unwrap_widget()),
                                             to_gboolean(search_sensitive))
        }
    }

    fn cancel(&mut self) {
        unsafe {
            ffi::gtk_menu_shell_cancel(GTK_MENU_SHELL(self.unwrap_widget()))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_menu_shell_get_take_focus(GTK_MENU_SHELL(self.unwrap_widget())))
        }
    }

    fn set_take_focus(&mut self, take_focus: bool) {
        unsafe {
            ffi::gtk_menu_shell_set_take_focus(GTK_MENU_SHELL(self.unwrap_widget()),
                                               to_gboolean(take_focus))
        }
    }

    fn get_selected_item<T: ::WidgetTrait>(&self) -> T {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_menu_shell_get_selected_item(GTK_MENU_SHELL(self.unwrap_widget())))
        }
    }

    fn get_parent_shell<T: ::MenuShellTrait>(&self) -> T {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_menu_shell_get_parent_shell(GTK_MENU_SHELL(self.unwrap_widget())))
        }
    }
}
