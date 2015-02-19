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

use std::ffi::CString;
use gtk::{self, ffi};
use gtk::cast::{GTK_CELL_LAYOUT, GTK_CELL_RENDERER};
use glib;

pub trait CellLayoutTrait: gtk::WidgetTrait {
    fn pack_start<T: gtk::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    fn pack_end<T: gtk::CellRendererTrait>(&self, cell: &T, expand: bool) {
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
            Some(gtk::FFIWidget::wrap(tmp_pointer))
        }
    }*/

    fn get_cells<T: gtk::CellRendererTrait>(&self) -> glib::List<T> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_cells(GTK_CELL_LAYOUT(self.get_widget())) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let list: glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);

            list.iter().map(|it| gtk::FFIWidget::wrap(*it)).collect()
        }
    }

    fn reorder<T: gtk::CellRendererTrait>(&self, cell: &T, position: i32) {
        unsafe { ffi::gtk_cell_layout_reorder(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()), position) }
    }

    fn clear(&self) {
        unsafe { ffi::gtk_cell_layout_clear(GTK_CELL_LAYOUT(self.get_widget())) }
    }

    fn add_attribute<T: gtk::CellRendererTrait>(&self, cell: &T, attribute: &str, column: i32) {
        let c_str = CString::from_slice(attribute.as_bytes());

        unsafe {
                ffi::gtk_cell_layout_add_attribute(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget()),
                    c_str.as_ptr(), column)
            }
    }

    fn clear_attributes<T: gtk::CellRendererTrait>(&self, cell: &T) {
        unsafe { ffi::gtk_cell_layout_clear_attributes(GTK_CELL_LAYOUT(self.get_widget()), GTK_CELL_RENDERER(cell.get_widget())) }
    }
}
