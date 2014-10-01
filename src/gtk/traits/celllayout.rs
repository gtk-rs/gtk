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

use gtk::ffi;
use gtk::traits;
use gtk::cast::{GTK_CELL_LAYOUT, GTK_CELL_RENDERER};
use glib;

pub trait CellLayout: traits::Widget {
    fn pack_start(&self, cell: &traits::CellRenderer, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    fn pack_end(&self, cell: &traits::CellRenderer, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    /*fn get_area(&self) -> Option<gtk::CellArea> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_area(GTK_CELL_LAYOUT(self.get_widget())) };
        
        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }*/

    fn get_cells<T: traits::CellRenderer>(&self) -> glib::List<T> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_cells(GTK_CELL_LAYOUT(self.get_widget())) };
        
        if tmp.is_null() {
            glib::List::new()
        } else {
            let list: glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);
        
            list.iter().map(|it| ffi::FFIWidget::wrap(*it)).collect()
        }
    }

    fn reorder(&self, cell: &traits::CellRenderer, position: i32) {
        unsafe { ffi::gtk_cell_layout_reorder(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()), position) }
    }

    fn clear(&self) {
        unsafe { ffi::gtk_cell_layout_clear(GTK_CELL_LAYOUT(self.get_widget())) }
    }

    fn add_attribute(&self, cell: &traits::CellRenderer, attribute: &str, column: i32) {
        attribute.with_c_str(|c_str| {
            unsafe {
                ffi::gtk_cell_layout_add_attribute(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()),
                    c_str, column)
            }
        })
    }

    fn clear_attributes(&self, cell: &traits::CellRenderer) {
        unsafe { ffi::gtk_cell_layout_clear_attributes(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget())) }
    }
}