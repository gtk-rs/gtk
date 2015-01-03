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

//! A widget which controls the alignment and size of its child

use libc::{c_float, c_uint};

use gtk::cast::GTK_ALIGNMENT;
use gtk::{self, ffi};

/// Alignment — A widget which controls the alignment and size of its child
struct_Widget!(Alignment);

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
        let mut top =       0;
        let mut bottom =    0;
        let mut left =      0;
        let mut right =     0;
        unsafe {
            ffi::gtk_alignment_get_padding(GTK_ALIGNMENT(self.pointer), &mut top, &mut bottom, &mut left, &mut right);
        }
        (top, bottom, left, right)
    }
}

impl_drop!(Alignment);
impl_TraitWidget!(Alignment);

impl gtk::ContainerTrait for Alignment {}
impl gtk::BinTrait for Alignment {}

impl_widget_events!(Alignment);
