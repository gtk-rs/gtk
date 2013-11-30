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

//! A button which pops up a volume control

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkWidget, GtkButton, GtkContainer, GtkScaleButton, GtkOrientable, Signal};
use ffi;

/// VolumeButton — A button which pops up a volume control
pub struct VolumeButton {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl VolumeButton {
    pub fn new() -> Option<VolumeButton> {
        let tmp_pointer = unsafe { ffi::gtk_volume_button_new() };
        check_pointer!(tmp_pointer, VolumeButton)
    }
}

impl_GtkWidget!(VolumeButton)
redirect_callback!(VolumeButton)
redirect_callback_widget!(VolumeButton)
struct_signal!(VolumeButton)
impl_signals!(VolumeButton)

impl GtkContainer for VolumeButton {}
impl GtkButton for VolumeButton {}
impl GtkScaleButton for VolumeButton {}
impl GtkOrientable for VolumeButton {}
