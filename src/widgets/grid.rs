// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Pack widgets in a rows and columns

// FIXME: Missings methods

use libc::{c_int, c_uint};

use {PositionType};
use cast::GTK_GRID;
use ffi;
use glib::{to_bool, to_gboolean};

/// Grid — Pack widgets in a rows and columns
struct_Widget!(Grid);

impl Grid {
    pub fn new() -> Option<Grid> {
        let tmp_pointer = unsafe { ffi::gtk_grid_new() };
        check_pointer!(tmp_pointer, Grid)
    }

    pub fn attach<T: ::WidgetTrait>(&self,
                                child: &T,
                                left: i32,
                                top: i32,
                                width: i32,
                                height: i32) -> () {
        unsafe {
            ffi::gtk_grid_attach(GTK_GRID(self.pointer),
                                 child.unwrap_widget(),
                                 left as c_int,
                                 top as c_int,
                                 width as c_int,
                                 height as c_int);
        }
    }

    pub fn attach_next_to<T: ::WidgetTrait>(&self,
                                        child: &T,
                                        sibling: &T,
                                        side: PositionType,
                                        width: i32,
                                        height: i32) -> () {
        unsafe {
            ffi::gtk_grid_attach_next_to(GTK_GRID(self.pointer),
                                         child.unwrap_widget(),
                                         sibling.unwrap_widget(),
                                         side,
                                         width as c_int,
                                         height as c_int);
        }
    }

    pub fn insert_row(&self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_insert_row(GTK_GRID(self.pointer), position as c_int);
        }
    }

     pub fn insert_column(&self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_insert_column(GTK_GRID(self.pointer), position as c_int);
        }
    }

    #[cfg(feature = "gtk_3_10")]
     pub fn remove_row(&self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_remove_row(GTK_GRID(self.pointer), position as c_int);
        }
    }

    #[cfg(feature = "gtk_3_10")]
     pub fn remove_column(&self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_remove_column(GTK_GRID(self.pointer), position as c_int);
        }
    }

    pub fn insert_next_to<T: ::WidgetTrait>(&self, sibling: &T, side: PositionType) -> () {
        unsafe {
            ffi::gtk_grid_insert_next_to(GTK_GRID(self.pointer), sibling.unwrap_widget(), side);
        }
    }

    pub fn set_row_homogeneous(&self, homogeneous: bool) -> () {
        unsafe { ffi::gtk_grid_set_row_homogeneous(GTK_GRID(self.pointer), to_gboolean(homogeneous)); }
    }

    pub fn get_row_homogeneous(&self) -> bool {
        unsafe { to_bool(ffi::gtk_grid_get_row_homogeneous(GTK_GRID(self.pointer))) }
    }

    pub fn set_row_spacing(&self, spacing: u32) -> () {
        unsafe {
            ffi::gtk_grid_set_row_spacing(GTK_GRID(self.pointer), spacing as c_uint);
        }
    }

    pub fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_row_spacing(GTK_GRID(self.pointer)) as u32
        }
    }

    pub fn set_column_homogeneous(&self, homogeneous: bool) -> () {
        unsafe { ffi::gtk_grid_set_column_homogeneous(GTK_GRID(self.pointer), to_gboolean(homogeneous)); }
    }

    pub fn get_column_homogeneous(&self) -> bool {
        unsafe { to_bool(ffi::gtk_grid_get_column_homogeneous(GTK_GRID(self.pointer))) }
    }

    pub fn set_column_spacing(&self, spacing: u32) -> () {
        unsafe {
            ffi::gtk_grid_set_column_spacing(GTK_GRID(self.pointer), spacing as c_uint);
        }
    }

    pub fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_column_spacing(GTK_GRID(self.pointer)) as u32
        }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn get_baseline_row(&self) -> i32 {
        unsafe {
            ffi::gtk_grid_get_baseline_row(GTK_GRID(self.pointer)) as i32
        }
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn set_baseline_row(&self, row: i32) -> () {
        unsafe {
            ffi::gtk_grid_set_baseline_row(GTK_GRID(self.pointer), row as c_int);
        }
    }
}

impl_drop!(Grid);
impl_TraitWidget!(Grid);

impl ::ContainerTrait for Grid {}
impl ::OrientableTrait for Grid {}

impl_widget_events!(Grid);
