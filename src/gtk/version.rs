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

//! Fonction to handle the Gtk+ version

use libc::c_uint;
use std::{ptr, str};

use ffi;
use std;

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
        Some(unsafe {str::raw::from_c_str(c_str) })
    }
 }
