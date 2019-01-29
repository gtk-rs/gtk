// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Switch;
use ffi;
use glib::object::{IsA, Cast};
use glib::signal::{SignalHandlerId, connect_raw};
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait SwitchExtManual: 'static {
    fn connect_changed_active<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Switch>> SwitchExtManual for O {
    fn connect_changed_active<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::active\0".as_ptr() as *mut _,
                Some(transmute(changed_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_active_trampoline<T, F: Fn(&T) + 'static>(this: *mut ffi::GtkSwitch, _gparamspec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where T: IsA<Switch> {
    let f: &F = transmute(f);
    f(&Switch::from_glib_borrow(this).unsafe_cast())
}
