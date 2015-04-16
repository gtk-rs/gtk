// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! Renders a toggle button in a cell

use ffi;
use glib::{to_bool, to_gboolean};

struct_Widget!(CellRendererToggle);

impl CellRendererToggle {
    pub fn new() -> Option<CellRendererToggle> {
        let tmp_pointer = unsafe { ffi::gtk_cell_renderer_toggle_new() as *mut ffi::C_GtkWidget };

        check_pointer!(tmp_pointer, CellRendererToggle)
    }

    pub fn get_radio(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_cell_renderer_toggle_get_radio(
                    self.pointer as *mut ffi::C_GtkCellRendererToggle))
        }
    }

    pub fn set_radio(&self, radio: bool) -> () {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(
                self.pointer as *mut ffi::C_GtkCellRendererToggle, to_gboolean(radio));
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_cell_renderer_toggle_get_active(
                self.pointer as *mut ffi::C_GtkCellRendererToggle))
        }
    }

    pub fn set_active(&self, active: bool) -> () {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(
                self.pointer as *mut ffi::C_GtkCellRendererToggle, to_gboolean(active));
        }
    }
}

impl_drop!(CellRendererToggle);
impl_TraitWidget!(CellRendererToggle);

impl ::CellRendererTrait for CellRendererToggle {}

impl_widget_events!(CellRendererToggle);
