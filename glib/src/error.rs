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

use ffi::{self, GQuark};
use glib_container::GlibContainer;
use translate::ToGlibPtr;

pub struct Error {
    pointer: *mut ffi::C_GError
}

impl Error {
    pub fn new_literal(domain: GQuark, code: i32, message: &str) -> Option<Error> {
        let tmp_pointer = unsafe {
            ffi::g_error_new_literal(domain, code, message.borrow_to_glib().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(Error{pointer: tmp_pointer})
        }
    }

    pub fn release(&mut self) -> () {
        if !self.pointer.is_null() {
            unsafe { ffi::g_error_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }

    pub fn matches(&self, domain: GQuark, code: i32) -> bool {
        match unsafe { ffi::g_error_matches(self.pointer, domain, code) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set(&mut self, domain: GQuark, code: i32, message: &str) -> () {
        unsafe {
            ffi::g_set_error_literal(&mut self.pointer, domain, code, message.borrow_to_glib().0)
        }
    }

    pub fn propagate(&mut self, other: &Error) -> () {
        unsafe { ffi::g_propagate_error(&mut self.pointer, other.pointer) }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        let tmp_pointer = unsafe { ffi::g_error_copy(self.pointer) };

        if tmp_pointer.is_null() {
            Error {
                pointer: ::std::ptr::null_mut()
            }
        } else {
            GlibContainer::wrap(tmp_pointer)
        }
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        self.release();
    }
}

impl GlibContainer<*mut ffi::C_GError> for Error {
    fn wrap(pointer: *mut ffi::C_GError) -> Error {
        Error {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut ffi::C_GError {
        self.pointer
    }
}
