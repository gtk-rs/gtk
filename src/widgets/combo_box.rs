// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

use mvc::tree_model::{TreeIter, TreeModel};
use glib::object::{Downcast, Upcast};
use super::widget::Widget;

use SensitivityType;

///////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct ComboBox(Object<ffi::GtkComboBox>): Widget, ::Container, ::Bin, ::CellEditable,
        ::CellLayout, ::Buildable;

    match fn {
        get_type => || ffi::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub fn new() -> ComboBox {
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_new()).downcast_unchecked() }
    }

    pub fn new_with_entry() -> ComboBox {
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_new_with_entry()).downcast_unchecked() }
    }

    pub fn new_with_model<T: Upcast<TreeModel>>(model: &T) -> ComboBox {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_combo_box_new_with_model(model.upcast().to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn new_with_model_and_entry<T: Upcast<TreeModel>>(model: &T) -> ComboBox {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_combo_box_new_with_model_and_entry(model.upcast().to_glib_none().0))
                .downcast_unchecked()
        }
    }

    /*pub fn new_with_area(area: &CellArea) -> ComboBox {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area(area.upcast().to_glib_none().0) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_area_and_entry(area: &CellArea) -> ComboBox {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area_and_entry(area.upcast().to_glib_none().0) };
        check_pointer!(tmp_pointer, ComboBox)
    }*/
}

pub trait ComboBoxExt {
    fn get_wrap_width(&self) -> i32;
    fn set_wrap_width(&self, width: i32);
    fn get_row_span_column(&self) -> i32;
    fn set_row_span_column(&self, row_span: i32);
    fn get_column_span_column(&self) -> i32;
    fn set_column_span_column(&self, column_span: i32);
    fn get_active(&self) -> i32;
    fn set_active(&self, active: i32);
    fn get_active_iter(&self) -> Option<TreeIter>;
    fn set_active_iter(&self, iter: &TreeIter);
    fn get_id_column(&self) -> i32;
    fn set_id_column(&self, id_column: i32);
    fn get_active_id(&self) -> Option<String>;
    fn set_active_id(&self, active_id: Option<&str>) -> bool;
    fn get_model(&self) -> Option<TreeModel>;
    fn set_model<T: Upcast<TreeModel>>(&self, model: Option<&T>);
    fn popup(&self);
    fn popdown(&self);
    fn get_focus_on_click(&self) -> bool;
    fn set_focus_on_click(&self, focus_on_click: bool);
    fn get_button_sensitivity(&self) -> SensitivityType;
    fn set_button_sensitivity(&self, sensitivity: SensitivityType);
    fn get_has_entry(&self) -> bool;
    fn set_entry_text_column(&self, text_column: i32);
    fn get_entry_text_column(&self) -> i32;
    fn set_popup_fixed_width(&self, fixed: bool);
    fn get_popup_fixed_width(&self) -> bool;
}

