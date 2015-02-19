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

//! GPermission — An object representing the permission to perform a certain action

use glib_container::GlibContainer;
use ffi;

#[derive(Copy)]
pub struct Permission {
    pointer: *mut ffi::C_GPermission
}

impl Permission {
    pub fn get_allowed(&self) -> bool {
        match unsafe { ffi::g_permission_get_allowed(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_can_acquire(&self) -> bool {
        match unsafe { ffi::g_permission_get_can_acquire(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_can_release(&self) -> bool {
        match unsafe { ffi::g_permission_get_can_release(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe { ffi::g_permission_impl_update(self.pointer,
            if allowed == true { 1 } else { 0 },
            if can_acquire == true { 1 } else { 0 },
            if can_release == true { 1 } else { 0 }) }
    }
}

/*impl Drop for Permission {
    fn drop(&mut self) {
        self.release();
    }
}*/

impl GlibContainer<*mut ffi::C_GPermission> for Permission {
    fn wrap(pointer: *mut ffi::C_GPermission) -> Permission {
        Permission {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut ffi::C_GPermission {
        self.pointer
    }
}