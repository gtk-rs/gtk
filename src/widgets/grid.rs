// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use PositionType;

/// Pack widgets in a rows and columns.
pub type Grid = Object<ffi::GtkGrid>;

unsafe impl Upcast<Widget> for Grid { }
unsafe impl Upcast<super::container::Container> for Grid { }
unsafe impl Upcast<super::orientable::Orientable> for Grid { }
unsafe impl Upcast<::builder::Buildable> for Grid { }

impl Grid {
    pub fn new() -> Grid {
        unsafe { Widget::from_glib_none(ffi::gtk_grid_new()).downcast_unchecked() }
    }

    pub fn attach<T: Upcast<Widget>>(&self, child: &T, left: i32, top: i32, width: i32,
                                     height: i32) {
        unsafe {
            ffi::gtk_grid_attach(self.to_glib_none().0, child.upcast().to_glib_none().0,
                left, top, width, height);
        }
    }

    pub fn attach_next_to<T: Upcast<Widget>>(&self, child: &T, sibling: &T, side: PositionType,
                                             width: i32, height: i32) {
        unsafe {
            ffi::gtk_grid_attach_next_to(self.to_glib_none().0, child.upcast().to_glib_none().0,
                sibling.upcast().to_glib_none().0, side, width, height);
        }
    }

    pub fn insert_row(&self, position: i32) {
        unsafe { ffi::gtk_grid_insert_row(self.to_glib_none().0, position); }
    }

     pub fn insert_column(&self, position: i32) {
        unsafe { ffi::gtk_grid_insert_column(self.to_glib_none().0, position); }
    }

    #[cfg(feature = "gtk_3_10")]
     pub fn remove_row(&self, position: i32) {
        unsafe { ffi::gtk_grid_remove_row(self.to_glib_none().0, position); }
    }

    #[cfg(feature = "gtk_3_10")]
     pub fn remove_column(&self, position: i32) {
        unsafe { ffi::gtk_grid_remove_column(self.to_glib_none().0, position); }
    }

    pub fn insert_next_to<T: Upcast<Widget>>(&self, sibling: &T, side: PositionType) {
        unsafe {
            ffi::gtk_grid_insert_next_to(self.to_glib_none().0, sibling.upcast().to_glib_none().0,
                side);
        }
    }

    pub fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe { ffi::gtk_grid_set_row_homogeneous(self.to_glib_none().0, homogeneous.to_glib()); }
    }

    pub fn get_row_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_grid_get_row_homogeneous(self.to_glib_none().0)) }
    }

    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe { ffi::gtk_grid_set_row_spacing(self.to_glib_none().0, spacing); }
    }

    pub fn get_row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_get_row_spacing(self.to_glib_none().0) }
    }

    pub fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_column_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    pub fn get_column_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_grid_get_column_homogeneous(self.to_glib_none().0)) }
    }

    pub fn set_column_spacing(&self, spacing: u32) {
        unsafe { ffi::gtk_grid_set_column_spacing(self.to_glib_none().0, spacing); }
    }

    pub fn get_column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_get_column_spacing(self.to_glib_none().0) }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn get_baseline_row(&self) -> i32 {
        unsafe { ffi::gtk_grid_get_baseline_row(self.to_glib_none().0) }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn set_baseline_row(&self, row: i32) {
        unsafe { ffi::gtk_grid_set_baseline_row(self.to_glib_none().0, row); }
    }
}

impl types::StaticType for Grid {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_grid_get_type()) }
    }
}
