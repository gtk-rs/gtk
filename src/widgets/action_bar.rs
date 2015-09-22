// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use object::{Object, Upcast, Downcast};
use glib::types;
use glib::translate::*;
use super::widget::Widget;

unsafe impl Upcast<Widget> for ActionBar { }
unsafe impl Upcast<::Container> for ActionBar { }
unsafe impl Upcast<::Bin> for ActionBar { }
unsafe impl Upcast<::Buildable> for ActionBar { }

/// Hide and show with animation
pub type ActionBar = Object<ffi::GtkActionBar>;

impl ActionBar {
    pub fn new() -> ActionBar {
        unsafe { Widget::from_glib_none(ffi::gtk_action_bar_new()).downcast_unchecked() }
    }

    pub fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_action_bar_get_center_widget(self.to_glib_none().0))
        }
    }

    pub fn set_center_widget<T: Upcast<Widget>>(&self, center_widget: &T) {
        unsafe {
            ffi::gtk_action_bar_set_center_widget(self.to_glib_none().0,
                                                  center_widget.upcast().to_glib_none().0);
        }
    }

    pub fn pack_start<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_start(self.to_glib_none().0, child.upcast().to_glib_none().0);
        }
    }

    pub fn pack_end<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_end(self.to_glib_none().0, child.upcast().to_glib_none().0);
        }
    }
}

impl types::StaticType for ActionBar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_action_bar_get_type()) }
    }
}
