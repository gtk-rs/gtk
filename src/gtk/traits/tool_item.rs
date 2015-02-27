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

use std::ffi::CString;
use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::cast::GTK_TOOLITEM;
use gtk::{IconSize, Orientation, ReliefStyle, ToolbarStyle};

pub trait ToolItemTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait {
    fn set_homogeneous(&mut self, homogeneous: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_homogeneous(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(homogeneous)); }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_homogeneous(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_expand(&mut self, expand: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_expand(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(expand)); }
    }

    fn get_expand(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_expand(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_use_drag_window(&mut self, use_drag_window: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_use_drag_window(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(use_drag_window)); }
    }

    fn get_use_drag_window(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_use_drag_window(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_visible_horizontal(&mut self, visible_horizontal: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_visible_horizontal(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(visible_horizontal)); }
    }

    fn get_visible_horizontal(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_visible_horizontal(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_visible_vertical(&mut self, visible_vertical: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_visible_vertical(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(visible_vertical)); }
    }

    fn get_visible_vertical(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_visible_vertical(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_is_important(&mut self, is_important: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_is_important(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(is_important)); }
    }

    fn get_is_important(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_is_important(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_tooltip_text(&mut self, text: &str) -> () {
        let c_str = CString::from_slice(text.as_bytes());

        unsafe {
            ffi::gtk_tool_item_set_tooltip_text(GTK_TOOLITEM(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_tooltip_markup(&mut self, markup: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(markup.as_bytes());

            ffi::gtk_tool_item_set_tooltip_markup(GTK_TOOLITEM(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn get_icon_size(&self) -> IconSize {
        unsafe {
            ffi::gtk_tool_item_get_icon_size(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_item_get_orientation(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn get_toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_tool_item_get_toolbar_style(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_tool_item_get_relief_style(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_item_get_text_alignment(GTK_TOOLITEM(self.unwrap_widget())) as f32
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_item_get_text_orientation(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn rebuild_menu(&mut self) -> () {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn toolbar_reconfigured(&mut self) -> () {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn get_text_size_group(&self) -> Option<gtk::SizeGroup> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_get_text_size_group(GTK_TOOLITEM(self.unwrap_widget()) as *const ffi::C_GtkToolItem) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::SizeGroup::wrap_pointer(tmp_pointer))
        }
    }
}
