use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use ContainerClass;
use TreeViewClass;

pub trait TreeViewImpl: ContainerImpl {}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for TreeViewClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
