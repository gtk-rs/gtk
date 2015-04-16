// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::{to_bool, to_gboolean};
use FFIWidget;
use cast::{GTK_PAPER_SIZE};
use glib;

// FIXME: PaperSize is not a widget nor a GObject -> GBoxed
struct_Widget!(PaperSize);

impl PaperSize {
    pub fn new(name: &str) -> Option<PaperSize> {
        let tmp_pointer = unsafe {
            ffi::gtk_paper_size_new(name.borrow_to_glib().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn new_from_ppd(ppd_name: &str, ppd_display_name: &str, width: f64, height: f64) -> Option<PaperSize> {
        let tmp_pointer = unsafe {
            ffi::gtk_paper_size_new_from_ppd(ppd_name.borrow_to_glib().0,
                                             ppd_display_name.borrow_to_glib().0,
                                             width, height)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn new_custom(name: &str, display_name: &str, width: f64, height: f64, unit: ::Unit) -> Option<PaperSize> {
        let tmp_pointer = unsafe {
            ffi::gtk_paper_size_new_custom(name.borrow_to_glib().0,
                                           display_name.borrow_to_glib().0,
                                           width, height, unit)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<PaperSize> {
        let tmp_pointer = unsafe { ffi::gtk_paper_size_copy(GTK_PAPER_SIZE(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn is_equal(&self, other: &PaperSize) -> bool {
        unsafe { to_bool(ffi::gtk_paper_size_is_equal(GTK_PAPER_SIZE(self.unwrap_widget()), GTK_PAPER_SIZE(other.unwrap_widget()))) }
    }

    pub fn get_paper_sizes(include_custom: bool) -> glib::List<Box<PaperSize>> {
        let tmp = unsafe { ffi::gtk_paper_size_get_paper_sizes(to_gboolean(include_custom))
        };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<PaperSize>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(::FFIWidget::wrap_widget(*it)));
            }
            tmp_vec
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_paper_size_get_name(GTK_PAPER_SIZE(self.unwrap_widget())))
        }
    }

    pub fn get_display_name_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_paper_size_get_display_name(GTK_PAPER_SIZE(self.unwrap_widget())))
        }
    }

    pub fn get_ppd_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_paper_size_get_ppd_name(GTK_PAPER_SIZE(self.unwrap_widget())))
        }
    }

    pub fn get_width(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_width(GTK_PAPER_SIZE(self.unwrap_widget()), unit) }
    }

    pub fn get_height(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_height(GTK_PAPER_SIZE(self.unwrap_widget()), unit) }
    }

    pub fn is_custom(&self) -> bool {
        unsafe { to_bool(ffi::gtk_paper_size_is_custom(GTK_PAPER_SIZE(self.unwrap_widget()))) }
    }

    pub fn set_size(&self, width: f64, height: f64, unit: ::Unit) {
        unsafe { ffi::gtk_paper_size_set_size(GTK_PAPER_SIZE(self.unwrap_widget()), width, height, unit) }
    }

    pub fn get_default_top_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_top_margin(GTK_PAPER_SIZE(self.unwrap_widget()), unit) }
    }

    pub fn get_default_bottom_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_bottom_margin(GTK_PAPER_SIZE(self.unwrap_widget()), unit) }
    }

    pub fn get_default_left_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_left_margin(GTK_PAPER_SIZE(self.unwrap_widget()), unit) }
    }

    pub fn get_default_right_margin(&self, unit: ::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_left_margin(GTK_PAPER_SIZE(self.unwrap_widget()), unit) }
    }

    pub fn get_default() -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_paper_size_get_default())
        }
    }
}

impl Drop for PaperSize {
    fn drop(&mut self) {
        unsafe {
            ffi::gtk_paper_size_free(GTK_PAPER_SIZE(self.unwrap_widget()));
            ::glib::ffi::g_object_unref(self.pointer as *mut ::glib::ffi::C_GObject);
        }
    }
}

impl Clone for PaperSize {
    fn clone(&self) -> PaperSize {
        let pointer = unsafe {
            ::glib::ffi::g_object_ref(self.pointer as *mut ::glib::ffi::C_GObject)
        };

        PaperSize {
            pointer: pointer as *mut ffi::C_GtkWidget
        }
    }
}

impl_TraitWidget!(PaperSize);

impl_widget_events!(PaperSize);
