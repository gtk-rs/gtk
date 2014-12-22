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

//! A container for arranging buttons

use gtk::cast::GTK_LOCKBUTTON;
use gtk::{mod, ffi};
use glib::Permission;
use glib::GlibContainer;

/// GtkLockButton — A widget to unlock or lock privileged operations
struct_Widget!(LockButton);

impl LockButton {
    pub fn new(permission: &Permission) -> Option<LockButton> {
        let tmp_pointer = unsafe { ffi::gtk_lock_button_new(permission.unwrap()) };
        check_pointer!(tmp_pointer, LockButton)
    }

    pub fn get_permission(&self) -> Option<Permission> {
        let tmp_pointer = unsafe { ffi::gtk_lock_button_get_permission(GTK_LOCKBUTTON(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(GlibContainer::wrap(tmp_pointer))
        }
    }

    pub fn set_permission(&self, permission: &Permission) {
        unsafe { ffi::gtk_lock_button_set_permission(GTK_LOCKBUTTON(self.pointer), permission.unwrap()) }
    }
}

impl_drop!(LockButton);
impl_TraitWidget!(LockButton);

impl gtk::ContainerTrait for LockButton {}
impl gtk::ButtonTrait for LockButton {}
impl gtk::ActionableTrait for LockButton {}

impl_widget_events!(LockButton);
impl_button_events!(LockButton);
