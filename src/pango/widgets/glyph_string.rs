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

use pango::ffi;
use libc::c_int;

/// The PangoGlyphString structure is used to store strings of glyphs with geometry and visual
/// attribute information. The storage for the glyph information is owned by the structure which
/// simplifies memory management.
#[repr(C)]
pub struct GlyphString {
    pointer: *mut ffi::C_PangoGlyphString
}

impl GlyphString {
    pub fn new() -> Option<GlyphString> {
        let tmp = unsafe { ffi::pango_glyph_string_new() };

        if tmp.is_null() {
            None
        } else {
            Some(GlyphString {
                pointer: tmp
            })
        }
    }

    pub fn copy(&self) -> Option<GlyphString> {
        let tmp = unsafe { ffi::pango_glyph_string_copy(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(GlyphString {
                pointer: tmp
            })
        }
    }

    pub fn set_size(&self, new_len: i32) {
        unsafe { ffi::pango_glyph_string_set_size(self.pointer, new_len as c_int) }
    }
}

impl Drop for GlyphString {
    fn drop(&mut self) {
        if pointer.is_null() {
            return;
        }
        unsafe { ffi::pango_glyph_string_free(self.pointer); }
        self.pointer = ::std::ptr::null_mut();
    }
}