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

//! A widget that emits a signal when clicked on

use gtk::{self, ffi};
use std::ffi::CString;
#[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
use gtk::IconSize;

/**
* Button — A widget that emits a signal when clicked on
*
* # Availables signals :
* * `activate` : Action
* * `clicked`: Action
* * `enter` : Run First
* * `leave` : Run First
* * `pressed` : Run First
* * `released` : Run First
*/
struct_Widget!(Button);


impl Button {
    pub fn new() -> Option<Button> {
        let tmp_pointer = unsafe { ffi::gtk_button_new() };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_with_label(label: &str) -> Option<Button> {
        let tmp_pointer = unsafe {
            let c_str = CString::from_slice(label.as_bytes());

            ffi::gtk_button_new_with_label(c_str.as_ptr())
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Button> {
        let c_str = CString::from_slice(mnemonic.as_bytes());

        let tmp_pointer = unsafe {
                ffi::gtk_button_new_with_mnemonic(c_str.as_ptr())
        };
        check_pointer!(tmp_pointer, Button)
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    pub fn new_from_icon_name(icon_name: &str, size: IconSize) -> Option<Button> {
        let c_str = CString::from_slice(icon_name.as_bytes());

        let tmp_pointer = unsafe {
            ffi::gtk_button_new_from_icon_name(c_str.as_ptr(), size)
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<Button> {
        let c_str = CString::from_slice(stock_id.as_bytes());

        let tmp_pointer = unsafe {
            ffi::gtk_button_new_from_stock(c_str.as_ptr())
        };
        check_pointer!(tmp_pointer, Button)
    }
}

impl_drop!(Button);
impl_TraitWidget!(Button);

impl gtk::ContainerTrait for Button {}
impl gtk::ButtonTrait for Button {}
impl gtk::BinTrait for Button {}

impl_widget_events!(Button);
impl_button_events!(Button);
