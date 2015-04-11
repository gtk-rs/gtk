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

use PackType;
use cast::GTK_BOX;
use ffi;
use glib::{to_bool, to_gboolean};

pub trait BoxTrait: ::WidgetTrait {
    fn pack_start<'r, T: ::WidgetTrait>(&'r mut self, child: &'r T, expand: bool, fill: bool, padding: u32) -> () {
        unsafe {
            ffi::gtk_box_pack_start(GTK_BOX(self.unwrap_widget()), child.unwrap_widget(),
                                    to_gboolean(expand), to_gboolean(fill),
                                    padding as c_uint);
        }
    }

    fn pack_end<'r, T: ::WidgetTrait>(&'r mut self, child: &'r T, expand: bool, fill: bool, padding: u32) -> () {
        unsafe {
            ffi::gtk_box_pack_end(GTK_BOX(self.unwrap_widget()), child.unwrap_widget(),
                                  to_gboolean(expand), to_gboolean(fill),
                                  padding as c_uint);
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe { to_bool(ffi::gtk_box_get_homogeneous(GTK_BOX(self.unwrap_widget()))) }
    }

    fn set_homogeneouse(&mut self, homogeneous: bool) -> () {
        unsafe { ffi::gtk_box_set_homogeneous(GTK_BOX(self.unwrap_widget()), to_gboolean(homogeneous)); }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(GTK_BOX(self.unwrap_widget())) as i32
        }
    }

    fn set_spacing(&mut self, spacing: i32) -> () {
        unsafe {
            ffi::gtk_box_set_spacing(GTK_BOX(self.unwrap_widget()), spacing as c_int);
        }
    }

    fn reorder_child<'r, T: ::WidgetTrait>(&'r mut self, child: &'r T, position: i32) -> () {
        unsafe {
            ffi::gtk_box_reorder_child(GTK_BOX(self.unwrap_widget()), child.unwrap_widget(), position as c_int);
        }
    }

    fn query_child_packing<'r, T: ::WidgetTrait>(&self, child: &'r T) -> (bool, bool, u32, PackType) {
        let mut c_expand = 0;
        let mut c_padding = 0;
        let mut c_fill = 0;
        let mut pack_type: PackType = PackType::Start;
        unsafe {
            ffi::gtk_box_query_child_packing(GTK_BOX(self.unwrap_widget()),
                                             child.unwrap_widget(),
                                             &mut c_expand,
                                             &mut c_fill,
                                             &mut c_padding,
                                             &mut pack_type);
        }
        (to_bool(c_expand), to_bool(c_fill), c_padding as u32, pack_type)
    }

    fn set_child_packing<'r, T: ::WidgetTrait>(&mut self,
                                                  child: &'r T,
                                                  expand: bool,
                                                  fill: bool,
                                                  padding: u32,
                                                  pack_type: PackType) {
        unsafe {
            ffi::gtk_box_set_child_packing(GTK_BOX(self.unwrap_widget()),
                                           child.unwrap_widget(),
                                           to_gboolean(expand),
                                           to_gboolean(fill),
                                           padding as c_uint,
                                           pack_type);
        }
    }
}
