// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A widget which controls the alignment and size of its child

use std::ptr;
use libc::{c_float, c_uint, c_void};

use traits::{GtkWidget, GtkContainer, GtkBin, Signal};
use utils::cast::GTK_ALIGNMENT;
use ffi;
use std;

/// Alignment — A widget which controls the alignment and size of its child
pub struct Alignment {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Alignment {
    pub fn new(x_align: f32,
               y_align: f32,
               x_scale: f32,
               y_scale: f32) -> Option<Alignment> {
        let tmp_pointer = unsafe { ffi::gtk_alignment_new(x_align as c_float, y_align as c_float, x_scale as c_float, y_scale as c_float) };
        check_pointer!(tmp_pointer, Alignment)
    }

    pub fn set(&mut self,
               x_align: f32,
               y_align: f32,
               x_scale: f32,
               y_scale: f32) -> () {
        unsafe { 
            ffi::gtk_alignment_set(GTK_ALIGNMENT(self.pointer), x_align as c_float, y_align as c_float, x_scale as c_float, y_scale as c_float) 
        }
    }

    pub fn set_padding(&mut self,
                       padding_top: u32,
                       padding_bottom: u32,
                       padding_left: u32,
                       padding_right: u32) -> () {
        unsafe {
            ffi::gtk_alignment_set_padding(GTK_ALIGNMENT(self.pointer), padding_top as c_uint, padding_bottom as c_uint, padding_left as c_uint, padding_right as c_uint);
        }
    }

    pub fn get_padding(&self) -> (u32, u32, u32, u32) {
        let top =       0;
        let bottom =    0;
        let left =      0;
        let right =     0;
        unsafe {
            ffi::gtk_alignment_get_padding(GTK_ALIGNMENT(self.pointer), &top, &bottom, &left, &right);
        }
        (top, bottom, left, right)
    }
}

impl_GtkWidget!(Alignment)
redirect_callback!(Alignment)
redirect_callback_widget!(Alignment)
struct_signal!(Alignment)
impl_signals!(Alignment)

impl GtkContainer for Alignment {}
impl GtkBin for Alignment {}


