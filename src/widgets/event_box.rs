// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::{GTK_EVENT_BOX};
use ffi;
use glib::{to_bool, to_gboolean};

struct_Widget!(EventBox);

impl EventBox {
    pub fn new() -> Option<EventBox> {
        assert_initialized_main_thread!();
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
