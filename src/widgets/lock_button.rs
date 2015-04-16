// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container for arranging buttons

use cast::GTK_LOCKBUTTON;
use ffi;
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

impl ::ContainerTrait for LockButton {}
impl ::ButtonTrait for LockButton {}
impl ::ActionableTrait for LockButton {}

impl_widget_events!(LockButton);
impl_button_events!(LockButton);
