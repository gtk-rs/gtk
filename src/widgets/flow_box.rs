// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use adjustment::Adjustment;
use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use SelectionMode;

///////////////////////////////////////////////////////////////////////////////

/// A container that allows reflowing its children.
pub type FlowBox = Object<ffi::GtkFlowBox>;

unsafe impl Upcast<Widget> for FlowBox { }
unsafe impl Upcast<super::container::Container> for FlowBox { }
unsafe impl Upcast<super::orientable::Orientable> for FlowBox { }
unsafe impl Upcast<::builder::Buildable> for FlowBox { }

impl FlowBox {
    pub fn new() -> FlowBox {
        unsafe { Widget::from_glib_none(ffi::gtk_flow_box_new()).downcast_unchecked() }
    }

    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe { ffi::gtk_flow_box_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib()); }
    }

    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_flow_box_get_homogeneous(self.to_glib_none().0)) }
    }

    pub fn get_row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_row_spacing(self.to_glib_none().0) }
    }

    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe { ffi::gtk_flow_box_set_row_spacing(self.to_glib_none().0, spacing) }
    }

    pub fn set_colum_spacing(&self, spacing: u32) {
        unsafe { ffi::gtk_flow_box_set_column_spacing(self.to_glib_none().0, spacing) }
    }

    pub fn get_column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_column_spacing(self.to_glib_none().0) }
    }

    pub fn set_min_children_per_line(&self, n_children: u32) {
        unsafe { ffi::gtk_flow_box_set_min_children_per_line(self.to_glib_none().0, n_children) }
    }

    pub fn get_min_children_per_line(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_min_children_per_line(self.to_glib_none().0) }
    }

    pub fn set_max_children_per_line(&self, n_children: u32) {
        unsafe { ffi::gtk_flow_box_set_max_children_per_line(self.to_glib_none().0, n_children) }
    }

    pub fn get_max_children_per_line(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_max_children_per_line(self.to_glib_none().0) }
    }

    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(self.to_glib_none().0,
                                                           single.to_glib())
        }
    }

    pub fn is_activate_on_single_click(&self) -> bool {
        unsafe { from_glib(ffi::gtk_flow_box_get_activate_on_single_click(self.to_glib_none().0)) }
    }

    pub fn insert<T: Upcast<Widget>>(&self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(self.to_glib_none().0,
                                     widget.upcast().to_glib_none().0,
                                     position)
        }
    }

    pub fn get_child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_index(self.to_glib_none().0, idx))
        }
    }

    pub fn select_child(&self, child: &FlowBoxChild) {
        unsafe { ffi::gtk_flow_box_select_child(self.to_glib_none().0, child.to_glib_none().0) }
    }

    pub fn unselect_child(&self, child: &FlowBoxChild) {
        unsafe { ffi::gtk_flow_box_unselect_child(self.to_glib_none().0, child.to_glib_none().0) }
    }

    pub fn select_all(&self) {
        unsafe { ffi::gtk_flow_box_select_all(self.to_glib_none().0) }
    }

    pub fn unselect_all(&self) {
        unsafe { ffi::gtk_flow_box_unselect_all(self.to_glib_none().0) }
    }

    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe { ffi::gtk_flow_box_set_selection_mode(self.to_glib_none().0, mode) }
    }

    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            ffi::gtk_flow_box_get_selection_mode(self.to_glib_none().0)
        }
    }

    pub fn set_hadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(self.to_glib_none().0, adjustment.to_glib_none().0)
        }
    }

    pub fn set_vadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(self.to_glib_none().0, adjustment.to_glib_none().0)
        }
    }
}


impl types::StaticType for FlowBox {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_flow_box_get_type()) }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub type FlowBoxChild = Object<ffi::GtkFlowBoxChild>;

unsafe impl Upcast<Widget> for FlowBoxChild { }
unsafe impl Upcast<super::container::Container> for FlowBoxChild { }
unsafe impl Upcast<super::bin::Bin> for FlowBoxChild { }
unsafe impl Upcast<::builder::Buildable> for FlowBoxChild { }

impl FlowBoxChild {
    pub fn new() -> FlowBoxChild {
        unsafe { Widget::from_glib_none(ffi::gtk_flow_box_child_new()).downcast_unchecked() }
    }

    pub fn get_index(&self) -> i32 {
        unsafe { ffi::gtk_flow_box_child_get_index(self.to_glib_none().0) }
    }

    pub fn is_selected(&self) -> bool {
        unsafe { from_glib(ffi::gtk_flow_box_child_is_selected(self.to_glib_none().0)) }
    }

    pub fn changed(&self) {
        unsafe { ffi::gtk_flow_box_child_changed(self.to_glib_none().0) }
    }
}

impl types::StaticType for FlowBoxChild {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_flow_box_child_get_type()) }
    }
}
