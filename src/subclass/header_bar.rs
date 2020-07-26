use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use ContainerClass;
use HeaderBarClass;

pub trait HeaderBarImpl: ContainerImpl {}

unsafe impl<T: HeaderBarImpl> IsSubclassable<T> for HeaderBarClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
