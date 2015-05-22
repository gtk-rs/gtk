// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_float;
use glib::translate::{from_glib_none, ToGlibPtr};

use {ReliefStyle, PositionType};
use cast::GTK_BUTTON;
use ffi;
use glib::{to_bool, to_gboolean};

pub trait ButtonTrait: ::WidgetTrait + ::ContainerTrait {
    fn pressed(&self) -> () {
        unsafe {
            ffi::gtk_button_pressed(GTK_BUTTON(self.unwrap_widget()));
        }
    }

    fn released(&self) -> () {
        unsafe {
            ffi::gtk_button_released(GTK_BUTTON(self.unwrap_widget()));
        }
    }

    fn clicked(&self) -> () {
        unsafe {
            ffi::gtk_button_clicked(GTK_BUTTON(self.unwrap_widget()));
        }
    }

    fn enter(&self) -> () {
        unsafe {
            ffi::gtk_button_enter(GTK_BUTTON(self.unwrap_widget()));
        }
    }

    fn leave(&self) -> () {
        unsafe {
            ffi::gtk_button_leave(GTK_BUTTON(self.unwrap_widget()));
        }
    }

    fn set_relief(&self, new_style: ReliefStyle) -> () {
        unsafe {
            ffi::gtk_button_set_relief(GTK_BUTTON(self.unwrap_widget()), new_style);
        }
    }

    fn get_relief(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_button_get_relief(GTK_BUTTON(self.unwrap_widget()))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_label(GTK_BUTTON(self.unwrap_widget())))
        }
    }

    fn set_label(&self, label: &str) -> () {
        unsafe {
            ffi::gtk_button_set_label(GTK_BUTTON(self.unwrap_widget()), label.to_glib_none().0)
        }
    }

    fn get_use_stock(&self) -> bool {
        unsafe { to_bool(ffi::gtk_button_get_use_stock(GTK_BUTTON(self.unwrap_widget()))) }
    }

    fn set_use_stock(&self, use_stock: bool) -> () {
        unsafe { ffi::gtk_button_set_use_stock(GTK_BUTTON(self.unwrap_widget()), to_gboolean(use_stock)); }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_button_get_use_underline(GTK_BUTTON(self.unwrap_widget()))) }
    }

    fn set_use_underline(&self, use_underline: bool) -> () {
        unsafe { ffi::gtk_button_set_use_underline(GTK_BUTTON(self.unwrap_widget()), to_gboolean(use_underline)); }
    }

    fn set_focus_on_click(&self, focus_on_click: bool) -> () {
        unsafe { ffi::gtk_button_set_focus_on_click(GTK_BUTTON(self.unwrap_widget()), to_gboolean(focus_on_click)); }
    }

    fn get_focus_on_click(&self) -> bool {
        unsafe { to_bool(ffi::gtk_button_get_focus_on_click(GTK_BUTTON(self.unwrap_widget()))) }
    }

    fn set_alignment(&self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_button_set_alignment(GTK_BUTTON(self.unwrap_widget()), x_align as c_float, y_align as c_float)
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let mut x_align = 0.1;
        let mut y_align = 0.1;
        unsafe {
            ffi::gtk_button_get_alignment(GTK_BUTTON(self.unwrap_widget()), &mut x_align, &mut y_align);
        }
        (x_align as f32, y_align as f32)
    }

    fn set_image<T: ::WidgetTrait>(&self, image: &T) -> () {
        unsafe {
            ffi::gtk_button_set_image(GTK_BUTTON(self.unwrap_widget()), image.unwrap_widget());
        }
    }

    fn set_image_position(&self, position: PositionType) -> () {
        unsafe {
            ffi::gtk_button_set_image_position(GTK_BUTTON(self.unwrap_widget()), position);
        }
    }

    fn get_image_position(&self) -> PositionType {
        unsafe {
            ffi::gtk_button_get_image_position(GTK_BUTTON(self.unwrap_widget()))
        }
    }

    #[cfg(feature = "gtk_3_6")]
    fn set_always_show_image(&self, always_show: bool) -> () {
        unsafe { ffi::gtk_button_set_always_show_image(GTK_BUTTON(self.unwrap_widget()), to_gboolean(always_show)); }
    }

    #[cfg(feature = "gtk_3_6")]
    fn get_always_show_image(&self) -> bool {
        unsafe { to_bool(ffi::gtk_button_get_always_show_image(GTK_BUTTON(self.unwrap_widget()))) }
    }
}
