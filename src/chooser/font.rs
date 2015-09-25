// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast};

pub type FontChooser = Object<ffi::GtkFontChooser>;

impl types::StaticType for FontChooser {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_font_chooser_get_type()) }
    }
}

pub trait FontChooserExt {
    fn get_font_size(&self) -> i32;
    fn get_font(&self) -> Option<String>;
    fn set_font(&self, font_name: &str);
    fn get_preview_text(&self) -> Option<String>;
    fn set_preview_text(&self, text: &str);
    fn get_show_preview_entry(&self) -> bool;
    fn set_show_preview_entry(&self, show_preview_entry: bool);
}

impl<O: Upcast<FontChooser>> FontChooserExt for O {
    fn get_font_size(&self) -> i32 {
        unsafe { ffi::gtk_font_chooser_get_font_size(self.upcast().to_glib_none().0) }
    }

    fn get_font(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_font_chooser_get_font(self.upcast().to_glib_none().0))
        }
    }

    fn set_font(&self, font_name: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(
                self.upcast().to_glib_none().0,
                font_name.to_glib_none().0)
        }
    }

    fn get_preview_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_font_chooser_get_preview_text(self.upcast().to_glib_none().0))
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(self.upcast().to_glib_none().0, text.to_glib_none().0)
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_show_preview_entry(self.upcast().to_glib_none().0))
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe { ffi::gtk_font_chooser_set_show_preview_entry(self.upcast().to_glib_none().0,
                                                              show_preview_entry.to_glib());
        }
    }
}
