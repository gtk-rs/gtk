// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use std::mem;
use std::ptr;
use IconSet;
use IconSize;

impl IconSet {
    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_sizes(&self) -> Vec<IconSize> {
        unsafe {
            let mut sizes = ptr::null_mut();
            let mut n_sizes = mem::uninitialized();
            ffi::gtk_icon_set_get_sizes(self.to_glib_none().0, &mut sizes, &mut n_sizes);
            let v: Vec<i32> = FromGlibContainer::from_glib_full_num(sizes, n_sizes as usize);
            v.iter().map(|i| IconSize::from_glib(*i)).collect()
        }
    }
}
