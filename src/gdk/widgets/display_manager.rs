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

//! GdkDisplayManager — Maintains a list of all open GdkDisplays

use gdk::{mod, ffi};

#[repr(C)]
#[deriving(Copy)]
pub struct DisplayManager {
    pointer: *mut ffi::C_GdkDisplayManager
}

impl DisplayManager {
    pub fn get() -> Option<DisplayManager> {
        let tmp = unsafe { ffi::gdk_display_manager_get() };

        if tmp.is_null() {
            None
        } else {
            Some(DisplayManager {
                pointer: tmp
            })
        }
    }

    pub fn get_default_display(&self) -> Option<gdk::Display> {
        let tmp = unsafe { ffi::gdk_display_manager_get_default_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Display::wrap_pointer(tmp))
        }
    }

    pub fn set_default_display(&self, display: &gdk::Display) {
        unsafe { ffi::gdk_display_manager_set_default_display(self.pointer, display.get_pointer()) }
    }

    pub fn open_display(&self, name: &str) -> Option<gdk::Display> {
        let tmp = unsafe {
            ffi::gdk_display_manager_open_display(self.pointer, name.with_c_str(|c_str| {
                c_str
            }))
        };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Display::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(DisplayManager, C_GdkDisplayManager);