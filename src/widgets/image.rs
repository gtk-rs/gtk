// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget displaying an image

use glib::translate::*;
use glib::types;
use gdk::pixbuf::{Pixbuf, PixbufAnimation};
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// A widget displaying an image.
pub type Image = Object<ffi::GtkImage>;

impl Image {
    pub fn new() -> Image {
        unsafe { Widget::from_glib_none(ffi::gtk_image_new()).downcast_unchecked() }
    }

    pub fn new_from_file(filename: &str) -> Image {
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(filename.to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn new_from_pixbuf(pixbuf: &Pixbuf) -> Image {
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn new_from_icon_name(icon_name: &str, size: i32) -> Image {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_image_new_from_icon_name(icon_name.to_glib_none().0, size))
                .downcast_unchecked()
        }
    }

    pub fn set_from_file(&self, filename: &str) {
        unsafe { ffi::gtk_image_set_from_file(self.to_glib_none().0, filename.to_glib_none().0); }
    }

    pub fn set_from_pixbuf(&self, pixbuf: &Pixbuf) {
        unsafe { ffi::gtk_image_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0); }
    }

    pub fn set_from_icon_name(&self, icon_name: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(self.to_glib_none().0,
                                              icon_name.to_glib_none().0, size);
        }
    }

    pub fn get_pixbuf(&self) -> Option<Pixbuf> {
        unsafe { from_glib_none(ffi::gtk_image_get_pixbuf(self.to_glib_none().0)) }
    }

    pub fn get_animation(&self) -> Option<PixbufAnimation> {
        unsafe { from_glib_none(ffi::gtk_image_get_animation(self.to_glib_none().0)) }
    }
}

impl types::StaticType for Image {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_image_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Image { }
unsafe impl Upcast<::Misc> for Image { }
unsafe impl Upcast<::Buildable> for Image { }
