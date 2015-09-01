// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

    pub fn put<T: ::WidgetTrait>(&self, child: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(GTK_LAYOUT(self.pointer),
                                child.unwrap_widget(),
                                x,
                                y)
        }
    }

    // FIXME: search a new name
    pub fn move_<T: ::WidgetTrait>(&self, child: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(GTK_LAYOUT(self.pointer),
                                 child.unwrap_widget(),
                                 x,
                                 y)
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(GTK_LAYOUT(self.pointer), width, height)
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
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
