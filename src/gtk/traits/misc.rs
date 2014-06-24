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

use libc::{c_float, c_int};
use utils::cast::GTK_MISC;
use ffi;
use gtk::traits::Widget;

pub trait Misc : Widget {
    fn set_alignment(&mut self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_misc_set_alignment(GTK_MISC(self.get_widget()), x_align as c_float, y_align as c_float);
        }
    }

    fn set_padding(&mut self, x_pad: i32, y_pad: i32) -> () {
        unsafe {
            ffi::gtk_misc_set_padding(GTK_MISC(self.get_widget()), x_pad as c_int, y_pad as c_int);
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let x: c_float = 0.;
        let y: c_float = 0.;
        unsafe {
            ffi::gtk_misc_get_alignment(GTK_MISC(self.get_widget()), &x, &y);
        }
        (x as f32, y as f32)
    }

    fn get_padding(&self) -> (i32, i32) {
        let x: c_int = 0;
        let y: c_int = 0;
        unsafe {
            ffi::gtk_misc_get_padding(GTK_MISC(self.get_widget()), &x, &y);
        }
        (x as i32, y as i32)
    }
}