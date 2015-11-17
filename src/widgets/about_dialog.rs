// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::{to_bool, to_gboolean};
use FFIWidget;
use cast::GTK_ABOUT_DIALOG;
use glib::translate::{from_glib_none, FromGlibPtrContainer, ToGlibPtr};

struct_Widget!(AboutDialog);

impl AboutDialog {
    pub fn new() -> Option<AboutDialog> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_about_dialog_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_program_name(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_program_name(&self, name: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(self.unwrap_widget()), name.to_glib_none().0)
        };
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_version(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_version(&self, version: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(self.unwrap_widget()), version.to_glib_none().0)
        };
    }

    pub fn get_copyright(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_copyright(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_copyright(&self, copyright: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(self.unwrap_widget()), copyright.to_glib_none().0)
        };
    }

    pub fn get_comments(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_comments(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_comments(&self, comments: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(self.unwrap_widget()), comments.to_glib_none().0)
        };
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_license(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_license(&self, license: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_license(GTK_ABOUT_DIALOG(self.unwrap_widget()), license.to_glib_none().0)
        };
    }

    pub fn get_wrap_license(&self) -> bool {
        unsafe { to_bool(ffi::gtk_about_dialog_get_wrap_license(GTK_ABOUT_DIALOG(self.unwrap_widget()))) }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) -> () {
        unsafe { ffi::gtk_about_dialog_set_wrap_license(GTK_ABOUT_DIALOG(self.unwrap_widget()), to_gboolean(wrap_license)) }
    }

    pub fn get_license_type(&self) -> ::License {
        unsafe { ffi::gtk_about_dialog_get_license_type(GTK_ABOUT_DIALOG(self.unwrap_widget())) }
    }

    pub fn set_license_type(&self, license_type: ::License) -> () {
        unsafe { ffi::gtk_about_dialog_set_license_type(GTK_ABOUT_DIALOG(self.unwrap_widget()), license_type) }
    }

    pub fn get_website(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_website(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_website(&self, website: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(self.unwrap_widget()), website.to_glib_none().0)
        };
    }

    pub fn get_website_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_website_label(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_website_label(&self, website_label: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(self.unwrap_widget()), website_label.to_glib_none().0)
        };
    }

    pub fn get_authors(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_none(
                ffi::gtk_about_dialog_get_authors(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                authors.to_glib_none().0);
        }
    }

    pub fn get_artists(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_none(
                ffi::gtk_about_dialog_get_artists(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                artists.to_glib_none().0);
        }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_none(
                ffi::gtk_about_dialog_get_documenters(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                documenters.to_glib_none().0);
        }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_translator_credits(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_translator_credits(&self, translator_credits: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                translator_credits.to_glib_none().0)
        };
    }

    /*pub fn get_logo(&self) -> Option<String> {
        let logo = unsafe { ffi::gtk_about_dialog_set_logo(self.pointer)) };

        if logo.is_null() {
            None
        } else {
            Some(unsafe { ::FFIWidget::wrap_widget(logo) })
        }
    }

    pub fn set_logo(&self, logo: Pixbuf) -> () {
        unsafe { ffi::gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(self.unwrap_widget()), GDK_PIXBUF(logo.unwrap_widget())) }
    }*/

    pub fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_logo_icon_name(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_logo_icon_name(&self, logo_icon_name: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                logo_icon_name.to_glib_none().0)
        };
    }

    pub fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                section_name.to_glib_none().0,
                people.to_glib_none().0)
        }
    }

    /*pub fn show(parent: Window, properties: Vec<String>) -> () {
        assert_initialized_main_thread!();
        unsafe { ffi::gtk_show_about_dialog(GTK_WINDOW(parent), first_property_name, ...) }
    }*/
}

impl_drop!(AboutDialog);
impl_TraitWidget!(AboutDialog);

impl ::ContainerTrait for AboutDialog {}
impl ::BinTrait for AboutDialog {}
impl ::WindowTrait for AboutDialog {}
impl ::DialogTrait for AboutDialog {}
