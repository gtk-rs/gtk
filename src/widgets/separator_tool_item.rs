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

//! The base class of widgets that can be added to ToolShe

use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_SEPARATORTOOLITEM;

/// ToolItem — The base class of widgets that can be added to ToolShe
struct_Widget!(SeparatorToolItem);

impl SeparatorToolItem {
    pub fn new() -> Option<SeparatorToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_separator_tool_item_new() };
        check_pointer!(tmp_pointer, SeparatorToolItem)
    }

    pub fn set_draw(&self, draw: bool) -> () {
        unsafe { ffi::gtk_separator_tool_item_set_draw(GTK_SEPARATORTOOLITEM(self.pointer), to_gboolean(draw)); }
    }

    pub fn get_draw(&self) -> bool {
        unsafe { to_bool(ffi::gtk_separator_tool_item_get_draw(GTK_SEPARATORTOOLITEM(self.pointer))) }
    }
}

impl_drop!(SeparatorToolItem);
impl_TraitWidget!(SeparatorToolItem);

impl ::ContainerTrait for SeparatorToolItem {}
impl ::BinTrait for SeparatorToolItem {}
impl ::ToolItemTrait for SeparatorToolItem {}

impl_widget_events!(SeparatorToolItem);
