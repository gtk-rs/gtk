// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_SWITCH;
use ffi;
use glib::{to_bool, to_gboolean};

/*
* # Availables signals:
* * `activate` : Action
*/
struct_Widget!(Switch);

impl Switch {
    pub fn new() -> Option<Switch> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_switch_new() };
        check_pointer!(tmp_pointer, Switch)
    }

    pub fn set_active(&self, is_active: bool) -> () {
        unsafe { ffi::gtk_switch_set_active(GTK_SWITCH(self.pointer), to_gboolean(is_active)); }
    }

    pub fn get_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_switch_get_active(GTK_SWITCH(self.pointer))) }
    }
}

impl_drop!(Switch);
impl_TraitWidget!(Switch);
