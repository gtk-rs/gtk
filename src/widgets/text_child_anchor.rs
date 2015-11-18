// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::to_bool;

pub struct TextChildAnchor {
    pointer: *mut ffi::GtkTextChildAnchor
}

impl TextChildAnchor {
    pub fn new() -> Option<TextChildAnchor> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_text_child_anchor_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextChildAnchor { pointer: tmp_pointer })
        }
    }

    pub fn get_deleted(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_child_anchor_get_deleted(self.pointer)) }
    }
}

impl_GObjectFunctions!(TextChildAnchor, GtkTextChildAnchor);
impl_TraitObject!(TextChildAnchor, GtkTextChildAnchor);