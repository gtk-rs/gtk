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

//! A bin with a decorative frame and optional label

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkWidget, GtkFrame, GtkContainer, GtkBin, Signal};
use ffi;

/// Frame — A bin with a decorative frame and optional label
pub struct Frame {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl Frame {
    pub fn new(label: Option<&str>) -> Option<Frame> {
        let tmp_pointer = match label {
            Some(l) => unsafe { l.with_c_str(|c_str| { ffi::gtk_frame_new(c_str) }) },
            None    => unsafe { ffi::gtk_frame_new(ptr::null()) }
        };
        check_pointer!(tmp_pointer, Frame)
    }
}

impl_GtkWidget!(Frame)
redirect_callback!(Frame)
redirect_callback_widget!(Frame)
struct_signal!(Frame)
impl_signals!(Frame)

impl GtkFrame for Frame {}
impl GtkContainer for Frame {}
impl GtkBin for Frame {}