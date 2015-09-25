// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast};
use Orientation;

pub type Orientable = Object<ffi::GtkOrientable>;

impl types::StaticType for Orientable {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_orientable_get_type()) }
    }
}

pub trait OrientableExt {
    fn get_orientation(&self) -> Orientation;
    fn set_orientation(&self, orientation: Orientation);
}

impl<O: Upcast<Orientable>> OrientableExt for O {
    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_orientable_get_orientation(self.upcast().to_glib_none().0)
        }
    }

    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_orientable_set_orientation(self.upcast().to_glib_none().0, orientation)
        }
    }
}
