// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

struct_Widget!(VolumeButton);

impl VolumeButton {
    pub fn new() -> Option<VolumeButton> {
        assert_initialized_main_thread!();
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
