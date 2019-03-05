// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use Buildable;
use IsA;

pub trait BuildableExtManual: 'static {
    fn get_name(&self) -> Option<String>;

    fn set_name(&self, name: &str);
}

impl<O: IsA<Buildable>> BuildableExtManual for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(gtk_sys::gtk_buildable_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            gtk_sys::gtk_buildable_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}
