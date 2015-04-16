// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_SCROLLED_WINDOW;
use ffi;
use FFIWidget;

pub trait ScrolledWindowTrait: ::WidgetTrait {
    fn set_policy(&self, h_scrollbar_policy: ::PolicyType, v_scrollbar_policy: ::PolicyType) {
        unsafe {
            ffi::gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(self.unwrap_widget()), h_scrollbar_policy, v_scrollbar_policy);
        }
    }
}