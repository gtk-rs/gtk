// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Displays an arrow.

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use {ArrowType, ShadowType};

/// Displays an arrow.
pub type Arrow = Object<ffi::GtkArrow>;

impl Arrow {
    pub fn new(arrow_type: ArrowType, shadow_type: ShadowType) -> Arrow {
        unsafe {
            Widget::from_glib_none(ffi::gtk_arrow_new(arrow_type, shadow_type)).downcast_unchecked()
        }
    }

    pub fn set(&self, arrow_type: ArrowType, shadow_type: ShadowType) {
        unsafe { ffi::gtk_arrow_set(self.to_glib_none().0, arrow_type, shadow_type); }
    }
}

impl types::StaticType for Arrow {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_arrow_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Arrow { }
unsafe impl Upcast<super::misc::Misc> for Arrow { }
unsafe impl Upcast<::builder::Buildable> for Arrow { }
