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

//! A container which allows you to position widgets at fixed coordinates

use libc::c_int;

use utils::cast::GTK_FIXED;
use ffi;
use gtk::traits::*;
/// Fixed — A container which allows you to position widgets at fixed coordinates
struct_Widget!(Fixed)


impl Fixed {
    pub fn new() -> Option<Fixed> {
        let tmp_pointer = unsafe { ffi::gtk_fixed_new() };
        check_pointer!(tmp_pointer, Fixed)
    }

    pub fn put<T: WidgetTrait>(&mut self,
                             widget: &T,
                             x: i32,
                             y: i32) -> () {
        unsafe {
            ffi::gtk_fixed_put(GTK_FIXED(self.pointer), widget.get_widget(), x as c_int, y as c_int);
        }
    }

    pub fn move<T: WidgetTrait>(&mut self,
                              widget: &T,
                              x: i32,
                              y: i32) -> () {
        unsafe {
            ffi::gtk_fixed_move(GTK_FIXED(self.pointer), widget.get_widget(), x as c_int, y as c_int);
        }
    }
}

impl_GtkWidget!(Fixed)


impl ContainerTrait for Fixed {}
