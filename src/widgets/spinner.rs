// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Show a spinner animation

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// Show a spinner animation.
pub type Spinner = Object<ffi::GtkSpinner>;

impl Spinner {
    pub fn new() -> Spinner {
        unsafe { Widget::from_glib_none(ffi::gtk_spinner_new()).downcast_unchecked() }
    }

    pub fn start(&self) {
        unsafe { ffi::gtk_spinner_start(self.to_glib_none().0) }
    }

    pub fn stop(&self) {
        unsafe { ffi::gtk_spinner_stop(self.to_glib_none().0) }
    }

}

impl types::StaticType for Spinner {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_spinner_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Spinner { }
unsafe impl Upcast<::Buildable> for Spinner { }
