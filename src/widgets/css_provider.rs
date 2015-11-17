// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fmt::{self, Display, Formatter};
use ffi::{self, GtkCssProvider};
use glib::translate::{ToGlibPtr, from_glib_full};
use glib::{self, GlibContainer};

#[repr(C)]
pub struct CssProvider {
    pointer: *mut GtkCssProvider
}

impl ::StyleProviderTrait for CssProvider {}

impl CssProvider {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { CssProvider { pointer: ffi::gtk_css_provider_new() } }
    }

    pub fn get_default() -> Self {
        assert_initialized_main_thread!();
        unsafe { CssProvider { pointer: ffi::gtk_css_provider_get_default() } }
    }

    pub fn get_named(name: &str, variant: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CssProvider { pointer: ffi::gtk_css_provider_get_named(name.to_glib_none().0,
                                                                   variant.to_glib_none().0) }
        }
    }

    pub fn load_from_path(path: &str) -> Result<CssProvider, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let pointer = ffi::gtk_css_provider_new();
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_css_provider_load_from_path(pointer, path.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(CssProvider { pointer: pointer })
            } else {
                Err(glib::Error::wrap(error))
            }

        }
    }
}

impl Display for CssProvider {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tmp: String = unsafe { from_glib_full(ffi::gtk_css_provider_to_string(self.pointer)) };
        write!(f, "{}", tmp)
    }
}

impl_GObjectFunctions!(CssProvider, GtkCssProvider);
impl_TraitObject!(CssProvider, GtkCssProvider);
