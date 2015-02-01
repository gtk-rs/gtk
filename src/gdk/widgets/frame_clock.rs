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

//! Frame clock — Frame clock syncs painting to a window or display

use gdk::{self, ffi};

#[repr(C)]
#[derive(Copy)]
pub struct FrameClock {
    pointer: *mut ffi::C_GdkFrameClock
}

impl FrameClock {
    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_time(self.pointer) }
    }

    pub fn request_phase(&self, phase: gdk::FrameClockPhase) {
        unsafe { ffi::gdk_frame_clock_request_phase(self.pointer, phase) }
    }

    pub fn begin_updating(&self) {
        unsafe { ffi::gdk_frame_clock_begin_updating(self.pointer) }
    }

    pub fn end_updating(&self) {
        unsafe { ffi::gdk_frame_clock_end_updating(self.pointer) }
    }

    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_counter(self.pointer) }
    }

    pub fn get_history_start(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_history_start(self.pointer) }
    }

    pub fn get_timings(&self, frame_counter: i64) -> Option<gdk::FrameTimings> {
        let tmp = unsafe { ffi::gdk_frame_clock_get_timings(self.pointer, frame_counter) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::FrameTimings::wrap_pointer(tmp))
        }
    }

    pub fn get_current_timings(&self) -> Option<gdk::FrameTimings> {
        let tmp = unsafe { ffi::gdk_frame_clock_get_current_timings(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::FrameTimings::wrap_pointer(tmp))
        }
    }

    pub fn get_refresh_info(&self, base_time: i64, refresh_interval_return: &mut i64, presentation_time_return: &mut i64) {
        unsafe { ffi::gdk_frame_clock_get_refresh_info(self.pointer, base_time, refresh_interval_return, presentation_time_return) }
    }
}

impl_GObjectFunctions!(FrameClock, C_GdkFrameClock);