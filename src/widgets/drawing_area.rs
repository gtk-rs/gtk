// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

struct_Widget!(DrawingArea);

impl DrawingArea {
    pub fn new() -> Option<DrawingArea> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_drawing_area_new() };
        check_pointer!(tmp_pointer, DrawingArea)
    }
}

impl_TraitWidget!(DrawingArea);
