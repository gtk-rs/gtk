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
use gdk_ffi::C_GdkRectangle;
use glib_ffi::to_bool;

pub trait Rectangle {
    fn intersect(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle) -> bool;
    fn union(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle);
}

impl Rectangle for C_GdkRectangle {
    fn intersect(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle) -> bool {
        unsafe { to_bool(ffi::gdk_rectangle_intersect(self, other, dest)) }
    }

    fn union(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle) {
        unsafe { ffi::gdk_rectangle_union(self, other, dest) }
    }
}
