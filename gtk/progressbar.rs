// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.

//! A widget which indicates progress visually

use std::libc::{c_void, c_double};
use std::{ptr, str, cast};

use traits::{GtkWidget, GtkOrientable, Signal};
use ffi;
use utils::cast::GTK_PROGRESSBAR;


/// ProgressBar — A widget which indicates progress visually
pub struct ProgressBar {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl ProgressBar {
    pub fn new() -> Option<ProgressBar> {
        let tmp_pointer = unsafe { ffi::gtk_progress_bar_new() };
        check_pointer!(tmp_pointer, ProgressBar)
    }

    pub fn pulse(&mut self) -> () {
        unsafe {
            ffi::gtk_progress_bar_pulse(GTK_PROGRESSBAR(self.pointer))
        }
    }

    pub fn set_fraction(&mut self, fraction: f64) -> () {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(GTK_PROGRESSBAR(self.pointer), fraction as c_double)
        }
    }

    pub fn get_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_fraction(GTK_PROGRESSBAR(self.pointer)) as f64
        }
    }

    pub fn set_text(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_progress_bar_set_text(GTK_PROGRESSBAR(self.pointer), c_str) 
            });
        }
    }

    pub fn get_text(&self) -> ~str {
        unsafe {
            let c_str = ffi::gtk_progress_bar_get_text(GTK_PROGRESSBAR(self.pointer));
            str::raw::from_c_str(c_str)
        }
    }

    pub fn set_inverted(&mut self, inverted: bool) -> () {
        match inverted {
            true    => unsafe { ffi::gtk_progress_bar_set_inverted(GTK_PROGRESSBAR(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_progress_bar_set_inverted(GTK_PROGRESSBAR(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_inverted(&self) -> bool {
        match unsafe { ffi::gtk_progress_bar_get_inverted(GTK_PROGRESSBAR(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_show_text(&mut self, show_text: bool) -> () {
        match show_text {
            true    => unsafe { ffi::gtk_progress_bar_set_show_text(GTK_PROGRESSBAR(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_progress_bar_set_show_text(GTK_PROGRESSBAR(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_show_text(&self) -> bool {
        match unsafe { ffi::gtk_progress_bar_get_show_text(GTK_PROGRESSBAR(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_pulse_step(&mut self, pulse_step: f64) -> () {
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

impl_GtkWidget!(ProgressBar)
redirect_callback!(ProgressBar)
redirect_callback_widget!(ProgressBar)
struct_signal!(ProgressBar)
impl_signals!(ProgressBar)

impl GtkOrientable for ProgressBar {}

