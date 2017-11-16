// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use Buildable;
use IsA;

pub trait BuildableExtManual {
    fn get_name(&self) -> Option<String>;

    fn set_name(&self, name: &str);
}

impl<O: IsA<Buildable>> BuildableExtManual for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_buildable_get_name(self.to_glib_none().0))
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_buildable_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}
