// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

struct_Widget!(ComboBox);

impl ComboBox {
    pub fn new() -> Option<ComboBox> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new() };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_entry() -> Option<ComboBox> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_entry() };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_model(model: &::TreeModel) -> Option<ComboBox> {
        skip_assert_initialized!();
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_model(model.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_model_and_entry(model: &::TreeModel) -> Option<ComboBox> {
        skip_assert_initialized!();
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_model_and_entry(model.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    /*pub fn new_with_area(area: &::CellArea) -> Option<ComboBox> {
        skip_assert_initialized!();
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area(area.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_area_and_entry(area: &::CellArea) -> Option<ComboBox> {
        skip_assert_initialized!();
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area_and_entry(area.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }*/
}

impl_drop!(ComboBox);
impl_TraitWidget!(ComboBox);

impl ::ContainerTrait for ComboBox {}
impl ::BinTrait for ComboBox {}
impl ::ComboBoxTrait for ComboBox {}
impl ::CellLayoutTrait for ComboBox {}
impl ::CellEditableTrait for ComboBox {}