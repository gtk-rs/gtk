// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A Scrollbar

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;
use Adjustment;
use Orientation;

/// GtkScrollBar — A Scrollbar
pub type Scrollbar = Object<ffi::GtkScrollbar>;

unsafe impl Upcast<Widget> for Scrollbar { }
unsafe impl Upcast<super::range::Range> for Scrollbar { }

unsafe impl Upcast<::builder::Buildable> for Scrollbar { }
unsafe impl Upcast<super::orientable::Orientable> for Scrollbar { }

impl Scrollbar {
    /// Creates a new scrollbar with the given orientation.
    pub fn new(orientation: Orientation, adjustment: Option<&Adjustment>) -> Scrollbar {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_scrollbar_new(orientation, adjustment.to_glib_none().0))
                .downcast_unchecked()
        }
    }
}

impl types::StaticType for Scrollbar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_scrollbar_get_type()) }
    }
}
