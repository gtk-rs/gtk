// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use TextAttributes;
use TextIter;

impl TextIter {
    pub fn get_attributes(&self, values: &TextAttributes) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_text_iter_get_attributes(self.to_glib_none().0,
                    mut_override(values.to_glib_none().0)))
        }
    }
}
