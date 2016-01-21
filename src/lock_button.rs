// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

use glib::object::Downcast;
use Widget;
use glib::Permission;
use glib::GlibContainer;

glib_wrapper! {
    pub struct LockButton(Object<ffi::GtkLockButton>): Widget, ::Container, ::Bin, ::Button,
        ::Actionable, ::Buildable;

    match fn {
        get_type => || ffi::gtk_lock_button_get_type(),
    }
}

impl LockButton {
    pub fn new(permission: &Permission) -> LockButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_lock_button_new(permission.unwrap()))
                .downcast_unchecked()
        }
    }
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
    pub fn set_permission(&self, permission: &Permission) {
        unsafe {
            ffi::gtk_lock_button_set_permission(self.to_glib_none().0,
                permission.unwrap())
        }
    }
}
