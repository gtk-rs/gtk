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

//! Interface for widgets which can are used for editing cells

use ffi;
use cast::GTK_CELL_EDITABLE;

pub trait CellEditableTrait : ::WidgetTrait {
    fn editing_done(&self) {
        unsafe { ffi::gtk_cell_editable_editing_done(GTK_CELL_EDITABLE(self.unwrap_widget())) }
    }

    fn remove_widget(&self) {
        unsafe { ffi::gtk_cell_editable_remove_widget(GTK_CELL_EDITABLE(self.unwrap_widget())) }
    }
}