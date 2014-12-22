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

//! Create widgets with a discrete toggle button

use gtk::{mod, ffi};

/// CheckButton — Create widgets with a discrete toggle button
struct_Widget!(CheckButton);

impl CheckButton {
    pub fn new() -> Option<CheckButton> {
        let tmp_pointer = unsafe { ffi::gtk_check_button_new() };
        check_pointer!(tmp_pointer, CheckButton)
    }

    pub fn new_with_label(label: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_check_button_new_with_label(c_str)
            })
        };
        check_pointer!(tmp_pointer, CheckButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            mnemonic.with_c_str(|c_str| {
                ffi::gtk_check_button_new_with_mnemonic(c_str)
            })
        };
        check_pointer!(tmp_pointer, CheckButton)
    }

}

impl_drop!(CheckButton);
impl_TraitWidget!(CheckButton);

impl gtk::ContainerTrait for CheckButton {}
impl gtk::ButtonTrait for CheckButton {}
impl gtk::ToggleButtonTrait for CheckButton {}

impl_widget_events!(CheckButton);
