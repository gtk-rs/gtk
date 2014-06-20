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

//! A GtkToolItem subclass that displays buttons

use libc::{c_void};
use std::ptr;

use traits::{GtkContainer, GtkWidget, GtkBin, GtkToolItem, GtkToolButton, Signal};
use ffi;

/// ToolButton — A GtkToolItem subclass that displays buttons
pub struct ToolButton {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl ToolButton {
    pub fn new<T: GtkWidget>(icon_widget: Option<&T>, label: Option<&str>) -> Option<ToolButton> {
        let tmp_pointer: *ffi::C_GtkWidget = unsafe {
            match label {
                Some(l) => {
                    l.with_c_str(|c_str| {
                        match icon_widget {
                            Some(i) => ffi::gtk_tool_button_new(i.get_widget(), c_str),
                            None    => ffi::gtk_tool_button_new(ptr::null(), c_str)
                        }
                    })
                },
                None    => {
                    match icon_widget {
                        Some(i) => ffi::gtk_tool_button_new(i.get_widget(), ptr::null()),
                        None    => ffi::gtk_tool_button_new(ptr::null(), ptr::null())
                    }
                }
            }
        };
        check_pointer!(tmp_pointer, ToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<ToolButton> {
        let tmp_pointer = stock_id.with_c_str(|c_str| {
            unsafe { ffi::gtk_tool_button_new_from_stock(c_str) }
        });
        check_pointer!(tmp_pointer, ToolButton)
    }
}

impl_GtkWidget!(ToolButton)
redirect_callback!(ToolButton)
redirect_callback_widget!(ToolButton)
struct_signal!(ToolButton)
impl_signals!(ToolButton)

impl GtkContainer for ToolButton {}
impl GtkBin for ToolButton {}
impl GtkToolItem for ToolButton {}
impl GtkToolButton for ToolButton {}