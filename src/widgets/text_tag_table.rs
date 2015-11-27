// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;


pub struct TextTagTable {
    pointer: *mut ffi::GtkTextTagTable
}

impl TextTagTable {
    pub fn new() -> Option<TextTagTable> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_text_tag_table_new() };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextTagTable { pointer: tmp_pointer })
        }
    }

    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTextTagTable {
        self.pointer
    }
}

impl_drop!(TextTagTable, GTK_TEXT_TAG_TABLE);
