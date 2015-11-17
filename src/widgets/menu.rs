// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;

use cast::GTK_MENU;
use gdk;
use glib::translate::*;
use traits::FFIWidget;
use ffi;

struct_Widget!(Menu);
impl_TraitWidget!(Menu);

impl Menu {
    pub fn new() -> Option<Menu> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_menu_new() };
        check_pointer!(tmp_pointer, Menu)
    }

    //#[cfg(gtk_3_4)]
    //pub fn new_from_model<T: Upcast<gio::MenuModel>>(model: &T) -> Menu {
    //    unsafe { TODO: call ffi:gtk_menu_new_from_model() }
    //}

    pub fn attach<T: ::WidgetTrait>(&self, child: &T, left_attach: u32, right_attach: u32, top_attach: u32, bottom_attach: u32) {
        unsafe {
            ffi::gtk_menu_attach(GTK_MENU(self.unwrap_widget()), child.unwrap_widget(), left_attach, right_attach, top_attach, bottom_attach);
        }
    }

    //pub fn attach_to_widget<T: ::WidgetTrait>(&self, attach_widget: &T, detacher: /*Unknown kind*/Unknown rust type: "MenuDetachFunc") {
    //    unsafe { TODO: call ffi:gtk_menu_attach_to_widget() }
    //}

    pub fn detach(&self) {
        unsafe {
            ffi::gtk_menu_detach(GTK_MENU(self.unwrap_widget()));
        }
    }

    // pub fn get_accel_group(&self) -> Option<AccelGroup> {
    //     unsafe {
    //         from_glib_none(ffi::gtk_menu_get_accel_group(GTK_MENU(self.unwrap_widget())))
    //     }
    // }

    pub fn get_accel_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_accel_path(GTK_MENU(self.unwrap_widget())))
        }
    }

    // pub fn get_active<T: ::WidgetTrait>(&self) -> Option<T> {
    //     unsafe {
    //         from_glib_none(ffi::gtk_menu_get_active(GTK_MENU(self.unwrap_widget())))
    //     }
    // }

    // pub fn get_attach_widget<T: ::WidgetTrait>(&self) -> Option<T> {
    //     unsafe {
    //         from_glib_none(ffi::gtk_menu_get_attach_widget(GTK_MENU(self.unwrap_widget())))
    //     }
    // }

    pub fn get_monitor(&self) -> i32 {
        unsafe {
            ffi::gtk_menu_get_monitor(GTK_MENU(self.unwrap_widget()))
        }
    }

    pub fn get_reserve_toggle_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_get_reserve_toggle_size(GTK_MENU(self.unwrap_widget())))
        }
    }

    pub fn get_tearoff_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_get_tearoff_state(GTK_MENU(self.unwrap_widget())))
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_get_title(GTK_MENU(self.unwrap_widget())))
        }
    }

    pub fn popdown(&self) {
        unsafe {
            ffi::gtk_menu_popdown(GTK_MENU(self.unwrap_widget()));
        }
    }

    pub fn popup<T: ::WidgetTrait, U: ::WidgetTrait>(&self, parent_menu_shell: Option<&T>, parent_menu_item: Option<&U>, button: u32, activate_time: u32) {
        unsafe {
            let parent_menu_shell = parent_menu_shell.map(|p| p.unwrap_widget()).unwrap_or(ptr::null_mut());
            let parent_menu_item = parent_menu_item.map(|p| p.unwrap_widget()).unwrap_or(ptr::null_mut());
            ffi::gtk_menu_popup(
                GTK_MENU(self.unwrap_widget()),
                parent_menu_shell,
                parent_menu_item,
                None,
                ptr::null_mut(),
                button,
                activate_time
                );
        }
    }

    pub fn reorder_child<T: ::WidgetTrait>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_reorder_child(GTK_MENU(self.unwrap_widget()), child.unwrap_widget(), position);
        }
    }

    pub fn reposition(&self) {
        unsafe {
            ffi::gtk_menu_reposition(GTK_MENU(self.unwrap_widget()));
        }
    }

    pub fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            ffi::gtk_menu_set_accel_path(GTK_MENU(self.unwrap_widget()), accel_path.to_glib_none().0);
        }
    }

    pub fn set_active(&self, index: u32) {
        unsafe {
            ffi::gtk_menu_set_active(GTK_MENU(self.unwrap_widget()), index);
        }
    }

    pub fn set_monitor(&self, monitor_num: i32) {
        unsafe {
            ffi::gtk_menu_set_monitor(GTK_MENU(self.unwrap_widget()), monitor_num);
        }
    }

    pub fn set_reserve_toggle_size(&self, reserve_toggle_size: bool) {
        unsafe {
            ffi::gtk_menu_set_reserve_toggle_size(GTK_MENU(self.unwrap_widget()), reserve_toggle_size.to_glib());
        }
    }

    pub fn set_screen(&self, screen: Option<&gdk::Screen>) {
        unsafe {
            ffi::gtk_menu_set_screen(GTK_MENU(self.unwrap_widget()), screen.to_glib_none().0);
        }
    }

    pub fn set_tearoff_state(&self, torn_off: bool) {
        unsafe {
            ffi::gtk_menu_set_tearoff_state(GTK_MENU(self.unwrap_widget()), torn_off.to_glib());
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_menu_set_title(GTK_MENU(self.unwrap_widget()), title.to_glib_none().0);
        }
    }
}

impl ::ContainerTrait for Menu {}
impl ::MenuShellTrait for Menu {}
