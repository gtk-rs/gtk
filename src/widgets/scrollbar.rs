// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

struct_Widget!(ScrollBar);

impl ScrollBar {
    pub fn new(orientation: ::Orientation, adjustment: &::Adjustment) -> Option<ScrollBar> {
        skip_assert_initialized!();
        let tmp_pointer = unsafe { ffi::gtk_scrollbar_new(orientation, adjustment.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ScrollBar)
    }
}

impl_drop!(ScrollBar);
impl_TraitWidget!(ScrollBar);

impl ::RangeTrait for ScrollBar {}
impl ::OrientableTrait for ScrollBar {}
