// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTreeSelection — The selection object for GtkTreeView

use std::mem;
use std::ptr;

use glib::translate::*;
use ffi;

use {
    TreeIter,
    TreeModel,
    TreePath,
    TreeView,
};

use SelectionMode;

glib_wrapper! {
    pub struct TreeSelection(Object<ffi::GtkTreeSelection>);

    match fn {
        get_type => || ffi::gtk_tree_selection_get_type(),
    }
}

impl TreeSelection {
    pub fn set_mode(&self, mode: SelectionMode) {
        unsafe { ffi::gtk_tree_selection_set_mode(self.to_glib_none().0, mode) }
    }

    pub fn get_mode(&self) -> SelectionMode{
        unsafe { ffi::gtk_tree_selection_get_mode(self.to_glib_none().0) }
    }

    pub unsafe fn get_user_data<'r, T>(&self) -> &'r mut T {
        mem::transmute(ffi::gtk_tree_selection_get_user_data(self.to_glib_none().0))
    }

    pub fn get_tree_view(&self) -> TreeView {
        unsafe { from_glib_none(ffi::gtk_tree_selection_get_tree_view(self.to_glib_none().0)) }
    }

    pub fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ok = from_glib(
                ffi::gtk_tree_selection_get_selected(self.to_glib_none().0,
                    &mut model, iter.to_glib_none_mut().0));
            if ok {
                Some((from_glib_none(model), iter))
            }
            else {
                None
            }
        }
    }

    pub fn count_selected_rows(&self) -> i32 {
        unsafe { ffi::gtk_tree_selection_count_selected_rows(self.to_glib_none().0) }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_path(self.to_glib_none().0,
                mut_override(path.to_glib_none().0))
        }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_path(self.to_glib_none().0,
                mut_override(path.to_glib_none().0))
        }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_selection_path_is_selected(self.to_glib_none().0,
                    mut_override(path.to_glib_none().0)))
        }
    }

    pub fn select_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_select_iter(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0)) }
    }

    pub fn unselect_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_unselect_iter(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0))
        }
    }

    pub fn iter_is_selected(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_selection_iter_is_selected(self.to_glib_none().0,
                    mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn select_all(&self) {
        unsafe { ffi::gtk_tree_selection_select_all(self.to_glib_none().0) }
    }

    pub fn unselect_all(&self) {
        unsafe { ffi::gtk_tree_selection_unselect_all(self.to_glib_none().0) }
    }

    pub fn select_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_range(self.to_glib_none().0,
                mut_override(start_path.to_glib_none().0), mut_override(end_path.to_glib_none().0))
        }
    }

    pub fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_range(self.to_glib_none().0,
                mut_override(start_path.to_glib_none().0), mut_override(end_path.to_glib_none().0))
        }
    }
}
