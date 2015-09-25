// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container for arranging buttons

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;
use glib::Permission;
use glib::GlibContainer;

/// GtkLockButton — A widget to unlock or lock privileged operations
pub type LockButton = Object<ffi::GtkLockButton>;

unsafe impl Upcast<Widget> for LockButton { }
unsafe impl Upcast<::Container> for LockButton { }
unsafe impl Upcast<::Bin> for LockButton { }
unsafe impl Upcast<::Button> for LockButton { }

unsafe impl Upcast<::Actionable> for LockButton { }
unsafe impl Upcast<::Buildable> for LockButton { }

impl LockButton {
    /// Creates a new lock button which reflects the `permission`.
    pub fn new(permission: &Permission) -> LockButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_lock_button_new(permission.unwrap()))
                .downcast_unchecked()
        }
    }
    /// Obtains the `GPermission` object that controls this button.
    pub fn get_permission(&self) -> Option<Permission> {
        let tmp_pointer = unsafe {
            ffi::gtk_lock_button_get_permission(self.to_glib_none().0)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(GlibContainer::wrap(tmp_pointer))
        }
    }
    /// Sets the `GPermission` object that controls this button.
    pub fn set_permission(&self, permission: &Permission) {
        unsafe {
            ffi::gtk_lock_button_set_permission(self.to_glib_none().0,
                permission.unwrap())
        }
    }
}

impl types::StaticType for LockButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_lock_button_get_type()) }
    }
}
