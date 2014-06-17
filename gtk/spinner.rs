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

//! Show a spinner animation

use std::ptr;
use std::num::cast;
use libc::{c_void};

use traits::{GtkWidget, Signal};
use utils::cast::GTK_SPINNER;
use ffi;
use std;
use std::owned;

/// Spinner — Show a spinner animation
pub struct Spinner {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Spinner {
    pub fn new() -> Option<Spinner> {
        let tmp_pointer = unsafe { ffi::gtk_spinner_new() };
        check_pointer!(tmp_pointer, Spinner)
    }

    pub fn start(&mut self) -> () {
        unsafe {
            ffi::gtk_spinner_start(GTK_SPINNER(self.pointer))
        }
    }

    pub fn stop(&mut self) -> () {
        unsafe {
            ffi::gtk_spinner_stop(GTK_SPINNER(self.pointer))
        }
    }

}

impl_GtkWidget!(Spinner)
redirect_callback!(Spinner)
redirect_callback_widget!(Spinner)
struct_signal!(Spinner)
impl_signals!(Spinner)
