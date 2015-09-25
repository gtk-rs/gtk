// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use gdk;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use Orientation;

/// A widget with two adjustable panes.
pub type Paned = Object<ffi::GtkPaned>;

unsafe impl Upcast<Widget> for Paned { }
unsafe impl Upcast<::Container> for Paned { }
unsafe impl Upcast<::Orientable> for Paned { }
unsafe impl Upcast<::Buildable> for Paned { }

impl Paned {
    pub fn new(orientation: Orientation) -> Paned {
        unsafe { Widget::from_glib_none(ffi::gtk_paned_new(orientation)).downcast_unchecked() }
    }

    pub fn add1<T: Upcast<Widget>>(&self, child: &T) {
        unsafe { ffi::gtk_paned_add1(self.to_glib_none().0, child.upcast().to_glib_none().0) }
    }

    pub fn add2<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_paned_add2(self.to_glib_none().0, child.upcast().to_glib_none().0)
        }
    }

    pub fn pack1<T: Upcast<Widget>>(&self, child: &T, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack1(self.to_glib_none().0, child.upcast().to_glib_none().0,
                                 resize.to_glib(), shrink.to_glib());
        }
    }

    pub fn pack2<T: Upcast<Widget>>(&self, child: &T, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack2(self.to_glib_none().0, child.upcast().to_glib_none().0,
                                 resize.to_glib(), shrink.to_glib());
        }
    }

    pub fn set_position(&self, position: i32) {
        unsafe { ffi::gtk_paned_set_position(self.to_glib_none().0, position); }
    }

    pub fn get_position(&self) -> i32 {
        unsafe { ffi::gtk_paned_get_position(self.to_glib_none().0) }
    }

    pub fn get_handle_window(&self) -> gdk::Window {
        unsafe { from_glib_none(ffi::gtk_paned_get_handle_window(self.to_glib_none().0)) }
    }
}

impl types::StaticType for Paned {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_paned_get_type()) }
    }
}
