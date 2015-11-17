// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi::{self, GtkStyleContext};
use glib::translate::*;
use glib::traits::FFIGObject;
use gdk;

pub struct StyleContext {
    // FIXME: This doesn't need to be public after object reform lands
    pub pointer: *mut GtkStyleContext
}

impl StyleContext {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { StyleContext { pointer: ffi::gtk_style_context_new() } }
    }

    pub fn add_provider<T: ::StyleProviderTrait>(&self, provider: &T, priority: u32) {
        unsafe {
            ffi::gtk_style_context_add_provider(
                self.pointer,
                ffi::cast_GtkStyleProvider(provider.unwrap_gobject()),
                priority)
        };
    }

    pub fn add_provider_for_screen<T: ::StyleProviderTrait>(screen: &gdk::Screen, provider: &T,
                                                            priority: u32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_style_context_add_provider_for_screen(
                screen.to_glib_none().0,
                ffi::cast_GtkStyleProvider(provider.unwrap_gobject()),
                priority)
        };
    }
}

impl_GObjectFunctions!(StyleContext, GtkStyleContext);
impl_TraitObject!(StyleContext, GtkStyleContext);
