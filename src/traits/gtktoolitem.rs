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

use ffi;
use std;
use traits::{GtkWidget, GtkContainer, GtkBin};
use utils::cast::GTK_TOOLITEM;
use gtk::enums::{GtkIconSize, GtkOrientation, GtkReliefStyle, GtkToolbarStyle};

pub trait GtkToolItem: GtkWidget + GtkContainer + GtkBin {
    fn set_homogeneous(&mut self, homogeneous: bool) -> () {
         match homogeneous {
            true    => unsafe { ffi::gtk_tool_item_set_homogeneous(GTK_TOOLITEM(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_item_set_homogeneous(GTK_TOOLITEM(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_homogeneous(&self) -> bool {
        match unsafe { ffi::gtk_tool_item_get_homogeneous(GTK_TOOLITEM(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_expand(&mut self, expand: bool) -> () {
         match expand {
            true    => unsafe { ffi::gtk_tool_item_set_expand(GTK_TOOLITEM(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_item_set_expand(GTK_TOOLITEM(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_expand(&self) -> bool {
        match unsafe { ffi::gtk_tool_item_get_expand(GTK_TOOLITEM(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_use_drag_window(&mut self, use_drag_window: bool) -> () {
         match use_drag_window {
            true    => unsafe { ffi::gtk_tool_item_set_use_drag_window(GTK_TOOLITEM(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_item_set_use_drag_window(GTK_TOOLITEM(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_use_drag_window(&self) -> bool {
        match unsafe { ffi::gtk_tool_item_get_use_drag_window(GTK_TOOLITEM(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_visible_horizontal(&mut self, visible_horizontal: bool) -> () {
         match visible_horizontal {
            true    => unsafe { ffi::gtk_tool_item_set_visible_horizontal(GTK_TOOLITEM(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_item_set_visible_horizontal(GTK_TOOLITEM(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_visible_horizontal(&self) -> bool {
        match unsafe { ffi::gtk_tool_item_get_visible_horizontal(GTK_TOOLITEM(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_visible_vertical(&mut self, visible_vertical: bool) -> () {
         match visible_vertical {
            true    => unsafe { ffi::gtk_tool_item_set_visible_vertical(GTK_TOOLITEM(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_item_set_visible_vertical(GTK_TOOLITEM(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_visible_vertical(&self) -> bool {
        match unsafe { ffi::gtk_tool_item_get_visible_vertical(GTK_TOOLITEM(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_is_important(&mut self, is_important: bool) -> () {
         match is_important {
            true    => unsafe { ffi::gtk_tool_item_set_is_important(GTK_TOOLITEM(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_tool_item_set_is_important(GTK_TOOLITEM(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_is_important(&self) -> bool {
        match unsafe { ffi::gtk_tool_item_get_is_important(GTK_TOOLITEM(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_tooltip_text(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_tool_item_set_tooltip_text(GTK_TOOLITEM(self.get_widget()), c_str)
            })
        }
    }

    fn set_tooltip_markup(&mut self, markup: &str) -> () {
        unsafe {
            markup.with_c_str(|c_str| {
                ffi::gtk_tool_item_set_tooltip_markup(GTK_TOOLITEM(self.get_widget()), c_str)
            })
        }
    }

    fn get_icon_size(&self) -> GtkIconSize {
        unsafe {
            ffi::gtk_tool_item_get_icon_size(GTK_TOOLITEM(self.get_widget()))
        }
    }

    fn get_orientation(&self) -> GtkOrientation {
        unsafe {
            ffi::gtk_tool_item_get_orientation(GTK_TOOLITEM(self.get_widget()))
        }
    }

    fn get_toolbar_style(&self) -> GtkToolbarStyle {
        unsafe {
            ffi::gtk_tool_item_get_toolbar_style(GTK_TOOLITEM(self.get_widget()))
        }
    }

    fn get_relief_style(&self) -> GtkReliefStyle {
        unsafe {
            ffi::gtk_tool_item_get_relief_style(GTK_TOOLITEM(self.get_widget()))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_item_get_text_alignment(GTK_TOOLITEM(self.get_widget())) as f32
        }
    }

    fn get_text_orientation(&self) -> GtkOrientation {
        unsafe {
            ffi::gtk_tool_item_get_text_orientation(GTK_TOOLITEM(self.get_widget()))
        }
    }

    fn rebuild_menu(&mut self) -> () {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(GTK_TOOLITEM(self.get_widget()))
        }
    }

    fn toolbar_reconfigured(&mut self) -> () {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(GTK_TOOLITEM(self.get_widget()))
        }
    }
}

