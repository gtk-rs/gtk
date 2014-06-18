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

//! Toplevel which can contain other widgets

use std::ptr;
use libc::{c_void};

use traits::{GtkWidget, GtkWindow, GtkContainer, GtkBin, Signal};
use ffi;
use std;
use gtk::enums::GtkWindowType;

/**
* Window — Toplevel which can contain other widgets
*
* # Available signals:
* * `activate-default` : Action
* * `activate-focus` : Action
* * `keys-changed` : Run First
* * `set-focus` : Run Last
*/

pub struct Window {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Window {
    pub fn new(window_type: GtkWindowType) -> Option<Window> {
        let tmp_pointer = unsafe { ffi::gtk_window_new(window_type) };
        check_pointer!(tmp_pointer, Window)
    }
}

impl_GtkWidget!(Window)
redirect_callback!(Window)
redirect_callback_widget!(Window)
struct_signal!(Window)
impl_signals!(Window)

impl GtkContainer for Window {}
impl GtkWindow for Window {}
impl GtkBin for Window {}
