// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use std::mem;
use gtk_sys;
use SelectionData;

impl SelectionData {
    pub fn get_data(&self) -> Vec<u8> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_none_num(gtk_sys::gtk_selection_data_get_data_with_length(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }
}
