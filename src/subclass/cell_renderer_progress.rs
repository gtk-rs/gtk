use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRendererClass;
use CellRendererProgressClass;

pub trait CellRendererProgressImpl: CellRendererImpl {}

unsafe impl<T: CellRendererProgressImpl> IsSubclassable<T> for CellRendererProgressClass {
    fn override_vfuncs(&mut self) {
        <CellRendererClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
