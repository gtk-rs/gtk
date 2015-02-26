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

//! Drag And Drop — Functions for controlling drag and drop handling

use gdk::{self, ffi};
use glib::{to_bool, to_gboolean};
use libc::c_int;

#[repr(C)]
#[derive(Copy)]
pub struct DragContext {
    pointer: *mut ffi::C_GdkDragContext
}

impl DragContext {
    pub fn drag_get_selection(&self) -> Option<gdk::Atom> {
        let tmp = unsafe { ffi::gdk_drag_get_selection(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Atom::wrap_pointer(tmp))
        }
    }

    pub fn drag_abort(&self, time_: u32) {
        unsafe { ffi::gdk_drag_abort(self.pointer, time_) }
    }

    pub fn drop_reply(&self, accepted: bool, time_: u32) {
        unsafe { ffi::gdk_drop_reply(self.pointer, to_gboolean(accepted), time_) }
    }

    pub fn drop(&self, time_: u32) {
        unsafe { ffi::gdk_drag_drop(self.pointer, time_) }
    }

    pub fn drag_find_window_for_screen(&self, drag_window: &gdk::Window, screen: &gdk::Screen, x_root: i32, y_root: i32,
        dest_window: &mut gdk::Window, protocol: &mut gdk::DragProtocol) {
        unsafe { ffi::gdk_drag_find_window_for_screen(self.pointer, drag_window.get_pointer(), screen.get_pointer(), x_root as c_int,
            y_root as c_int, &mut dest_window.get_pointer(), protocol) }
    }

    pub fn drag_motion(&self, dest_window: &gdk::Window, protocol: gdk::DragProtocol, x_root: i32, y_root: i32,
        suggested_action: gdk::DragAction, possible_actions: gdk::DragAction, time_: u32) -> bool {
        unsafe { to_bool(ffi::gdk_drag_motion(self.pointer, dest_window.get_pointer(), protocol, x_root as c_int,
            y_root as c_int, suggested_action, possible_actions, time_)) }
    }

    pub fn drop_finish(&self, success: bool, time_: u32) {
        unsafe { ffi::gdk_drop_finish(self.pointer, to_gboolean(success), time_) }
    }

    pub fn drag_status(&self, action: gdk::DragAction, time_: u32) {
        unsafe { ffi::gdk_drag_status(self.pointer, action, time_) }
    }

    pub fn drag_drop_succeeded(&self) -> bool {
        unsafe { to_bool(ffi::gdk_drag_drop_succeeded(self.pointer)) }
    }

    pub fn get_actions(&self) -> gdk::DragAction {
        unsafe { ffi::gdk_drag_context_get_actions(self.pointer) }
    }

    pub fn get_suggested_action(&self) -> gdk::DragAction {
        unsafe { ffi::gdk_drag_context_get_suggested_action(self.pointer) }
    }

    pub fn get_selected_action(&self) -> gdk::DragAction {
        unsafe { ffi::gdk_drag_context_get_selected_action(self.pointer) }
    }

    pub fn get_device(&self) -> Option<gdk::Device> {
        let tmp = unsafe { ffi::gdk_drag_context_get_device(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Device::wrap_pointer(tmp))
        }
    }

    pub fn set_device(&self, device: &gdk::Device) {
        unsafe { ffi::gdk_drag_context_set_device(self.pointer, device.get_pointer()) }
    }

    pub fn get_source_window(&self) -> Option<gdk::Window> {
        let tmp = unsafe { ffi::gdk_drag_context_get_source_window(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Window::wrap_pointer(tmp))
        }
    }

    pub fn get_dest_window(&self) -> Option<gdk::Window> {
        let tmp = unsafe { ffi::gdk_drag_context_get_dest_window(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Window::wrap_pointer(tmp))
        }
    }

    pub fn get_protocol(&self) -> gdk::DragProtocol {
        unsafe { ffi::gdk_drag_context_get_protocol(self.pointer) }
    }
}

impl_GObjectFunctions!(DragContext, C_GdkDragContext);
