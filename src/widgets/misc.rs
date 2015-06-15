// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_float, c_int};
use cast::GTK_MISC;
use ffi;

pub trait MiscTrait: ::WidgetTrait {
    fn set_alignment(&self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_misc_set_alignment(GTK_MISC(self.unwrap_widget()), x_align as c_float, y_align as c_float);
        }
    }

    fn set_padding(&self, x_pad: i32, y_pad: i32) -> () {
        unsafe {
            ffi::gtk_misc_set_padding(GTK_MISC(self.unwrap_widget()), x_pad as c_int, y_pad as c_int);
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let x: c_float = 0.;
        let y: c_float = 0.;
        unsafe {
            ffi::gtk_misc_get_alignment(GTK_MISC(self.unwrap_widget()), &x, &y);
        }
        (x as f32, y as f32)
    }

    fn get_padding(&self) -> (i32, i32) {
        let x: c_int = 0;
        let y: c_int = 0;
        unsafe {
            ffi::gtk_misc_get_padding(GTK_MISC(self.unwrap_widget()), &x, &y);
        }
        (x as i32, y as i32)
    }
}