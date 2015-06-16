// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast};
use super::widget::Widget;

pub type Misc = Object<ffi::GtkMisc>;

impl types::StaticType for Misc {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_misc_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Misc { }
unsafe impl Upcast<::builder::Buildable> for Misc { }

pub trait MiscExt {
    fn set_alignment(&self, x_align: f32, y_align: f32);
    fn set_padding(&self, x_pad: i32, y_pad: i32);
    fn get_alignment(&self) -> (f32, f32);
    fn get_padding(&self) -> (i32, i32);
}

impl<O: Upcast<Misc>> MiscExt for O {
    fn set_alignment(&self, x_align: f32, y_align: f32) {
        unsafe { ffi::gtk_misc_set_alignment(self.upcast().to_glib_none().0, x_align, y_align); }
    }

    fn set_padding(&self, x_pad: i32, y_pad: i32) {
        unsafe { ffi::gtk_misc_set_padding(self.upcast().to_glib_none().0, x_pad, y_pad); }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let x = 0.;
        let y = 0.;
        unsafe { ffi::gtk_misc_get_alignment(self.upcast().to_glib_none().0, &x, &y); }
        (x, y)
    }

    fn get_padding(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe { ffi::gtk_misc_get_padding(self.upcast().to_glib_none().0, &x, &y); }
        (x, y)
    }
}
