// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget which indicates progress visually.

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// A widget which indicates progress visually.
pub type ProgressBar = Object<ffi::GtkProgressBar>;

impl ProgressBar {
    pub fn new() -> ProgressBar {
        unsafe { Widget::from_glib_none(ffi::gtk_progress_bar_new()).downcast_unchecked() }
    }

    pub fn pulse(&self) {
        unsafe { ffi::gtk_progress_bar_pulse(self.to_glib_none().0) }
    }

    pub fn set_fraction(&self, fraction: f64) {
        unsafe { ffi::gtk_progress_bar_set_fraction(self.to_glib_none().0, fraction) }
    }

    pub fn get_fraction(&self) -> f64 {
        unsafe { ffi::gtk_progress_bar_get_fraction(self.to_glib_none().0) }
    }

    pub fn set_text(&self, text: &str) {
        unsafe { ffi::gtk_progress_bar_set_text(self.to_glib_none().0, text.to_glib_none().0); }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_progress_bar_get_text(self.to_glib_none().0)) }
    }

    pub fn set_inverted(&self, inverted: bool) {
        unsafe { ffi::gtk_progress_bar_set_inverted(self.to_glib_none().0, inverted.to_glib()); }
    }

    pub fn get_inverted(&self) -> bool {
        unsafe { from_glib(ffi::gtk_progress_bar_get_inverted(self.to_glib_none().0)) }
    }

    pub fn set_show_text(&self, show_text: bool) {
        unsafe { ffi::gtk_progress_bar_set_show_text(self.to_glib_none().0, show_text.to_glib()); }
    }

    pub fn get_show_text(&self) -> bool {
        unsafe { from_glib(ffi::gtk_progress_bar_get_show_text(self.to_glib_none().0)) }
    }

    pub fn set_pulse_step(&self, pulse_step: f64) {
        unsafe { ffi::gtk_progress_bar_set_pulse_step(self.to_glib_none().0, pulse_step) }
    }

    pub fn get_pulse_step(&self) -> f64 {
        unsafe { ffi::gtk_progress_bar_get_pulse_step(self.to_glib_none().0) }
    }
}

impl types::StaticType for ProgressBar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_progress_bar_get_type()) }
    }
}

unsafe impl Upcast<Widget> for ProgressBar { }
unsafe impl Upcast<::Orientable> for ProgressBar { }
unsafe impl Upcast<::Buildable> for ProgressBar { }
