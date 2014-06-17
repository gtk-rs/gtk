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

//! The base class of widgets that can be added to GtkToolShe

use std::{ptr, mem};
use libc::c_void;

use traits::{GtkContainer, GtkWidget, GtkBin, GtkToolItem, Signal};
use ffi;
use utils::cast::GTK_SEPARATORTOOLITEM;

/// ToolItem — The base class of widgets that can be added to GtkToolShe
pub struct SeparatorToolItem {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl SeparatorToolItem {
    pub fn new() -> Option<SeparatorToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_separator_tool_item_new() };
        check_pointer!(tmp_pointer, SeparatorToolItem)
    }

    pub fn set_draw(&mut self, draw: bool) -> () {
    	match draw {
            true    => unsafe { ffi::gtk_separator_tool_item_set_draw(GTK_SEPARATORTOOLITEM(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_separator_tool_item_set_draw(GTK_SEPARATORTOOLITEM(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_draw(&self) -> bool {
    	match unsafe { ffi::gtk_separator_tool_item_get_draw(GTK_SEPARATORTOOLITEM(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }
}

impl_GtkWidget!(SeparatorToolItem)
redirect_callback!(SeparatorToolItem)
redirect_callback_widget!(SeparatorToolItem)
struct_signal!(SeparatorToolItem)
impl_signals!(SeparatorToolItem)

impl GtkContainer for SeparatorToolItem {}
impl GtkBin for SeparatorToolItem {}
impl GtkToolItem for SeparatorToolItem {}
