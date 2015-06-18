// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::types;
use glib::translate::*;
use ffi;

use object::{Object, Upcast, Downcast};
use super::widget::Widget;

/// A widget which controls the alignment and size of its child
pub type Alignment = Object<ffi::GtkAlignment>;

unsafe impl Upcast<Widget> for Alignment { }
unsafe impl Upcast<super::container::Container> for Alignment { }
unsafe impl Upcast<super::bin::Bin> for Alignment { }
unsafe impl Upcast<::builder::Buildable> for Alignment { }

impl Alignment {
    pub fn new(x_align: f32, y_align: f32, x_scale: f32, y_scale: f32) -> Alignment {
        unsafe {
            Widget::from_glib_none(ffi::gtk_alignment_new(x_align, y_align, x_scale, y_scale))
                .downcast_unchecked()
        }
    }

    pub fn set(&self, x_align: f32, y_align: f32, x_scale: f32, y_scale: f32) {
        unsafe {
            ffi::gtk_alignment_set(self.to_glib_none().0, x_align, y_align, x_scale, y_scale)
        }
    }

    pub fn set_padding(&self, padding_top: u32, padding_bottom: u32, padding_left: u32,
                       padding_right: u32) {
        unsafe {
            ffi::gtk_alignment_set_padding(self.to_glib_none().0, padding_top, padding_bottom,
                                           padding_left, padding_right);
        }
    }

    pub fn get_padding(&self) -> (u32, u32, u32, u32) {
        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;
        unsafe {
            ffi::gtk_alignment_get_padding(self.to_glib_none().0, &mut top, &mut bottom, &mut left,
                                           &mut right);
        }
        (top, bottom, left, right)
    }
}

impl types::StaticType for Alignment {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_alignment_get_type()) }
    }
}
