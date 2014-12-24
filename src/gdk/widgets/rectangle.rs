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

//! Rectangles — Simple graphical data type

use gdk::ffi;
use libc::{c_int};
use gtk;

#[repr(C)]
#[deriving(Copy)]
pub struct Rectangle { // FIXME should be just an alias to cairo_rectangle_int_t
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int
}

impl Rectangle {
    pub fn intersect(&self, other: &Rectangle, dest: &mut Rectangle) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_rectangle_intersect(self, other, dest)) }
    }

    pub fn union(&self, other: &Rectangle, dest: &mut Rectangle) {
        unsafe { ffi::gdk_rectangle_union(self, other, dest) }
    }
}