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

//! Renders text in a cell

use ffi;
use cast::GTK_CELL_RENDERER_TEXT;

struct_Widget!(CellRendererText);

impl CellRendererText {
    pub fn new() -> Option<CellRendererText> {
        let tmp_pointer = unsafe { ffi::gtk_cell_renderer_text_new() as *mut ffi::C_GtkWidget };

        check_pointer!(tmp_pointer, CellRendererText)
    }

    fn set_fixed_height_from_font(&self, number_of_rows: i32) {
        unsafe { ffi::gtk_cell_renderer_text_set_fixed_height_from_font(GTK_CELL_RENDERER_TEXT(self.pointer), number_of_rows) }
    }
}

impl_drop!(CellRendererText);
impl_TraitWidget!(CellRendererText);

impl ::CellRendererTrait for CellRendererText {}

impl_widget_events!(CellRendererText);
