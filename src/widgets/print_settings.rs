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

use ffi;
use glib::{to_bool, to_gboolean};
use FFIWidget;
use cast::{GTK_PRINT_SETTINGS, GTK_PAPER_SIZE};
use glib::translate::{FromGlibPtr, ToGlibPtr};

struct_Widget!(PrintSettings);

impl PrintSettings {
    pub fn new() -> Option<PrintSettings> {
        let tmp_pointer = unsafe { ffi::gtk_print_settings_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<PrintSettings> {
        let tmp_pointer = unsafe { ffi::gtk_print_settings_copy(GTK_PRINT_SETTINGS(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        unsafe {
            to_bool(
                ffi::gtk_print_settings_has_key(GTK_PRINT_SETTINGS(self.unwrap_widget()),
                                                key.borrow_to_glib().0))
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_print_settings_get(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0))
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        unsafe {
            ffi::gtk_print_settings_set(GTK_PRINT_SETTINGS(
                self.unwrap_widget()),
                key.borrow_to_glib().0,
                value.borrow_to_glib().0)
        }
    }

    pub fn unset(&self, key: &str) {
        unsafe {
            ffi::gtk_print_settings_unset(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0)
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_print_settings_get_bool(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0))
        }
    }

    pub fn set_bool(&self, key: &str, value: bool) {
        unsafe {
            ffi::gtk_print_settings_set_bool(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, to_gboolean(value))
        }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0)
        }
    }

    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            ffi::gtk_print_settings_set_double(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, value)
        }
    }

    pub fn get_double_with_default(&self, key: &str, def: f64) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double_with_default(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, def)
        }
    }

    pub fn get_length(&self, key: &str, unit: ::Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_length(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, unit)
        }
    }

    pub fn set_length(&self, key: &str, value: f64, unit: ::Unit) {
        unsafe {
            ffi::gtk_print_settings_set_length(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, value, unit)
        }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0)
        }
    }

    pub fn set_int(&self, key: &str, value: i32) {
        unsafe {
            ffi::gtk_print_settings_set_int(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, value)
        }
    }

    pub fn get_int_with_default(&self, key: &str, def: i32) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int_with_default(GTK_PRINT_SETTINGS(self.unwrap_widget()), key.borrow_to_glib().0, def)
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
            ffi::gtk_print_settings_set_printer(GTK_PRINT_SETTINGS(self.unwrap_widget()), printer.borrow_to_glib().0)
        }
    }

    pub fn get_orientation(&self) -> ::PageOrientation {
        unsafe { ffi::gtk_print_settings_get_orientation(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_orientation(&self, orientation: ::PageOrientation) {
        unsafe { ffi::gtk_print_settings_set_orientation(GTK_PRINT_SETTINGS(self.unwrap_widget()), orientation) }
    }

    pub fn get_paper_size(&self) -> Option<::PaperSize> {
        let tmp = unsafe { ffi::gtk_print_settings_get_paper_size(GTK_PRINT_SETTINGS(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp as *mut ffi::C_GtkWidget))
        }
    }

    pub fn set_paper_size(&self, paper_size: &::PaperSize) {
        unsafe { ffi::gtk_print_settings_set_paper_size(GTK_PRINT_SETTINGS(self.unwrap_widget()), GTK_PAPER_SIZE(paper_size.unwrap_widget())) }
    }

    pub fn get_paper_width(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_print_settings_get_paper_width(GTK_PRINT_SETTINGS(self.unwrap_widget()), unit) }
    }

    pub fn set_paper_width(&self, width: f64, unit: ::Unit) {
        unsafe { ffi::gtk_print_settings_set_paper_width(GTK_PRINT_SETTINGS(self.unwrap_widget()), width, unit) }
    }

    pub fn get_paper_height(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_print_settings_get_paper_height(GTK_PRINT_SETTINGS(self.unwrap_widget()), unit) }
    }

    pub fn set_paper_height(&self, height: f64, unit: ::Unit) {
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

    pub fn get_number_up(&self) -> ::NumberUpLayout {
        unsafe { ffi::gtk_print_settings_get_number_up(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_number_up(&self, number_up: ::NumberUpLayout) {
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

    pub fn get_print_pages(&self) -> ::PrintPages {
        unsafe { ffi::gtk_print_settings_get_print_pages(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_print_pages(&self, pages: ::PrintPages) {
        unsafe { ffi::gtk_print_settings_set_print_pages(GTK_PRINT_SETTINGS(self.unwrap_widget()), pages) }
    }

    pub fn get_page_set(&self) -> ::PageSet {
        unsafe { ffi::gtk_print_settings_get_page_set(GTK_PRINT_SETTINGS(self.unwrap_widget())) }
    }

    pub fn set_page_set(&self, page_set: ::PageSet) {
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
            ffi::gtk_print_settings_set_default_source(GTK_PRINT_SETTINGS(self.unwrap_widget()), default_source.borrow_to_glib().0)
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
            ffi::gtk_print_settings_set_media_type(GTK_PRINT_SETTINGS(self.unwrap_widget()), media_type.borrow_to_glib().0)
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
            ffi::gtk_print_settings_set_dither(GTK_PRINT_SETTINGS(self.unwrap_widget()), dither.borrow_to_glib().0)
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
            ffi::gtk_print_settings_set_finishings(GTK_PRINT_SETTINGS(self.unwrap_widget()), finishings.borrow_to_glib().0)
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
            ffi::gtk_print_settings_set_output_bin(GTK_PRINT_SETTINGS(self.unwrap_widget()), output_bin.borrow_to_glib().0)
        }
    }
}

impl_drop!(PrintSettings);
impl_TraitWidget!(PrintSettings);

impl_widget_events!(PrintSettings);
