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
use gtk::ffi;
use gtk::traits;

/// MenuToolButton — A ToolItem containing a button with an additional dropdown menu
struct_Widget!(MenuToolButton)


impl MenuToolButton {
    pub fn new<T: traits::Widget>(icon_widget: Option<&T>, label: Option<&str>) -> Option<MenuToolButton> {
        let tmp_pointer: *ffi::C_GtkWidget = unsafe {
            match label {
                Some(l) => {
                    l.with_c_str(|c_str| {
                        match icon_widget {
                            Some(i) => ffi::gtk_menu_tool_button_new(i.get_widget(), c_str),
                            None    => ffi::gtk_menu_tool_button_new(ptr::null(), c_str)
                        }
                    })
                },
                None    => {
                    match icon_widget {
                        Some(i) => ffi::gtk_menu_tool_button_new(i.get_widget(), ptr::null()),
                        None    => ffi::gtk_menu_tool_button_new(ptr::null(), ptr::null())
                    }
                }
            }
        };
        check_pointer!(tmp_pointer, MenuToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<MenuToolButton> {
        let tmp_pointer = stock_id.with_c_str(|c_str| {
            unsafe { ffi::gtk_menu_tool_button_new_from_stock(c_str) }
        });
        check_pointer!(tmp_pointer, MenuToolButton)
    }

    pub fn set_arrow_tooltip_text(&mut self, text: &str) -> () {
        text.with_c_str(|c_str| {
            unsafe {
                ffi::gtk_menu_tool_button_set_arrow_tooltip_text(GTK_MENUTOOLBUTTON(self.pointer), c_str)
            }
        })
    }

    pub fn set_arrow_tooltip_markup(&mut self, markup: &str) -> () {
        markup.with_c_str(|c_str| {
            unsafe {
                ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(GTK_MENUTOOLBUTTON(self.pointer), c_str)
            }
        })
    }
}

impl_drop!(MenuToolButton)
impl_TraitWidget!(MenuToolButton)

impl traits::Container for MenuToolButton {}
impl traits::Bin for MenuToolButton {}
impl traits::ToolItem for MenuToolButton {}
impl traits::ToolButton for MenuToolButton {}

