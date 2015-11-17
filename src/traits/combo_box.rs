// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_COMBO_BOX;
use TreeIter;

pub trait ComboBoxTrait: ::WidgetTrait + ::ContainerTrait + ::BinTrait {
    fn get_wrap_width(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_wrap_width(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_wrap_width(&self, width: i32) {
        unsafe { ffi::gtk_combo_box_set_wrap_width(GTK_COMBO_BOX(self.unwrap_widget()), width) }
    }

    fn get_row_span_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_row_span_column(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_row_span_column(&self, row_span: i32) {
        unsafe { ffi::gtk_combo_box_set_row_span_column(GTK_COMBO_BOX(self.unwrap_widget()), row_span) }
    }

    fn get_column_span_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_column_span_column(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_column_span_column(&self, column_span: i32) {
        unsafe { ffi::gtk_combo_box_set_column_span_column(GTK_COMBO_BOX(self.unwrap_widget()), column_span) }
    }

    fn get_active(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_active(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_active(&self, active: i32) {
        unsafe { ffi::gtk_combo_box_set_active(GTK_COMBO_BOX(self.unwrap_widget()), active) }
    }

    fn get_active_iter(&self) -> Option<TreeIter> {
         unsafe {
            let mut iter = TreeIter::uninitialized();
            let ok = ffi::gtk_combo_box_get_active_iter(GTK_COMBO_BOX(self.unwrap_widget()),
                iter.to_glib_none_mut().0);
            some_if(ok, iter)
        }
    }

    fn set_active_iter(&self, mut iter: Option<&mut TreeIter>) {
        unsafe {
            ffi::gtk_combo_box_set_active_iter(GTK_COMBO_BOX(self.unwrap_widget()),
                iter.to_glib_none_mut().0)
        }
    }

    fn get_id_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_id_column(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe { ffi::gtk_combo_box_set_id_column(GTK_COMBO_BOX(self.unwrap_widget()), id_column) }
    }

    fn get_active_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_combo_box_get_active_id(GTK_COMBO_BOX(self.unwrap_widget())))
        }
    }

    fn set_active_id(&self, active_id: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_combo_box_set_active_id(GTK_COMBO_BOX(self.unwrap_widget()), active_id.to_glib_none().0))
        }
    }

    fn get_model(&self) -> Option<::TreeModel> {
        unsafe {
            let ptr = ffi::gtk_combo_box_get_model(GTK_COMBO_BOX(self.unwrap_widget()));
            if ptr.is_null() {
                None
            } else {
                Some(::TreeModel::wrap_pointer(ptr))
            }
        }
    }

    fn set_model(&self, model: ::TreeModel) {
        unsafe { ffi::gtk_combo_box_set_model(GTK_COMBO_BOX(self.unwrap_widget()), model.unwrap_pointer()) }
    }

    fn popup(&self) {
        unsafe { ffi::gtk_combo_box_popup(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn popdown(&self) {
        unsafe { ffi::gtk_combo_box_popdown(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn get_focus_on_click(&self) -> bool {
        unsafe { to_bool(ffi::gtk_combo_box_get_focus_on_click(GTK_COMBO_BOX(self.unwrap_widget()))) }
    }

    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe { ffi::gtk_combo_box_set_focus_on_click(GTK_COMBO_BOX(self.unwrap_widget()), to_gboolean(focus_on_click)) }
    }

    fn get_button_sensitivity(&self) -> ::SensitivityType {
        unsafe { ffi::gtk_combo_box_get_button_sensitivity(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_button_sensitivity(&self, sensitivity: ::SensitivityType) {
        unsafe { ffi::gtk_combo_box_set_button_sensitivity(GTK_COMBO_BOX(self.unwrap_widget()), sensitivity) }
    }

    fn get_has_entry(&self) -> bool {
        unsafe { to_bool(ffi::gtk_combo_box_get_has_entry(GTK_COMBO_BOX(self.unwrap_widget()))) }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe { ffi::gtk_combo_box_set_entry_text_column(GTK_COMBO_BOX(self.unwrap_widget()), text_column) }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_entry_text_column(GTK_COMBO_BOX(self.unwrap_widget())) }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe { ffi::gtk_combo_box_set_popup_fixed_width(GTK_COMBO_BOX(self.unwrap_widget()), to_gboolean(fixed)) }
    }

    fn get_popup_fixed_width(&self) -> bool {
        unsafe { to_bool(ffi::gtk_combo_box_get_popup_fixed_width(GTK_COMBO_BOX(self.unwrap_widget()))) }
    }
}
