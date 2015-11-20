// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::ToGlibPtr;
use ffi;
use cast::{GTK_CELL_LAYOUT, GTK_CELL_RENDERER};
use glib;

pub trait CellLayoutTrait: ::WidgetTrait {
    fn pack_start<T: ::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    fn pack_end<T: ::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    /*fn get_area(&self) -> Option<::CellArea> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_area(GTK_CELL_LAYOUT(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }*/

    unsafe fn get_cells<T: ::CellRendererTrait>(&self) -> glib::List<T> {
        let tmp = ffi::gtk_cell_layout_get_cells(GTK_CELL_LAYOUT(self.unwrap_widget()));

        if tmp.is_null() {
            glib::List::new()
        } else {
            let list: glib::List<*mut ffi::GtkWidget> = glib::GlibContainer::wrap(tmp);

            list.iter().map(|it| ::FFIWidget::wrap_widget(*it)).collect()
        }
    }

    fn reorder<T: ::CellRendererTrait>(&self, cell: &T, position: i32) {
        unsafe { ffi::gtk_cell_layout_reorder(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget()), position) }
    }

    fn clear(&self) {
        unsafe { ffi::gtk_cell_layout_clear(GTK_CELL_LAYOUT(self.unwrap_widget())) }
    }

    fn add_attribute<T: ::CellRendererTrait>(&self, cell: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(
                GTK_CELL_LAYOUT(self.unwrap_widget()),
                GTK_CELL_RENDERER(cell.unwrap_widget()),
                attribute.to_glib_none().0,
                column)
            }
    }

    fn clear_attributes<T: ::CellRendererTrait>(&self, cell: &T) {
        unsafe { ffi::gtk_cell_layout_clear_attributes(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget())) }
    }
}
