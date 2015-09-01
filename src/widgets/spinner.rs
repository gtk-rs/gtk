// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Show a spinner animation

use cast::GTK_SPINNER;
use ffi;

/// Spinner — Show a spinner animation
struct_Widget!(Spinner);

impl Spinner {
    pub fn new() -> Option<Spinner> {
        let tmp_pointer = unsafe { ffi::gtk_spinner_new() };
        check_pointer!(tmp_pointer, Spinner)
    }

    pub fn start(&self) -> () {
        unsafe {
            ffi::gtk_spinner_start(GTK_SPINNER(self.pointer))
        }
    }

    pub fn stop(&self) -> () {
        unsafe {
            ffi::gtk_spinner_stop(GTK_SPINNER(self.pointer))
        }
    }

}

impl_drop!(Spinner);
impl_TraitWidget!(Spinner);
