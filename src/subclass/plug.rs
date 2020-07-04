// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::subclass::prelude::*;

use glib::translate::*;

use super::window::WindowImpl;
use Plug;
use PlugClass;
use WindowClass;

pub trait PlugImpl: PlugImplExt + WindowImpl + 'static {
    fn embedded(&self, plug: &Plug) {
        self.parent_embedded(plug)
    }
}

pub trait PlugImplExt {
    fn parent_embedded(&self, plug: &Plug);
}

impl<T: PlugImpl + ObjectImpl> PlugImplExt for T {
    fn parent_embedded(&self, plug: &Plug) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkPlugClass;
            if let Some(f) = (*parent_class).embedded {
                f(plug.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + PlugImpl> IsSubclassable<T> for PlugClass {
    fn override_vfuncs(&mut self) {
        <WindowClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkPlugClass);
            klass.embedded = Some(plug_embedded::<T>);
        }
    }
}

unsafe extern "C" fn plug_embedded<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkPlug)
where
    T: PlugImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Plug> = from_glib_borrow(ptr);

    imp.embedded(&wrap)
}
