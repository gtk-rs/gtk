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
use gtk::cast::GTK_ABOUT_DIALOG;
use std::ffi::CString;
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};

struct_Widget!(AboutDialog);

impl AboutDialog {
    pub fn new() -> Option<AboutDialog> {
        let tmp_pointer = unsafe { ffi::gtk_about_dialog_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_program_name(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_program_name(&self, name: &str) -> () {
        unsafe {
            let mut tmp_name = name.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_name.to_glib_ptr())
        };
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_version(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_version(&self, version: &str) -> () {
        unsafe {
            let mut tmp_version = version.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_version.to_glib_ptr())
        };
    }

    pub fn get_copyright(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_copyright(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_copyright(&self, copyright: &str) -> () {
        unsafe {
            let mut tmp_copyright = copyright.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_copyright.to_glib_ptr())
        };
    }

    pub fn get_comments(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_comments(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_comments(&self, comments: &str) -> () {
        unsafe {
            let mut tmp_comments = comments.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_comments.to_glib_ptr())
        };
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_license(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_license(&self, license: &str) -> () {
        unsafe {
            let mut tmp_license = license.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_license(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_license.to_glib_ptr())
        };
    }

    pub fn get_wrap_license(&self) -> bool {
        unsafe { to_bool(ffi::gtk_about_dialog_get_wrap_license(GTK_ABOUT_DIALOG(self.unwrap_widget()))) }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) -> () {
        unsafe { ffi::gtk_about_dialog_set_wrap_license(GTK_ABOUT_DIALOG(self.unwrap_widget()), to_gboolean(wrap_license)) }
    }

    pub fn get_license_type(&self) -> gtk::License {
        unsafe { ffi::gtk_about_dialog_get_license_type(GTK_ABOUT_DIALOG(self.unwrap_widget())) }
    }

    pub fn set_license_type(&self, license_type: gtk::License) -> () {
        unsafe { ffi::gtk_about_dialog_set_license_type(GTK_ABOUT_DIALOG(self.unwrap_widget()), license_type) }
    }

    pub fn get_website(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_website(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_website(&self, website: &str) -> () {
        unsafe {
            let mut tmp_website = website.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_website.to_glib_ptr())
        };
    }

    pub fn get_website_label(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_website_label(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_website_label(&self, website_label: &str) -> () {
        unsafe {
            let mut tmp_website_label = website_label.to_tmp_for_borrow();
            ffi::gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_website_label.to_glib_ptr())
        };
    }

    pub fn get_authors(&self) -> Vec<String> {
        let authors = unsafe { ffi::gtk_about_dialog_get_authors(GTK_ABOUT_DIALOG(self.unwrap_widget())) };
        let mut ret = Vec::new();

        if !authors.is_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = authors.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&*tmp)).to_string());
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn set_authors(&self, authors: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in authors.iter() {
            let c_str = CString::from_slice(tmp.as_bytes());

            tmp_vec.push(c_str.as_ptr());
        }
        tmp_vec.push(::std::ptr::null_mut());
        unsafe { ffi::gtk_about_dialog_set_authors(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_vec.as_slice().as_ptr()) }
    }

    pub fn get_artists(&self) -> Vec<String> {
        let artists = unsafe { ffi::gtk_about_dialog_get_artists(GTK_ABOUT_DIALOG(self.unwrap_widget())) };
        let mut ret = Vec::new();

        if !artists.is_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = artists.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&*tmp)).to_string());
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn set_artists(&self, artists: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in artists.iter() {
            let c_str = CString::from_slice(tmp.as_bytes());

            tmp_vec.push(c_str.as_ptr());
        }
        tmp_vec.push(::std::ptr::null_mut());
        unsafe { ffi::gtk_about_dialog_set_artists(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_vec.as_slice().as_ptr()) }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        let documenters = unsafe { ffi::gtk_about_dialog_get_documenters(GTK_ABOUT_DIALOG(self.unwrap_widget())) };
        let mut ret = Vec::new();

        if !documenters.is_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = documenters.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&*tmp)).to_string());
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn set_documenters(&self, documenters: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in documenters.iter() {
            let c_str = CString::from_slice(tmp.as_bytes());
            
            tmp_vec.push(c_str.as_ptr());
        }
        tmp_vec.push(::std::ptr::null_mut());
        unsafe { ffi::gtk_about_dialog_set_documenters(GTK_ABOUT_DIALOG(self.unwrap_widget()), tmp_vec.as_slice().as_ptr()) }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_translator_credits(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_translator_credits(&self, translator_credits: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(translator_credits.as_bytes());

            ffi::gtk_about_dialog_set_translator_credits(GTK_ABOUT_DIALOG(self.unwrap_widget()), c_str.as_ptr())
        };
    }

    /*pub fn get_logo(&self) -> Option<String> {
        let logo = unsafe { ffi::gtk_about_dialog_set_logo(self.pointer)) };

        if logo.is_null() {
            None
        } else {
            Some(unsafe { gtk::FFIWidget::wrap_widget(logo) })
        }
    }

    pub fn set_logo(&self, logo: Pixbuf) -> () {
        unsafe { ffi::gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(self.unwrap_widget()), GDK_PIXBUF(logo.unwrap_widget())) }
    }*/

    pub fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_logo_icon_name(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_logo_icon_name(&self, logo_icon_name: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(logo_icon_name.as_bytes());

            ffi::gtk_about_dialog_set_logo_icon_name(GTK_ABOUT_DIALOG(self.unwrap_widget()), c_str.as_ptr())
        };
    }

    pub fn add_credit_section(&self, section_name: &str, people: &Vec<String>) -> () {
        let mut tmp_vec = Vec::new();

        for tmp in people.iter() {
            let c_str = CString::from_slice(tmp.as_bytes());

            tmp_vec.push(c_str.as_ptr());
        }
        tmp_vec.push(::std::ptr::null_mut());
        unsafe {
            let c_str = CString::from_slice(section_name.as_bytes());

            ffi::gtk_about_dialog_add_credit_section(GTK_ABOUT_DIALOG(self.unwrap_widget()), c_str.as_ptr(), tmp_vec.as_slice().as_ptr())
        }
    }

    /*pub fn show(parent: Window, properties: Vec<String>) -> () {
        unsafe { ffi::gtk_show_about_dialog(GTK_WINDOW(parent), first_property_name, ...) }
    }*/
}

impl_drop!(AboutDialog);
impl_TraitWidget!(AboutDialog);

impl gtk::ContainerTrait for AboutDialog {}
impl gtk::BinTrait for AboutDialog {}
impl gtk::WindowTrait for AboutDialog {}
impl gtk::DialogTrait for AboutDialog {}

impl_widget_events!(AboutDialog);
