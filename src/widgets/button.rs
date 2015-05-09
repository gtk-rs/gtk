// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget that emits a signal when clicked on

use ffi;
use glib::translate::ToGlibPtr;
#[cfg(feature = "gtk_3_10")]
use IconSize;

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
            ffi::gtk_button_new_with_label(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_button_new_with_mnemonic(mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Button)
    }

    #[cfg(feature = "gtk_3_10")]
    pub fn new_from_icon_name(icon_name: &str, size: IconSize) -> Option<Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_button_new_from_icon_name(icon_name.to_glib_none().0, size)
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_button_new_from_stock(stock_id.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Button)
    }
}

impl_drop!(Button);
impl_TraitWidget!(Button);

impl ::ContainerTrait for Button {}
impl ::ButtonTrait for Button {}
impl ::BinTrait for Button {}
