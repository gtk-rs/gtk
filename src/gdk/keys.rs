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

//! Keyboard Handling Functions

use std::ptr;
use gdk::ffi;
use std::ffi::CString;
use libc::{c_uint, c_void, c_char};

pub fn keyval_name(keyval: u32) -> Option<String> {
    let tmp = unsafe { ffi::gdk_keyval_name(keyval as c_uint) as *const c_char };

    if tmp.is_null() {
        None
    } else {
        unsafe {
            let ret = Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string());

            ret
        }
    }
}
