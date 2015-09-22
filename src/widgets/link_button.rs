// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Create buttons bound to a URL

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// LinkButton — Create buttons bound to a URL
pub type LinkButton = Object<ffi::GtkLinkButton>;

unsafe impl Upcast<Widget> for LinkButton { }
unsafe impl Upcast<::Container> for LinkButton { }
unsafe impl Upcast<::Bin> for LinkButton { }
unsafe impl Upcast<::Button> for LinkButton { }

unsafe impl Upcast<::Actionable> for LinkButton { }
unsafe impl Upcast<::Buildable> for LinkButton { }

impl LinkButton {
    /// Creates a new `LinkButton` with the URI as its text.
    pub fn new(uri: &str) -> LinkButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new(uri.to_glib_none().0))
                .downcast_unchecked()
        }
    }
    /// Creates a new `LinkButton` containing a label.
    pub fn new_with_label(uri: &str, label: &str) -> LinkButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new_with_label(uri.to_glib_none().0,
                label.to_glib_none().0))
                .downcast_unchecked()
        }
    }
    /// Retrieves the URI set using `set_uri()`.
    pub fn get_uri(&self) -> String {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(self.to_glib_none().0))
        }
    }
    /// Sets `uri` as the URI where the `LinkButton` points.
    /// As a side-effect this unsets the `visited` state of the button.
    pub fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_link_button_set_uri(self.to_glib_none().0, uri.to_glib_none().0)
        }
    }
    /// Sets the `visited` state of the URI where the `GtkLinkButton` points.
    /// See `get_visited()` for more details.
    pub fn set_visited(&self, visited: bool) {
        unsafe {
            ffi::gtk_link_button_set_visited(self.to_glib_none().0, visited.to_glib());
        }
    }
    /// Retrieves the `visited` state of the URI where the `LinkButton` points.
    /// The button becomes visited when it is clicked.
    /// If the URI is changed on the button, the `visited` state is unset again.
    /// The state may also be changed using `set_visited()`.
    pub fn get_visited(&self) -> bool {
        unsafe { from_glib(ffi::gtk_link_button_get_visited(self.to_glib_none().0)) }
    }
}


impl types::StaticType for LinkButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_link_button_get_type()) }
    }
}
