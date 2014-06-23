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
use gtk::traits::WidgetTrait;
use utils::cast::GTK_TOOLSHELL;
use gtk::enums::{IconSize, Orientation, ReliefStyle, ToolbarStyle};

pub trait ToolShellTrait: WidgetTrait {
    fn get_icon_size(&self) -> IconSize {
        unsafe {
            ffi::gtk_tool_shell_get_icon_size(GTK_TOOLSHELL(self.get_widget()))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_shell_get_orientation(GTK_TOOLSHELL(self.get_widget()))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_tool_shell_get_relief_style(GTK_TOOLSHELL(self.get_widget()))
        }
    }

    fn get_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_tool_shell_get_style(GTK_TOOLSHELL(self.get_widget()))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_shell_get_text_alignment(GTK_TOOLSHELL(self.get_widget()))
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_shell_get_text_orientation(GTK_TOOLSHELL(self.get_widget()))
        }
    }

    fn rebuild_menu(&mut self) -> () {
        unsafe {
            ffi::gtk_tool_shell_rebuild_menu(GTK_TOOLSHELL(self.get_widget()))
        }
    }
}
