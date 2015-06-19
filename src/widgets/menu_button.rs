// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget that shows a menu when clicked on

use glib::translate::*;
use glib::types;
use ffi;
use {ArrowType};

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

/// MenuButton — A widget that shows a menu when clicked on
pub type MenuButton = Object<ffi::GtkMenuButton>;

unsafe impl Upcast<Widget> for MenuButton { }
unsafe impl Upcast<super::container::Container> for MenuButton { }
unsafe impl Upcast<super::bin::Bin> for MenuButton { }
unsafe impl Upcast<super::button::Button> for MenuButton { }
unsafe impl Upcast<super::toggle_button::ToggleButton> for MenuButton { }

unsafe impl Upcast<super::actionable::Actionable> for MenuButton { }
unsafe impl Upcast<::builder::Buildable> for MenuButton { }

impl MenuButton {
    pub fn new() -> MenuButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_button_new()).downcast_unchecked()
        }
    }

    pub fn set_popup<T: Upcast<Widget>>(&self, popup: &T) {
        unsafe {
            ffi::gtk_menu_button_set_popup(self.to_glib_none().0,
                popup.upcast().to_glib_none().0)
        }
    }
    pub fn set_direction(&self, direction: ArrowType) {
        unsafe {
            ffi::gtk_menu_button_set_direction(self.to_glib_none().0, direction)
        }
    }

    pub fn get_direction(&self) -> ArrowType {
        unsafe {
            ffi::gtk_menu_button_get_direction(self.to_glib_none().0)
        }
    }

    pub fn set_align_widget<T: Upcast<Widget>>(&self, align_widget: &T) {
        unsafe {
            ffi::gtk_menu_button_set_align_widget(self.to_glib_none().0,
                align_widget.upcast().to_glib_none().0)
        }
    }
}

impl types::StaticType for MenuButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_menu_button_get_type()) }
    }
}
