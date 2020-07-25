use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use CellRendererComboClass;
use CellRendererTextClass;

pub trait CellRendererComboImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererComboImpl> IsSubclassable<T> for CellRendererComboClass {
    fn override_vfuncs(&mut self) {
        <CellRendererTextClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
