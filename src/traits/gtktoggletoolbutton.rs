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


use traits::{GtkWidget, GtkContainer, GtkBin, GtkToolItem, GtkToolButton};
use utils::cast::GTK_TOGGLETOOLBUTTON;
use ffi;
use std;

pub trait GtkToggleToolButton: GtkWidget + GtkContainer + GtkBin + GtkToolItem + GtkToolButton {

    fn get_active(&self) -> bool {
        match unsafe { ffi::gtk_toggle_tool_button_get_active(GTK_TOGGLETOOLBUTTON(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_active(&mut self, set_underline: bool) -> () {
         match set_underline {
            true    => unsafe { ffi::gtk_toggle_tool_button_set_active(GTK_TOGGLETOOLBUTTON(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_toggle_tool_button_set_active(GTK_TOGGLETOOLBUTTON(self.get_widget()), ffi::Gfalse) }
        }
    }
}