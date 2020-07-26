use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use CellRendererSpinClass;
use CellRendererTextClass;

pub trait CellRendererSpinImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererSpinImpl> IsSubclassable<T> for CellRendererSpinClass {
    fn override_vfuncs(&mut self) {
        <CellRendererTextClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
