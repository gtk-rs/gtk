// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Create widgets with a discrete toggle button

use glib::translate::ToGlibPtr;
use ffi;

/// CheckButton — Create widgets with a discrete toggle button
struct_Widget!(CheckButton);

impl CheckButton {
    pub fn new() -> Option<CheckButton> {
        let tmp_pointer = unsafe { ffi::gtk_check_button_new() };
        check_pointer!(tmp_pointer, CheckButton)
    }

    pub fn new_with_label(label: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_button_new_with_label(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, CheckButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_button_new_with_mnemonic(mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, CheckButton)
    }
}

impl_drop!(CheckButton);
impl_TraitWidget!(CheckButton);

impl ::ContainerTrait for CheckButton {}
impl ::ButtonTrait for CheckButton {}
impl ::ToggleButtonTrait for CheckButton {}
