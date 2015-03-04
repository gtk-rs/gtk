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

use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::FFIWidget;
use gtk::cast::{GTK_PRINT_SETTINGS, GTK_PAPER_SIZE};
use std::ffi::CString;
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};

struct_Widget!(PrintSettings);

impl PrintSettings {
    pub fn new() -> Option<PrintSettings> {
        let tmp_pointer = unsafe { ffi::gtk_print_settings_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<PrintSettings> {
        let tmp_pointer = unsafe { ffi::gtk_print_settings_copy(GTK_PRINT_SETTINGS(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        let c_str = CString::from_slice(key.as_bytes());

        unsafe {
            to_bool(ffi::gtk_print_settings_has_key(GTK_PRINT_SETTINGS(self.unwrap_widget()), c_str.as_ptr()))
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr()))
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        unsafe {
            let c_str = CString::from_slice(key.as_bytes());
            let mut tmp_value = value.to_tmp_for_borrow();
            ffi::gtk_print_settings_set(GTK_PRINT_SETTINGS(self.unwrap_widget()), c_str.as_ptr(), tmp_value.to_glib_ptr())
        }
    }

    pub fn unset(&self, key: &str) {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_unset(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr())
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            to_bool(ffi::gtk_print_settings_get_bool(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr()))
        }
    }

    pub fn set_bool(&self, key: &str, value: bool) {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_bool(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), to_gboolean(value))
        }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_get_double(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr())
        }
    }

    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_double(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), value)
        }
    }

    pub fn get_double_with_default(&self, key: &str, def: f64) -> f64 {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_get_double_with_default(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), def)
        }
    }

    pub fn get_length(&self, key: &str, unit: gtk::Unit) -> f64 {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_get_length(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), unit)
        }
    }

    pub fn set_length(&self, key: &str, value: f64, unit: gtk::Unit) {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_length(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), value, unit)
        }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_get_int(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr())
        }
    }

    pub fn set_int(&self, key: &str, value: i32) {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_int(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), value)
        }
    }

    pub fn get_int_with_default(&self, key: &str, def: i32) -> i32 {
        unsafe {
            let mut tmp_key = key.to_tmp_for_borrow();
            ffi::gtk_print_settings_get_int_with_default(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_key.to_glib_ptr(), def)
        }
    }

    pub fn get_printer(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get_printer(GTK_PRINT_SETTINGS(self.unwrap_widget())))
        }
    }

    pub fn set_printer(&self, printer: &str) {
        unsafe {
            let mut tmp_printer = printer.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_printer(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_printer.to_glib_ptr())
        }
    }

    pub fn get_orientation(&self) -> gtk::PageOrientation {
        unsafe { ffi::gtk_print_settings_get_orientation(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_orientation(&self, orientation: gtk::PageOrientation) {
        unsafe { ffi::gtk_print_settings_set_orientation(GTK_PRINT_SETTINGS(self.unwrap_widget()), orientation) }
    }

    pub fn get_paper_size(&self) -> Option<gtk::PaperSize> {
        let tmp = unsafe { ffi::gtk_print_settings_get_paper_size(GTK_PRINT_SETTINGS(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp as *mut ffi::C_GtkWidget))
        }
    }

    pub fn set_paper_size(&self, paper_size: &gtk::PaperSize) {
        unsafe { ffi::gtk_print_settings_set_paper_size(GTK_PRINT_SETTINGS(self.unwrap_widget()), GTK_PAPER_SIZE(paper_size.unwrap_widget())) }
    }

    pub fn get_paper_width(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_print_settings_get_paper_width(GTK_PRINT_SETTINGS(self.unwrap_widget()), unit) }
    }

    pub fn set_paper_width(&self, width: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_print_settings_set_paper_width(GTK_PRINT_SETTINGS(self.unwrap_widget()), width, unit) }
    }

    pub fn get_paper_height(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_print_settings_get_paper_height(GTK_PRINT_SETTINGS(self.unwrap_widget()), unit) }
    }

    pub fn set_paper_height(&self, height: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_print_settings_set_paper_height(GTK_PRINT_SETTINGS(self.unwrap_widget()), height, unit) }
    }

    pub fn get_use_color(&self) -> bool {
        unsafe { to_bool(ffi::gtk_print_settings_get_use_color(GTK_PRINT_SETTINGS(self.unwrap_widget()))) }
    }

    pub fn set_use_color(&self, use_color: bool) {
        unsafe { ffi::gtk_print_settings_set_use_color(GTK_PRINT_SETTINGS(self.unwrap_widget()), to_gboolean(use_color)) }
    }

    pub fn get_collate(&self) -> bool {
        unsafe { to_bool(ffi::gtk_print_settings_get_collate(GTK_PRINT_SETTINGS(self.unwrap_widget()))) }
    }

    pub fn set_collate(&self, collate: bool) {
        unsafe { ffi::gtk_print_settings_set_collate(GTK_PRINT_SETTINGS(self.unwrap_widget()), to_gboolean(collate)) }
    }

    pub fn get_reverse(&self) -> bool {
        unsafe { to_bool(ffi::gtk_print_settings_get_reverse(GTK_PRINT_SETTINGS(self.unwrap_widget()))) }
    }

    pub fn set_reverse(&self, reverse: bool) {
        unsafe { ffi::gtk_print_settings_set_reverse(GTK_PRINT_SETTINGS(self.unwrap_widget()),
                                                     to_gboolean(reverse))
        }
    }

    pub fn get_n_copies(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_n_copies(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_n_copies(&self, num_copies: i32) {
        unsafe { ffi::gtk_print_settings_set_n_copies(GTK_PRINT_SETTINGS(self.unwrap_widget()), num_copies) }
    }

    pub fn get_number_up(&self) -> gtk::NumberUpLayout {
        unsafe { ffi::gtk_print_settings_get_number_up(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_number_up(&self, number_up: gtk::NumberUpLayout) {
        unsafe { ffi::gtk_print_settings_set_number_up(GTK_PRINT_SETTINGS(self.unwrap_widget()), number_up) }
    }

    pub fn get_resolution(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_resolution(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_resolution(&self, resolution: i32) {
        unsafe { ffi::gtk_print_settings_set_resolution(GTK_PRINT_SETTINGS(self.unwrap_widget()), resolution) }
    }

    pub fn set_resolution_xy(&self, resolution_x: i32, resolution_y: i32) {
        unsafe { ffi::gtk_print_settings_set_resolution_xy(GTK_PRINT_SETTINGS(self.unwrap_widget()), resolution_x, resolution_y) }
    }

    pub fn get_resolution_x(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_resolution_x(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn get_resolution_y(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_resolution_y(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn get_printer_lpi(&self) -> f64 {
        unsafe { ffi::gtk_print_settings_get_printer_lpi(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_printer_lpi(&self, lpi: f64) {
        unsafe { ffi::gtk_print_settings_set_printer_lpi(GTK_PRINT_SETTINGS(self.unwrap_widget()), lpi) }
    }

    pub fn get_scale(&self) -> f64 {
        unsafe { ffi::gtk_print_settings_get_scale(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_scale(&self, scale: f64) {
        unsafe { ffi::gtk_print_settings_set_scale(GTK_PRINT_SETTINGS(self.unwrap_widget()), scale) }
    }

    pub fn get_print_pages(&self) -> gtk::PrintPages {
        unsafe { ffi::gtk_print_settings_get_print_pages(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_print_pages(&self, pages: gtk::PrintPages) {
        unsafe { ffi::gtk_print_settings_set_print_pages(GTK_PRINT_SETTINGS(self.unwrap_widget()), pages) }
    }

    pub fn get_page_set(&self) -> gtk::PageSet {
        unsafe { ffi::gtk_print_settings_get_page_set(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_page_set(&self, page_set: gtk::PageSet) {
        unsafe { ffi::gtk_print_settings_set_page_set(GTK_PRINT_SETTINGS(self.unwrap_widget()), page_set) }
    }

    pub fn get_default_source(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get_default_source(GTK_PRINT_SETTINGS(self.unwrap_widget())))
        }
    }

    pub fn set_default_source(&self, default_source: &str) {
        unsafe {
            let mut tmp_default_source = default_source.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_default_source(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_default_source.to_glib_ptr())
        }
    }

    pub fn get_media_type(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get_media_type(GTK_PRINT_SETTINGS(self.unwrap_widget())))
        }
    }

    pub fn set_media_type(&self, media_type: &str) {
        unsafe {
            let mut tmp_media_type = media_type.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_media_type(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_media_type.to_glib_ptr())
        }
    }

    pub fn get_dither(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get_dither(GTK_PRINT_SETTINGS(self.unwrap_widget())))
        }
    }

    pub fn set_dither(&self, dither: &str) {
        unsafe {
            let mut tmp_dither = dither.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_dither(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_dither.to_glib_ptr())
        }
    }

    pub fn get_finishings(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get_finishings(GTK_PRINT_SETTINGS(self.unwrap_widget())))
        }
    }

    pub fn set_finishings(&self, finishings: &str) {
        unsafe {
            let mut tmp_finishings = finishings.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_finishings(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_finishings.to_glib_ptr())
        }
    }

    pub fn get_output_bin(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get_output_bin(GTK_PRINT_SETTINGS(self.unwrap_widget())))
        }
    }

    pub fn set_output_bin(&self, output_bin: &str) {
        unsafe {
            let mut tmp_output_bin = output_bin.to_tmp_for_borrow();
            ffi::gtk_print_settings_set_output_bin(GTK_PRINT_SETTINGS(self.unwrap_widget()), tmp_output_bin.to_glib_ptr())
        }
    }
}

impl_drop!(PrintSettings);
impl_TraitWidget!(PrintSettings);

impl_widget_events!(PrintSettings);
