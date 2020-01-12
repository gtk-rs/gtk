use gtk_sys;

use libc::c_char;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::GString;

use super::cell_renderer::CellRendererImpl;
use CellRendererClass;
use CellRendererText;
use CellRendererTextClass;

pub trait CellRendererTextImpl: CellRendererTextImplExt + CellRendererImpl + 'static {
    fn edited(&self, renderer: &CellRendererText, path: &str, new_text: &str) {
        self.parent_edited(renderer, path, new_text);
    }
}

pub trait CellRendererTextImplExt {
    fn parent_edited(&self, renderer: &CellRendererText, path: &str, new_text: &str);
}

impl<T: CellRendererTextImpl + ObjectImpl> CellRendererTextImplExt for T {
    fn parent_edited(&self, renderer: &CellRendererText, path: &str, new_text: &str) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererTextClass;
            if let Some(f) = (*parent_class).edited {
                f(
                    renderer.to_glib_none().0,
                    path.to_glib_none().0,
                    new_text.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + CellRendererTextImpl> IsSubclassable<T> for CellRendererTextClass {
    fn override_vfuncs(&mut self) {
        <CellRendererClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkCellRendererTextClass);
            klass.edited = Some(cell_renderer_text_edited::<T>);
        }
    }
}

unsafe extern "C" fn cell_renderer_text_edited<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkCellRendererText,
    path: *const c_char,
    new_text: *const c_char,
) where
    T: CellRendererTextImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.edited(
        &wrap,
        &GString::from_glib_borrow(path),
        &GString::from_glib_borrow(new_text),
    )
}
