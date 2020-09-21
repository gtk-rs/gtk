// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use BinClass;
use ListBoxRow;
use ListBoxRowClass;

use glib::subclass::prelude::*;
use glib::translate::*;

use super::bin::BinImpl;

pub trait ListBoxRowImpl: ListBoxRowImplExt + BinImpl {
    fn activate(&self, list_box_row: &ListBoxRow) {
        self.list_box_row_activate(list_box_row)
    }
}

pub trait ListBoxRowImplExt {
    fn list_box_row_activate(&self, list_box_row: &ListBoxRow);
}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRowClass {
    fn override_vfuncs(&mut self) {
        <BinClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkListBoxRowClass);
            klass.activate = Some(list_box_row_activate::<T>);
        }
    }
}

unsafe extern "C" fn list_box_row_activate<T: ListBoxRowImpl>(ptr: *mut gtk_sys::GtkListBoxRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBoxRow> = from_glib_borrow(ptr);

    imp.activate(&wrap)
}
