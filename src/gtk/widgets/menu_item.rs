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

//! The widget used for item in menus

use gtk::{self, ffi};
use std::ffi::CString;

/// MenuItem — The widget used for item in menus
struct_Widget!(MenuItem);

impl MenuItem {
    pub fn new() -> Option<MenuItem> {
        let tmp_pointer = unsafe { ffi::gtk_menu_item_new() };
        check_pointer!(tmp_pointer, MenuItem)
    }

    pub fn new_with_label(label: &str) -> Option<MenuItem> {
        let tmp_pointer = unsafe {
            let c_str = CString::from_slice(label.as_bytes());
            ffi::gtk_menu_item_new_with_label(c_str)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<MenuItem> {
        let c_str = CString::from_slice(mnemonic.as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_menu_item_new_with_mnemonic(c_str)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }
}

impl_drop!(MenuItem);
impl_TraitWidget!(MenuItem);

impl gtk::ContainerTrait for MenuItem {}
impl gtk::BinTrait for MenuItem {}
impl gtk::MenuItemTrait for MenuItem {}

impl_widget_events!(MenuItem);
