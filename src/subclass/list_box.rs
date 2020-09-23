// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
use gtk_sys;

use glib::subclass::prelude::*;
use glib::translate::*;

use super::{container::ContainerImpl, widget::WidgetImpl};

use libc::c_int;

use ContainerClass;
use ListBox;
use ListBoxClass;
use ListBoxRow;
use MovementStep;

pub trait ListBoxImpl: ListBoxImplExt + ContainerImpl + WidgetImpl {
    fn activate_cursor_row(&self, list_box: &ListBox) {
        self.list_box_activate_cursor_row(list_box)
    }

    fn move_cursor(&self, list_box: &ListBox, step: MovementStep, count: i32) {
        self.list_box_move_cursor(list_box, step, count)
    }

    fn row_activated(&self, list_box: &ListBox, row: &ListBoxRow) {
        self.list_box_row_activated(list_box, row)
    }

    fn row_selected(&self, list_box: &ListBox, row: Option<&ListBoxRow>) {
        self.list_box_row_selected(list_box, row)
    }

    fn select_all(&self, list_box: &ListBox) {
        self.list_box_select_all(list_box)
    }

    fn selected_rows_changed(&self, list_box: &ListBox) {
        self.list_box_selected_rows_changed(list_box)
    }

    fn toggle_cursor_row(&self, list_box: &ListBox) {
        self.list_box_toggle_cursor_row(list_box)
    }

    fn unselect_all(&self, list_box: &ListBox) {
        self.list_box_unselect_all(list_box)
    }
}

pub trait ListBoxImplExt {
    fn list_box_activate_cursor_row(&self, list_box: &ListBox);
    fn list_box_move_cursor(&self, list_box: &ListBox, step: MovementStep, count: i32);
    fn list_box_row_activated(&self, list_box: &ListBox, row: &ListBoxRow);
    fn list_box_row_selected(&self, list_box: &ListBox, row: Option<&ListBoxRow>);
    fn list_box_select_all(&self, list_box: &ListBox);
    fn list_box_selected_rows_changed(&self, list_box: &ListBox);
    fn list_box_toggle_cursor_row(&self, list_box: &ListBox);
    fn list_box_unselect_all(&self, list_box: &ListBox);
}

impl<T: ListBoxImpl> ListBoxImplExt for T {
    fn list_box_activate_cursor_row(&self, list_box: &ListBox) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).activate_cursor_row {
                f(list_box.to_glib_none().0)
            }
        }
    }

    fn list_box_move_cursor(&self, list_box: &ListBox, step: MovementStep, count: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).move_cursor {
                f(list_box.to_glib_none().0, step.to_glib(), count);
            }
        }
    }

    fn list_box_row_activated(&self, list_box: &ListBox, row: &ListBoxRow) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).row_activated {
                f(list_box.to_glib_none().0, row.to_glib_none().0)
            }
        }
    }

    fn list_box_row_selected(&self, list_box: &ListBox, row: Option<&ListBoxRow>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).row_selected {
                f(
                    list_box.to_glib_none().0,
                    mut_override(row.to_glib_none().0),
                )
            }
        }
    }

    fn list_box_select_all(&self, list_box: &ListBox) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).select_all {
                f(list_box.to_glib_none().0)
            }
        }
    }

    fn list_box_selected_rows_changed(&self, list_box: &ListBox) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).selected_rows_changed {
                f(list_box.to_glib_none().0)
            }
        }
    }

    fn list_box_toggle_cursor_row(&self, list_box: &ListBox) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).toggle_cursor_row {
                f(list_box.to_glib_none().0)
            }
        }
    }

    fn list_box_unselect_all(&self, list_box: &ListBox) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxClass;
            if let Some(f) = (*parent_class).unselect_all {
                f(list_box.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ListBoxImpl> IsSubclassable<T> for ListBoxClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkListBoxClass);
            klass.activate_cursor_row = Some(list_box_activate_cursor_row::<T>);
            klass.move_cursor = Some(list_box_move_cursor::<T>);
            klass.row_activated = Some(list_box_row_activated::<T>);
            klass.row_selected = Some(list_box_row_selected::<T>);
            klass.select_all = Some(list_box_select_all::<T>);
            klass.selected_rows_changed = Some(list_box_selected_rows_changed::<T>);
            klass.toggle_cursor_row = Some(list_box_toggle_cursor_row::<T>);
            klass.unselect_all = Some(list_box_unselect_all::<T>);
        }
    }
}

unsafe extern "C" fn list_box_activate_cursor_row<T: ListBoxImpl>(ptr: *mut gtk_sys::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.activate_cursor_row(&wrap)
}

unsafe extern "C" fn list_box_move_cursor<T: ListBoxImpl>(
    ptr: *mut gtk_sys::GtkListBox,
    step: gtk_sys::GtkMovementStep,
    count: c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.move_cursor(&wrap, from_glib(step), count)
}

unsafe extern "C" fn list_box_row_activated<T: ListBoxImpl>(
    ptr: *mut gtk_sys::GtkListBox,
    rowptr: *mut gtk_sys::GtkListBoxRow,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);
    let row: Borrowed<ListBoxRow> = from_glib_borrow(rowptr);

    imp.row_activated(&wrap, &row)
}

unsafe extern "C" fn list_box_row_selected<T: ListBoxImpl>(
    ptr: *mut gtk_sys::GtkListBox,
    rowptr: *mut gtk_sys::GtkListBoxRow,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);
    let row: Borrowed<Option<ListBoxRow>> = from_glib_borrow(rowptr);

    imp.row_selected(&wrap, row.as_ref().as_ref())
}

unsafe extern "C" fn list_box_select_all<T: ListBoxImpl>(ptr: *mut gtk_sys::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.select_all(&wrap)
}

unsafe extern "C" fn list_box_selected_rows_changed<T: ListBoxImpl>(ptr: *mut gtk_sys::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.selected_rows_changed(&wrap)
}

unsafe extern "C" fn list_box_toggle_cursor_row<T: ListBoxImpl>(ptr: *mut gtk_sys::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.toggle_cursor_row(&wrap)
}

unsafe extern "C" fn list_box_unselect_all<T: ListBoxImpl>(ptr: *mut gtk_sys::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.unselect_all(&wrap)
}
