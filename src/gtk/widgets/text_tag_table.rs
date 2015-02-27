// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use gtk::ffi;

/// GtkTextTagTable — Collection of tags that can be used together

pub struct TextTagTable {
    pointer: *mut ffi::C_GtkTextTagTable
}

impl TextTagTable {
    pub fn new() -> Option<TextTagTable> {
        let tmp_pointer = unsafe { ffi::gtk_text_tag_table_new() };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextTagTable { pointer: tmp_pointer })
        }
    }

    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkTextTagTable {
        self.pointer
    }
}

impl_drop!(TextTagTable, GTK_TEXT_TAG_TABLE);
