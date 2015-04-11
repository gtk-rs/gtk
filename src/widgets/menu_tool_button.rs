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

    pub fn set_arrow_tooltip_text(&mut self, text: &str) -> () {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(GTK_MENUTOOLBUTTON(self.pointer), text.borrow_to_glib().0)
        }
    }

    pub fn set_arrow_tooltip_markup(&mut self, markup: &str) -> () {
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
