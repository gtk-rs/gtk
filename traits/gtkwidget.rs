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

use libc::c_int;

use ffi;

pub trait GtkWidget {
    #[doc(hidden)]
    fn get_widget(&self) -> *ffi::C_GtkWidget;
    fn wrap_widget(widget: *ffi::C_GtkWidget) -> Self;

    fn show_all(&mut self) -> () {
        unsafe {
            ffi::gtk_widget_show_all(self.get_widget());
        }
    }

    fn set_margin_right(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_right(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_left(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_left(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_top(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_top(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_bottom(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_bottom(self.get_widget(), margin as c_int)
        }
    }

    fn get_margin_right(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_right(self.get_widget()) as i32
        }
    }

    fn get_margin_left(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_left(self.get_widget()) as i32
        }
    }

    fn get_margin_top(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_top(self.get_widget()) as i32
        }
    }

    fn get_margin_bottom(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_bottom(self.get_widget()) as i32
        }
    }

    fn hide(&mut self) -> () {
        unsafe {
            ffi::gtk_widget_hide(self.get_widget());
        }
    }
}

