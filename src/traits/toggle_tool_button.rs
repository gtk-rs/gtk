// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_TOGGLETOOLBUTTON;
use ffi;
use glib::{to_bool, to_gboolean};

pub trait ToggleToolButtonTrait: ::WidgetTrait +
                                 ::ContainerTrait +
                                 ::BinTrait +
                                 ::ToolItemTrait +
                                 ::ToolButtonTrait {

    fn get_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_tool_button_get_active(GTK_TOGGLETOOLBUTTON(self.unwrap_widget()))) }
    }

    fn set_active(&self, set_underline: bool) -> () {
         unsafe { ffi::gtk_toggle_tool_button_set_active(GTK_TOGGLETOOLBUTTON(self.unwrap_widget()), to_gboolean(set_underline)); }
    }
}