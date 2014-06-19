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

use libc::{c_int, c_uint};

use traits::GtkWidget;
use gtk::enums::{GtkPackType, GtkPackStart};
use utils::cast::GTK_BOX;
use ffi;

pub trait GtkBox: GtkWidget {
    fn pack_start<'r, T: GtkWidget>(&'r mut self, child: &'r T, expand: bool, fill: bool, padding: u32) -> () {
        let c_expand = if expand { ffi::Gtrue } else { ffi::Gfalse };
        let c_fill = if fill { ffi::Gtrue } else { ffi::Gfalse };
        unsafe {
            ffi::gtk_box_pack_start(GTK_BOX(self.get_widget()), child.get_widget(), c_expand, c_fill, padding as c_uint);
        }
    }

    fn pack_end<'r, T: GtkWidget>(&'r mut self, child: &'r T, expand: bool, fill: bool, padding: u32) -> () {
        let c_expand = if expand { ffi::Gtrue } else { ffi::Gfalse };
        let c_fill = if fill { ffi::Gtrue } else { ffi::Gfalse };
        unsafe {
            ffi::gtk_box_pack_end(GTK_BOX(self.get_widget()), child.get_widget(), c_expand, c_fill, padding as c_uint);
        }
    }

    fn get_homogeneous(&self) -> bool {
        match unsafe { ffi::gtk_box_get_homogeneous(GTK_BOX(self.get_widget())) } {
            ffi::Gfalse => false,
            _           => true
        }
    }

    fn set_homogeneouse(&mut self, homogeneous: bool) -> () {
        match homogeneous {
            true    => unsafe { ffi::gtk_box_set_homogeneous(GTK_BOX(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_box_set_homogeneous(GTK_BOX(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(GTK_BOX(self.get_widget())) as i32
        }
    }

    fn set_spacing(&mut self, spacing: i32) -> () {
        unsafe {
            ffi::gtk_box_set_spacing(GTK_BOX(self.get_widget()), spacing as c_int);
        }
    }

    fn reorder_child<'r, T: GtkWidget>(&'r mut self, child: &'r T, position: i32) -> () {
        unsafe {
            ffi::gtk_box_reorder_child(GTK_BOX(self.get_widget()), child.get_widget(), position as c_int);
        }
    }

    fn query_child_packing<'r, T: GtkWidget>(&self, child: &'r T) -> (bool, bool, u32, GtkPackType) {
        let c_expand = 0;
        let c_padding = 0;
        let c_fill = 0;
        let pack_type: GtkPackType = GtkPackStart;
        unsafe {
            ffi::gtk_box_query_child_packing(GTK_BOX(self.get_widget()), child.get_widget(), &c_expand, &c_fill, &c_padding, &pack_type);
        }
        let expand = if c_expand == ffi::Gfalse { false } else { true };
        let fill = if c_fill == ffi::Gfalse { false } else { true };
        (expand, fill, c_padding as u32, pack_type)
    }

    fn set_child_packing<'r, T: GtkWidget>(&mut self, child: &'r T, expand: bool, fill: bool, padding: u32, pack_type: GtkPackType) -> () {
        let c_expand = if expand { ffi::Gtrue } else { ffi::Gfalse };
        let c_fill = if fill { ffi::Gtrue } else { ffi::Gfalse };
        unsafe {
            ffi::gtk_box_set_child_packing(GTK_BOX(self.get_widget()), child.get_widget(), c_expand, c_fill, padding as c_uint, pack_type);
        }
    }
}

