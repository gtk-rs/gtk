// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use TreeIter;
use TreeModel;
use TreeModelFlags;
use TreePath;
use auto::traits::TreeModelExt as Auto;

pub trait TreeModelExt {
    // -- manual --
    fn get_value(&self, iter: &TreeIter, column: i32) -> Value;
    // -- auto --
    fn filter_new(&self, root: Option<&TreePath>) -> Option<TreeModel>;
    fn get_column_type(&self, index_: i32) -> glib::types::Type;
    fn get_flags(&self) -> TreeModelFlags;
    fn get_iter(&self, path: &TreePath) -> Option<TreeIter>;
    fn get_iter_first(&self) -> Option<TreeIter>;
    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter>;
    fn get_n_columns(&self) -> i32;
    fn get_path(&self, iter: &TreeIter) -> TreePath;
    fn get_string_from_iter(&self, iter: &mut TreeIter) -> Option<String>;
    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter>;
    fn iter_has_child(&self, iter: &TreeIter) -> bool;
    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;
    fn iter_next(&self, iter: &mut TreeIter) -> bool;
    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter>;
    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter>;
    fn iter_previous(&self, iter: &mut TreeIter) -> bool;
    fn row_changed(&self, path: &TreePath, iter: &TreeIter);
    fn row_deleted(&self, path: &TreePath);
    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter);
    fn row_inserted(&self, path: &TreePath, iter: &TreeIter);
    fn sort_new_with_model(&self) -> Option<TreeModel>;
}

impl<O: IsA<TreeModel>> TreeModelExt for O {
    fn get_value(&self, iter: &TreeIter, column: i32) -> Value {
        unsafe {
            let mut value = Value::new();
            ffi::gtk_tree_model_get_value(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0), column, value.as_mut_ptr());
            value
        }
    }

    #[inline]
    fn filter_new(&self, root: Option<&TreePath>) -> Option<TreeModel> {
        Auto::filter_new(self, root)
    }

    #[inline]
    fn get_column_type(&self, index_: i32) -> glib::types::Type {
        Auto::get_column_type(self, index_)
    }

    #[inline]
    fn get_flags(&self) -> TreeModelFlags {
        Auto::get_flags(self)
    }

    #[inline]
    fn get_iter(&self, path: &TreePath) -> Option<TreeIter> {
        Auto::get_iter(self, path)
    }

    #[inline]
    fn get_iter_first(&self) -> Option<TreeIter> {
        Auto::get_iter_first(self)
    }

    #[inline]
    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        Auto::get_iter_from_string(self, path_string)
    }

    #[inline]
    fn get_n_columns(&self) -> i32 {
        Auto::get_n_columns(self)
    }

    #[inline]
    fn get_path(&self, iter: &TreeIter) -> TreePath {
        Auto::get_path(self, iter)
    }

    #[inline]
    fn get_string_from_iter(&self, iter: &mut TreeIter) -> Option<String> {
        Auto::get_string_from_iter(self, iter)
    }

    #[inline]
    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        Auto::iter_children(self, parent)
    }

    #[inline]
    fn iter_has_child(&self, iter: &TreeIter) -> bool {
        Auto::iter_has_child(self, iter)
    }

    #[inline]
    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        Auto::iter_n_children(self, iter)
    }

    #[inline]
    fn iter_next(&self, iter: &mut TreeIter) -> bool {
        Auto::iter_next(self, iter)
    }

    #[inline]
    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        Auto::iter_nth_child(self, parent, n)
    }

    #[inline]
    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter> {
        Auto::iter_parent(self, child)
    }

    #[inline]
    fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        Auto::iter_previous(self, iter)
    }

    #[inline]
    fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        Auto::row_changed(self, path, iter)
    }

    #[inline]
    fn row_deleted(&self, path: &TreePath) {
        Auto::row_deleted(self, path)
    }

    #[inline]
    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        Auto::row_has_child_toggled(self, path, iter)
    }

    #[inline]
    fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        Auto::row_inserted(self, path, iter)
    }

    #[inline]
    fn sort_new_with_model(&self) -> Option<TreeModel> {
        Auto::sort_new_with_model(self)
    }

}
