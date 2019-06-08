// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::Cast;
use glib::translate::*;
use glib_sys;
use libc::c_int;
use std::boxed::Box as Box_;
use std::ptr;
use gtk_sys;
use IsA;
use Menu;
use Widget;

pub trait GtkMenuExtManual: 'static {
    fn popup<T: IsA<Widget>, U: IsA<Widget>,
                 F: Fn(&Self, &mut i32, &mut i32) -> bool + 'static>(
        &self, parent_menu_shell: Option<&T>, parent_menu_item: Option<&U>, f: F,
        button: u32, activate_time: u32);

    fn popup_easy(&self, button: u32, activate_time: u32);
}

impl<O: IsA<Menu>> GtkMenuExtManual for O {
    fn popup<T: IsA<Widget>, U: IsA<Widget>,
                 F: FnOnce(&Self, &mut i32, &mut i32) -> bool + 'static>(
        &self,
        parent_menu_shell: Option<&T>,
        parent_menu_item: Option<&U>,
        f: F,
        button: u32,
        activate_time: u32,
    ) {
        unsafe extern "C" fn position_callback<T, F: FnOnce(&T, &mut i32, &mut i32) -> bool + 'static>(
            this: *mut gtk_sys::GtkMenu,
            x: *mut c_int,
            y: *mut c_int,
            push_in: *mut glib_sys::gboolean,
            f: glib_sys::gpointer,
        )
            where T: IsA<Menu>
        {
            let mut f: Box<Option<F>> = Box::from_raw(f as *mut _);
            let f = f.take().expect("No callback");
            *push_in = f(&Menu::from_glib_none(this).unsafe_cast(), x.as_mut().unwrap(),
                         y.as_mut().unwrap()).to_glib();
        }
        unsafe {
            let f: Box_<Option<F>> = Box_::new(Some(f));
            gtk_sys::gtk_menu_popup(self.as_ref().to_glib_none().0, parent_menu_shell.map(|p| p.as_ref()).to_glib_none().0,
                                parent_menu_item.map(|p| p.as_ref()).to_glib_none().0,
                                Some(position_callback::<Self, F>),
                                Box_::into_raw(f) as *mut _, button, activate_time)
        }
    }

    fn popup_easy(&self, button: u32, activate_time: u32) {
        unsafe {
            gtk_sys::gtk_menu_popup(self.as_ref().to_glib_none().0, ptr::null_mut(),
                                ptr::null_mut(), None, ptr::null_mut(),
                                button, activate_time)
        }
    }
}
