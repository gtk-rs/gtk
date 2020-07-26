use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRendererClass;
use CellRendererPixbufClass;

pub trait CellRendererPixbufImpl: CellRendererImpl {}

unsafe impl<T: CellRendererPixbufImpl> IsSubclassable<T> for CellRendererPixbufClass {
    fn override_vfuncs(&mut self) {
        <CellRendererClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
