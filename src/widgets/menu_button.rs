// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget that shows a menu when clicked on

use ffi;
use cast::GTK_MENUBUTTON;
use ArrowType;

/// MenuButton — A widget that shows a menu when clicked on
struct_Widget!(MenuButton);

impl MenuButton {
    pub fn new() -> Option<MenuButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_menu_button_new() };
        check_pointer!(tmp_pointer, MenuButton)
    }

    pub fn set_popup<T: ::WidgetTrait>(&self, popup: &T) -> () {
        unsafe {
            ffi::gtk_menu_button_set_popup(GTK_MENUBUTTON(self.pointer), popup.unwrap_widget());
        }
    }

    pub fn set_direction(&self, direction: ArrowType) -> () {
        unsafe {
            ffi::gtk_menu_button_set_direction(GTK_MENUBUTTON(self.pointer), direction);
        }
    }

    pub fn get_direction(&self) -> ArrowType {
        unsafe {
            ffi::gtk_menu_button_get_direction(GTK_MENUBUTTON(self.pointer))
        }
    }

    pub fn set_align_widget<T: ::WidgetTrait>(&self, align_widget: &T) -> () {
        unsafe {
            ffi::gtk_menu_button_set_align_widget(GTK_MENUBUTTON(self.pointer), align_widget.unwrap_widget())
        }
    }
}

impl_drop!(MenuButton);
impl_TraitWidget!(MenuButton);

impl ::ContainerTrait for MenuButton {}
impl ::ButtonTrait for MenuButton {}
impl ::ToggleButtonTrait for MenuButton {}
