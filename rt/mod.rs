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

use std::ptr;

use ffi;
use std;
use std::owned;

pub fn init() {
    unsafe {
        ffi::gtk_init(ptr::null(), ptr::null());
    }
}

pub fn _main() {
    unsafe {
        ffi::gtk_main();
    }
}

pub fn _main_quit() {
    unsafe {
        ffi::gtk_main_quit();
    }
}

pub fn _main_level() -> u32 {
    unsafe {
        ffi::gtk_main_level() as u32
    }
}

pub fn _main_iteration() -> bool {
    match unsafe { ffi::gtk_main_iteration() } {
        ffi::Gfalse => false,
        _           => true
    }
}

pub fn _main_iteration_do(blocking: bool) -> bool {
    let c_blocking = if blocking { ffi::Gtrue } else { ffi::Gfalse };
    match unsafe { ffi::gtk_main_iteration_do(c_blocking) } {
        ffi::Gfalse => false,
        _           => true
    }
}
