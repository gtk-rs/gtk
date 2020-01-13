use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRendererClass;
use CellRendererSpinnerClass;

pub trait CellRendererSpinnerImpl: CellRendererImpl + 'static {}

unsafe impl<T: ObjectSubclass + CellRendererSpinnerImpl> IsSubclassable<T>
    for CellRendererSpinnerClass
{
    fn override_vfuncs(&mut self) {
        <CellRendererClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
