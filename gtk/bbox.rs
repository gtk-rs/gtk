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

//! A container box

use std::{ptr, mem};
use libc::{c_void, c_int};

use gtk::enums::GtkOrientation;
use traits::{GtkContainer, GtkWidget, GtkBox, GtkOrientable, Signal};
use ffi;

/// Box — A container box
pub struct BBox {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl BBox {
    pub fn new(orientation: GtkOrientation, spacing: i32) -> Option<BBox> {
        let tmp_pointer = unsafe { ffi::gtk_box_new(orientation, spacing as c_int) };
        check_pointer!(tmp_pointer, BBox)
    }
}

impl_GtkWidget!(BBox)
redirect_callback!(BBox)
redirect_callback_widget!(BBox)
struct_signal!(BBox)
impl_signals!(BBox)

impl GtkContainer for BBox {}
impl GtkBox for BBox {}
impl GtkOrientable for BBox {}