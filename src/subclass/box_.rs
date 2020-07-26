use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use BoxClass;
use ContainerClass;

pub trait BoxImpl: ContainerImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for BoxClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
