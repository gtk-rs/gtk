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

use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::cast::GTK_COLOR_CHOOSER;
use gdk_ffi;

pub trait ColorChooserTrait: gtk::WidgetTrait {
    fn get_rgba(&self) -> gdk_ffi::C_GdkRGBA {
        let color = gdk_ffi::C_GdkRGBA {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
            alpha: 0f64
        };
        unsafe { ffi::gtk_color_chooser_get_rgba(GTK_COLOR_CHOOSER(self.unwrap_widget()), &color) };
        color
    }

    fn set_rgba(&self, color: gdk_ffi::C_GdkRGBA) -> () {
        unsafe { ffi::gtk_color_chooser_set_rgba(GTK_COLOR_CHOOSER(self.unwrap_widget()), &color) };
    }

    fn get_use_alpha(&self) -> bool {
        unsafe { to_bool(ffi::gtk_color_chooser_get_use_alpha(GTK_COLOR_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_use_alpha(&self, use_alpha: bool) -> () {
        unsafe { ffi::gtk_color_chooser_set_use_alpha(GTK_COLOR_CHOOSER(self.unwrap_widget()), to_gboolean(use_alpha)) }
    }

    fn add_palette(&self, orientation: gtk::Orientation, colors_per_line: i32, colors: Vec<gdk_ffi::C_GdkRGBA>) -> () {
        unsafe { ffi::gtk_color_chooser_add_palette(GTK_COLOR_CHOOSER(self.unwrap_widget()), orientation, colors_per_line, colors.len() as i32, colors.as_slice().as_ptr()) }
    }
}
