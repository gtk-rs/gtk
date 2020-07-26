use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use ContainerClass;
use StackClass;

pub trait StackImpl: ContainerImpl {}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for StackClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
