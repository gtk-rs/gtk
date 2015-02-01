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

//! RGBA Colors — RGBA colors

use gdk::ffi;
use gtk;
use std::ffi::CString;
use libc::{c_char, c_void};

/// The GdkRGBA structure is used to represent a (possibly translucent) color, in a way that is compatible with cairos notion of color.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct RGBA {
    /// The intensity of the red channel from 0.0 to 1.0 inclusive
    pub red: f64,
    /// The intensity of the green channel from 0.0 to 1.0 inclusive
    pub green: f64,
    /// The intensity of the blue channel from 0.0 to 1.0 inclusive
    pub blue: f64,
    /// The opacity of the color from 0.0 for completely translucent to 1.0 for opaque
    pub alpha: f64
}

impl RGBA {
    pub fn white() -> RGBA {
        RGBA {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
            alpha: 1f64
        }
    }

    pub fn blue() -> RGBA {
        RGBA {
            red: 0f64,
            green: 0f64,
            blue: 1f64,
            alpha: 1f64
        }
    }

    pub fn green() -> RGBA {
        RGBA {
            red: 0f64,
            green: 1f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    pub fn red() -> RGBA {
        RGBA {
            red: 1f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    pub fn black() -> RGBA {
        RGBA {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    pub fn copy(&self) -> RGBA {
        RGBA {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha
        }
    }

    pub fn parse(&mut self, spec: &str) -> bool {
        unsafe {
            let c_str = CString::from_slice(spec.as_bytes());

            gtk::ffi::to_bool(ffi::gdk_rgba_parse(self, c_str.as_ptr()))
        }
    }

    pub fn equal(&self, other: &RGBA) -> bool {
        unsafe { gtk::ffi::to_bool(ffi::gdk_rgba_equal(self, other)) }
    }

    pub fn hash(&self) -> u32 {
        unsafe { ffi::gdk_rgba_hash(self) }
    }

    pub fn to_string(&self) -> Option<String> {
        let tmp = unsafe { ffi::gdk_rgba_to_string(self) as *const c_char };

        if tmp.is_null() {
            None
        } else {
            unsafe { 
                let ret = Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string());

                ::libc::funcs::c95::stdlib::free(tmp as *mut c_void);
                ret
            }
        }
    }
}