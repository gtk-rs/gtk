// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib_ffi;
use glib::IsA;
use glib::object::Downcast;
use glib::translate::*;

use libc::c_char;
use std::boxed::Box as Box_;
use std::mem::transmute;

use EntryCompletion;
use TreeIter;

pub trait EntryCompletionExtManual {
    fn set_match_func<F: Fn(&Self, &str, &TreeIter) -> bool + 'static>(&self, f: F);
}

impl<O: IsA<EntryCompletion>> EntryCompletionExtManual for O {
    fn set_match_func<F: Fn(&Self, &str, &TreeIter) -> bool + 'static>(&self, f: F) {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &TreeIter) -> bool + 'static>> = Box_::new(Box_::new(f));
            let callback = transmute(set_match_func_trampoline::<Self> as usize);
            ffi::gtk_entry_completion_set_match_func(self.to_glib_none().0,
                                                     callback,
                                                     Box_::into_raw(f) as *mut _,
                                                     None);
        }
    }
}

unsafe extern "C" fn set_match_func_trampoline<P>(this: *mut ffi::GtkEntryCompletion,
                                                  key: *const c_char,
                                                  iter: *mut ffi::GtkTreeIter,
                                                  f: glib_ffi::gpointer)
                                                  -> glib_ffi::gboolean
where P: IsA<EntryCompletion> {
    let f: &&(Fn(&P, &str, &TreeIter) -> bool + 'static) = transmute(f);
    f(&EntryCompletion::from_glib_borrow(this).downcast_unchecked(),
      &String::from_glib_none(key),
      &TreeIter::from_glib_borrow(iter)).to_glib()
}
