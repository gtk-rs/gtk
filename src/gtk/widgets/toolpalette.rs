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

use gtk::{ffi, ToolItem};
use gtk::ffi::FFIWidget;
use gtk::traits;
use gtk::cast::GTK_TOOL_PALETTE;
use gtk;

struct_Widget!(ToolPalette)

impl ToolPalette {
    pub fn new() -> Option<ToolPalette> {
        let tmp_pointer = unsafe { ffi::gtk_tool_palette_new() };
        check_pointer!(tmp_pointer, ToolPalette)
    }

    pub fn get_icon_size(&self) -> gtk::enums::IconSize {
        unsafe { ffi::gtk_tool_palette_get_icon_size(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn set_icon_size(&self, icon_size: gtk::enums::IconSize) {
        unsafe { ffi::gtk_tool_palette_set_icon_size(GTK_TOOL_PALETTE(self.get_widget()), icon_size) }
    }

    pub fn unset_icon_size(&self) {
        unsafe { ffi::gtk_tool_palette_unset_icon_size(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn get_style(&self) -> gtk::enums::ToolbarStyle {
        unsafe { ffi::gtk_tool_palette_get_style(GTK_TOOL_PALETTE(self.get_widget())) }
    }

    pub fn set_style(&self, style: gtk::enums::ToolbarStyle) {
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

    pub fn set_drag_source(&self, targets: gtk::enums::ToolPaletteDragTargets) {
        unsafe { ffi::gtk_tool_palette_set_drag_source(GTK_TOOL_PALETTE(self.get_widget()), targets) }
    }
}

impl_drop!(ToolPalette)
impl_TraitWidget!(ToolPalette)

impl traits::Container for ToolPalette {}

impl_widget_events!(ToolPalette)