use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use BinClass;
use ContainerClass;

pub trait BinImpl: ContainerImpl {}

unsafe impl<T: BinImpl> IsSubclassable<T> for BinClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
