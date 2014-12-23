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

//! Visuals — Low-level display hardware information

use gdk::{mod, ffi};
use libc::{c_int};

#[repr(C)]
#[deriving(Copy)]
pub struct Visual {
    pointer: *mut ffi::C_GdkVisual
}

impl Visual {
    pub fn query_depths() -> Vec<i32> {
        let mut tmp = ::std::ptr::null_mut();
        let mut count = 0i32;

        unsafe {
            ffi::gdk_query_depths(&mut tmp, &mut count);
            Vec::from_raw_buf(tmp, count as uint)
        }
    }

    pub fn get_bits_per_rgb(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_bits_per_rgb(self.pointer) }
    }

    pub fn get_blue_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe { ffi::gdk_visual_get_blue_pixel_details(self.pointer, mask, shift as *mut c_int, precision as *mut c_int) }
    }

    pub fn get_colormap_size(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_colormap_size(self.pointer) }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_depth(self.pointer) }
    }

    pub fn get_green_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe { ffi::gdk_visual_get_green_pixel_details(self.pointer, mask, shift as *mut c_int, precision as *mut c_int) }
    }

    pub fn get_red_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe { ffi::gdk_visual_get_red_pixel_details(self.pointer, mask, shift as *mut c_int, precision as *mut c_int) }
    }

    pub fn get_best_depth() -> i32 {
        unsafe { ffi::gdk_visual_get_best_depth() }
    }

    pub fn get_system() -> Option<Visual> {
        let tmp = unsafe { ffi::gdk_visual_get_system() };

        if tmp.is_null() {
            None
        } else {
            Some(Visual {
                pointer: tmp
            })
        }
    }

    pub fn get_best() -> Option<Visual> {
        let tmp = unsafe { ffi::gdk_visual_get_best() };

        if tmp.is_null() {
            None
        } else {
            Some(Visual {
                pointer: tmp
            })
        }
    }

    pub fn get_best_with_depth(depth: i32) -> Option<Visual> {
        let tmp = unsafe { ffi::gdk_visual_get_best_with_depth(depth as c_int) };

        if tmp.is_null() {
            None
        } else {
            Some(Visual {
                pointer: tmp
            })
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        let tmp = unsafe { ffi::gdk_visual_get_screen(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Screen::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(Visual, C_GdkVisual);