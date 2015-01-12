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

use libc::c_uint;
use std::ptr;
use gtk::ffi;
use std::ffi::c_str_to_bytes;

pub fn init() {
    unsafe {
        ffi::gtk_init(ptr::null(), ptr::null());
    }
}

pub fn main() {
    unsafe {
        ffi::gtk_main();
    }
}

pub fn main_quit() {
    unsafe {
        ffi::gtk_main_quit();
    }
}

pub fn main_level() -> u32 {
    unsafe {
        ffi::gtk_main_level() as u32
    }
}

pub fn main_iteration() -> bool {
    match unsafe { ffi::gtk_main_iteration() } {
        ffi::GFALSE => false,
        _           => true
    }
}

pub fn main_iteration_do(blocking: bool) -> bool {
    let c_blocking = if blocking { ffi::GTRUE } else { ffi::GFALSE };
    match unsafe { ffi::gtk_main_iteration_do(c_blocking) } {
        ffi::GFALSE => false,
        _           => true
    }
}

pub fn events_pending() -> bool {
    unsafe {
        ffi::to_bool(ffi::gtk_events_pending())
    }
}


pub fn get_major_version() -> u32 {
    unsafe {
        ffi::gtk_get_major_version() as u32
    }
}

pub fn get_minor_version() -> u32 {
    unsafe {
        ffi::gtk_get_minor_version() as u32
    }
}

pub fn get_micro_version() -> u32 {
    unsafe {
        ffi::gtk_get_micro_version() as u32
    }
}

pub fn get_binary_age() -> u32 {
    unsafe {
        ffi::gtk_get_binary_age() as u32
    }
}

pub fn get_interface_age() -> u32 {
    unsafe {
        ffi::gtk_get_interface_age() as u32
    }
}

pub fn check_version(required_major: u32,
                     required_minor: u32,
                     required_micro: u32)
                     -> Option<String> {
    let c_str = unsafe { ffi::gtk_check_version(required_major as c_uint, required_minor as c_uint, required_micro as c_uint) };
    if c_str.is_null() {
        None
    } else {
        Some(unsafe { String::from_utf8(c_str_to_bytes(c_str)) }) // as const *u8?
    }
 }
