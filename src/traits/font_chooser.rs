// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{from_glib_none, ToGlibPtr};
use cast::{GTK_FONT_CHOOSER};
use ffi;
use glib::{to_bool, to_gboolean};
use FFIWidget;

pub trait FontChooserTrait: ::WidgetTrait {
    fn get_font_size(&self) -> i32 {
        unsafe { ffi::gtk_font_chooser_get_font_size(GTK_FONT_CHOOSER(self.unwrap_widget())) }
    }

    fn get_font(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_font_chooser_get_font(GTK_FONT_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_font(&self, font_name: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(
                GTK_FONT_CHOOSER(self.unwrap_widget()),
                font_name.to_glib_none().0)
        }
    }

    fn get_preview_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_font_chooser_get_preview_text(GTK_FONT_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(GTK_FONT_CHOOSER(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_chooser_get_show_preview_entry(GTK_FONT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe { ffi::gtk_font_chooser_set_show_preview_entry(GTK_FONT_CHOOSER(self.unwrap_widget()),
                                                              to_gboolean(show_preview_entry));
        }
    }
}
