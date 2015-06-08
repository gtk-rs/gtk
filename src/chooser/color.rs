// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_COLOR_CHOOSER;
use gdk_ffi;

pub trait ColorChooserTrait: ::WidgetTrait {
    fn get_rgba(&self) -> gdk_ffi::GdkRGBA {
        let color = gdk_ffi::GdkRGBA {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
            alpha: 0f64
        };
        unsafe { ffi::gtk_color_chooser_get_rgba(GTK_COLOR_CHOOSER(self.unwrap_widget()), &color) };
        color
    }

    fn set_rgba(&self, color: gdk_ffi::GdkRGBA) -> () {
        unsafe { ffi::gtk_color_chooser_set_rgba(GTK_COLOR_CHOOSER(self.unwrap_widget()), &color) };
    }

    fn get_use_alpha(&self) -> bool {
        unsafe { to_bool(ffi::gtk_color_chooser_get_use_alpha(GTK_COLOR_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_use_alpha(&self, use_alpha: bool) -> () {
        unsafe { ffi::gtk_color_chooser_set_use_alpha(GTK_COLOR_CHOOSER(self.unwrap_widget()), to_gboolean(use_alpha)) }
    }

    fn add_palette(&self, orientation: ::Orientation, colors_per_line: i32, colors: Vec<gdk_ffi::GdkRGBA>) -> () {
        unsafe { ffi::gtk_color_chooser_add_palette(GTK_COLOR_CHOOSER(self.unwrap_widget()), orientation, colors_per_line, colors.len() as i32, colors.as_ptr()) }
    }
}
