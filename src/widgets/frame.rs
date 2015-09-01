// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A bin with a decorative frame and optional label

use glib::translate::ToGlibPtr;
use ffi;

/// Frame — A bin with a decorative frame and optional label
struct_Widget!(Frame);

impl Frame {
    pub fn new(label: Option<&str>) -> Option<Frame> {
        let tmp_pointer = unsafe {
            ffi::gtk_frame_new(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Frame)
    }
}

impl_drop!(Frame);
impl_TraitWidget!(Frame);

impl ::FrameTrait for Frame {}
impl ::ContainerTrait for Frame {}
impl ::BinTrait for Frame {}
