// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A separator widget

use ffi;
use Orientation;

/// Separator — A separator widget
struct_Widget!(Separator);

impl Separator {
    pub fn new(orientation: Orientation) -> Option<Separator> {
        let tmp_pointer = unsafe { ffi::gtk_separator_new(orientation) };
        check_pointer!(tmp_pointer, Separator)
    }
}

impl_drop!(Separator);
impl_TraitWidget!(Separator);

impl ::OrientableTrait for Separator {}

impl_widget_events!(Separator);
