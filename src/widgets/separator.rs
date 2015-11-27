// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use Orientation;

struct_Widget!(Separator);

impl Separator {
    pub fn new(orientation: Orientation) -> Option<Separator> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_separator_new(orientation) };
        check_pointer!(tmp_pointer, Separator)
    }
}

impl_drop!(Separator);
impl_TraitWidget!(Separator);

impl ::OrientableTrait for Separator {}
