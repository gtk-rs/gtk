// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_float, c_uint};

use cast::GTK_ALIGNMENT;
use ffi;

struct_Widget!(Alignment);

impl Alignment {
    pub fn new(x_align: f32,
               y_align: f32,
               x_scale: f32,
               y_scale: f32) -> Option<Alignment> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_alignment_new(x_align as c_float, y_align as c_float, x_scale as c_float, y_scale as c_float) };
        check_pointer!(tmp_pointer, Alignment)
    }

    pub fn set(&self,
               x_align: f32,
               y_align: f32,
               x_scale: f32,
               y_scale: f32) -> () {
        unsafe {
            ffi::gtk_alignment_set(GTK_ALIGNMENT(self.pointer), x_align as c_float, y_align as c_float, x_scale as c_float, y_scale as c_float)
        }
    }

    pub fn set_padding(&self,
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

impl ::ContainerTrait for Alignment {}
impl ::BinTrait for Alignment {}
