// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib_sys;
use gtk_sys;
use Fixed;
use IsA;
use Widget;

// All this is in order to avoid the segfault. More info in :
// https://github.com/gtk-rs/gtk/issues/565
fn has_widget<O: IsA<Fixed>, T: IsA<Widget>>(c: &O, item: &T) -> bool {
    skip_assert_initialized!();
    unsafe {
        let glist = gtk_sys::gtk_container_get_children(c.to_glib_none().0 as *mut _);
        let found = !glib_sys::g_list_find(glist, item.to_glib_none().0 as _).is_null();
        glib_sys::g_list_free(glist);
        found
    }
}

pub trait FixedExtManual: 'static {
    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32);

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32);
}

impl<O: IsA<Fixed>> FixedExtManual for O {
    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32 {
        assert!(
            has_widget(self, item),
            "this item isn't in the Fixed's widget list"
        );
        let mut value = glib::Value::from(&0);
        unsafe {
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut _,
                item.as_ref().to_glib_none().0,
                "x".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
        value
            .get_some()
            .expect("Return Value for `FixedExtManual::get_child_x`")
    }

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32) {
        assert!(
            has_widget(self, item),
            "this item isn't in the Fixed's widget list"
        );
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut _,
                item.as_ref().to_glib_none().0,
                "x".to_glib_none().0,
                glib::Value::from(&x).to_glib_none().0,
            );
        }
    }

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32 {
        assert!(
            has_widget(self, item),
            "this item isn't in the Fixed's widget list"
        );
        let mut value = glib::Value::from(&0);
        unsafe {
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut _,
                item.as_ref().to_glib_none().0,
                "y".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
        value
            .get_some()
            .expect("Return Value for `FixedExtManual::get_child_y`")
    }

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32) {
        assert!(
            has_widget(self, item),
            "this item isn't in the Fixed's widget list"
        );
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut _,
                item.as_ref().to_glib_none().0,
                "y".to_glib_none().0,
                glib::Value::from(&y).to_glib_none().0,
            );
        }
    }
}
