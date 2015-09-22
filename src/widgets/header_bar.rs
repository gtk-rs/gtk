// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// GtkHeaderBar — A Box-like container
pub type HeaderBar = Object<ffi::GtkHeaderBar>;

// FIXME: add missing methods (3.12)

impl HeaderBar {
    pub fn new() -> HeaderBar {
        unsafe { Widget::from_glib_none(ffi::gtk_header_bar_new()).downcast_unchecked() }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_header_bar_set_title(self.to_glib_none().0, title.to_glib_none().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(self.to_glib_none().0))
        }
    }

    pub fn set_subtitle(&self, subtitle: &str) {
        unsafe {
            ffi::gtk_header_bar_set_subtitle(self.to_glib_none().0, subtitle.to_glib_none().0)
        }
    }

    pub fn get_subtitle(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(self.to_glib_none().0))
        }
    }

    pub fn set_custom_title<T: Upcast<Widget> = Widget>(&self, title_widget: Option<&T>) {
        unsafe {
            ffi::gtk_header_bar_set_custom_title(self.to_glib_none().0,
                title_widget.map(|w| w.upcast()).to_glib_none().0);
        }
    }

    pub fn get_custom_title(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_custom_title(self.to_glib_none().0))
        }
    }

    pub fn pack_start<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_start(self.to_glib_none().0, child.upcast().to_glib_none().0)
        }
    }

    pub fn pack_end<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_end(self.to_glib_none().0, child.upcast().to_glib_none().0)
        }
    }

    pub fn is_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    pub fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_close_button(self.to_glib_none().0, setting.to_glib())
        }
    }
}

impl types::StaticType for HeaderBar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_header_bar_get_type()) }
    }
}

unsafe impl Upcast<Widget> for HeaderBar { }
unsafe impl Upcast<::Container> for HeaderBar { }
unsafe impl Upcast<::Buildable> for HeaderBar { }
