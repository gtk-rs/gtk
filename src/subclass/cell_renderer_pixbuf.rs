use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRendererClass;
use CellRendererPixbufClass;

pub trait CellRendererPixbufImpl: CellRendererImpl + 'static {}

unsafe impl<T: ObjectSubclass + CellRendererPixbufImpl> IsSubclassable<T>
    for CellRendererPixbufClass
{
    fn override_vfuncs(&mut self) {
        <CellRendererClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
