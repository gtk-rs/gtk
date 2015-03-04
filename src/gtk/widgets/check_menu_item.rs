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
use glib::translate::{ToGlibPtr, ToTmp};

/// CheckMenuItem — The widget used for item in menus
struct_Widget!(CheckMenuItem);

impl CheckMenuItem {
    pub fn new() -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe { ffi::gtk_check_menu_item_new() };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }

    pub fn new_with_label(label: &str) -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe {
            let mut tmp_label = label.to_tmp_for_borrow();
            ffi::gtk_check_menu_item_new_with_label(tmp_label.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<CheckMenuItem> {
        let c_str = CString::from_slice(mnemonic.as_bytes());

        let tmp_pointer = unsafe {
            ffi::gtk_check_menu_item_new_with_mnemonic(c_str.as_ptr())
        };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }
}

impl_drop!(CheckMenuItem);
impl_TraitWidget!(CheckMenuItem);

impl gtk::ContainerTrait for CheckMenuItem {}
impl gtk::BinTrait for CheckMenuItem {}
impl gtk::MenuItemTrait for CheckMenuItem {}
impl gtk::CheckMenuItemTrait for CheckMenuItem {}

impl_widget_events!(CheckMenuItem);
