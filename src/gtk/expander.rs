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
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A container which can hide its child

use std;
use std::{ptr, str, owned};
use libc::{c_int};
use libc::{c_void};

use traits::{GtkContainer, GtkWidget, GtkBin, Signal};
use gtk;
use utils::cast::GTK_EXPANDER;
use ffi;

/// Expander — A container which can hide its child
pub struct Expander {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Expander {
    pub fn new(label: &str) -> Option<Expander> {
        let tmp_pointer = unsafe { 
            label.with_c_str(|c_str| {
                ffi::gtk_expander_new(c_str) 
            }) 
        };
        check_pointer!(tmp_pointer, Expander)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Expander> {
        let tmp_pointer = unsafe { 
            mnemonic.with_c_str(|c_str| {
                ffi::gtk_expander_new_with_mnemonic(c_str) 
            }) 
        };
        check_pointer!(tmp_pointer, Expander)
    }

   
    pub fn set_expanded(&mut self, expanded: bool) -> () {
        match expanded {
            true    => unsafe { ffi::gtk_expander_set_expanded(GTK_EXPANDER(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_expander_set_expanded(GTK_EXPANDER(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_expanded(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_expanded(GTK_EXPANDER(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_use_underline(&mut self, use_underline: bool) -> () {
        match use_underline {
            true    => unsafe { ffi::gtk_expander_set_use_underline(GTK_EXPANDER(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_expander_set_use_underline(GTK_EXPANDER(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_use_underline(GTK_EXPANDER(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_use_markup(&mut self, use_markup: bool) -> () {
        match use_markup {
            true    => unsafe { ffi::gtk_expander_set_use_markup(GTK_EXPANDER(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_expander_set_use_markup(GTK_EXPANDER(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_use_markup(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_use_markup(GTK_EXPANDER(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_label_fill(&mut self, label_fill: bool) -> () {
        match label_fill {
            true    => unsafe { ffi::gtk_expander_set_label_fill(GTK_EXPANDER(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_expander_set_label_fill(GTK_EXPANDER(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_label_fill(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_label_fill(GTK_EXPANDER(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn set_resize_toplevel(&mut self, resize_toplevel: bool) -> () {
        match resize_toplevel {
            true    => unsafe { ffi::gtk_expander_set_resize_toplevel(GTK_EXPANDER(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_expander_set_resize_toplevel(GTK_EXPANDER(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_resize_toplevel(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_resize_toplevel(GTK_EXPANDER(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn get_label(&self) -> String {
        unsafe {
            let c_str = ffi::gtk_expander_get_label(GTK_EXPANDER(self.pointer));
            str::raw::from_c_str(c_str)
        }
    }

    pub fn set_label(&mut self, label: &str) -> () {
        unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_expander_set_label(GTK_EXPANDER(self.pointer), c_str) 
            });
        }
    }

    pub fn set_spacing(&mut self, spacing: i32) -> () {
        unsafe {
            ffi::gtk_expander_set_spacing(GTK_EXPANDER(self.pointer), spacing as c_int)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_expander_get_spacing(GTK_EXPANDER(self.pointer)) as i32
        }
    }

    pub fn set_label_widget(&mut self, label: &gtk::Label) -> () {
        unsafe {
            ffi::gtk_expander_set_label_widget(GTK_EXPANDER(self.pointer), label.get_widget());
        }
    }

    pub fn get_label_widget(&mut self) -> gtk::Label {
        unsafe {
            GtkWidget::wrap_widget(ffi::gtk_expander_get_label_widget(GTK_EXPANDER(self.pointer)))
        }
    }

}

impl_GtkWidget!(Expander)
redirect_callback!(Expander)
redirect_callback_widget!(Expander)
struct_signal!(Expander)
impl_signals!(Expander)

impl GtkContainer for Expander {}
impl GtkBin for Expander {}

