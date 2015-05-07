// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkActionable — An interface for widgets that can be associated with actions

use glib::translate::{from_glib_none, ToGlibPtr};
use cast::GTK_ACTIONABLE;
use ffi;

pub trait ActionableTrait: ::WidgetTrait {
    fn get_action_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_actionable_get_action_name(GTK_ACTIONABLE(self.unwrap_widget())))
        }
    }

    fn set_action_name(&self, action_name: &str) {
        unsafe {
            ffi::gtk_actionable_set_action_name(GTK_ACTIONABLE(self.unwrap_widget()), action_name.to_glib_none().0)
        }
    }

    fn set_detailed_action_name(&self, detailed_action_name: &str) {
        unsafe {
            ffi::gtk_actionable_set_detailed_action_name(GTK_ACTIONABLE(self.unwrap_widget()), detailed_action_name.to_glib_none().0)
        }
    }
}
