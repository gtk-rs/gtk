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

#![allow(visible_private_types)]

use libc::c_int;
use gtk::ffi;
use gtk::signals::Signal;

pub trait Widget {
    #[doc(hidden)]
    fn get_widget(&self) -> *mut ffi::C_GtkWidget;
    fn wrap(widget: *mut ffi::C_GtkWidget) -> Self;

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

    fn set_size_request(&mut self, width: i32, height: i32) {
        unsafe{
            ffi::gtk_widget_set_size_request(self.get_widget(), width, height)
        }
    }

    fn hide(&mut self) -> () {
        unsafe {
            ffi::gtk_widget_hide(self.get_widget());
        }
    }

    fn connect<'a>(&self, signal: Box<Signal<'a>>) -> () {
        use std::mem::transmute;

        unsafe {
            let signal_name     = signal.get_signal_name();
            let trampoline      = signal.get_trampoline();

            let user_data_ptr   = transmute(box signal);

            signal_name.replace("_", "-").with_c_str(|signal_name| {
                ffi::glue_signal_connect(
                    self.get_widget(),
                    signal_name,
                    Some(trampoline),
                    user_data_ptr
                )
            });
        }
    }

    fn get_allocated_width(&self) -> i32{
        unsafe{
            ffi::gtk_widget_get_allocated_width(self.get_widget()) as i32
        }
    }

    fn get_allocated_height(&self) -> i32{
        unsafe{
            ffi::gtk_widget_get_allocated_height(self.get_widget()) as i32
        }
    }
}

