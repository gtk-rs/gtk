// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_double;

use cast::GTK_SCALEBUTTON;
use ffi;

pub trait ScaleButtonTrait: ::WidgetTrait + ::ContainerTrait + ::ButtonTrait {
    fn set_adjustment(&self, adjustment: &::Adjustment) -> () {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(GTK_SCALEBUTTON(self.unwrap_widget()), adjustment.unwrap_pointer());
        }
    }

    fn set_value(&self, value: f64) -> () {
        unsafe {
            ffi::gtk_scale_button_set_value(GTK_SCALEBUTTON(self.unwrap_widget()), value as c_double);
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(GTK_SCALEBUTTON(self.unwrap_widget())) as f64
        }
    }

    fn get_adjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_scale_button_get_adjustment(GTK_SCALEBUTTON(self.unwrap_widget())))
        }
    }
}
