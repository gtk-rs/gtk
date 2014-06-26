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

//! Hide and show with animation

use gtk;
use gtk::cast::{GTK_REVEALER};
use gtk::ffi;
use gtk::traits;

/// GtkRevealer — Hide and show with animation
struct_Widget!(Revealer)

impl Revealer {
    pub fn new() -> Option<Revealer> {
        let tmp_pointer = unsafe { ffi::gtk_revealer_new() };
        check_pointer!(tmp_pointer, Revealer)
    }

    pub fn get_reveal_child(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_revealer_get_reveal_child(GTK_REVEALER(self.pointer)))
        }
    }

    pub fn set_reveal_child(&mut self, reveal_child: bool) {
        unsafe {
            ffi::gtk_revealer_set_reveal_child(GTK_REVEALER(self.pointer),
                                               ffi::to_gboolean(reveal_child))
        }
    }

    pub fn is_child_revealed(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_revealer_get_child_revealed(GTK_REVEALER(self.pointer)))
        }
    }

    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_revealer_get_transition_duration(GTK_REVEALER(self.pointer))
        }
    }

    pub fn set_transition_duration(&mut self, duration: u32) {
        unsafe {
            ffi::gtk_revealer_set_transition_duration(GTK_REVEALER(self.pointer), duration)
        }
    }

    pub fn set_transition_type(&mut self, transition: gtk::RevealerTransitionType) {
        unsafe {
            ffi::gtk_revealer_set_transition_type(GTK_REVEALER(self.pointer), transition)
        }
    }

    pub fn get_transition_type(&self) -> gtk::RevealerTransitionType {
        unsafe {
            ffi::gtk_revealer_get_transition_type(GTK_REVEALER(self.pointer))
        }
    }
}

impl_drop!(Revealer)
impl_TraitWidget!(Revealer)

impl traits::Container for Revealer {}
impl traits::_Box for Revealer {}
