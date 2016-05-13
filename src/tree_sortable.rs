// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use SortType;
use ffi;
use glib::object::{Downcast, IsA};
use glib::translate::*;
use std::mem::{self, transmute};
use std::cmp::Ordering;

use glib_ffi::gpointer;
use {TreeIter, TreeModel, TreeSortable};
use ffi::{GtkTreeIter, GtkTreeModel};

pub enum SortColumn {
    Default,
    Index(u32),
}

#[doc(hidden)]
impl ToGlib for SortColumn {
    type GlibType = i32;

    #[inline]
    fn to_glib(&self) -> i32 {
        match *self {
            SortColumn::Default => ffi::GTK_TREE_SORTABLE_DEFAULT_SORT_COLUMN_ID,
            SortColumn::Index(x) => {
                assert!(x <= i32::max_value() as u32, "column index is too big");
                x as i32
            }
        }
    }
}

#[doc(hidden)]
impl FromGlib<i32> for SortColumn {
    #[inline]
    fn from_glib(val: i32) -> SortColumn {
        skip_assert_initialized!();
        match val {
            ffi::GTK_TREE_SORTABLE_DEFAULT_SORT_COLUMN_ID => SortColumn::Default,
            x => {
                assert!(x >= 0, "invalid column index");
                SortColumn::Index(x as u32)
            }
        }
    }
}

pub trait TreeSortableExtManual {
    fn set_default_sort_func<F>(&self, sort_func: F)
        where F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static;
    fn set_sort_func<F>(&self, sort_column_id: SortColumn, sort_func: F)
        where F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static;
    fn get_sort_column_id(&self) -> Option<(SortColumn, SortType)>;
    fn set_sort_column_id(&self, sort_column_id: SortColumn, order: SortType);
    fn set_unsorted(&self);
}

unsafe extern "C" fn trampoline<T>(this: *mut GtkTreeModel, iter: *mut GtkTreeIter,
                                   iter2: *mut GtkTreeIter, f: gpointer) -> i32
where T: IsA<TreeModel> {
    callback_guard!();
    let f: &Box<Fn(&T, &TreeIter, &TreeIter) -> Ordering> = transmute(f);
    f(&TreeModel::from_glib_none(this).downcast_unchecked(), &from_glib_borrow(iter),
      &from_glib_borrow(iter2)).to_glib()
}

unsafe extern "C" fn destroy_closure<T>(ptr: gpointer) {
    callback_guard!();
    Box::<Box<Fn(&T, &TreeIter, &TreeIter) -> Ordering + 'static>>::from_raw(ptr as *mut _);
}

fn into_raw<F, T>(func: F) -> gpointer
    where F: Fn(&T, &TreeIter, &TreeIter) -> Ordering + 'static {
    skip_assert_initialized!();
    let func: Box<Box<Fn(&T, &TreeIter, &TreeIter) -> Ordering + 'static>> =
        Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}

impl<O: IsA<TreeModel> + IsA<TreeSortable>> TreeSortableExtManual for O {
    #[inline]
    fn get_sort_column_id(&self) -> Option<(SortColumn, SortType)> {
        unsafe {
            let mut sort_column_id = mem::uninitialized();
            let mut order = mem::uninitialized();
            ffi::gtk_tree_sortable_get_sort_column_id(self.to_glib_none().0, &mut sort_column_id, &mut order);
            if sort_column_id != ffi::GTK_TREE_SORTABLE_UNSORTED_SORT_COLUMN_ID {
                Some((from_glib(sort_column_id), from_glib(order)))
            } else {
                None
            }
        }
    }

    fn set_default_sort_func<F>(&self, sort_func: F)
    where F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static {
        unsafe {
            ffi::gtk_tree_sortable_set_default_sort_func(self.to_glib_none().0,
                                                         Some(trampoline::<Self>),
                                                         into_raw(sort_func),
                                                         Some(destroy_closure::<Self>))
        }
    }

    #[inline]
    fn set_sort_column_id(&self, sort_column_id: SortColumn, order: SortType) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(self.to_glib_none().0, sort_column_id.to_glib(), order.to_glib());
        }
    }

    fn set_unsorted(&self) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(self.to_glib_none().0,
                                                      ffi::GTK_TREE_SORTABLE_UNSORTED_SORT_COLUMN_ID,
                                                      SortType::Ascending.to_glib());
        }
    }

    fn set_sort_func<F>(&self, sort_column_id: SortColumn, sort_func: F)
    where F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_func(self.to_glib_none().0,
                                                 sort_column_id.to_glib(),
                                                 Some(trampoline::<Self>),
                                                 into_raw(sort_func),
                                                 Some(destroy_closure::<Self>))
        }
    }
}
