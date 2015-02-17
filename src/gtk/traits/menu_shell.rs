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

use gtk::{self, ffi};
use gtk::cast::GTK_MENU_SHELL;

/// A base class for menu objects
pub trait MenuShellTrait: gtk::WidgetTrait + gtk::ContainerTrait {
    fn append<T: gtk::WidgetTrait>(&mut self, widget: &T) {
        unsafe {
            ffi::gtk_menu_shell_append(GTK_MENU_SHELL(self.get_widget()), widget.get_widget())
        }
    }

    fn prepend<T: gtk::WidgetTrait>(&mut self, widget: &T) {
        unsafe {
            ffi::gtk_menu_shell_prepend(GTK_MENU_SHELL(self.get_widget()), widget.get_widget())
        }
    }

    fn insert<T: gtk::WidgetTrait>(&mut self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_shell_insert(GTK_MENU_SHELL(self.get_widget()),
                                       widget.get_widget(),
                                       position)
        }
    }

    fn deactivate(&mut self) {
        unsafe {
            ffi::gtk_menu_shell_deactivate(GTK_MENU_SHELL(self.get_widget()))
        }
    }

    fn select_item<T: gtk::MenuItemTrait>(&mut self, menu_item: &T) {
        unsafe {
            ffi::gtk_menu_shell_select_item(GTK_MENU_SHELL(self.get_widget()),
                                            menu_item.get_widget())
        }
    }

    fn deselect(&mut self) {
        unsafe {
            ffi::gtk_menu_shell_deselect(GTK_MENU_SHELL(self.get_widget()))
        }
    }

    fn activate_item<T: gtk::MenuItemTrait>(&mut self, menu_item: &T, force_deactivate: bool) {
        unsafe {
            ffi::gtk_menu_shell_activate_item(GTK_MENU_SHELL(self.get_widget()),
                                              menu_item.get_widget(),
                                              ffi::to_gboolean(force_deactivate))
        }
    }

    fn select_first(&mut self, search_sensitive: bool) {
        unsafe {
            ffi::gtk_menu_shell_select_first(GTK_MENU_SHELL(self.get_widget()),
                                             ffi::to_gboolean(search_sensitive))
        }
    }

    fn cancel(&mut self) {
        unsafe {
            ffi::gtk_menu_shell_cancel(GTK_MENU_SHELL(self.get_widget()))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_menu_shell_get_take_focus(GTK_MENU_SHELL(self.get_widget())))
        }
    }

    fn set_take_focus(&mut self, take_focus: bool) {
        unsafe {
            ffi::gtk_menu_shell_set_take_focus(GTK_MENU_SHELL(self.get_widget()),
                                               ffi::to_gboolean(take_focus))
        }
    }

    fn get_selected_item<T: gtk::WidgetTrait>(&self) -> T {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_menu_shell_get_selected_item(GTK_MENU_SHELL(self.get_widget())))
        }
    }

    fn get_parent_shell<T: gtk::MenuShellTrait>(&self) -> T {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_menu_shell_get_parent_shell(GTK_MENU_SHELL(self.get_widget())))
        }
    }
}
