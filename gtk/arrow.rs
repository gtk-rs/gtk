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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.

//! Displays an arrow

use std::{ptr, cast};
use std::libc::c_void;

use gtk::enums::{GtkShadowType, GtkArrowType};
use traits::{GtkWidget, Signal, GtkMisc};
use utils::cast::GTK_ARROW;
use ffi;

/// Arrow — Displays an arrow
pub struct Arrow {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl Arrow {
    pub fn new(arrow_type: GtkArrowType, shadow_type: GtkShadowType) -> Option<Arrow> {
        let tmp_pointer = unsafe { ffi::gtk_arrow_new(arrow_type, shadow_type) };
        check_pointer!(tmp_pointer, Arrow)
    }

    pub fn set(&mut self, arrow_type: GtkArrowType, shadow_type: GtkShadowType) -> () {
        unsafe {
            ffi::gtk_arrow_set(GTK_ARROW(self.pointer), arrow_type, shadow_type);
        }
    }
}

impl_GtkWidget!(Arrow)
redirect_callback!(Arrow)
redirect_callback_widget!(Arrow)
struct_signal!(Arrow)
impl_signals!(Arrow)

impl GtkMisc for Arrow {}
