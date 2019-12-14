use glib::subclass::prelude::*;

use super::bin::BinImpl;
use BinClass;
use EventBoxClass;

pub trait EventBoxImpl: BinImpl + 'static {}

unsafe impl<T: ObjectSubclass + EventBoxImpl> IsSubclassable<T> for EventBoxClass {
    fn override_vfuncs(&mut self) {
        <BinClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
