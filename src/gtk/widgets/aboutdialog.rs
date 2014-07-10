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
use gtk::traits;
use std::str;
use gtk::cast::GTK_ABOUT_DIALOG;
use gtk::enums;

struct_Widget!(AboutDialog)

impl AboutDialog {
    pub fn new() -> Option<AboutDialog> {
        let tmp_pointer = unsafe { ffi::gtk_about_dialog_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        let name = unsafe { ffi::gtk_about_dialog_get_program_name(GTK_ABOUT_DIALOG(self.get_widget())) };

        if name.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(name) })
        }
    }

    pub fn set_program_name(&self, name: &str) -> () {
        unsafe {
            name.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_version(&self) -> Option<String> {
        let version = unsafe { ffi::gtk_about_dialog_get_version(GTK_ABOUT_DIALOG(self.get_widget())) };

        if version.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(version) })
        }
    }

    pub fn set_version(&self, version: &str) -> () {
        unsafe {
            version.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_copyright(&self) -> Option<String> {
        let copyright = unsafe { ffi::gtk_about_dialog_get_copyright(GTK_ABOUT_DIALOG(self.get_widget())) };

        if copyright.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(copyright) })
        }
    }

    pub fn set_copyright(&self, copyright: &str) -> () {
        unsafe {
            copyright.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_comments(&self) -> Option<String> {
        let comments = unsafe { ffi::gtk_about_dialog_get_comments(GTK_ABOUT_DIALOG(self.get_widget())) };

        if comments.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(comments) })
        }
    }

    pub fn set_comments(&self, comments: &str) -> () {
        unsafe {
            comments.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_license(&self) -> Option<String> {
        let license = unsafe { ffi::gtk_about_dialog_get_license(GTK_ABOUT_DIALOG(self.get_widget())) };

        if license.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(license) })
        }
    }

    pub fn set_license(&self, license: &str) -> () {
        unsafe {
            license.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_license(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_wrap_license(&self) -> bool {
        match unsafe { ffi::gtk_about_dialog_get_wrap_license(GTK_ABOUT_DIALOG(self.get_widget())) } {
            ffi::Gfalse => false,
            _ => true
        }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) -> () {
        unsafe { ffi::gtk_about_dialog_set_wrap_license(GTK_ABOUT_DIALOG(self.get_widget()), match wrap_license {
            true => ffi::Gtrue,
            _ => ffi::Gfalse
        }) }
    }

    pub fn get_license_type(&self) -> enums::License {
        unsafe { ffi::gtk_about_dialog_get_license_type(GTK_ABOUT_DIALOG(self.get_widget())) }
    }

    pub fn set_license_type(&self, license_type: enums::License) -> () {
        unsafe { ffi::gtk_about_dialog_set_license_type(GTK_ABOUT_DIALOG(self.get_widget()), license_type) }
    }

    pub fn get_website(&self) -> Option<String> {
        let website = unsafe { ffi::gtk_about_dialog_get_website(GTK_ABOUT_DIALOG(self.get_widget())) };

        if website.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(website) })
        }
    }

    pub fn set_website(&self, website: &str) -> () {
        unsafe {
            website.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_website_label(&self) -> Option<String> {
        let website_label = unsafe { ffi::gtk_about_dialog_get_website_label(GTK_ABOUT_DIALOG(self.get_widget())) };

        if website_label.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(website_label) })
        }
    }

    pub fn set_website_label(&self, website_label: &str) -> () {
        unsafe {
            website_label.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_authors(&self) -> Vec<String> {
        let authors = unsafe { ffi::gtk_about_dialog_get_authors(GTK_ABOUT_DIALOG(self.get_widget())) };
        let mut ret = Vec::new();

        if !authors.is_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = authors.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(str::raw::from_c_str(*tmp));
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn set_authors(&self, authors: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in authors.iter() {
            tmp.as_slice().with_c_str(|c_str|{tmp_vec.push(c_str)});
        }
        unsafe { ffi::gtk_about_dialog_set_authors(GTK_ABOUT_DIALOG(self.get_widget()), tmp_vec.as_slice().as_ptr()) }
    }

    pub fn get_artists(&self) -> Vec<String> {
        let artists = unsafe { ffi::gtk_about_dialog_get_artists(GTK_ABOUT_DIALOG(self.get_widget())) };
        let mut ret = Vec::new();

        if !artists.is_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = artists.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(str::raw::from_c_str(*tmp));
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn set_artists(&self, artists: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in artists.iter() {
            tmp.as_slice().with_c_str(|c_str|{tmp_vec.push(c_str)});
        }
        unsafe { ffi::gtk_about_dialog_set_artists(GTK_ABOUT_DIALOG(self.get_widget()), tmp_vec.as_slice().as_ptr()) }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        let documenters = unsafe { ffi::gtk_about_dialog_get_documenters(GTK_ABOUT_DIALOG(self.get_widget())) };
        let mut ret = Vec::new();

        if !documenters.is_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = documenters.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(str::raw::from_c_str(*tmp));
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn set_documenters(&self, documenters: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in documenters.iter() {
            tmp.as_slice().with_c_str(|c_str|{tmp_vec.push(c_str)});
        }
        unsafe { ffi::gtk_about_dialog_set_documenters(GTK_ABOUT_DIALOG(self.get_widget()), tmp_vec.as_slice().as_ptr()) }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        let translator_credits = unsafe { ffi::gtk_about_dialog_get_translator_credits(GTK_ABOUT_DIALOG(self.get_widget())) };

        if translator_credits.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(translator_credits) })
        }
    }

    pub fn set_translator_credits(&self, translator_credits: &str) -> () {
        unsafe {
            translator_credits.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_translator_credits(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    /*pub fn get_logo(&self) -> Option<String> {
        let logo = unsafe { ffi::gtk_about_dialog_set_logo(self.pointer)) };

        if logo.is_null() {
            None
        } else {
            Some(unsafe { ffi::FFIWidget::wrap(logo) })
        }
    }

    pub fn set_logo(&self, logo: Pixbuf) -> () {
        unsafe { ffi::gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(self.get_widget()), GDK_PIXBUF(logo.get_widget())) }
    }*/

    pub fn get_logo_icon_name(&self) -> Option<String> {
        let logo_icon_name = unsafe { ffi::gtk_about_dialog_get_logo_icon_name(GTK_ABOUT_DIALOG(self.get_widget())) };

        if logo_icon_name.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(logo_icon_name) })
        }
    }

    pub fn set_logo_icon_name(&self, logo_icon_name: &str) -> () {
        unsafe {
            logo_icon_name.with_c_str(|c_str| {
                ffi::gtk_about_dialog_set_logo_icon_name(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn add_credit_section(&self, section_name: &str, people: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in people.iter() {
            tmp.as_slice().with_c_str(|c_str|{tmp_vec.push(c_str)});
        }
        unsafe {
            section_name.with_c_str(|c_str| {
                ffi::gtk_about_dialog_add_credit_section(GTK_ABOUT_DIALOG(self.get_widget()), c_str, tmp_vec.as_slice().as_ptr())
            })
        }
    }

    /*pub fn show(parent: Window, properties: Vec<String>) -> () {
        unsafe { ffi::gtk_show_about_dialog(GTK_WINDOW(parent), first_property_name, ...) }
    }*/
}

impl_drop!(AboutDialog)
impl_TraitWidget!(AboutDialog)

impl traits::Container for AboutDialog {}
impl traits::Bin for AboutDialog {}
impl traits::Window for AboutDialog {}
impl traits::Dialog for AboutDialog {}