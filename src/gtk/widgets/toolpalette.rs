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

use gtk::{mod, ffi, ToolItem};
use gtk::ffi::FFIWidget;
use gtk::cast::{GTK_TOOL_PALETTE, GTK_TOOL_ITEM_GROUP};

struct_Widget!(ToolPalette)

impl ToolPalette {
    pub fn new() -> Option<ToolPalette> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_new() };
        check_pointer!(tmp_pointer, ToolPalette)
    }

    pub fn get_icon_size(&self) -> gtk::IconSize {
        unsafe { ffi::gtk_tool_palette_get_icon_size(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn set_icon_size(&self, icon_size: gtk::IconSize) {
        unsafe { ffi::gtk_tool_palette_set_icon_size(GTK_TOOL_PALETTE(self.get_widget()), icon_size) }
    }

    pub fn unset_icon_size(&self) {
        unsafe { ffi::gtk_tool_palette_unset_icon_size(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn get_style(&self) -> gtk::ToolbarStyle {
        unsafe { ffi::gtk_tool_palette_get_style(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn set_style(&self, style: gtk::ToolbarStyle) {
        unsafe { ffi::gtk_tool_palette_set_style(GTK_TOOL_PALETTE(self.get_widget()), style) }
    }

    pub fn unset_style(&self) {
        unsafe { ffi::gtk_tool_palette_unset_style(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_get_drop_item(GTK_TOOL_PALETTE(self.get_widget()),
            x as ::libc::c_int, y as ::libc::c_int) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn set_drag_source(&self, targets: gtk::ToolPaletteDragTargets) {
        unsafe { ffi::gtk_tool_palette_set_drag_source(GTK_TOOL_PALETTE(self.get_widget()), targets) }
    }

    pub fn get_exclusive(&self, group: &gtk::ToolItemGroup) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_tool_palette_get_exclusive(GTK_TOOL_PALETTE(self.get_widget()),
            GTK_TOOL_ITEM_GROUP(group.get_widget()))) }
    }

    pub fn set_exclusive(&self, group: &gtk::ToolItemGroup, exclusive: bool) {
        unsafe { ffi::gtk_tool_palette_set_exclusive(GTK_TOOL_PALETTE(self.get_widget()),
            GTK_TOOL_ITEM_GROUP(group.get_widget()), ffi::to_gboolean(exclusive)) }
    }

    pub fn get_expand(&self, group: &gtk::ToolItemGroup) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_tool_palette_get_expand(GTK_TOOL_PALETTE(self.get_widget()),
            GTK_TOOL_ITEM_GROUP(group.get_widget()))) }
    }

    pub fn set_expand(&self, group: &gtk::ToolItemGroup, expand: bool) {
        unsafe { ffi::gtk_tool_palette_set_expand(GTK_TOOL_PALETTE(self.get_widget()),
            GTK_TOOL_ITEM_GROUP(group.get_widget()), ffi::to_gboolean(expand)) }
    }

    pub fn get_group_position(&self, group: &gtk::ToolItemGroup) -> i32 {
        unsafe { ffi::gtk_tool_palette_get_group_position(GTK_TOOL_PALETTE(self.get_widget()),
            GTK_TOOL_ITEM_GROUP(group.get_widget())) }
    }

    pub fn set_group_position(&self, group: &gtk::ToolItemGroup, expand: i32) {
        unsafe { ffi::gtk_tool_palette_set_group_position(GTK_TOOL_PALETTE(self.get_widget()),
            GTK_TOOL_ITEM_GROUP(group.get_widget()), expand as ::libc::c_int) }
    }

    pub fn get_drop_group(&self, x: i32, y: i32) -> Option<gtk::ToolItemGroup> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_get_drop_group(GTK_TOOL_PALETTE(self.get_widget()),
            x as ::libc::c_int, y as ::libc::c_int) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }
}

impl_drop!(ToolPalette)
impl_TraitWidget!(ToolPalette)

impl gtk::ContainerTrait for ToolPalette {}

impl_widget_events!(ToolPalette)