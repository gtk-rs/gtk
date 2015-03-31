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
use glib::translate::ToGlibPtr;
#[cfg(feature = "GTK_3_10")]
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
            ffi::gtk_button_new_with_label(label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_button_new_with_mnemonic(mnemonic.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Button)
    }

    #[cfg(feature = "GTK_3_10")]
    pub fn new_from_icon_name(icon_name: &str, size: IconSize) -> Option<Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_button_new_from_icon_name(icon_name.borrow_to_glib().0, size)
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_button_new_from_stock(stock_id.borrow_to_glib().0)
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
