use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use ContainerClass;
use IconViewClass;

pub trait IconViewImpl: ContainerImpl + 'static {}

unsafe impl<T: ObjectSubclass + IconViewImpl> IsSubclassable<T> for IconViewClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
