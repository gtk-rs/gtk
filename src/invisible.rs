// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::IsA;
use gtk_sys;
use Invisible;

// For some reasons, it's not generated...
pub trait InvisibleExtManual: 'static {
    fn get_screen(&self) -> Option<gdk::Screen>;
}

impl<T: IsA<Invisible>> InvisibleExtManual for T {
    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(gtk_sys::gtk_invisible_get_screen(self.as_ref().to_glib_none().0))
        }
    }
}
