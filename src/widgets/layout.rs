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

//! Infinite scrollable area containing child widgets and/or custom drawing

use ffi;
use cast::{GTK_LAYOUT};

/// GtkLayout — Infinite scrollable area containing child widgets and/or custom drawing
struct_Widget!(Layout);

impl Layout {
    pub fn new(hadjustment: &::Adjustment, vadjustment: &::Adjustment) -> Option<Layout> {
        let tmp_pointer = unsafe {
            ffi::gtk_layout_new(hadjustment.unwrap_pointer(),
                                vadjustment.unwrap_pointer())
        };
        check_pointer!(tmp_pointer, Layout)
    }

    pub fn put<T: ::WidgetTrait>(&mut self, child: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(GTK_LAYOUT(self.pointer),
                                child.unwrap_widget(),
                                x,
                                y)
        }
    }

    // FIXME: search a new name
    pub fn move_<T: ::WidgetTrait>(&mut self, child: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(GTK_LAYOUT(self.pointer),
                                 child.unwrap_widget(),
                                 x,
                                 y)
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(GTK_LAYOUT(self.pointer), width, height)
        }
    }

    pub fn get_size(&mut self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;

        unsafe { ffi::gtk_layout_get_size(GTK_LAYOUT(self.pointer), &mut width, &mut height); }
        (width, height)
    }
}

impl_drop!(Layout);
impl_TraitWidget!(Layout);

impl ::ContainerTrait for Layout {}
impl ::ScrollableTrait for Layout {}

impl_widget_events!(Layout);
