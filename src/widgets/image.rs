// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget displaying an image

use ffi;
use cast::GTK_IMAGE;
use FFIWidget;
use glib::translate::ToGlibPtr;
use gdk;

/// Image — A widget displaying an image
struct_Widget!(Image);

impl Image {
    pub fn new() -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new()
        };
        check_pointer!(tmp_pointer, Image)
    }

    pub fn new_from_file(filename: &str) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new_from_file(filename.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Image)
    }

    pub fn new_from_pixbuf(pixbuf: &gdk::Pixbuf) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new_from_pixbuf(pixbuf.unwrap_pointer())
        };
        check_pointer!(tmp_pointer, Image)
    }

    pub fn new_from_icon_name(icon_name: &str, size: ::IconSize) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new_from_icon_name(icon_name.to_glib_none().0, size)
        };
        check_pointer!(tmp_pointer, Image)
    }

    pub fn set_from_file(&self, filename: &str) {
        unsafe {
            ffi::gtk_image_set_from_file(GTK_IMAGE(self.unwrap_widget()),
                                         filename.to_glib_none().0);
        };
    }

    pub fn set_from_pixbuf(&self, pixbuf: &gdk::Pixbuf) {
        unsafe {
            ffi::gtk_image_set_from_pixbuf(GTK_IMAGE(self.unwrap_widget()), pixbuf.unwrap_pointer())
        };
    }

    pub fn set_from_icon_name(&self, icon_name: &str, size: ::IconSize) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(GTK_IMAGE(self.unwrap_widget()),
                                              icon_name.to_glib_none().0, size)
        };
    }

    pub fn get_pixbuf(&self) -> gdk::Pixbuf {
        gdk::Pixbuf::wrap_pointer(unsafe {
            ffi::gtk_image_get_pixbuf(GTK_IMAGE(self.unwrap_widget()))
        })
    }
}

impl_drop!(Image);
impl_TraitWidget!(Image);

impl ::MiscTrait for Image {}
