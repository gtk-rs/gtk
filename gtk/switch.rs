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

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkWidget, Signal};
use utils::cast::GTK_SWITCH;
use ffi;

/*
* Switch — A "light switch" style toggle
*
* # Availables signals:
* * `activate` : Action
*/
pub struct Switch {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl Switch {
    pub fn new() -> Option<Switch> {
        let tmp_pointer = unsafe { ffi::gtk_switch_new() };
        check_pointer!(tmp_pointer, Switch)
    }

    pub fn set_active(&mut self, is_active: bool) -> () {
        match is_active {
            true    => unsafe { ffi::gtk_switch_set_active(GTK_SWITCH(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_switch_set_active(GTK_SWITCH(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_active(&self) -> bool {
        match unsafe { ffi::gtk_switch_get_active(GTK_SWITCH(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }
}

impl_GtkWidget!(Switch)
redirect_callback!(Switch)
redirect_callback_widget!(Switch)
struct_signal!(Switch)
impl_signals!(Switch)
