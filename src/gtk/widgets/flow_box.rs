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

use gtk::cast::{GTK_FLOW_BOX_CHILD, GTK_FLOW_BOX};
use gtk::{self, ffi};
use gtk::ffi::FFIWidget;
use glib::{to_bool, to_gboolean};

/// GtkFlowBox — A container that allows reflowing its children
struct_Widget!(FlowBox);

impl FlowBox {
    pub fn new() -> Option<FlowBox> {
        let tmp_pointer = unsafe { ffi::gtk_revealer_new() };
        check_pointer!(tmp_pointer, FlowBox)
    }

    pub fn set_homogeneous(&mut self, homogeneous: bool) {
        unsafe {
            ffi::gtk_flow_box_set_homogeneous(GTK_FLOW_BOX(self.pointer),
                                              to_gboolean(homogeneous))
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_flow_box_get_homogeneous(GTK_FLOW_BOX(self.pointer)))
        }
    }

    pub fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_row_spacing(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn set_row_spacing(&mut self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_row_spacing(GTK_FLOW_BOX(self.pointer), spacing)
        }
    }

    pub fn set_colum_spacing(&mut self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_column_spacing(GTK_FLOW_BOX(self.pointer), spacing)
        }
    }

    pub fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_column_spacing(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn set_min_children_per_line(&mut self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_min_children_per_line(GTK_FLOW_BOX(self.pointer), n_children)
        }
    }

    pub fn get_min_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_min_children_per_line(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn set_max_children_per_line(&mut self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_max_children_per_line(GTK_FLOW_BOX(self.pointer), n_children)
        }
    }

    pub fn get_max_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_max_children_per_line(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn set_activate_on_single_click(&mut self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(GTK_FLOW_BOX(self.pointer),
                                                           to_gboolean(single))
        }
    }

    pub fn is_activate_on_single_click(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_flow_box_get_activate_on_single_click(GTK_FLOW_BOX(self.pointer)))
        }
    }

    pub fn insert<T: gtk::WidgetTrait>(&mut self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(GTK_FLOW_BOX(self.pointer),
                                     widget.get_widget(),
                                     position)
        }
    }

    pub fn get_child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        let tmp_pointer = unsafe {
            ffi::gtk_flow_box_get_child_at_index(GTK_FLOW_BOX(self.pointer), idx)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn select_child(&mut self, child: &FlowBoxChild) {
        unsafe {
            ffi::gtk_flow_box_select_child(GTK_FLOW_BOX(self.pointer),
                                           GTK_FLOW_BOX_CHILD(child.get_widget()))
        }
    }

    pub fn unselect_child(&mut self, child: &FlowBoxChild) {
        unsafe {
            ffi::gtk_flow_box_unselect_child(GTK_FLOW_BOX(self.pointer),
                                             GTK_FLOW_BOX_CHILD(child.get_widget()))
        }
    }

    pub fn select_all(&mut self) {
        unsafe {
            ffi::gtk_flow_box_select_all(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn unselect_all(&mut self) {
        unsafe {
            ffi::gtk_flow_box_unselect_all(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn set_selection_mode(&mut self, mode: gtk::SelectionMode) {
        unsafe {
            ffi::gtk_flow_box_set_selection_mode(GTK_FLOW_BOX(self.pointer), mode)
        }
    }

    pub fn get_selection_mode(&self) -> gtk::SelectionMode {
        unsafe {
            ffi::gtk_flow_box_get_selection_mode(GTK_FLOW_BOX(self.pointer))
        }
    }

    pub fn set_hadjustment(&mut self, adjustment: gtk::Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(GTK_FLOW_BOX(self.pointer),
                                              adjustment.get_pointer())
        }
    }

    pub fn set_vadjustment(&mut self, adjustment: gtk::Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(GTK_FLOW_BOX(self.pointer),
                                              adjustment.get_pointer())
        }
    }
}

impl_drop!(FlowBox);
impl_TraitWidget!(FlowBox);

impl gtk::ContainerTrait for FlowBox {}

impl_widget_events!(FlowBox);

struct_Widget!(FlowBoxChild);

impl FlowBoxChild {
    pub fn new() -> Option<FlowBoxChild> {
        let tmp_pointer = unsafe { ffi::gtk_flow_box_child_new() };
        check_pointer!(tmp_pointer, FlowBoxChild)
    }

    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(GTK_FLOW_BOX_CHILD(self.pointer))
        }
    }

    pub fn is_selected(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_flow_box_child_is_selected(GTK_FLOW_BOX_CHILD(self.pointer)))
        }
    }

    pub fn changed(&mut self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(GTK_FLOW_BOX_CHILD(self.pointer))
        }
    }
}

impl_drop!(FlowBoxChild);
impl_TraitWidget!(FlowBoxChild);

impl gtk::ContainerTrait for FlowBoxChild {}
impl gtk::BinTrait for FlowBoxChild {}

impl_widget_events!(FlowBoxChild);
