// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A base class for menu objects

use ffi;
use cast::{GTK_MENU_ITEM, GTK_MENU_SHELL};
use glib::{to_bool, to_gboolean};

/// A base class for menu objects
pub trait MenuShellTrait: ::WidgetTrait + ::ContainerTrait {
    fn append<T: ::MenuItemTrait>(&self, widget: &T) {
        unsafe {
            ffi::gtk_menu_shell_append(GTK_MENU_SHELL(self.unwrap_widget()),
                GTK_MENU_ITEM(widget.unwrap_widget()))
        }
    }

    fn prepend<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe {
            ffi::gtk_menu_shell_prepend(GTK_MENU_SHELL(self.unwrap_widget()), widget.unwrap_widget())
        }
    }

    fn insert<T: ::WidgetTrait>(&self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_shell_insert(GTK_MENU_SHELL(self.unwrap_widget()),
                                       widget.unwrap_widget(),
                                       position)
        }
    }

    fn deactivate(&self) {
        unsafe {
            ffi::gtk_menu_shell_deactivate(GTK_MENU_SHELL(self.unwrap_widget()))
        }
    }

    fn select_item<T: ::MenuItemTrait>(&self, menu_item: &T) {
        unsafe {
            ffi::gtk_menu_shell_select_item(GTK_MENU_SHELL(self.unwrap_widget()),
                                            menu_item.unwrap_widget())
        }
    }

    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_shell_deselect(GTK_MENU_SHELL(self.unwrap_widget()))
        }
    }

    fn activate_item<T: ::MenuItemTrait>(&self, menu_item: &T, force_deactivate: bool) {
        unsafe {
            ffi::gtk_menu_shell_activate_item(GTK_MENU_SHELL(self.unwrap_widget()),
                                              menu_item.unwrap_widget(),
                                              to_gboolean(force_deactivate))
        }
    }

    fn select_first(&self, search_sensitive: bool) {
        unsafe {
            ffi::gtk_menu_shell_select_first(GTK_MENU_SHELL(self.unwrap_widget()),
                                             to_gboolean(search_sensitive))
        }
    }

    fn cancel(&self) {
        unsafe {
            ffi::gtk_menu_shell_cancel(GTK_MENU_SHELL(self.unwrap_widget()))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_menu_shell_get_take_focus(GTK_MENU_SHELL(self.unwrap_widget())))
        }
    }

    fn set_take_focus(&self, take_focus: bool) {
        unsafe {
            ffi::gtk_menu_shell_set_take_focus(GTK_MENU_SHELL(self.unwrap_widget()),
                                               to_gboolean(take_focus))
        }
    }

    unsafe fn get_selected_item<T: ::WidgetTrait>(&self) -> T {
        ::FFIWidget::wrap_widget(ffi::gtk_menu_shell_get_selected_item(GTK_MENU_SHELL(self.unwrap_widget())))
    }

    unsafe fn get_parent_shell<T: ::MenuShellTrait>(&self) -> T {
        ::FFIWidget::wrap_widget(ffi::gtk_menu_shell_get_parent_shell(GTK_MENU_SHELL(self.unwrap_widget())))
    }
}
