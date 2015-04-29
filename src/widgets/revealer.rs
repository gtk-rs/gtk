// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Hide and show with animation

use ffi;
use cast::{GTK_REVEALER};
use glib::{to_bool, to_gboolean};

/// GtkRevealer — Hide and show with animation
struct_Widget!(Revealer);

impl Revealer {
    pub fn new() -> Option<Revealer> {
        let tmp_pointer = unsafe { ffi::gtk_revealer_new() };
        check_pointer!(tmp_pointer, Revealer)
    }

    pub fn get_reveal_child(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_revealer_get_reveal_child(GTK_REVEALER(self.pointer)))
        }
    }

    pub fn set_reveal_child(&self, reveal_child: bool) {
        unsafe {
            ffi::gtk_revealer_set_reveal_child(GTK_REVEALER(self.pointer),
                                               to_gboolean(reveal_child))
        }
    }

    pub fn is_child_revealed(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_revealer_get_child_revealed(GTK_REVEALER(self.pointer)))
        }
    }

    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_revealer_get_transition_duration(GTK_REVEALER(self.pointer))
        }
    }

    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_revealer_set_transition_duration(GTK_REVEALER(self.pointer), duration)
        }
    }

    pub fn set_transition_type(&self, transition: ::RevealerTransitionType) {
        unsafe {
            ffi::gtk_revealer_set_transition_type(GTK_REVEALER(self.pointer), transition)
        }
    }

    pub fn get_transition_type(&self) -> ::RevealerTransitionType {
        unsafe {
            ffi::gtk_revealer_get_transition_type(GTK_REVEALER(self.pointer))
        }
    }
}

impl_drop!(Revealer);
impl_TraitWidget!(Revealer);

impl ::ContainerTrait for Revealer {}
impl ::BinTrait for Revealer {}
