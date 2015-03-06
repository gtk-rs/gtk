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

//! Application launching — Startup notification for applications

use gdk::{self, ffi};
use libc::c_int;
use glib::translate::{ToGlibPtr, ToTmp};

// FIXME: should inherit from GAppLaunchContext
#[repr(C)]
#[derive(Copy)]
pub struct AppLaunchContext {
    pointer: *mut ffi::C_GdkAppLaunchContext
}

impl AppLaunchContext {
    pub fn set_screen(&self, screen: &gdk::Screen) {
        unsafe { ffi::gdk_app_launch_context_set_screen(self.pointer, screen.unwrap_pointer()) }
    }

    pub fn set_desktop(&self, desktop: i32) {
        unsafe { ffi::gdk_app_launch_context_set_desktop(self.pointer, desktop as c_int) }
    }

    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.pointer, timestamp) }
    }

    /*pub fn set_icon(&self, icon: GIO::Icon) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.pointer, icon.unwrap_pointer()) }
    }*/

    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            let mut tmp_icon_name = icon_name.to_tmp_for_borrow();
            ffi::gdk_app_launch_context_set_icon_name(self.pointer, tmp_icon_name.to_glib_ptr())
        }
    }
}

impl_GObjectFunctions!(AppLaunchContext, C_GdkAppLaunchContext);