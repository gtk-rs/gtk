// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Cell-related interfaces.

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast};
use widgets::widget::Widget;
use super::cell_renderer::CellRenderer;

///////////////////////////////////////////////////////////////////////////////

pub type CellEditable = Object<ffi::GtkCellEditable>;

impl types::StaticType for CellEditable {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_cell_editable_get_type()) }
    }
}

unsafe impl Upcast<Widget> for CellEditable { }

pub trait CellEditableExt {
    fn editing_done(&self);
    fn remove_widget(&self);
}

impl<O: Upcast<CellEditable>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe { ffi::gtk_cell_editable_editing_done(self.upcast().to_glib_none().0) }
    }

    fn remove_widget(&self) {
        unsafe { ffi::gtk_cell_editable_remove_widget(self.upcast().to_glib_none().0) }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub type CellLayout = Object<ffi::GtkCellLayout>;

impl types::StaticType for CellLayout {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_cell_layout_get_type()) }
    }
}

pub trait CellLayoutExt {
    fn pack_start<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool);
    fn pack_end<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool);
    fn get_cells(&self) -> Vec<CellRenderer>;
    fn reorder<T: Upcast<CellRenderer>>(&self, cell: &T, position: i32);
    fn clear(&self);
    fn add_attribute<T: Upcast<CellRenderer>>(&self, cell: &T, attribute: &str, column: i32);
    fn clear_attributes<T: Upcast<CellRenderer>>(&self, cell: &T);
}

impl<O: Upcast<CellLayout>> CellLayoutExt for O {
    fn pack_start<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(self.upcast().to_glib_none().0,
                cell.upcast().to_glib_none().0, expand.to_glib())
        }
    }

    fn pack_end<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(self.upcast().to_glib_none().0,
                cell.upcast().to_glib_none().0, expand.to_glib())
        }
    }

    /*fn get_area(&self) -> Option<::CellArea> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_area(self.upcast().to_glib_none().0) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }*/

    fn get_cells(&self) -> Vec<CellRenderer> {
        unsafe {
            Vec::from_glib_container(ffi::gtk_cell_layout_get_cells(self.upcast().to_glib_none().0))
        }
    }

    fn reorder<T: Upcast<CellRenderer>>(&self, cell: &T, position: i32) {
        unsafe {
            ffi::gtk_cell_layout_reorder(self.upcast().to_glib_none().0,
                cell.upcast().to_glib_none().0, position)
        }
    }

    fn clear(&self) {
        unsafe { ffi::gtk_cell_layout_clear(self.upcast().to_glib_none().0) }
    }

    fn add_attribute<T: Upcast<CellRenderer>>(&self, cell: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(self.upcast().to_glib_none().0,
                cell.upcast().to_glib_none().0, attribute.to_glib_none().0, column)
        }
    }

    fn clear_attributes<T: Upcast<CellRenderer>>(&self, cell: &T) {
        unsafe {
            ffi::gtk_cell_layout_clear_attributes(self.upcast().to_glib_none().0,
                cell.upcast().to_glib_none().0)
        }
    }
}
