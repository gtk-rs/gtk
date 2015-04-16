// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::{GTK_WINDOW, GTK_PAGE_SETUP_UNIX_DIALOG, GTK_PAGE_SETUP, GTK_PRINT_SETTINGS};
use std::str;

struct_Widget!(PageSetupUnixDialog);

impl PageSetupUnixDialog {
    pub fn new(title: &str, parent: Option<::Window>) -> Option<PageSetupUnixDialog> {
        let tmp_pointer = unsafe {
            title.with_c_str(|c_str|{
                ffi::gtk_page_setup_unix_dialog_new(match parent {
                    Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                    None => ::std::ptr::null_mut()
                })
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_page_setup(&self, page_setup: &::PageSetup) {
        unsafe { ffi::gtk_page_setup_unix_dialog_set_page_setup(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget()), GTK_PAGE_SETUP(page_setup.unwrap_widget())) }
    }

    pub fn get_page_setup(&self) -> Option<PageSetup> {
        let tmp = unsafe { ffi::gtk_page_setup_unix_dialog_get_page_setup(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_print_settings(&self, print_settings: &::PrintSettings) {
        unsafe { ffi::gtk_page_setup_unix_dialog_set_print_settings(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget()), GTK_PRINT_SETTINGS(print_settings.unwrap_widget())) }
    }

    pub fn get_print_settings(&self) -> Option<PrintSettings> {
        let tmp = unsafe { ffi::gtk_page_setup_unix_dialog_get_print_settings(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(PageSetupUnixDialog);
impl_TraitWidget!(PageSetupUnixDialog);

impl ::ContainerTrait for PageSetupUnixDialog {}
impl ::BinTrait for PageSetupUnixDialog {}
impl ::WindowTrait for PageSetupUnixDialog {}
impl ::DialogTrait for PageSetupUnixDialog {}

impl_widget_events!(PageSetupUnixDialog);