// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A separator widget.

use glib::translate::*;
use ffi;

use glib::object::Downcast;
use super::widget::Widget;

use Orientation;

/// A separator widget.
glib_wrapper! {
    pub struct Separator(Object<ffi::GtkSeparator>): Widget, ::Orientable, ::Buildable;

    match fn {
        get_type => || ffi::gtk_separator_get_type(),
    }
}

impl Separator {
    pub fn new(orientation: Orientation) -> Separator {
        unsafe { Widget::from_glib_none(ffi::gtk_separator_new(orientation)).downcast_unchecked() }
    }
}
