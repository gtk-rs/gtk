// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_double;
use glib::translate::{from_glib_none, ToGlibPtr};

use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_PROGRESSBAR;

struct_Widget!(ProgressBar);

impl ProgressBar {
    pub fn new() -> Option<ProgressBar> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_progress_bar_new() };
        check_pointer!(tmp_pointer, ProgressBar)
    }

    pub fn pulse(&self) -> () {
        unsafe {
            ffi::gtk_progress_bar_pulse(GTK_PROGRESSBAR(self.pointer))
        }
    }

    pub fn set_fraction(&self, fraction: f64) -> () {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(GTK_PROGRESSBAR(self.pointer), fraction as c_double)
        }
    }

    pub fn get_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_fraction(GTK_PROGRESSBAR(self.pointer)) as f64
        }
    }

    pub fn set_text(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_progress_bar_set_text(GTK_PROGRESSBAR(self.pointer), text.to_glib_none().0);
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_progress_bar_get_text(GTK_PROGRESSBAR(self.pointer)))
        }
    }

    pub fn set_inverted(&self, inverted: bool) -> () {
        unsafe { ffi::gtk_progress_bar_set_inverted(GTK_PROGRESSBAR(self.pointer), to_gboolean(inverted)); }
    }

    pub fn get_inverted(&self) -> bool {
        unsafe { to_bool(ffi::gtk_progress_bar_get_inverted(GTK_PROGRESSBAR(self.pointer))) }
    }

    pub fn set_show_text(&self, show_text: bool) -> () {
        unsafe { ffi::gtk_progress_bar_set_show_text(GTK_PROGRESSBAR(self.pointer), to_gboolean(show_text)); }
    }

    pub fn get_show_text(&self) -> bool {
        unsafe { to_bool(ffi::gtk_progress_bar_get_show_text(GTK_PROGRESSBAR(self.pointer))) }
    }

    pub fn set_pulse_step(&self, pulse_step: f64) -> () {
        unsafe {
            ffi::gtk_progress_bar_set_pulse_step(GTK_PROGRESSBAR(self.pointer), pulse_step as c_double)
        }
    }

    pub fn get_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_pulse_step(GTK_PROGRESSBAR(self.pointer)) as f64
        }
    }
}

impl_drop!(ProgressBar);
impl_TraitWidget!(ProgressBar);

impl ::OrientableTrait for ProgressBar {}
