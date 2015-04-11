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

/// ToolItem — The base class of widgets that can be added to ToolShe
struct_Widget!(ToolItem);

impl ToolItem {
    pub fn new() -> Option<ToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_new() };
        check_pointer!(tmp_pointer, ToolItem)
    }
}

impl_drop!(ToolItem);
impl_TraitWidget!(ToolItem);

impl ::ContainerTrait for ToolItem {}
impl ::BinTrait for ToolItem {}
impl ::ToolItemTrait for ToolItem {}

impl_widget_events!(ToolItem);
