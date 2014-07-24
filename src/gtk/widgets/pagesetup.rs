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

use gtk::ffi;
use gtk::ffi::FFIWidget;
use std::str;
use gtk::cast::{GTK_PAGE_SETUP, GTK_PAPER_SIZE};

struct_Widget!(PageSetup)

impl PageSetup {
    pub fn new() -> Option<PageSetup> {
        let tmp_pointer = unsafe { ffi::gtk_page_setup_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<PageSetup> {
        let tmp_pointer = unsafe { ffi::gtk_page_setup_copy(GTK_PAGE_SETUP(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn get_orientation(&self) -> gtk::PageOrientation {
        unsafe { ffi::gtk_page_setup_get_orientation(GTK_PAGE_SETUP(self.get_widget())) }
    }

    pub fn set_orientation(&self, orientation: gtk::PageOrientation) {
        unsafe { ffi::gtk_page_setup_set_orientation(GTK_PAGE_SETUP(self.get_widget()), orientation) }
    }

    pub fn get_paper_size(&self) -> gtk::PaperSize {
        let tmp_pointer = unsafe { ffi::gtk_page_setup_get_paper_size(GTK_PAGE_SETUP(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn set_paper_size(&self, size: &gtk::PaperSize) {
        unsafe { ffi::gtk_page_setup_set_paper_size(GTK_PAGE_SETUP(self.get_widget()), GTK_PAPER_SIZE(size.get_widget())) }
    }

    pub fn get_top_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_top_margin(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn set_top_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_page_setup_set_top_margin(GTK_PAGE_SETUP(self.get_widget()), margin, unit) }
    }

    pub fn get_bottom_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_bottom_margin(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_page_setup_set_bottom_margin(GTK_PAGE_SETUP(self.get_widget()), margin, unit) }
    }

    pub fn get_left_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_left_margin(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn set_left_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_page_setup_set_left_margin(GTK_PAGE_SETUP(self.get_widget()), margin, unit) }
    }

    pub fn get_right_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_right_margin(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn set_right_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_page_setup_set_right_margin(GTK_PAGE_SETUP(self.get_widget()), margin, unit) }
    }

    pub fn set_paper_size_and_default_margins(&self, size: &PaperSize) {
        unsafe { ffi::gtk_page_setup_set_paper_size_and_default_margins(GTK_PAGE_SETUP(self.get_widget()), GTK_PAPER_SIZE(size.get_widget())) }
    }

    pub fn get_paper_width(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_paper_width(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn get_paper_height(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_paper_height(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn get_page_width(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_page_width(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }

    pub fn get_page_height(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_page_height(GTK_PAGE_SETUP(self.get_widget()), unit) }
    }
}

impl_drop!(PageSetup)
impl_TraitWidget!(PageSetup)