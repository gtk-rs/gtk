// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib_ffi;
use glib::object::Downcast;
use glib::translate::*;
use Menu;
use IsA;
use Widget;
use std::boxed::Box as Box_;
use std::mem::transmute;
use libc::c_int;
use std::ptr;

impl Menu {
    pub fn popup<T: IsA<Widget>, U: IsA<Widget>,
                 F: Fn(&Menu, &mut i32, &mut i32) -> bool + 'static>(
        &self, parent_menu_shell: Option<&T>, parent_menu_item: Option<&U>, f: F,
        button: u32, activate_time: u32) {
        unsafe {
            let f: Box_<Box_<Fn(&Menu, &mut i32, &mut i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            ffi::gtk_menu_popup(self.to_glib_none().0, parent_menu_shell.to_glib_none().0,
                                parent_menu_item.to_glib_none().0,
                                Some(position_callback),
                                Box_::into_raw(f) as *mut _, button, activate_time)
        }
    }

    pub fn popup_easy(&self, button: u32, activate_time: u32) {
        unsafe {
            ffi::gtk_menu_popup(self.to_glib_none().0, ptr::null_mut(),
                                ptr::null_mut(), None, ptr::null_mut(),
                                button, activate_time)
        }
    }
}

unsafe extern "C" fn position_callback(this: *mut ffi::GtkMenu, x: *mut c_int, y: *mut c_int,
                                       push_in: *mut glib_ffi::gboolean, f: glib_ffi::gpointer) {
    let f: &Box_<Fn(&Menu, &mut i32, &mut i32) -> bool + 'static> = transmute(f);
    *push_in = f(&Menu::from_glib_none(this).downcast_unchecked(), x.as_mut().unwrap(),
                 y.as_mut().unwrap()).to_glib();
}
