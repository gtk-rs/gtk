// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::{to_bool, to_gboolean};

struct_Widget!(CellRendererToggle);

impl CellRendererToggle {
    pub fn new() -> Option<CellRendererToggle> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_cell_renderer_toggle_new() as *mut ffi::GtkWidget };

        check_pointer!(tmp_pointer, CellRendererToggle)
    }

    pub fn get_radio(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_cell_renderer_toggle_get_radio(
                    self.pointer as *mut ffi::GtkCellRendererToggle))
        }
    }

    pub fn set_radio(&self, radio: bool) -> () {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(
                self.pointer as *mut ffi::GtkCellRendererToggle, to_gboolean(radio));
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_cell_renderer_toggle_get_active(
                self.pointer as *mut ffi::GtkCellRendererToggle))
        }
    }

    pub fn set_active(&self, active: bool) -> () {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(
                self.pointer as *mut ffi::GtkCellRendererToggle, to_gboolean(active));
        }
    }
}

impl_drop!(CellRendererToggle);
impl_TraitWidget!(CellRendererToggle);

impl ::CellRendererTrait for CellRendererToggle {}
