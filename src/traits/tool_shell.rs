// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use cast::GTK_TOOLSHELL;
use {Orientation, ReliefStyle, ToolbarStyle};

pub trait ToolShellTrait: ::WidgetTrait {
    fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_shell_get_icon_size(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_shell_get_orientation(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_tool_shell_get_relief_style(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn get_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_tool_shell_get_style(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_shell_get_text_alignment(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_shell_get_text_orientation(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn rebuild_menu(&self) -> () {
        unsafe {
            ffi::gtk_tool_shell_rebuild_menu(GTK_TOOLSHELL(self.unwrap_widget()))
        }
    }

    fn get_text_size_group(&self) -> Option<::SizeGroup> {
        let tmp_pointer = unsafe { ffi::gtk_tool_shell_get_text_size_group(GTK_TOOLSHELL(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::SizeGroup::wrap_pointer(tmp_pointer))
        }
    }
}
