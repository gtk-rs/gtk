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

pub struct TextChildAnchor {
    pointer: *mut ffi::C_GtkTextChildAnchor
}

impl TextChildAnchor {
    pub fn new() -> Option<TextChildAnchor> {
        let tmp_pointer = unsafe { ffi::gtk_text_child_anchor_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextChildAnchor { pointer: tmp_pointer })
        }
    }

    pub fn get_deleted(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_child_anchor_get_deleted(self.pointer)) }
    }
}

impl_GObjectFunctions!(TextChildAnchor, C_GtkTextChildAnchor)