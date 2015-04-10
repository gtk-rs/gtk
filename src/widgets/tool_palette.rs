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

//! GtkToolPalette — A tool palette with categories

use ffi;
use ToolItem;
use FFIWidget;
use cast::{GTK_TOOL_PALETTE, GTK_TOOL_ITEM_GROUP};
use glib::{to_bool, to_gboolean};

struct_Widget!(ToolPalette);

impl ToolPalette {
    pub fn new() -> Option<ToolPalette> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_new() };
        check_pointer!(tmp_pointer, ToolPalette)
    }

    pub fn get_icon_size(&self) -> ::IconSize {
        unsafe { ffi::gtk_tool_palette_get_icon_size(GTK_TOOL_PALETTE(self.unwrap_widget())) }
    }

    pub fn set_icon_size(&self, icon_size: ::IconSize) {
        unsafe { ffi::gtk_tool_palette_set_icon_size(GTK_TOOL_PALETTE(self.unwrap_widget()), icon_size) }
    }

    pub fn unset_icon_size(&self) {
        unsafe { ffi::gtk_tool_palette_unset_icon_size(GTK_TOOL_PALETTE(self.unwrap_widget())) }
    }

    pub fn get_style(&self) -> ::ToolbarStyle {
        unsafe { ffi::gtk_tool_palette_get_style(GTK_TOOL_PALETTE(self.unwrap_widget())) }
    }

    pub fn set_style(&self, style: ::ToolbarStyle) {
        unsafe { ffi::gtk_tool_palette_set_style(GTK_TOOL_PALETTE(self.unwrap_widget()), style) }
    }

    pub fn unset_style(&self) {
        unsafe { ffi::gtk_tool_palette_unset_style(GTK_TOOL_PALETTE(self.unwrap_widget())) }
    }

    pub fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_get_drop_item(GTK_TOOL_PALETTE(self.unwrap_widget()),
            x as ::libc::c_int, y as ::libc::c_int) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn set_drag_source(&self, targets: ::ToolPaletteDragTargets) {
        unsafe { ffi::gtk_tool_palette_set_drag_source(GTK_TOOL_PALETTE(self.unwrap_widget()), targets) }
    }

    pub fn get_exclusive(&self, group: &::ToolItemGroup) -> bool {
        unsafe { to_bool(ffi::gtk_tool_palette_get_exclusive(GTK_TOOL_PALETTE(self.unwrap_widget()),
            GTK_TOOL_ITEM_GROUP(group.unwrap_widget()))) }
    }

    pub fn set_exclusive(&self, group: &::ToolItemGroup, exclusive: bool) {
        unsafe { ffi::gtk_tool_palette_set_exclusive(GTK_TOOL_PALETTE(self.unwrap_widget()),
            GTK_TOOL_ITEM_GROUP(group.unwrap_widget()), to_gboolean(exclusive)) }
    }

    pub fn get_expand(&self, group: &::ToolItemGroup) -> bool {
        unsafe { to_bool(ffi::gtk_tool_palette_get_expand(GTK_TOOL_PALETTE(self.unwrap_widget()),
            GTK_TOOL_ITEM_GROUP(group.unwrap_widget()))) }
    }

    pub fn set_expand(&self, group: &::ToolItemGroup, expand: bool) {
        unsafe { ffi::gtk_tool_palette_set_expand(GTK_TOOL_PALETTE(self.unwrap_widget()),
            GTK_TOOL_ITEM_GROUP(group.unwrap_widget()), to_gboolean(expand)) }
    }

    pub fn get_group_position(&self, group: &::ToolItemGroup) -> i32 {
        unsafe { ffi::gtk_tool_palette_get_group_position(GTK_TOOL_PALETTE(self.unwrap_widget()),
            GTK_TOOL_ITEM_GROUP(group.unwrap_widget())) }
    }

    pub fn set_group_position(&self, group: &::ToolItemGroup, expand: i32) {
        unsafe { ffi::gtk_tool_palette_set_group_position(GTK_TOOL_PALETTE(self.unwrap_widget()),
            GTK_TOOL_ITEM_GROUP(group.unwrap_widget()), expand as ::libc::c_int) }
    }

    pub fn get_drop_group(&self, x: i32, y: i32) -> Option<::ToolItemGroup> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_get_drop_group(GTK_TOOL_PALETTE(self.unwrap_widget()),
            x as ::libc::c_int, y as ::libc::c_int) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }
}

impl_drop!(ToolPalette);
impl_TraitWidget!(ToolPalette);

impl ::ContainerTrait for ToolPalette {}

impl_widget_events!(ToolPalette);
