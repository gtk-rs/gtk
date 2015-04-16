// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A ToolItem containing a button with an additional dropdown menu

use std::ptr;

use cast::GTK_MENUTOOLBUTTON;
use ffi;
use glib::translate::ToGlibPtr;

/// MenuToolButton — A ToolItem containing a button with an additional dropdown menu
struct_Widget!(MenuToolButton);

impl MenuToolButton {
    pub fn new<T: ::WidgetTrait>(icon_widget: Option<&T>, label: Option<&str>) -> Option<MenuToolButton> {
        let tmp_pointer = unsafe {
            let icon_widget_ptr = match icon_widget {
                Some(i) => i.unwrap_widget(),
                None    => ptr::null_mut(),
            };

            ffi::gtk_menu_tool_button_new(icon_widget_ptr, label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, MenuToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<MenuToolButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_menu_tool_button_new_from_stock(stock_id.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, MenuToolButton)
    }

    pub fn set_arrow_tooltip_text(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(GTK_MENUTOOLBUTTON(self.pointer), text.borrow_to_glib().0)
        }
    }

    pub fn set_arrow_tooltip_markup(&self, markup: &str) -> () {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(
                GTK_MENUTOOLBUTTON(self.pointer),
                markup.borrow_to_glib().0)
        }
    }
}

impl_drop!(MenuToolButton);
impl_TraitWidget!(MenuToolButton);

impl ::ContainerTrait for MenuToolButton {}
impl ::BinTrait for MenuToolButton {}
impl ::ToolItemTrait for MenuToolButton {}
impl ::ToolButtonTrait for MenuToolButton {}

impl_widget_events!(MenuToolButton);
