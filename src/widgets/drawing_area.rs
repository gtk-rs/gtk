// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// A widget for custom user interface elements.
pub type DrawingArea = Object<ffi::GtkDrawingArea>;

impl DrawingArea {
    pub fn new() -> DrawingArea {
        unsafe { Widget::from_glib_none(ffi::gtk_drawing_area_new()).downcast_unchecked() }
    }
}

impl types::StaticType for DrawingArea {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_drawing_area_get_type()) }
    }
}

unsafe impl Upcast<Widget> for DrawingArea { }
unsafe impl Upcast<::builder::Buildable> for DrawingArea { }
