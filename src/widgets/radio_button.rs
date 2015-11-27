// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;

use glib::translate::ToGlibPtr;
use ffi;
use cast::GTK_RADIOBUTTON;

struct_Widget!(RadioButton);

impl RadioButton {
    pub fn new() -> Option<RadioButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_radio_button_new(ptr::null_mut()) };
        check_pointer!(tmp_pointer, RadioButton)
    }

    pub fn new_with_label(label: &str) -> Option<RadioButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_radio_button_new_with_label(ptr::null_mut(),
                                                 label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, RadioButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<RadioButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_radio_button_new_with_mnemonic(ptr::null_mut(),
                                                    mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, RadioButton)
    }

    pub fn join(&self, group_source: &RadioButton) {
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
