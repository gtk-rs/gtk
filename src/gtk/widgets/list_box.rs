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

//! A container that allows reflowing its children

use gtk::{self, ffi};
use gtk::cast::{GTK_LIST_BOX_ROW, GTK_LIST_BOX};
use gtk::ffi::FFIWidget;

/// GtkFlowBox — A container that allows reflowing its children
struct_Widget!(ListBox);

impl ListBox {
    pub fn new() -> Option<ListBox> {
        let tmp_pointer = unsafe { ffi::gtk_list_box_new() };
        check_pointer!(tmp_pointer, ListBox)
    }

    pub fn prepend<T: gtk::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_list_box_prepend(GTK_LIST_BOX(self.pointer),
                                      child.get_widget())
        }
    }

    pub fn insert<T: gtk::WidgetTrait>(&mut self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(GTK_LIST_BOX(self.pointer),
                                     child.get_widget(),
                                     position)
        }
    }

    pub fn get_selected_row(&self) -> Option<ListBoxRow> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_selected_row(GTK_LIST_BOX(self.pointer))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn get_row_at_index(&self, index: i32) -> Option<ListBoxRow> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_row_at_index(GTK_LIST_BOX(self.pointer), index)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn get_row_at_iy(&self, y: i32) -> Option<ListBoxRow> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_row_at_y(GTK_LIST_BOX(self.pointer), y)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn select_row(&mut self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_select_row(GTK_LIST_BOX(self.pointer),
                                         GTK_LIST_BOX_ROW(row.get_widget()))
        }
    }

    pub fn set_placeholder<T: gtk::WidgetTrait>(&mut self, placeholder: &T) {
        unsafe {
            ffi::gtk_list_box_set_placeholder(GTK_LIST_BOX(self.pointer),
                                              placeholder.get_widget())
        }
    }

    pub fn set_adjustment(&mut self, adjustment: &gtk::Adjustment) {
        unsafe {
            ffi::gtk_list_box_set_adjustment(GTK_LIST_BOX(self.pointer),
                                             adjustment.get_pointer())
        }
    }

    pub fn get_adjustment(&self) -> Option<gtk::Adjustment> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_get_adjustment(GTK_LIST_BOX(self.pointer))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::Adjustment::wrap_pointer(tmp_pointer))
        }
    }

    pub fn set_selection_mode(&mut self, mode: gtk::SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(GTK_LIST_BOX(self.pointer), mode)
        }
    }

    pub fn get_selection_mode(&self) -> gtk::SelectionMode {
        unsafe {
            ffi::gtk_list_box_get_selection_mode(GTK_LIST_BOX(self.pointer))
        }
    }

    pub fn invalidate_header(&mut self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(GTK_LIST_BOX(self.pointer))
        }
    }

    pub fn set_activate_on_single_click(&mut self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(GTK_LIST_BOX(self.pointer),
                                                           ffi::to_gboolean(single))
        }
    }

    pub fn is_activate_on_single_click(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_list_box_get_activate_on_single_click(GTK_LIST_BOX(self.pointer)))
        }
    }

    pub fn drag_unhighlight_row(&mut self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(GTK_LIST_BOX(self.pointer))
        }
    }

    pub fn drag_highlight_row(&mut self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(GTK_LIST_BOX(self.pointer),
                                                 row.get_widget() as *mut ffi::C_GtkListBoxRow)
        }
    }
}


// pub fn gtk_list_box_drag_unhighlight_row         (list_box: *C_GtkListBox);
// pub fn gtk_list_box_drag_highlight_row           (list_box: *C_GtkListBox, row: *C_GtkListBoxRow);

impl_drop!(ListBox);
impl_TraitWidget!(ListBox);

impl gtk::ContainerTrait for ListBox {}

struct_Widget!(ListBoxRow);

impl ListBoxRow {
    pub fn new() -> Option<ListBoxRow> {
        let tmp_pointer = unsafe { ffi::gtk_list_box_row_new() };
        check_pointer!(tmp_pointer, ListBoxRow)
    }

    pub fn changed(&mut self) {
        unsafe {
            ffi::gtk_list_box_row_changed(GTK_LIST_BOX_ROW(self.pointer))
        }
    }

    pub fn get_header<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_list_box_row_get_header(GTK_LIST_BOX_ROW(self.pointer))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn set_header<T: gtk::WidgetTrait>(&mut self, header: &T) {
        unsafe {
            ffi::gtk_list_box_row_set_header(GTK_LIST_BOX_ROW(self.pointer),
                                             header.get_widget())
        }
    }

    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_list_box_row_get_index(GTK_LIST_BOX_ROW(self.pointer))
        }
    }
}

impl_drop!(ListBoxRow);
impl_TraitWidget!(ListBoxRow);

impl gtk::ContainerTrait for ListBoxRow {}
impl gtk::BinTrait for ListBoxRow {}

impl_widget_events!(ListBox);
