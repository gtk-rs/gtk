// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A bar that can used level indicator

//#![cfg_attr(not(gtk_3_8), allow(unused_imports))]

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use {LevelBarMode};

/// LevelBar — A bar that can used level indicator
pub type LevelBar = Object<ffi::GtkLevelBar>;

impl LevelBar {
    pub fn new() -> LevelBar {
        unsafe { Widget::from_glib_none(ffi::gtk_level_bar_new()).downcast_unchecked() }
    }

    pub fn new_for_interval(min: f64, max: f64) -> LevelBar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min, max))
                .downcast_unchecked()
        }
    }

    pub fn set_value(&self, value: f64) {
        unsafe { ffi::gtk_level_bar_set_value(self.to_glib_none().0, value); }
    }

    pub fn get_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_value(self.to_glib_none().0) }
    }

    pub fn set_mode(&self, mode: LevelBarMode) {
        unsafe { ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode); }
    }

    pub fn get_mode(&self) -> LevelBarMode {
        unsafe { ffi::gtk_level_bar_get_mode(self.to_glib_none().0) }
    }

    pub fn set_min_value(&self, value: f64) {
        unsafe { ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value); }
    }

    pub fn get_min_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_min_value(self.to_glib_none().0) }
    }

    pub fn set_max_value(&self, value: f64) {
        unsafe { ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value); }
    }

    pub fn get_max_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_max_value(self.to_glib_none().0) }
    }

    #[cfg(gtk_3_8)]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe { ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib()); }
    }

    #[cfg(gtk_3_8)]
    pub fn get_inverted(&self) -> bool {
        unsafe { from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0)) }
    }

    pub fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(self.to_glib_none().0, name.to_glib_none().0,
                value)
        }
    }

    pub fn remove_offset_value(&self, name: &str) {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn get_offset_value(&self, name: &str) -> Option<f64> {
        unsafe {
            let mut value = 0.;
            let ok = from_glib(
                ffi::gtk_level_bar_get_offset_value(self.to_glib_none().0, name.to_glib_none().0,
                    &mut value));
            if ok { Some(value) } else { None }
        }
    }
}

impl types::StaticType for LevelBar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_level_bar_get_type()) }
    }
}

unsafe impl Upcast<Widget> for LevelBar { }
unsafe impl Upcast<super::orientable::Orientable> for LevelBar { }
unsafe impl Upcast<::builder::Buildable> for LevelBar { }
