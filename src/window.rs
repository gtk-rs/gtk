// Copyright 2015-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib;
use glib::object::IsA;
use Window;

pub trait GtkWindowExtManual {
    fn present(&self);
}

#[cfg(target_os = "macos")]
extern "C" {
    fn macos_force_foreground_level();
}

impl<O: IsA<Window> + IsA<glib::object::Object> + glib::object::ObjectExt> GtkWindowExtManual for O {
    fn present(&self) {
        unsafe {
            ffi::gtk_window_present(self.to_glib_none().0);
        }
        // This is a super wonderful hack to actually make this function work as expected.
        #[cfg(target_os = "macos")]
        unsafe { macos_force_foreground_level(); }
    }
}
