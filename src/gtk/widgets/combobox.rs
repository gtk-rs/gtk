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

//! GtkComboBox — A widget used to choose from a list of items

use gtk::ffi;
use gtk::traits;
use gtk;

struct_Widget!(ComboBox)

impl ComboBox {
    pub fn new() -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new() };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_entry() -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_entry() };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_model(model: &gtk::TreeModel) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_model(model.get_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_model_and_entry(model: &gtk::TreeModel) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_model_and_entry(model.get_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    /*pub fn new_with_area(area: &gtk::CellArea) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area(area.get_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    pub fn new_with_area_and_entry(area: &gtk::CellArea) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area_and_entry(area.get_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }*/
}

impl_drop!(ComboBox)
impl_TraitWidget!(ComboBox)

impl traits::Container for ComboBox {}
impl traits::Bin for ComboBox {}
impl traits::ComboBox for ComboBox {}

impl_widget_events!(ComboBox)

