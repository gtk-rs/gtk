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

use cast::GTK_TOGGLETOOLBUTTON;
use ffi;
use glib::{to_bool, to_gboolean};

pub trait ToggleToolButtonTrait: ::WidgetTrait +
                                 ::ContainerTrait +
                                 ::BinTrait +
                                 ::ToolItemTrait +
                                 ::ToolButtonTrait {

    fn get_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_tool_button_get_active(GTK_TOGGLETOOLBUTTON(self.unwrap_widget()))) }
    }

    fn set_active(&mut self, set_underline: bool) -> () {
         unsafe { ffi::gtk_toggle_tool_button_set_active(GTK_TOGGLETOOLBUTTON(self.unwrap_widget()), to_gboolean(set_underline)); }
    }
}