// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use ContainerClass;
use FixedClass;

pub trait FixedImpl: ContainerImpl {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for FixedClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
