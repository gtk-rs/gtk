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

//! GtkEventBox — A widget used to catch events for widgets which do not have their own window

use cast::{GTK_EVENT_BOX};
use ffi;
use glib::{to_bool, to_gboolean};

struct_Widget!(EventBox);

impl EventBox {
    pub fn new() -> Option<EventBox> {
        let tmp_pointer = unsafe { ffi::gtk_event_box_new() };
        check_pointer!(tmp_pointer, EventBox)
    }

    pub fn set_above_child(&self, above_child: bool) {
        unsafe {
            ffi::gtk_event_box_set_above_child(GTK_EVENT_BOX(self.pointer), to_gboolean(above_child))
        }
    }

    pub fn get_above_child(&self) -> bool {
        unsafe { to_bool(ffi::gtk_event_box_get_above_child(GTK_EVENT_BOX(self.pointer))) }
    }

    pub fn set_visible_window(&self, visible_window: bool) {
        unsafe {
            ffi::gtk_event_box_set_visible_window(GTK_EVENT_BOX(self.pointer), to_gboolean(visible_window))
        }
    }

    pub fn get_visible_window(&self) -> bool {
        unsafe { to_bool(ffi::gtk_event_box_get_visible_window(GTK_EVENT_BOX(self.pointer))) }
    }
}

impl_drop!(EventBox);
impl_TraitWidget!(EventBox);

impl ::ContainerTrait for EventBox {}
impl ::BinTrait for EventBox {}

impl_widget_events!(EventBox);
