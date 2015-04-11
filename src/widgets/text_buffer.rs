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

use std::ptr;

use ffi;
use FFIWidget;

/// GtkTextBuffer — Stores attributed text for display in a GtkTextView

struct_Widget!(TextBuffer);

impl TextBuffer {
    pub fn new(text_tag_table: Option<::TextTagTable>) -> Option<TextBuffer> {
        let tmp_pointer = unsafe {
            match text_tag_table {
                Some(ttl) => ffi::gtk_text_buffer_new(ttl.unwrap_pointer()),
                None      => ffi::gtk_text_buffer_new(ptr::null_mut())
            }
        };

        check_pointer!(tmp_pointer, TextBuffer)
    }
}

impl_drop!(TextBuffer);
impl_TraitWidget!(TextBuffer);

impl ::TextBufferTrait for TextBuffer {}

impl_widget_events!(TextBuffer);