// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use adjustment::Adjustment;
use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// Infinite scrollable area containing child widgets and/or custom drawing.
pub type Layout = Object<ffi::GtkLayout>;

unsafe impl Upcast<Widget> for Layout { }
unsafe impl Upcast<::Container> for Layout { }
unsafe impl Upcast<::Scrollable> for Layout { }
unsafe impl Upcast<::Buildable> for Layout { }

impl Layout {
    pub fn new(hadjustment: &Adjustment, vadjustment: &Adjustment) -> Layout {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_layout_new(hadjustment.to_glib_none().0, vadjustment.to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn put<T: Upcast<Widget>>(&self, child: &T, x: i32, y: i32) {
        unsafe { ffi::gtk_layout_put(self.to_glib_none().0, child.upcast().to_glib_none().0, x, y) }
    }

    pub fn move_<T: Upcast<Widget>>(&self, child: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(self.to_glib_none().0, child.upcast().to_glib_none().0, x, y)
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe { ffi::gtk_layout_set_size(self.to_glib_none().0, width, height) }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        unsafe { ffi::gtk_layout_get_size(self.to_glib_none().0, &mut width, &mut height); }
        (width, height)
    }
}

impl types::StaticType for Layout {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_layout_get_type()) }
    }
}
