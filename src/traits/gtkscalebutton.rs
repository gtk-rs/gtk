// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

use libc::c_double;

use traits::{GtkWidget, GtkContainer, GtkButton};
use utils::cast::GTK_SCALEBUTTON;
use gtk::Adjustment;
use ffi;
use std;

pub trait GtkScaleButton: GtkWidget + GtkContainer + GtkButton {
    fn set_adjustment(&mut self, adjustment: &Adjustment) -> () {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(GTK_SCALEBUTTON(self.get_widget()), adjustment.get_pointer());
        }
    }

    fn set_value(&mut self, value: f64) -> () {
        unsafe {
            ffi::gtk_scale_button_set_value(GTK_SCALEBUTTON(self.get_widget()), value as c_double);
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(GTK_SCALEBUTTON(self.get_widget())) as f64
        }
    }

    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            Adjustment::wrap_pointer(ffi::gtk_scale_button_get_adjustment(GTK_SCALEBUTTON(self.get_widget())))
        }
    }
}
