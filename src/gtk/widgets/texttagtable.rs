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

struct_Widget!(TextTagTable)

impl TextTagTable {
    pub fn new() -> Option<TextTagTable> {
        let tmp_pointer = unsafe { ffi::gtk_text_tag_table_new() };
        check_pointer!(tmp_pointer, TextTagTable)
    }
}

impl_drop!(TextTagTable)
impl_TraitWidget!(TextTagTable)

