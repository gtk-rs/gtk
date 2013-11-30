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

//! A container box

use std::{ptr, cast};
use std::libc::{c_void, c_int};

use gtk::enums::GtkOrientation;
use traits::{GtkContainer, GtkWidget, GtkBox, GtkOrientable, Signal};
use ffi;

/// Box — A container box
pub struct Box {
	priv pointer: 			*ffi::C_GtkWidget,
	priv can_drop: 			bool,
    priv signal_handlers: 	~[~SignalHandler]
}

impl Box {
	pub fn new(orientation: GtkOrientation, spacing: i32) -> Option<Box> {
		let tmp_pointer = unsafe { ffi::gtk_box_new(orientation, spacing as c_int) };
        check_pointer!(tmp_pointer, Box)
	}
}

impl_GtkWidget!(Box)
redirect_callback!(Box)
redirect_callback_widget!(Box)
struct_signal!(Box)
impl_signals!(Box)

impl GtkContainer for Box {}
impl GtkBox for Box {}
impl GtkOrientable for Box {}