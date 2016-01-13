// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use TextAttributes;
use TextView;

impl TextView {
    pub fn get_default_attributes(&self) -> TextAttributes {
        unsafe {
            from_glib_full(ffi::gtk_text_view_get_default_attributes(self.to_glib_none().0))
        }
    }
}
