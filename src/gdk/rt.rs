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

//! General — Library initialization and miscellaneous functions

use std::ptr;
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};
use gdk::ffi;

pub fn init() {
    unsafe { ffi::gdk_init(ptr::null_mut(), ptr::null_mut()) }
}

/*pub fn init_check(argc: *mut c_int, argv: *mut *mut *mut c_char) -> bool {
    unsafe { ::glib::to_bool(ffi::gdk_init_check(argc, argv)) }
}

pub fn parse_args(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { ffi::gdk_parse_args(argc, argv) }
}*/

pub fn get_display_arg_name() -> Option<String> {
    unsafe {
        FromGlibPtr::borrow(
            ffi::gdk_get_display_arg_name())
    }
}

pub fn notify_startup_complete() {
    unsafe { ffi::gdk_notify_startup_complete() }
}

pub fn notify_startup_complete_with_id(startup_id: &str) {
    unsafe {
        let mut tmp_startup_id = startup_id.to_tmp_for_borrow();
        ffi::gdk_notify_startup_complete_with_id(tmp_startup_id.to_glib_ptr());
    }
}

pub fn set_allowed_backends(backends: &str) {
    unsafe {
        let mut tmp_backends = backends.to_tmp_for_borrow();
        ffi::gdk_set_allowed_backends(tmp_backends.to_glib_ptr())
    }
}

pub fn get_program_class() -> Option<String> {
    unsafe {
        FromGlibPtr::borrow(
            ffi::gdk_get_program_class())
    }
}

pub fn set_program_class(program_class: &str) {
    unsafe {
        let mut tmp_program_class = program_class.to_tmp_for_borrow();
        ffi::gdk_set_program_class(tmp_program_class.to_glib_ptr())
    }
}

pub fn flush() {
    unsafe { ffi::gdk_flush() }
}

pub fn screen_width() -> i32 {
    unsafe { ffi::gdk_screen_width() }
}

pub fn screen_height() -> i32 {
    unsafe { ffi::gdk_screen_height() }
}

pub fn screen_width_mm() -> i32 {
    unsafe { ffi::gdk_screen_width_mm() }
}

pub fn screen_height_mm() -> i32 {
    unsafe { ffi::gdk_screen_height_mm() }
}

pub fn beep() {
    unsafe { ffi::gdk_flush() }
}

pub fn error_trap_push() {
    unsafe { ffi::gdk_error_trap_push() }
}

pub fn error_trap_pop() {
    unsafe { ffi::gdk_error_trap_pop() }
}

pub fn error_trap_pop_ignored() {
    unsafe { ffi::gdk_error_trap_pop_ignored() }
}