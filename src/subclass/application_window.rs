use glib::subclass::prelude::*;

use super::window::WindowImpl;
use ApplicationWindowClass;
use WindowClass;

pub trait ApplicationWindowImpl: WindowImpl {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindowClass {
    fn override_vfuncs(&mut self) {
        <WindowClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
