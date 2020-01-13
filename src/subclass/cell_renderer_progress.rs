use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRendererClass;
use CellRendererProgressClass;

pub trait CellRendererProgressImpl: CellRendererImpl + 'static {}

unsafe impl<T: ObjectSubclass + CellRendererProgressImpl> IsSubclassable<T>
    for CellRendererProgressClass
{
    fn override_vfuncs(&mut self) {
        <CellRendererClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
