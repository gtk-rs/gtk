use std::ptr;

use RadioButton;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

impl RadioButton {
    pub fn new() -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new(ptr::null_mut())).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label(ptr::null_mut(), label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic(ptr::null_mut(), label.to_glib_none().0)).downcast_unchecked()
        }
    }
}
