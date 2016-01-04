// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use std::cmp::{PartialEq, Eq};
use PaperSize;

impl PartialEq for PaperSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_paper_size_is_equal(mut_override(self.to_glib_none().0),
                    mut_override(other.to_glib_none().0)))
        }
    }
}

impl Eq for PaperSize {}
