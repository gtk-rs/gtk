// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::{to_bool, to_gboolean};

pub struct SizeGroup {
    pointer: *mut ffi::GtkSizeGroup
}

impl SizeGroup {
    pub fn new(mode: ::SizeGroupMode) -> Option<SizeGroup> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_size_group_new(mode) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(SizeGroup {
                pointer: tmp_pointer
            })
        }
    }

    pub fn set_mode(&self, mode: ::SizeGroupMode) {
        unsafe { ffi::gtk_size_group_set_mode(self.pointer, mode) }
    }

    pub fn get_mode(&self) -> ::SizeGroupMode {
        unsafe { ffi::gtk_size_group_get_mode(self.pointer) }
    }

    pub fn set_ignore_hidden(&self, ignore_hidden: bool) {
        unsafe { ffi::gtk_size_group_set_ignore_hidden(self.pointer, to_gboolean(ignore_hidden)) }
    }

    pub fn get_ignore_hidden(&self) -> bool {
        unsafe { to_bool(ffi::gtk_size_group_get_ignore_hidden(self.pointer)) }
    }

    pub fn add_widget<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe { ffi::gtk_size_group_add_widget(self.pointer, widget.unwrap_widget()) }
    }

    pub fn remove_widget<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe { ffi::gtk_size_group_remove_widget(self.pointer, widget.unwrap_widget()) }
    }
}

impl_GObjectFunctions!(SizeGroup, GtkSizeGroup);
