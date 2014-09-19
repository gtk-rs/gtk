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

use gtk::ffi;

pub struct TreeIter {
    pointer: *mut ffi::C_GtkTreeIter
}

impl TreeIter {
    pub fn copy(&self) -> Option<TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_tree_iter_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreeIter {
                pointer: tmp_pointer
            })
        }
    }

    pub fn drop(&mut self) {
        unsafe { ffi::gtk_tree_iter_free(self.pointer) };
        self.pointer = ::std::ptr::null_mut();
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_GtkTreeIter {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treeiter: *mut ffi::C_GtkTreeIter) -> TreeIter {
        TreeIter {
            pointer: c_treeiter
        }
    }
}