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

//! Text buffer for gtk::Entry


#![allow(visible_private_types)]

use libc::{c_int, c_uint};
use std::str;
use gtk::ffi;
// TODO:
// Implements custom signal : inserted-text + deleted-text

/**
* EntryBuffer — Text buffer for gtk::Entry
*
* # Signals available:
* * `deleted-text` : Run First
* * `inserted-text` : Run First
*
*/
pub struct EntryBuffer {
    pointer: *ffi::C_GtkEntryBuffer,
    can_drop: bool
}

impl EntryBuffer {
    pub fn new(initial_chars: &str) -> Option<EntryBuffer> {
        let tmp_pointer = unsafe {
            initial_chars.with_c_str(|c_str| {
                ffi::gtk_entry_buffer_new(c_str, initial_chars.len() as c_int)
            })
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(EntryBuffer {
                pointer: tmp_pointer,
                can_drop: true
            })
        }
    }

    pub fn get_text(&self) -> String {
        let c_str = unsafe { ffi::gtk_entry_buffer_get_text(self.pointer) };
        unsafe {str::raw::from_c_str(c_str) }
    }

    pub fn set_text(&mut self, text: &str) -> () {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_entry_buffer_set_text(self.pointer, c_str, text.len() as c_int)
            });
        }
    }

    pub fn get_bytes(&self) -> i64 {
        unsafe {
            ffi::gtk_entry_buffer_get_bytes(self.pointer) as i64
        }
    }

    pub fn get_length(&self) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_get_length(self.pointer) as u32
        }
    }

    pub fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_buffer_get_max_length(self.pointer) as i32
        }
    }

    pub fn set_max_length(&mut self, max_length: i32) -> () {
        unsafe {
            ffi::gtk_entry_buffer_set_max_length(self.pointer, max_length as c_int)
        }
    }

    pub fn insert_text(&mut self, position: u32, text: &str) -> () {
        unsafe {
            ffi::gtk_entry_buffer_insert_text(self.pointer, position as c_uint, text.to_c_str().unwrap(), text.len() as c_int);
        }
    }

    pub fn delete_text(&mut self, position: u32, n_chars: u32) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_delete_text(self.pointer, position as c_uint, n_chars as c_uint) as u32
        }
    }

    pub fn emit_deleted_test(&mut self, position: u32, n_chars: u32) -> () {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(self.pointer, position as c_uint, n_chars as c_uint)
        }
    }

    pub fn emit_inserted_text(&mut self, position: u32, text: &str) -> () {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(self.pointer, position as c_uint, text.to_c_str().unwrap(), text.len() as c_int);
        }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *ffi::C_GtkEntryBuffer {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(pointer: *ffi::C_GtkEntryBuffer) -> EntryBuffer {
        EntryBuffer {
            pointer:    pointer,
            can_drop:   false
        }
    }
}
