// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A Scrollbar

use glib::translate::*;
use ffi;

use glib::object::Upcast;
use super::widget::Widget;
use Adjustment;
use Orientation;

/// GtkScrollBar — A Scrollbar
glib_wrapper! {
    pub struct Scrollbar(Object<ffi::GtkScrollbar>): Widget, ::Range, ::Buildable, ::Orientable;

    match fn {
        get_type => || ffi::gtk_scrollbar_get_type(),
    }
}

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
