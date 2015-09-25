// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget for displaying both trees and lists

use std::ptr;

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use widgets::entry::Entry;
use widgets::widget::Widget;

use super::tree_model::{TreeModel, TreePath};
use super::tree_selection::TreeSelection;
use super::tree_view_column::TreeViewColumn;

use {
    TreeViewGridLines,
};

/// TreeView — A widget for displaying both trees and lists
pub type TreeView = Object<ffi::GtkTreeView>;

impl TreeView {
    pub fn new() -> TreeView {
        unsafe { Widget::from_glib_none(ffi::gtk_tree_view_new()).downcast_unchecked() }
    }

    pub fn new_with_model(model: &TreeModel) -> TreeView {
        unsafe {
            Widget::from_glib_none(ffi::gtk_tree_view_new_with_model(model.to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn get_headers_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_headers_visible(self.to_glib_none().0)) }
    }

    pub fn set_headers_visible(&self, visible: bool) {
        unsafe { ffi::gtk_tree_view_set_headers_visible(self.to_glib_none().0, visible.to_glib()); }
    }

    pub fn columns_autosize(&self) {
        unsafe { ffi::gtk_tree_view_columns_autosize(self.to_glib_none().0); }
    }

    pub fn get_headers_clickable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_headers_clickable(self.to_glib_none().0)) }
    }

    pub fn set_headers_clickable(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_clickable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn get_rules_hint(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_rules_hint(self.to_glib_none().0)) }
    }

    pub fn set_rules_hint(&self, setting: bool) {
        unsafe { ffi::gtk_tree_view_set_rules_hint(self.to_glib_none().0, setting.to_glib()); }
    }

    #[cfg(gtk_3_8)]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_activate_on_single_click(self.to_glib_none().0)) }
    }

    #[cfg(gtk_3_8)]
    pub fn set_activate_on_single_click(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_activate_on_single_click(self.to_glib_none().0,
                setting.to_glib());
        }
    }

    pub fn get_n_columns(&self) -> u32 {
        unsafe { ffi::gtk_tree_view_get_n_columns(self.to_glib_none().0) }
    }

    pub fn scroll_to_point(&self, tree_x: i32, tree_y: i32) {
        unsafe { ffi::gtk_tree_view_scroll_to_point(self.to_glib_none().0, tree_x, tree_y); }
    }

    pub fn expand_all(&self) {
        unsafe { ffi::gtk_tree_view_expand_all(self.to_glib_none().0) }
    }

    pub fn collapse_all(&self) {
        unsafe { ffi::gtk_tree_view_collapse_all(self.to_glib_none().0); }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_reorderable(self.to_glib_none().0)) }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe { ffi::gtk_tree_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib()); }
    }

    pub fn unset_rows_drag_source(&self) {
        unsafe { ffi::gtk_tree_view_unset_rows_drag_source(self.to_glib_none().0); }
    }

    pub fn unset_rows_drag_dest(&self) {
        unsafe { ffi::gtk_tree_view_unset_rows_drag_dest(self.to_glib_none().0); }
    }

    pub fn get_enable_search(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_enable_search(self.to_glib_none().0)) }
    }

    pub fn set_enable_search(&self, enable_search: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_search(self.to_glib_none().0, enable_search.to_glib());
        }
    }

    pub fn get_search_column(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_get_search_column(self.to_glib_none().0) }
    }

    pub fn set_search_column(&self, column: i32) {
        unsafe { ffi::gtk_tree_view_set_search_column(self.to_glib_none().0, column) }
    }

    pub fn get_search_entry(&self) -> Entry {
        unsafe { from_glib_none(ffi::gtk_tree_view_get_search_entry(self.to_glib_none().0)) }
    }

    pub fn set_search_entry(&self, entry: &Entry) {
        unsafe {
            ffi::gtk_tree_view_set_search_entry(self.to_glib_none().0, entry.to_glib_none().0);
        }
    }

    pub fn widget_to_tree_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        let mut tx = 0i32;
        let mut ty = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0,
                                                             wx, wy, &mut tx, &mut ty);
        }
        (tx, ty)
    }

    pub fn tree_to_widget_coords(&self, tx: i32, ty: i32) -> (i32, i32) {
        let mut wx = 0i32;
        let mut wy = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0,
                                                             tx, ty, &mut wx, &mut wy);
        }
        (wx, wy)
    }

    pub fn widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        let mut bx = 0i32;
        let mut by = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0,
                                                             wx, wy, &mut bx, &mut by);
        }
        (bx, by)
    }

    pub fn bin_window_to_widget_coords(&self, bx: i32, by: i32) -> (i32, i32) {
        let mut wx = 0i32;
        let mut wy = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0,
                                                             bx, by, &mut wx, &mut wy);
        }
        (wx, wy)
    }

    pub fn tree_to_bin_window_coords(&self, tx: i32, ty: i32) -> (i32, i32) {
        let mut bx = 0i32;
        let mut by = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0,
                                                             tx, ty, &mut bx, &mut by);
        }
        (bx, by)
    }

    pub fn bin_window_to_tree_coords(&self, bx: i32, by: i32) -> (i32, i32) {
        let mut tx = 0i32;
        let mut ty = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0,
                                                             bx, by, &mut tx, &mut ty);
        }
        (tx, ty)
    }

    pub fn get_fixed_height_mode(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_fixed_height_mode(self.to_glib_none().0)) }
    }

    pub fn set_fixed_height_mode(&self, enable: bool) {
        unsafe { ffi::gtk_tree_view_set_fixed_height_mode(self.to_glib_none().0, enable.to_glib()) }
    }

    pub fn get_hover_selection(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_hover_selection(self.to_glib_none().0)) }
    }

    pub fn set_hover_selection(&self, hover: bool) {
        unsafe { ffi::gtk_tree_view_set_hover_selection(self.to_glib_none().0, hover.to_glib()) }
    }

    pub fn get_hover_expand(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_hover_expand(self.to_glib_none().0)) }
    }

    pub fn set_hover_expand(&self, expand: bool) {
        unsafe { ffi::gtk_tree_view_set_hover_expand(self.to_glib_none().0, expand.to_glib()) }
    }

    pub fn get_rubber_banding(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_rubber_banding(self.to_glib_none().0)) }
    }

    pub fn set_rubber_banding(&self, enable: bool) {
        unsafe { ffi::gtk_tree_view_set_rubber_banding(self.to_glib_none().0, enable.to_glib()) }
    }

    pub fn is_rubber_banding_active(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_is_rubber_banding_active(self.to_glib_none().0)) }
    }

    pub fn get_grid_lines(&self) -> TreeViewGridLines {
        unsafe { ffi::gtk_tree_view_get_grid_lines(self.to_glib_none().0) }
    }

    pub fn set_grid_lines(&self, grid_lines: TreeViewGridLines) {
        unsafe { ffi::gtk_tree_view_set_grid_lines(self.to_glib_none().0, grid_lines) }
    }

    pub fn get_enable_tree_lines(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_enable_tree_lines(self.to_glib_none().0)) }
    }

    pub fn set_enable_tree_lines(&self, enable: bool) {
        unsafe { ffi::gtk_tree_view_set_enable_tree_lines(self.to_glib_none().0, enable.to_glib()) }
    }

    pub fn get_show_expanders(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_view_get_show_expanders(self.to_glib_none().0)) }
    }

    pub fn set_show_expanders(&self, enable: bool) {
        unsafe { ffi::gtk_tree_view_set_show_expanders(self.to_glib_none().0, enable.to_glib()) }
    }

    pub fn get_level_indentation(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_get_level_indentation(self.to_glib_none().0) }
    }

    pub fn set_level_indentation(&self, indentation: i32) {
        unsafe { ffi::gtk_tree_view_set_level_indentation(self.to_glib_none().0, indentation) }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe { ffi::gtk_tree_view_get_tooltip_column(self.to_glib_none().0) }
    }

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe { ffi::gtk_tree_view_set_tooltip_column(self.to_glib_none().0, column) }
    }

    pub fn set_model(&self, model: &TreeModel) {
        unsafe { ffi::gtk_tree_view_set_model(self.to_glib_none().0, model.to_glib_none().0); }
    }

    pub fn remove_model(&self) {
        unsafe { ffi::gtk_tree_view_set_model(self.to_glib_none().0, ptr::null_mut()) }
    }

    pub fn get_selection(&self) -> TreeSelection {
        unsafe { from_glib_none(ffi::gtk_tree_view_get_selection(self.to_glib_none().0)) }
    }

    pub fn set_cursor(&self, path: &TreePath, focus_column: Option<&TreeViewColumn>,
            start_editing: bool) {
        unsafe {
            ffi::gtk_tree_view_set_cursor(self.to_glib_none().0, path.unwrap_pointer(),
                focus_column.to_glib_none().0, start_editing.to_glib());
        }
    }

    pub fn get_cursor(&self) -> (Option<TreePath>, Option<TreeViewColumn>) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut focus_column = ptr::null_mut();
            ffi::gtk_tree_view_get_cursor(self.to_glib_none().0, &mut path, &mut focus_column);
            let path = if path.is_null() { None } else { Some(TreePath::wrap_pointer(path)) };
            (path, from_glib_none(focus_column))
        }
    }

    pub fn expand_row(&self, path: &TreePath, open_all: bool) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_tree_view_expand_row(self.to_glib_none().0, path.unwrap_pointer(),
                    open_all.to_glib()))
        }
    }

    pub fn collapse_row(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_collapse_row(self.to_glib_none().0, path.unwrap_pointer()))
        }
    }

    pub fn append_column(&self, column: &TreeViewColumn) -> i32 {
        unsafe { ffi::gtk_tree_view_append_column(self.to_glib_none().0, column.to_glib_none().0) }
    }
}

impl types::StaticType for TreeView {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_tree_view_get_type()) }
    }
}

unsafe impl Upcast<Widget> for TreeView { }
unsafe impl Upcast<::Container> for TreeView { }
unsafe impl Upcast<::Scrollable> for TreeView { }
unsafe impl Upcast<::Buildable> for TreeView { }
