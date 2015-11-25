// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkActionable — An interface for widgets that can be associated with actions

use glib::translate::*;
use ffi;

use glib::object::Upcast;
use super::widget::Widget;

glib_wrapper! {
    pub struct Actionable(Object<ffi::GtkActionable>): Widget;

    match fn {
        get_type => || ffi::gtk_actionable_get_type(),
    }
}

pub trait ActionableExt {
    fn get_action_name(&self) -> Option<String>;
    fn set_action_name(&self, action_name: &str);
    fn set_detailed_action_name(&self, detailed_action_name: &str);
}

impl<O: Upcast<Actionable>> ActionableExt for O {
    fn get_action_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_actionable_get_action_name(self.upcast().to_glib_none().0))
        }
    }

    fn set_action_name(&self, action_name: &str) {
        unsafe {
            ffi::gtk_actionable_set_action_name(self.upcast().to_glib_none().0,
                action_name.to_glib_none().0)
        }
    }

    fn set_detailed_action_name(&self, detailed_action_name: &str) {
        unsafe {
            ffi::gtk_actionable_set_detailed_action_name(self.upcast().to_glib_none().0,
                detailed_action_name.to_glib_none().0)
        }
    }
}
