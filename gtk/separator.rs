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

//! A separator widget

use std::ptr;
use std::num::cast;
use libc::{c_void};

use traits::{GtkWidget, GtkOrientable, Signal};
use ffi;
use std;
use std::owned;
use gtk::enums::GtkOrientation;

/// Separator — A separator widget
pub struct Separator {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Separator {
    pub fn new(orientation: GtkOrientation) -> Option<Separator> {
        let tmp_pointer = unsafe { ffi::gtk_separator_new(orientation) };
        check_pointer!(tmp_pointer, Separator)
    }
}

impl_GtkWidget!(Separator)
redirect_callback!(Separator)
redirect_callback_widget!(Separator)
struct_signal!(Separator)
impl_signals!(Separator)

impl GtkOrientable for Separator {}