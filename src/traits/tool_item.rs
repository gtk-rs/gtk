// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::ToGlibPtr;
use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_TOOLITEM;
use {IconSize, Orientation, ReliefStyle, ToolbarStyle};

pub trait ToolItemTrait: ::WidgetTrait + ::ContainerTrait + ::BinTrait {
    fn set_homogeneous(&self, homogeneous: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_homogeneous(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(homogeneous)); }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_homogeneous(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_expand(&self, expand: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_expand(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(expand)); }
    }

    fn get_expand(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_expand(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_use_drag_window(&self, use_drag_window: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_use_drag_window(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(use_drag_window)); }
    }

    fn get_use_drag_window(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_use_drag_window(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_visible_horizontal(&self, visible_horizontal: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_visible_horizontal(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(visible_horizontal)); }
    }

    fn get_visible_horizontal(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_visible_horizontal(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_visible_vertical(&self, visible_vertical: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_visible_vertical(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(visible_vertical)); }
    }

    fn get_visible_vertical(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_visible_vertical(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_is_important(&self, is_important: bool) -> () {
         unsafe { ffi::gtk_tool_item_set_is_important(GTK_TOOLITEM(self.unwrap_widget()), to_gboolean(is_important)); }
    }

    fn get_is_important(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_get_is_important(GTK_TOOLITEM(self.unwrap_widget()))) }
    }

    fn set_tooltip_text(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_text(GTK_TOOLITEM(self.unwrap_widget()),
                                                text.to_glib_none().0)
        }
    }

    fn set_tooltip_markup(&self, markup: &str) -> () {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_markup(GTK_TOOLITEM(self.unwrap_widget()), markup.to_glib_none().0)
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

    fn rebuild_menu(&self) -> () {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn toolbar_reconfigured(&self) -> () {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(GTK_TOOLITEM(self.unwrap_widget()))
        }
    }

    fn get_text_size_group(&self) -> Option<::SizeGroup> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_get_text_size_group(GTK_TOOLITEM(self.unwrap_widget()) as *const ffi::C_GtkToolItem) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::SizeGroup::wrap_pointer(tmp_pointer))
        }
    }
}
