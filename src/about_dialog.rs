// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use AboutDialog;

impl AboutDialog {
    pub fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(self.to_glib_none().0,
                section_name.to_glib_none().0, people.to_glib_none().0)
        }
    }

    pub fn get_artists(&self) -> Vec<String> {
        unsafe { Vec::from_glib_none(ffi::gtk_about_dialog_get_artists(self.to_glib_none().0)) }
    }

    pub fn get_authors(&self) -> Vec<String> {
        unsafe { Vec::from_glib_none(ffi::gtk_about_dialog_get_authors(self.to_glib_none().0)) }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        unsafe { Vec::from_glib_none(ffi::gtk_about_dialog_get_documenters(self.to_glib_none().0)) }
    }

    pub fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(self.to_glib_none().0, artists.to_glib_none().0);
        }
    }

    pub fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(self.to_glib_none().0, authors.to_glib_none().0);
        }
    }

    pub fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(self.to_glib_none().0,
                documenters.to_glib_none().0);
        }
    }
}
