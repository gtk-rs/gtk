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

/// The GdkPixbuf structure contains information that describes an image in memory.

use gdk::{mod, ffi};
use gtk;
use std::c_vec::CVec;

#[repr(C)]
#[deriving(Copy)]
/// This is the main structure in the &gdk-pixbuf; library. It is used to represent images. It contains information about the image's pixel 
/// data, its color space, bits per sample, width and height, and the rowstride (the number of bytes between the start of one row and the 
/// start of the next).
pub struct Pixbuf {
    pointer: *mut ffi::C_GdkPixbuf
}

impl Pixbuf {
    pub fn get_colorspace(&self) -> gdk::ColorSpace {
        unsafe { ffi::gdk_pixbuf_get_colorspace(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_n_channels(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_n_channels(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_pixbuf_get_has_alpha(self.pointer as *const ffi::C_GdkPixbuf)) }
    }

    pub fn get_bits_per_sample(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_bits_per_sample(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_pixels_with_length(&self, length: &mut u32) -> CVec<u8> {
        let tmp = unsafe { ffi::gdk_pixbuf_get_pixels_with_length(self.pointer as *const ffi::C_GdkPixbuf, length) };

        unsafe {
            if tmp.is_null() {
                CVec::new(1 as *mut u8, 0)
            } else {
                CVec::new(tmp, *length as uint)
            }
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_width(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_height(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_rowstride(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_rowstride(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_byte_length(&self) -> u64 {
        unsafe { ffi::gdk_pixbuf_get_byte_length(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_option(&self, key: &str) -> Option<String> {
        let tmp = unsafe {
            ffi::gdk_pixbuf_get_option(self.pointer as *const ffi::C_GdkPixbuf, key.with_c_str(|c_str| {
                c_str
            }))
        };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(String::from_raw_buf(tmp as *const u8)) }
        }
    }

    /// a convenient function
    /// It won't work for pixbufs with images that are other than 8 bits per sample or channel, but it will work for most of the
    /// pixbufs that GTK+ uses.
    pub fn put_pixel(&self, x: i32, y: i32, red: u8, green: u8, blue: u8, alpha: u8) {
        let n_channels = self.get_n_channels();
        let rowstride = self.get_rowstride();
        let mut length = 0u32;
        let mut pixels = self.get_pixels_with_length(&mut length);
        let s_pixels = pixels.as_mut_slice();
        let pos = (y * rowstride + x * n_channels) as uint;

        s_pixels[pos] = red;
        s_pixels[pos + 1] = green;
        s_pixels[pos + 2] = blue;
        s_pixels[pos + 3] = alpha;
    }
}

impl_GObjectFunctions!(Pixbuf, C_GdkPixbuf);