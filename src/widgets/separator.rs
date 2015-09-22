// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A separator widget.

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use Orientation;

/// A separator widget.
pub type Separator = Object<ffi::GtkSeparator>;

impl Separator {
    pub fn new(orientation: Orientation) -> Separator {
        unsafe { Widget::from_glib_none(ffi::gtk_separator_new(orientation)).downcast_unchecked() }
    }
}

impl types::StaticType for Separator {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_separator_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Separator { }
unsafe impl Upcast<::Orientable> for Separator { }
unsafe impl Upcast<::Buildable> for Separator { }
