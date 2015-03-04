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

//! Cursors — Standard and pixmap cursors

use gdk::{self, ffi};
use glib::translate::{ToGlibPtr, ToTmp};
//use libc::{c_int};

#[repr(C)]
#[derive(Copy)]
pub struct Cursor {
    pointer: *mut ffi::C_GdkCursor
}

impl Cursor {
    pub fn new(cursor_type: gdk::CursorType) -> Option<Cursor> {
        let tmp = unsafe { ffi::gdk_cursor_new(cursor_type) };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }

    /*pub fn new_from_pixbuf(display: &gdk::Display, pixbuf: &gdk::Pixbuf, x: i32, y: i32) -> Option<Cursor> {
        let tmp = unsafe { ffi::gdk_cursor_new_from_pixbuf(display.unwrap_pointer(), pixbuf.unwrap_pointer(), x as c_int, y as c_int) };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }*/

    pub fn new_from_name(display: &gdk::Display, name: &str) -> Option<Cursor> {
        let tmp = unsafe {
            let mut tmp_name = name.to_tmp_for_borrow();
            ffi::gdk_cursor_new_from_name(display.unwrap_pointer(), tmp_name.to_glib_ptr())
        };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }

    pub fn new_for_display(display: &gdk::Display, cursor_type: gdk::CursorType) -> Option<Cursor> {
        let tmp = unsafe { ffi::gdk_cursor_new_for_display(display.unwrap_pointer(), cursor_type) };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }

    pub fn get_display(&self) -> Option<gdk::Display> {
        let tmp = unsafe { ffi::gdk_cursor_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Display::wrap_pointer(tmp))
        }
    }

    /*pub fn get_image(&self) -> Option<gdk::Pixbuf> {
        let tmp = unsafe { ffi::gdk_cursor_get_image(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Pixbuf::wrap_pointer(tmp))
        }
    }*/

    pub fn get_cursor_type(&self) -> gdk::CursorType {
        unsafe { ffi::gdk_cursor_get_cursor_type(self.pointer) }
    }
}

impl_GObjectFunctions!(Cursor, C_GdkCursor);