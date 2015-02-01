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

//! GdkDeviceManager — Functions for handling input devices

use gdk::{self, ffi};

#[repr(C)]
#[derive(Copy)]
pub struct DeviceManager {
    pointer: *mut ffi::C_GdkDeviceManager
}

impl DeviceManager {
    pub fn disable_multidevice() {
        unsafe { ffi::gdk_disable_multidevice() }
    }

    pub fn get_display(&self) -> Option<gdk::Display> {
        let tmp = unsafe { ffi::gdk_device_manager_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Display::wrap_pointer(tmp))
        }
    }

    pub fn get_client(&self) -> Option<gdk::Device> {
        let tmp = unsafe { ffi::gdk_device_manager_get_client_pointer(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Device::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(DeviceManager, C_GdkDeviceManager);