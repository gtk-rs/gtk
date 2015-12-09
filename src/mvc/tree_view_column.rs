// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget that emits a signal when clicked on

use glib::translate::*;
use ffi;

use glib::object::{Downcast, Upcast};
use Widget;
use super::tree_view::TreeView;

use {
    CellRenderer,
    TreeViewColumnSizing,
    SortType,
};

glib_wrapper! {
    pub struct TreeViewColumn(Object<ffi::GtkTreeViewColumn>): ::CellLayout, ::Buildable;

    match fn {
        get_type => || ffi::gtk_tree_view_column_get_type(),
    }
}

impl TreeViewColumn {
    pub fn new() -> TreeViewColumn {
        unsafe { from_glib_full(ffi::gtk_tree_view_column_new()) }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clear(self.to_glib_none().0)
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_spacing(self.to_glib_none().0, spacing)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_spacing(self.to_glib_none().0)
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_visible(self.to_glib_none().0, visible.to_glib())
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_visible(self.to_glib_none().0))
        }
    }

    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_resizable(self.to_glib_none().0, resizable.to_glib())
        }
    }

    pub fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_resizable(self.to_glib_none().0))
        }
    }

    pub fn set_sizing(&self, ty: TreeViewColumnSizing) {
        unsafe {
            ffi::gtk_tree_view_column_set_sizing(self.to_glib_none().0, ty)
        }
    }

    pub fn get_sizing(&self) -> TreeViewColumnSizing {
        unsafe {
            ffi::gtk_tree_view_column_get_sizing(self.to_glib_none().0)
        }
    }

    pub fn get_x_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_x_offset(self.to_glib_none().0)
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_width(self.to_glib_none().0)
        }
    }

    pub fn get_fixed_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_fixed_width(self.to_glib_none().0)
        }
    }

    pub fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_fixed_width(self.to_glib_none().0, fixed_width)
        }
    }

    pub fn set_min_width(&self, min_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_min_width(self.to_glib_none().0, min_width)
        }
    }

    pub fn set_max_width(&self, max_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_max_width(self.to_glib_none().0, max_width)
        }
    }

    pub fn get_min_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_min_width(self.to_glib_none().0)
        }
    }

    pub fn get_max_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_max_width(self.to_glib_none().0)
        }
    }

    pub fn clicked(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clicked(self.to_glib_none().0)
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_tree_view_column_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_title(self.to_glib_none().0))
        }
    }

    pub fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_expand(self.to_glib_none().0, expand.to_glib())
        }
    }

    pub fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_expand(self.to_glib_none().0))
        }
    }

    pub fn set_clickable(&self, clickable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_clickable(self.to_glib_none().0, clickable.to_glib())
        }
    }

    pub fn get_clickable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_clickable(self.to_glib_none().0))
        }
    }

    pub fn set_widget<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_tree_view_column_set_widget(self.to_glib_none().0, widget.to_glib_none().0)
        }
    }

    pub fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_widget(self.to_glib_none().0))
        }
    }

    pub fn set_alignment(&self, x_align: f32) {
        unsafe {
            ffi::gtk_tree_view_column_set_alignment(self.to_glib_none().0, x_align)
        }
    }

    pub fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tree_view_column_get_alignment(self.to_glib_none().0)
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_reorderable(self.to_glib_none().0, reorderable.to_glib())
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_reorderable(self.to_glib_none().0))
        }
    }

    pub fn get_sort_column_id(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_column_id(self.to_glib_none().0)
        }
    }

    pub fn set_sort_column_id(&self, sort_column_id: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_column_id(self.to_glib_none().0, sort_column_id)
        }
    }

    pub fn set_sort_indicator(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_indicator(self.to_glib_none().0, setting.to_glib())
        }
    }

    pub fn get_sort_indicator(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sort_indicator(self.to_glib_none().0))
        }
    }

    pub fn set_sort_order(&self, order: SortType) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_order(self.to_glib_none().0, order)
        }
    }

    pub fn get_sort_order(&self) -> SortType {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_order(self.to_glib_none().0)
        }
    }

    pub fn column_cell_is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_cell_is_visible(self.to_glib_none().0))
        }
    }

    pub fn queue_resize(&self) {
        unsafe {
            ffi::gtk_tree_view_column_queue_resize(self.to_glib_none().0)
        }
    }

    pub fn get_tree_view(&self) -> Option<TreeView> {
        unsafe {
            Option::<Widget>::from_glib_none(
                ffi::gtk_tree_view_column_get_tree_view(self.to_glib_none().0))
                .map(|w| w.downcast_unchecked())
        }
    }

    pub fn get_button(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_button(self.to_glib_none().0))
        }
    }

    pub fn add_attribute<T: Upcast<CellRenderer>>(&self, cell: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_tree_view_column_add_attribute(
                self.to_glib_none().0,
                cell.to_glib_none().0,
                attribute.to_glib_none().0,
                column)
        }
}

    pub fn clear_attributes<T: Upcast<CellRenderer>>(&self, cell: &T) {
        unsafe {
            ffi::gtk_tree_view_column_clear_attributes(self.to_glib_none().0,
                cell.to_glib_none().0)
        }
    }

    pub fn pack_start<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_start(self.to_glib_none().0,
                cell.to_glib_none().0, expand.to_glib())
        }
    }

    pub fn pack_end<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_end(self.to_glib_none().0,
                cell.to_glib_none().0, expand.to_glib())
        }
    }
}
