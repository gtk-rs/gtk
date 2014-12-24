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

//! Frame timings — Object holding timing information for a single frame

use gdk::ffi;
use gtk;

#[repr(C)]
#[deriving(Copy)]
pub struct FrameTimings {
    pointer: *mut ffi::C_GdkFrameTimings
}

impl FrameTimings {
    // FIXME: should not be handled by user
    // Since 3.8
    pub fn _ref(&self) -> Option<FrameTimings> {
        let tmp = unsafe { ffi::gdk_frame_timings_ref(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(FrameTimings {
                pointer: tmp
            })
        }
    }

    // FIXME: should not be handled by user
    // Since 3.8
    pub fn unref(&self) {
        unsafe { ffi::gdk_frame_timings_unref(self.pointer) }
    }

    // Since 3.8
    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_counter(self.pointer) }
    }

    // Since 3.8
    pub fn get_complete(&self) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_frame_timings_get_complete(self.pointer)) }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_time(self.pointer) }
    }

    // Since 3.8
    pub fn get_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_presentation_time(self.pointer) }
    }

    // Since 3.8
    pub fn get_refresh_interval(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_refresh_interval(self.pointer) }
    }

    // Since 3.8
    pub fn get_predicted_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_predicted_presentation_time(self.pointer) }
    }
}

impl_GObjectFunctions!(FrameTimings, C_GdkFrameTimings);