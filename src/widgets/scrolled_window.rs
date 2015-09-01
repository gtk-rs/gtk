// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;

use ffi;
use cast::GTK_SCROLLED_WINDOW;

/// GtkScrolledWindow — Adds scrollbars to its child widget
struct_Widget!(ScrolledWindow);

impl ScrolledWindow {
    pub fn new(h_adjustment: Option<::Adjustment>, v_adjustment: Option<::Adjustment>) -> Option<ScrolledWindow> {

        let tmp_pointer = unsafe {
            ffi::gtk_scrolled_window_new(
                h_adjustment.map_or(ptr::null_mut(), |p| { p.unwrap_pointer() }),
                v_adjustment.map_or(ptr::null_mut(), |p| { p.unwrap_pointer() })
            )
        };

        check_pointer!(tmp_pointer, ScrolledWindow)
    }

    pub fn get_min_content_width(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_min_content_width(GTK_SCROLLED_WINDOW(self.pointer)) }
    }

    pub fn set_min_content_width(&self, width: i32) {
        unsafe { ffi::gtk_scrolled_window_set_min_content_width(GTK_SCROLLED_WINDOW(self.pointer), width) }
    }

    pub fn get_min_content_height(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_min_content_height(GTK_SCROLLED_WINDOW(self.pointer)) }
    }

    pub fn set_min_content_height(&self, height: i32) {
        unsafe { ffi::gtk_scrolled_window_set_min_content_height(GTK_SCROLLED_WINDOW(self.pointer), height) }
    }
}

impl_drop!(ScrolledWindow);
impl_TraitWidget!(ScrolledWindow);

impl ::ScrolledWindowTrait for ScrolledWindow {}
impl ::ContainerTrait for ScrolledWindow {}