impl<O: Upcast<ComboBox>> ComboBoxExt for O {
    fn get_wrap_width(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_wrap_width(self.upcast().to_glib_none().0) }
    }

    fn set_wrap_width(&self, width: i32) {
        unsafe { ffi::gtk_combo_box_set_wrap_width(self.upcast().to_glib_none().0, width) }
    }

    fn get_row_span_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_row_span_column(self.upcast().to_glib_none().0) }
    }

    fn set_row_span_column(&self, row_span: i32) {
        unsafe { ffi::gtk_combo_box_set_row_span_column(self.upcast().to_glib_none().0, row_span) }
    }

    fn get_column_span_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_column_span_column(self.upcast().to_glib_none().0) }
    }

    fn set_column_span_column(&self, column_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_column_span_column(self.upcast().to_glib_none().0, column_span)
        }
    }

    fn get_active(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_active(self.upcast().to_glib_none().0) }
    }

    fn set_active(&self, active: i32) {
        unsafe { ffi::gtk_combo_box_set_active(self.upcast().to_glib_none().0, active) }
    }

    fn get_active_iter(&self) -> Option<TreeIter> {
        unsafe {
            let iter = TreeIter::new();
            let ok = from_glib(
                ffi::gtk_combo_box_get_active_iter(self.upcast().to_glib_none().0,
                    iter.unwrap_pointer()));
            if ok { Some(iter) } else { None }
        }
    }

    fn set_active_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_combo_box_set_active_iter(self.upcast().to_glib_none().0,
                iter.unwrap_pointer())
        }
    }

    fn get_id_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_id_column(self.upcast().to_glib_none().0) }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe { ffi::gtk_combo_box_set_id_column(self.upcast().to_glib_none().0, id_column) }
    }

    fn get_active_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_combo_box_get_active_id(self.upcast().to_glib_none().0))
        }
    }

    fn set_active_id(&self, active_id: Option<&str>) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_combo_box_set_active_id(self.upcast().to_glib_none().0,
                    active_id.to_glib_none().0))
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_combo_box_get_model(self.upcast().to_glib_none().0)) }
    }

    fn set_model<T: Upcast<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_combo_box_set_model(self.upcast().to_glib_none().0,
                model.map(|p| p.upcast()).to_glib_none().0)
        }
    }

    fn popup(&self) {
        unsafe { ffi::gtk_combo_box_popup(self.upcast().to_glib_none().0) }
    }

    fn popdown(&self) {
        unsafe { ffi::gtk_combo_box_popdown(self.upcast().to_glib_none().0) }
    }

    fn get_focus_on_click(&self) -> bool {
        unsafe { from_glib(ffi::gtk_combo_box_get_focus_on_click(self.upcast().to_glib_none().0)) }
    }

    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_combo_box_set_focus_on_click(self.upcast().to_glib_none().0,
                focus_on_click.to_glib())
        }
    }

    fn get_button_sensitivity(&self) -> SensitivityType {
        unsafe { ffi::gtk_combo_box_get_button_sensitivity(self.upcast().to_glib_none().0) }
    }

    fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_combo_box_set_button_sensitivity(self.upcast().to_glib_none().0, sensitivity)
        }
    }

    fn get_has_entry(&self) -> bool {
        unsafe { from_glib(ffi::gtk_combo_box_get_has_entry(self.upcast().to_glib_none().0)) }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_entry_text_column(self.upcast().to_glib_none().0, text_column)
        }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_entry_text_column(self.upcast().to_glib_none().0) }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            ffi::gtk_combo_box_set_popup_fixed_width(self.upcast().to_glib_none().0,
                fixed.to_glib())
        }
    }

    fn get_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_popup_fixed_width(self.upcast().to_glib_none().0))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A widget used to choose from a list of items.
glib_wrapper! {
    pub struct ComboBoxText(Object<ffi::GtkComboBoxText>): Widget, ::Container, ::Bin, ComboBox,
        ::CellEditable, ::CellLayout, ::Buildable;

    match fn {
        get_type => || ffi::gtk_combo_box_text_get_type(),
    }
}

impl ComboBoxText {
    pub fn new() -> ComboBoxText {
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_text_new()).downcast_unchecked() }
    }

    pub fn new_with_entry() -> ComboBoxText {
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_text_new_with_entry()).downcast_unchecked()
        }
    }

    pub fn append(&self, id: &str, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append(self.to_glib_none().0,
                                           id.to_glib_none().0,
                                           text.to_glib_none().0)
        }
    }

    pub fn prepend(&self, id: &str, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend(self.to_glib_none().0,
                                            id.to_glib_none().0,
                                            text.to_glib_none().0)
        }
    }

    pub fn insert(&self, position: i32, id: &str, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert(self.to_glib_none().0,
                                           position,
                                           id.to_glib_none().0,
                                           text.to_glib_none().0)
        }
    }

    pub fn append_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append_text(self.to_glib_none().0, text.to_glib_none().0)
        }
    }

    pub fn prepend_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend_text(self.to_glib_none().0, text.to_glib_none().0)
        }
    }

    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert_text(self.to_glib_none().0, position,
                text.to_glib_none().0)
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe { ffi::gtk_combo_box_text_remove(self.to_glib_none().0, position) }
    }

    pub fn remove_all(&self) {
        unsafe { ffi::gtk_combo_box_text_remove_all(self.to_glib_none().0) }
    }

    pub fn get_active_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_combo_box_text_get_active_text(self.to_glib_none().0))
        }
    }
}
