// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_TOGGLEBUTTON;
use ffi;
use glib::{to_bool, to_gboolean};

pub trait ToggleButtonTrait: ::WidgetTrait + ::ContainerTrait + ::ButtonTrait {
    fn set_mode(&self, draw_indicate: bool) {
        unsafe { ffi::gtk_toggle_button_set_mode(GTK_TOGGLEBUTTON(self.unwrap_widget()), to_gboolean(draw_indicate)); }
    }

    fn get_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_button_get_mode(GTK_TOGGLEBUTTON(self.unwrap_widget()))) }
    }

    fn toggled(&self) -> () {
        unsafe {
            ffi::gtk_toggle_button_toggled(GTK_TOGGLEBUTTON(self.unwrap_widget()))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe { ffi::gtk_toggle_button_set_active(GTK_TOGGLEBUTTON(self.unwrap_widget()), to_gboolean(is_active)); }
    }

    fn get_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_button_get_active(GTK_TOGGLEBUTTON(self.unwrap_widget()))) }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe { ffi::gtk_toggle_button_set_inconsistent(GTK_TOGGLEBUTTON(self.unwrap_widget()), to_gboolean(setting)); }
    }

    fn get_inconsistent(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_button_get_inconsistent(GTK_TOGGLEBUTTON(self.unwrap_widget()))) }
    }
}

