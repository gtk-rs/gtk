// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Switch;
use ffi;
use glib;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait SwitchExtManual {
    fn connect_changed_active<F: Fn(&Switch) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Switch> + IsA<glib::object::Object>> SwitchExtManual for O {
    fn connect_changed_active<F: Fn(&Switch) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Switch) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(changed_active_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_active_trampoline(this: *mut ffi::GtkSwitch, _gparamspec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Switch) + 'static) = transmute(f);
    f(&from_glib_none(this))
}
