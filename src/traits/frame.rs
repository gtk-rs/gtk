// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_float;

use glib::translate::{FromGlibPtr, ToGlibPtr};
use ShadowType;
use cast::GTK_FRAME;
use ffi;

pub trait FrameTrait: ::WidgetTrait + ::ContainerTrait {
    fn set_label(&self, label: Option<&str>) -> () {
        unsafe {
            ffi::gtk_frame_set_label(GTK_FRAME(self.unwrap_widget()),
                                     label.borrow_to_glib().0);
        }
    }

    fn set_label_widget<T: ::WidgetTrait>(&self, label_widget: &T) -> () {
        unsafe {
            ffi::gtk_frame_set_label_widget(GTK_FRAME(self.unwrap_widget()), label_widget.unwrap_widget());
        }
    }

    fn set_label_align(&self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_frame_set_label_align(GTK_FRAME(self.unwrap_widget()), x_align as c_float, y_align as c_float);
        }
    }

    fn get_label_align(&self) -> (f32, f32) {
        let mut x_align = 0.;
        let mut y_align = 0.;
        unsafe {
            ffi::gtk_frame_get_label_align(GTK_FRAME(self.unwrap_widget()), &mut x_align, &mut y_align);
        }
        (x_align as f32, y_align as f32)
    }

    fn set_shadow_type(&self, st_type: ShadowType) -> () {
        unsafe {
            ffi::gtk_frame_set_shadow_type(GTK_FRAME(self.unwrap_widget()), st_type);
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_frame_get_label(GTK_FRAME(self.unwrap_widget())))
        }
    }

    fn gtk_frame_get_shadow_type(&self) -> ShadowType {
        unsafe {
            ffi::gtk_frame_get_shadow_type(GTK_FRAME(self.unwrap_widget()))
        }
    }
}
