use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use DrawingAreaClass;
use WidgetClass;

pub trait DrawingAreaImpl: WidgetImpl {}

unsafe impl<T: DrawingAreaImpl> IsSubclassable<T> for DrawingAreaClass {
    fn override_vfuncs(&mut self) {
        <WidgetClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
