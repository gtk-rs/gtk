// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib_ffi;
use gobject_ffi;
use glib::translate::*;
use glib::object::{Downcast, IsA};
use glib::Object;
use ListBox;
use ListBoxRow;

use gio;

use std::cell::RefCell;
use std::mem::transmute;

pub trait ListBoxExtManual {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn bind_model<'a, 'b, P: IsA<gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: FnMut(&Object) -> ListBoxRow + 'static>(&self, model: Q, create_widget_func: R);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn selected_foreach<P: FnMut(&Self, &ListBoxRow)>(&self, func: P);

    //#[cfg(any(feature = "v3_10", feature = "dox"))]
    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxFilterFunc>>>(&self, filter_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //#[cfg(any(feature = "v3_10", feature = "dox"))]
    //fn set_header_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxUpdateHeaderFunc>>>(&self, update_header: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //#[cfg(any(feature = "v3_10", feature = "dox"))]
    //fn set_sort_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxSortFunc>>>(&self, sort_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);
}

impl<O: IsA<ListBox> + IsA<Object>> ListBoxExtManual for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn bind_model<'a, 'b, P: IsA<gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: FnMut(&Object) -> ListBoxRow + 'static>(&self, model: Q, create_widget_func: R) {
        unsafe extern "C" fn bind_model_trampoline(
            item: *mut gobject_ffi::GObject,
            func: glib_ffi::gpointer,
        ) -> *mut ffi::GtkWidget
        {
            let func: &RefCell<Box<FnMut(&Object) -> ListBoxRow + 'static>> = transmute(func);

            (&mut *func.borrow_mut())(
                &from_glib_borrow(item),
            ).to_glib_full()
        }

        unsafe extern "C" fn destroy_closure(func: glib_ffi::gpointer) {
            Box::<RefCell<Box<FnMut(&Object) -> ListBoxRow + 'static>>>::from_raw(
                func as *mut _
            );
        }

        let func: Box<RefCell<Box<FnMut(&Object) -> ListBoxRow + 'static>>> =
            Box::new(RefCell::new(Box::new(create_widget_func)));
        let destroy_closure = destroy_closure;

        let model = model.into();
        let model = model.to_glib_none();

        unsafe {
            ffi::gtk_list_box_bind_model(
                self.to_glib_none().0,
                model.0,
                Some(bind_model_trampoline),
                Box::into_raw(func) as glib_ffi::gpointer,
                Some(destroy_closure),
            )
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn selected_foreach<P: FnMut(&Self, &ListBoxRow)>(&self, func: P) {
        unsafe extern "C" fn foreach_trampoline<T>(
            this: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            func: glib_ffi::gpointer,
        ) where T: IsA<ListBox>
        {
            let func = func as *const &mut (FnMut(&T, &ListBoxRow));

            (*func)(
                &ListBox::from_glib_borrow(this).downcast_unchecked(),
                &from_glib_borrow(row),
            )
        }

        unsafe {
            let mut func = func;
            let func_obj: &mut (FnMut(&Self, &ListBoxRow)) = &mut func;
            let func_ptr = &func_obj as *const &mut (FnMut(&Self, &ListBoxRow)) as glib_ffi::gpointer;

            ffi::gtk_list_box_selected_foreach(
                self.to_glib_none().0,
                Some(foreach_trampoline::<Self>),
                func_ptr
            )
        }
    }

    //#[cfg(any(feature = "v3_10", feature = "dox"))]
    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxFilterFunc>>>(&self, filter_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_filter_func() }
    //}

    //#[cfg(any(feature = "v3_10", feature = "dox"))]
    //fn set_header_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxUpdateHeaderFunc>>>(&self, update_header: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_header_func() }
    //}

    //#[cfg(any(feature = "v3_10", feature = "dox"))]
    //fn set_sort_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxSortFunc>>>(&self, sort_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_sort_func() }
    //}
}
