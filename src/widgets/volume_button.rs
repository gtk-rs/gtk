// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button which pops up a volume control

use ffi;

/// VolumeButton — A button which pops up a volume control
struct_Widget!(VolumeButton);

impl VolumeButton {
    pub fn new() -> Option<VolumeButton> {
        let tmp_pointer = unsafe { ffi::gtk_volume_button_new() };
        check_pointer!(tmp_pointer, VolumeButton)
    }
}

impl_drop!(VolumeButton);
impl_TraitWidget!(VolumeButton);

impl ::ContainerTrait for VolumeButton {}
impl ::ButtonTrait for VolumeButton {}
impl ::ScaleButtonTrait for VolumeButton {}
impl ::OrientableTrait for VolumeButton {}
