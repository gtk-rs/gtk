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

//! A widget with two adjustable panes

use libc::c_int;

use gtk::enums::Orientation;
use gtk::cast::GTK_PANED;
use gtk::ffi;
use gtk::traits;
use gtk;

/**
* Paned — A widget with two adjustable panes
*
* # Available signals:
* * `accept-position` : Action
* * `cancel-position` : Action
* * `cycle-child-focus` : Action
* * `cycle-handle-focus` : Action
* * `move-handle` : Action
* * `toggle-handle-focus` : Action
*/
struct_Widget!(Paned)

impl Paned {
    pub fn new(orientation: Orientation) -> Option<Paned> {
        let tmp_pointer = unsafe { ffi::gtk_paned_new(orientation) };
        check_pointer!(tmp_pointer, Paned)
    }

    pub fn add1<T: traits::Widget>(&mut self, child: &T) -> () {
        unsafe {
            ffi::gtk_paned_add1(GTK_PANED(self.pointer), child.get_widget())
        }
    }

    pub fn add2<T: traits::Widget>(&mut self, child: &T) -> () {
        unsafe {
            ffi::gtk_paned_add2(GTK_PANED(self.pointer), child.get_widget())
        }
    }

    pub fn pack1<T: traits::Widget>(&mut self, child: &T, resize: bool, schrink: bool) -> () {
        let r = if resize { ffi::Gtrue } else { ffi::Gfalse };
        let s = if schrink { ffi::Gtrue } else { ffi::Gfalse };
        unsafe {
            ffi::gtk_paned_pack1(GTK_PANED(self.pointer), child.get_widget(), r, s);
        }
    }

    pub fn pack2<T: traits::Widget>(&mut self, child: &T, resize: bool, schrink: bool) -> () {
        let r = if resize { ffi::Gtrue } else { ffi::Gfalse };
        let s = if schrink { ffi::Gtrue } else { ffi::Gfalse };
        unsafe {
            ffi::gtk_paned_pack2(GTK_PANED(self.pointer), child.get_widget(), r, s);
        }
    }

    pub fn set_position(&mut self, position: i32) -> () {
        unsafe {
            ffi::gtk_paned_set_position(GTK_PANED(self.pointer), position as c_int);
        }
    }

    pub fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_paned_get_position(GTK_PANED(self.pointer)) as i32
        }
    }

    pub fn get_handle_window(&self) -> gtk::Window {
        unsafe {
            traits::Widget::wrap(ffi::gtk_paned_get_handle_window(GTK_PANED(self.pointer)))
        }
    }
}

impl_drop!(Paned)
impl_TraitWidget!(Paned)

impl traits::Container for Paned {}
