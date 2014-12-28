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

/// The PangoItem structure stores information about a segment of text.
#[repr(C)]
#[deriving(Copy)]
pub struct Item {
    pointer: *mut ffi::C_PangoItem
}

impl Item {
    pub fn new() -> Option<Item> {
        let tmp = unsafe { ffi::pango_item_new() };

        if tmp.is_null() {
            None
        } else {
            Some(Item {
                pointer: tmp
            })
        }
    }

    pub fn copy(&self) -> Option<Item> {
        let tmp = unsafe { ffi::pango_item_copy(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Item {
                pointer: tmp
            })
        }
    }

    pub fn split(&self, split_index: i32, split_offset: i32) -> Option<Item> {
        let tmp = unsafe { ffi::pango_item_split(self.pointer, split_index as c_int, split_offset as c_int) };

        if tmp.is_null() {
            None
        } else {
            Some(Item {
                pointer: tmp
            })
        }
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        if self.pointer.is_not_null() {
            unsafe { ffi::pango_item_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}