// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! GtkTreeSelection — The selection object for GtkTreeView

use glib;
use gtk::{mod, ffi};
use gtk::{TreeView, TreePath, TreeIter};

pub struct TreeSelection {
    pointer: *mut ffi::C_GtkTreeSelection
}

impl TreeSelection {
    pub fn set_mode(&self, mode: gtk::SelectionMode) {
        unsafe { ffi::gtk_tree_selection_set_mode(self.pointer, mode) }
    }

    pub fn get_mode(&self) -> gtk::SelectionMode{
        unsafe { ffi::gtk_tree_selection_get_mode(self.pointer) }
    }

    pub fn get_user_data<'r, T>(&mut self) -> &'r mut T {
        unsafe { ::std::mem::transmute(ffi::gtk_tree_selection_get_user_data(self.pointer)) }
    }

    pub fn get_tree_view(&self) -> Option<TreeView> {
        let tmp_pointer = unsafe { ffi::gtk_tree_selection_get_tree_view(self.pointer) } as *mut ffi::C_GtkWidget;

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn get_selected(&self, model: &gtk::TreeModel, iter: &mut gtk::TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_selection_get_selected(self.pointer, &mut model.get_pointer(),
            iter.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn count_selected_rows(&self) -> i32 {
        unsafe { ffi::gtk_tree_selection_count_selected_rows(self.pointer) }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_select_path(self.pointer, path.get_pointer()) }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_unselect_path(self.pointer, path.get_pointer()) }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        match unsafe { ffi::gtk_tree_selection_path_is_selected(self.pointer, path.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn select_iter(&self, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_selection_select_iter(self.pointer, iter.get_pointer()) }
    }

    pub fn unselect_iter(&self, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_selection_unselect_iter(self.pointer, iter.get_pointer()) }
    }

    pub fn iter_is_selected(&self, iter: &TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_selection_iter_is_selected(self.pointer, iter.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn select_all(&self) {
        unsafe { ffi::gtk_tree_selection_select_all(self.pointer) }
    }

    pub fn unselect_all(&self) {
        unsafe { ffi::gtk_tree_selection_unselect_all(self.pointer) }
    }

    pub fn select_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_select_range(self.pointer, start_path.get_pointer(),
            end_path.get_pointer()) }
    }

    pub fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe { ffi::gtk_tree_selection_unselect_range(self.pointer, start_path.get_pointer(),
            end_path.get_pointer()) }
    }

    pub fn wrap(pointer: *mut ffi::C_GtkTreeSelection) -> Option<TreeSelection> {
        if pointer.is_null() {
            None
        } else {
            Some(TreeSelection { pointer: pointer })
        }
    }
}

impl glib::traits::FFIGObject for TreeSelection {
    fn get_gobject(&self) -> *mut glib::ffi::C_GObject {
        gtk::cast::G_OBJECT_FROM_TREE_SELECTION(self.pointer)
    }
}

impl_connect!(TreeSelection -> Changed);

impl_drop!(TreeSelection, GTK_TREE_SELECTION);
