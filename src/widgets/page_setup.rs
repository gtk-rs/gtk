// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::GTK_PAPER_SIZE;

pub struct PageSetup {
    pointer: *mut ffi::GtkPageSetup
}

impl PageSetup {
    pub fn new() -> Option<PageSetup> {
        let tmp_pointer = unsafe { ffi::gtk_page_setup_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(PageSetup { pointer: tmp_pointer })
        }
    }

    pub fn copy(&self) -> Option<PageSetup> {
        let tmp_pointer = unsafe { ffi::gtk_page_setup_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(PageSetup { pointer: tmp_pointer })
        }
    }

    pub fn get_orientation(&self) -> ::PageOrientation {
        unsafe { ffi::gtk_page_setup_get_orientation(self.pointer) }
    }

    pub fn set_orientation(&self, orientation: ::PageOrientation) {
        unsafe { ffi::gtk_page_setup_set_orientation(self.pointer, orientation) }
    }

    pub fn get_paper_size(&self) -> Option<::PaperSize> {
        let tmp_pointer = unsafe { ffi::gtk_page_setup_get_paper_size(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn set_paper_size(&self, size: &::PaperSize) {
        unsafe { ffi::gtk_page_setup_set_paper_size(self.pointer, GTK_PAPER_SIZE(size.unwrap_widget())) }
    }

    pub fn get_top_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_top_margin(self.pointer, unit) }
    }

    pub fn set_top_margin(&self, margin: f64, unit: ::Unit) {
        unsafe { ffi::gtk_page_setup_set_top_margin(self.pointer, margin, unit) }
    }

    pub fn get_bottom_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_bottom_margin(self.pointer, unit) }
    }

    pub fn set_bottom_margin(&self, margin: f64, unit: ::Unit) {
        unsafe { ffi::gtk_page_setup_set_bottom_margin(self.pointer, margin, unit) }
    }

    pub fn get_left_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_left_margin(self.pointer, unit) }
    }

    pub fn set_left_margin(&self, margin: f64, unit: ::Unit) {
        unsafe { ffi::gtk_page_setup_set_left_margin(self.pointer, margin, unit) }
    }

    pub fn get_right_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_right_margin(self.pointer, unit) }
    }

    pub fn set_right_margin(&self, margin: f64, unit: ::Unit) {
        unsafe { ffi::gtk_page_setup_set_right_margin(self.pointer, margin, unit) }
    }

    pub fn set_paper_size_and_default_margins(&self, size: &::PaperSize) {
        unsafe { ffi::gtk_page_setup_set_paper_size_and_default_margins(self.pointer, GTK_PAPER_SIZE(size.unwrap_widget())) }
    }

    pub fn get_paper_width(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_paper_width(self.pointer, unit) }
    }

    pub fn get_paper_height(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_paper_height(self.pointer, unit) }
    }

    pub fn get_page_width(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_page_width(self.pointer, unit) }
    }

    pub fn get_page_height(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_page_height(self.pointer, unit) }
    }
}

impl_drop!(PageSetup, GTK_PAGE_SETUP);
