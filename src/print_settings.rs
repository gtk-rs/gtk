// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::IsA;
use glib::translate::*;

use PageRange;
use PrintSettings;

pub trait PrintSettingsExtManual: 'static {
    fn set_page_ranges(&self, page_ranges: &[PageRange]);
}

impl<O: IsA<PrintSettings>> PrintSettingsExtManual for O {
    fn set_page_ranges(&self, page_ranges: &[PageRange]) {
        let num_ranges = page_ranges.len() as i32;
        unsafe {
            ffi::gtk_print_settings_set_page_ranges(self.as_ref().to_glib_none().0,
                                                    mut_override(page_ranges.as_ptr() as *const _),
                                                    num_ranges);
        }
    }
}
