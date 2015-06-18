// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// A container which allows you to position widgets at fixed coordinates.
pub type Fixed = Object<ffi::GtkFixed>;

unsafe impl Upcast<Widget> for Fixed { }
unsafe impl Upcast<super::container::Container> for Fixed { }
unsafe impl Upcast<::builder::Buildable> for Fixed { }

impl Fixed {
    pub fn new() -> Fixed {
        unsafe { Widget::from_glib_none(ffi::gtk_fixed_new()).downcast_unchecked() }
    }

    pub fn put<T: Upcast<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_put(self.to_glib_none().0, widget.upcast().to_glib_none().0, x, y);
        }
    }

    pub fn move_<T: Upcast<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_move(self.to_glib_none().0, widget.upcast().to_glib_none().0, x, y);
        }
    }
}

impl types::StaticType for Fixed {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_fixed_get_type()) }
    }
}
