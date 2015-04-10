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

//! A choice from multiple check buttons

use std::ptr;

use glib::translate::ToGlibPtr;
use ffi;
use cast::GTK_RADIOBUTTON;

/// A choice from multiple check buttons
struct_Widget!(RadioButton);

impl RadioButton {
    pub fn new() -> Option<RadioButton> {
        let tmp_pointer = unsafe { ffi::gtk_radio_button_new(ptr::null_mut()) };
        check_pointer!(tmp_pointer, RadioButton)
    }

    pub fn new_with_label(label: &str) -> Option<RadioButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_radio_button_new_with_label(ptr::null_mut(),
                                                 label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, RadioButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<RadioButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_radio_button_new_with_mnemonic(ptr::null_mut(),
                                                    mnemonic.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, RadioButton)
    }

    pub fn join(&mut self, group_source: &mut RadioButton) {
        unsafe {
            ffi::gtk_radio_button_join_group(GTK_RADIOBUTTON(self.pointer),
                                             GTK_RADIOBUTTON(group_source.pointer));
        }
    }
}

impl_drop!(RadioButton);
impl_TraitWidget!(RadioButton);

impl ::ContainerTrait for RadioButton {}
impl ::ButtonTrait for RadioButton {}
impl ::BinTrait for RadioButton {}
impl ::ToggleButtonTrait for RadioButton {}

impl_widget_events!(RadioButton);
