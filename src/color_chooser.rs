// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_int;

use glib::translate::*;
use ffi;

use glib::object::IsA;
use gdk::RGBA;
use gdk_ffi;
use Orientation;

glib_wrapper! {
    pub struct ColorChooser(Object<ffi::GtkColorChooser>);

    match fn {
        get_type => || ffi::gtk_color_chooser_get_type(),
    }
}

pub trait ColorChooserExt {
    fn get_rgba(&self) -> RGBA;
    fn set_rgba(&self, color: &RGBA);
    fn get_use_alpha(&self) -> bool;
    fn set_use_alpha(&self, use_alpha: bool);
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]);
}

impl<O: IsA<ColorChooser>> ColorChooserExt for O {
    fn get_rgba(&self) -> RGBA {
        unsafe {
            let mut color = RGBA::uninitialized();
            ffi::gtk_color_chooser_get_rgba(self.to_glib_none().0, color.to_glib_none_mut().0);
            color
        }
    }

    fn set_rgba(&self, color: &RGBA) {
        unsafe { ffi::gtk_color_chooser_set_rgba(self.to_glib_none().0, color.to_glib_none().0) };
    }

    fn get_use_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gtk_color_chooser_get_use_alpha(self.to_glib_none().0)) }
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_chooser_set_use_alpha(self.to_glib_none().0,
                use_alpha.to_glib())
        }
    }

    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        unsafe {
            ffi::gtk_color_chooser_add_palette(self.to_glib_none().0,
                                               orientation.to_glib(),
                                               colors_per_line,
                                               colors.len() as c_int,
                                               colors.as_ptr() as *mut gdk_ffi::GdkRGBA)
        }
    }
}
