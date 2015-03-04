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

use gtk::cast::GTK_MENUTOOLBUTTON;
use gtk::{self, ffi};
use std::ffi::CString;
use glib::translate::{ToGlibPtr, ToTmp};

/// MenuToolButton — A ToolItem containing a button with an additional dropdown menu
struct_Widget!(MenuToolButton);

impl MenuToolButton {
    pub fn new<T: gtk::WidgetTrait>(icon_widget: Option<&T>, label: Option<&str>) -> Option<MenuToolButton> {
        let tmp_pointer = unsafe {
            match label {
                Some(l) => {
                    let c_str = CString::from_slice(l.as_bytes());

                    match icon_widget {
                        Some(i) => ffi::gtk_menu_tool_button_new(i.unwrap_widget(), c_str.as_ptr()),
                        None    => ffi::gtk_menu_tool_button_new(ptr::null_mut(), c_str.as_ptr())
                    }
                },
                None    => {
                    match icon_widget {
                        Some(i) => ffi::gtk_menu_tool_button_new(i.unwrap_widget(), ptr::null()),
                        None    => ffi::gtk_menu_tool_button_new(ptr::null_mut(), ptr::null())
                    }
                }
            }
        };
        check_pointer!(tmp_pointer, MenuToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<MenuToolButton> {
        let tmp_pointer = unsafe {
            let mut tmp_stock_id = stock_id.to_tmp_for_borrow();
            ffi::gtk_menu_tool_button_new_from_stock(tmp_stock_id.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, MenuToolButton)
    }

    pub fn set_arrow_tooltip_text(&mut self, text: &str) -> () {
        unsafe {
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(GTK_MENUTOOLBUTTON(self.pointer), tmp_text.to_glib_ptr())
        }
    }

    pub fn set_arrow_tooltip_markup(&mut self, markup: &str) -> () {
        let c_str = CString::from_slice(markup.as_bytes());

        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(GTK_MENUTOOLBUTTON(self.pointer), c_str.as_ptr())
        }
    }
}

impl_drop!(MenuToolButton);
impl_TraitWidget!(MenuToolButton);

impl gtk::ContainerTrait for MenuToolButton {}
impl gtk::BinTrait for MenuToolButton {}
impl gtk::ToolItemTrait for MenuToolButton {}
impl gtk::ToolButtonTrait for MenuToolButton {}

impl_widget_events!(MenuToolButton);
