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

//! A "light switch" style toggle

use cast::GTK_SWITCH;
use ffi;
use glib::{to_bool, to_gboolean};

/// Switch — A "light switch" style toggle
/*
* # Availables signals:
* * `activate` : Action
*/
struct_Widget!(Switch);

impl Switch {
    pub fn new() -> Option<Switch> {
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

impl_widget_events!(Switch);
