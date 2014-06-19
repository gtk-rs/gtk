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

use libc::{c_int, c_void};

use gtk::enums::GtkOrientation;
use traits::{GtkContainer, GtkWidget, GtkBox, GtkOrientable, Signal};
use ffi;

/// Box — A container box
pub struct _Box {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl _Box {
    pub fn new(orientation: GtkOrientation, spacing: i32) -> Option<_Box> {
        let tmp_pointer = unsafe { ffi::gtk_box_new(orientation, spacing as c_int) };
        check_pointer!(tmp_pointer, _Box)
    }
}

impl_GtkWidget!(_Box)
redirect_callback!(_Box)
redirect_callback_widget!(_Box)
struct_signal!(_Box)
impl_signals!(_Box)

impl GtkContainer for _Box {}
impl GtkBox for _Box {}
impl GtkOrientable for _Box {}