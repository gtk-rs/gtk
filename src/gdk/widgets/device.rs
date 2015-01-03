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

//! GdkDevice — Object representing an input device

use gdk::{self, ffi};
use libc::{c_uint};
use gtk;

#[repr(C)]
#[derive(Copy)]
pub struct Device {
    pointer: *mut ffi::C_GdkDevice
}

impl Device {
    pub fn get_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gdk_device_get_name(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(String::from_raw_buf(tmp as *const u8)) }
        }
    }

    pub fn get_source(&self) -> gdk::InputSource {
        unsafe { ffi::gdk_device_get_source(self.pointer) }
    }

    pub fn set_mode(&self, mode: gdk::InputMode) {
        unsafe { ffi::gdk_device_set_mode(self.pointer, mode) }
    }

    pub fn get_mode(&self) -> gdk::InputMode {
        unsafe { ffi::gdk_device_get_mode(self.pointer) }
    }

    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: gdk::ModifierType) {
        unsafe { ffi::gdk_device_set_key(self.pointer, index_ as c_uint, keyval as c_uint, modifiers) }
    }

    pub fn get_key(&self, index_: u32, keyval: &mut u32, modifiers: &mut gdk::ModifierType) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_device_get_key(self.pointer, index_ as c_uint, keyval as *mut c_uint, modifiers)) }
    }

    pub fn set_axis_use(&self, index_: u32, use_: gdk::AxisUse) {
        unsafe { ffi::gdk_device_set_axis_use(self.pointer, index_ as c_uint, use_) }
    }

    pub fn get_axis_use(&self, index_: u32) -> gdk::AxisUse {
        unsafe { ffi::gdk_device_get_axis_use(self.pointer, index_ as c_uint) }
    }

    pub fn get_associated_device(&self) -> Option<Device> {
        let tmp = unsafe { ffi::gdk_device_get_associated_device(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Device {
                pointer: tmp
            })
        }
    }

    pub fn get_device_type(&self) -> gdk::DeviceType {
        unsafe { ffi::gdk_device_get_device_type(self.pointer) }
    }

    /*pub fn get_display(&self) -> Option<gdk::Display> {
        let tmp = unsafe { ffi::gdk_device_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Display::wrap_pointer(tmp))
        }
    }*/

    pub fn get_has_cursor(&self) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_device_get_has_cursor(self.pointer)) }
    }

    pub fn get_n_axes(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_axes(self.pointer) }
    }

    pub fn get_n_keys(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_keys(self.pointer) }
    }

    /*pub fn warp(&self, screen: &gdk::Screen, x: i32, y: i32) {
        unsafe { ffi::gdk_device_warp(self.pointer, screen.get_pointer(), x as c_int, y as c_int) }
    }

    pub fn grab(&self, window: &gdk::Window, grab_ownership: gdk::GrabOwnership, owner_events: bool, event_mask: gdk::EventMask,
        cursor: &mut gdk::Cursor, time_: u32) -> gdk::GrabStatus {
        unsafe {
            ffi::gdk_device_grab(self.pointer, window.get_pointer(), grab_ownership, gtk::ffi::to_gboolean(owner_events),
                event_mask, cursor.get_pointer(), time_)
        }
    }*/

    pub fn ungrab(&self, time_: u32) {
        unsafe { ffi::gdk_device_ungrab(self.pointer, time_) }
    }

    /*pub fn get_state(&self, window: &gdk::Window, axes: &mut [f64], mask: &mut gdk;:ModifierType) {
        unsafe { ffi::gdk_device_get_state(self.pointer, window.get_pointer(), axes.as_mut_ptr(), mask) }
    }

    pub fn get_position(&self, x: &mut i32, y: &mut i32) -> Option<gdk::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position(self.pointer, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(gdk::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_position_double(&self, x: &mut f64, y: &mut f64) -> Option<gdk::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position_double(self.pointer, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(gdk::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position(&self, x: &mut i32, y: &mut i32) -> Option<gdk::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position(self.pointer, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(gdk::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position_double(&self, x: &mut f64, y: &mut f64) -> Option<gdk::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position_double(self.pointer, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(gdk::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_history(&self, window: &gdk::Window, start: u32, stop: u32) -> Vec<gdk::TimeCoord> {
        let mut ptr = ::std::ptr::null_mut();
        let mut n_events : c_int = 0;

        unsafe { ffi::gdk_device_get_history(self.pointer, window.get_pointer(), start, stop, &mut ptr, &mut n_events) };
        
        let mut ret = Vec::with_capacity(n_events as uint);
        
        for i in range(0, n_events) {
            ret.push(gdk::TimeCoord::wrap_pointer(::std::ptr::read(ptr.offset(i))));
        }
        ret
    }

    pub fn free_history(events: &[gdk::TimeCoord]) {
        let mut tmp = Vec::with_capacity(events.len());

        for i in range(0, events.len()) {
            tmp.push(events[i].get_pointer());
        }
        unsafe { ffi::gdk_device_free_history(events.as_mut_ptr(), events.len()) }
    }*/

    pub fn get_axis(&self, axes: &mut [f64], use_: gdk::AxisUse, value: &mut f64) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_device_get_axis(self.pointer, axes.as_mut_ptr(), use_, value)) }
    }

    /*pub fn get_axis_value(&self, axes: &mut [f64], label: &mut gdk::Atom, value: &mut f64) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_device_get_axis_value(self.pointer, axes.as_mut_ptr(), label.get_pointer(), value)) }
    }*/

    /*pub fn get_last_event_window(&self) -> Option<gdk::Window> {
        let ptr = unsafe { ffi::gdk_device_get_last_event_window(self.pointer) };

        if ptr.is_null() {
            None
        } else {
            Some(gdk::Window::wrap_pointer(ptr))
        }
    }*/
}

impl_GObjectFunctions!(Device, C_GdkDevice);
