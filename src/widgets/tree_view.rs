// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget for displaying both trees and lists

use FFIWidget;
use ffi;
use cast::GTK_TREE_VIEW;
use widgets::{TreePath, TreeSelection, TreeViewColumn};
use glib::{to_bool, to_gboolean};

/// TreeView — A widget for displaying both trees and lists
struct_Widget!(TreeView);

impl TreeView {
    pub fn new() -> Option<TreeView> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_new() };
        check_pointer!(tmp_pointer, TreeView)
    }

    pub fn new_with_model(model: &::TreeModel) -> Option<TreeView> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_new_with_model(model.unwrap_pointer()) };
        check_pointer!(tmp_pointer, TreeView)
    }

    pub fn get_headers_visible(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_headers_visible(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_headers_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_visible(GTK_TREE_VIEW(self.pointer),
                                                   to_gboolean(visible))
        }
    }

    pub fn columns_autosize(&self) {
        unsafe {
            ffi::gtk_tree_view_columns_autosize(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn get_headers_clickable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_headers_clickable(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_headers_clickable(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_clickable(GTK_TREE_VIEW(self.pointer),
                                                     to_gboolean(setting))
        }
    }

    pub fn get_rules_hint(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_rules_hint(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_rules_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_rules_hint(GTK_TREE_VIEW(self.pointer),
                                              to_gboolean(setting))
        }
    }

    #[cfg(gtk_3_8)]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_activate_on_single_click(GTK_TREE_VIEW(self.pointer)))
        }
    }

    #[cfg(gtk_3_8)]
    pub fn set_activate_on_single_click(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_activate_on_single_click(GTK_TREE_VIEW(self.pointer),
                                                            to_gboolean(setting))
        }
    }

    pub fn get_n_columns(&self) -> usize {
        unsafe {
            ffi::gtk_tree_view_get_n_columns(GTK_TREE_VIEW(self.pointer)) as usize
        }
    }

    pub fn scroll_to_point(&self, tree_x: i32, tree_y: i32) {
        unsafe {
            ffi::gtk_tree_view_scroll_to_point(GTK_TREE_VIEW(self.pointer), tree_x, tree_y)
        }
    }

    pub fn expand_all(&self) {
        unsafe {
            ffi::gtk_tree_view_expand_all(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn collapse_all(&self) {
        unsafe {
            ffi::gtk_tree_view_collapse_all(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_reorderable(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_reorderable(GTK_TREE_VIEW(self.pointer),
                                               to_gboolean(reorderable))
        }
    }

    pub fn unset_rows_drag_source(&self) {
        unsafe {
            ffi::gtk_tree_view_unset_rows_drag_source(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn unset_rows_drag_dest(&self) {
        unsafe {
            ffi::gtk_tree_view_unset_rows_drag_dest(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn get_enable_search(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_enable_search(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_enable_search(&self, enable_search: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_search(GTK_TREE_VIEW(self.pointer),
                                                 to_gboolean(enable_search))
        }
    }

    pub fn get_search_column(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_search_column(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_search_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_view_set_search_column(GTK_TREE_VIEW(self.pointer), column)
        }
    }

    pub fn get_search_entry(&self) -> ::Entry {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_tree_view_get_search_entry(GTK_TREE_VIEW(self.pointer))
                                 as *mut ffi::GtkWidget)
        }
    }

    pub fn set_search_entry(&self, entry: &::Entry) {
        unsafe {
            ffi::gtk_tree_view_set_search_entry(GTK_TREE_VIEW(self.pointer),
                                                entry.unwrap_widget() as *mut ffi::GtkEntry)
        }
    }

    pub fn widget_to_tree_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        let mut tx = 0i32;
        let mut ty = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(GTK_TREE_VIEW(self.pointer),
                                                             wx, wy, &mut tx, &mut ty);
        }
        (tx, ty)
    }

    pub fn tree_to_widget_coords(&self, tx: i32, ty: i32) -> (i32, i32) {
        let mut wx = 0i32;
        let mut wy = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(GTK_TREE_VIEW(self.pointer),
                                                             tx, ty, &mut wx, &mut wy);
        }
        (wx, wy)
    }

    pub fn widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        let mut bx = 0i32;
        let mut by = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(GTK_TREE_VIEW(self.pointer),
                                                             wx, wy, &mut bx, &mut by);
        }
        (bx, by)
    }

    pub fn bin_window_to_widget_coords(&self, bx: i32, by: i32) -> (i32, i32) {
        let mut wx = 0i32;
        let mut wy = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(GTK_TREE_VIEW(self.pointer),
                                                             bx, by, &mut wx, &mut wy);
        }
        (wx, wy)
    }

    pub fn tree_to_bin_window_coords(&self, tx: i32, ty: i32) -> (i32, i32) {
        let mut bx = 0i32;
        let mut by = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(GTK_TREE_VIEW(self.pointer),
                                                             tx, ty, &mut bx, &mut by);
        }
        (bx, by)
    }

    pub fn bin_window_to_tree_coords(&self, bx: i32, by: i32) -> (i32, i32) {
        let mut tx = 0i32;
        let mut ty = 0i32;
        unsafe {
            ffi::gtk_tree_view_convert_widget_to_tree_coords(GTK_TREE_VIEW(self.pointer),
                                                             bx, by, &mut tx, &mut ty);
        }
        (tx, ty)
    }

    pub fn get_fixed_height_mode(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_fixed_height_mode(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_fixed_height_mode(&self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_fixed_height_mode(GTK_TREE_VIEW(self.pointer),
                                                     to_gboolean(enable))
        }
    }

    pub fn get_hover_selection(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_hover_selection(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_hover_selection(&self, hover: bool) {
        unsafe {
            ffi::gtk_tree_view_set_hover_selection(GTK_TREE_VIEW(self.pointer),
                                                   to_gboolean(hover))
        }
    }

    pub fn get_hover_expand(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_hover_expand(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_hover_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_set_hover_expand(GTK_TREE_VIEW(self.pointer),
                                                to_gboolean(expand))
        }
    }

    pub fn get_rubber_banding(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_rubber_banding(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_rubber_banding(&self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_rubber_banding(GTK_TREE_VIEW(self.pointer),
                                                  to_gboolean(enable))
        }
    }

    pub fn is_rubber_banding_active(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_is_rubber_banding_active(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn get_grid_lines(&self) -> ::TreeViewGridLines {
        unsafe {
            ffi::gtk_tree_view_get_grid_lines(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_grid_lines(&self, grid_lines: ::TreeViewGridLines) {
        unsafe {
            ffi::gtk_tree_view_set_grid_lines(GTK_TREE_VIEW(self.pointer), grid_lines)
        }
    }

    pub fn get_enable_tree_lines(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_enable_tree_lines(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_enable_tree_lines(&self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_tree_lines(GTK_TREE_VIEW(self.pointer),
                                                     to_gboolean(enable))
        }
    }

    pub fn get_show_expanders(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_get_show_expanders(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_show_expanders(&self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_show_expanders(GTK_TREE_VIEW(self.pointer),
                                                  to_gboolean(enable))
        }
    }

    pub fn get_level_indentation(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_level_indentation(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_level_indentation(&self, indentation: i32) {
        unsafe {
            ffi::gtk_tree_view_set_level_indentation(GTK_TREE_VIEW(self.pointer),
                                                     indentation)
        }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_tooltip_column(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_view_set_tooltip_column(GTK_TREE_VIEW(self.pointer),
                                                  column)
        }
    }

    pub fn get_model(&self) -> Option<::TreeModel> {
        unsafe {
            let ptr = ffi::gtk_tree_view_get_model(GTK_TREE_VIEW(self.pointer));
            if ptr.is_null() {
                None
            } else {
                Some(::TreeModel::wrap_pointer(ptr))
            }
        }
    }

    pub fn set_model(&self, model: &::TreeModel) {
        unsafe {
            ffi::gtk_tree_view_set_model(GTK_TREE_VIEW(self.pointer),
                                         model.unwrap_pointer())
        }
    }

    pub fn remove_model(&self) {
        unsafe {
            ffi::gtk_tree_view_set_model(GTK_TREE_VIEW(self.pointer),
                                         ::std::ptr::null_mut())
        }
    }

    pub fn get_selection(&self) -> Option<TreeSelection> {
        unsafe {
            let ptr = ffi::gtk_tree_view_get_selection(GTK_TREE_VIEW(self.pointer));
            TreeSelection::wrap(ptr)
        }
    }

    pub fn set_cursor(&self, path: &TreePath, focus_column: Option<&TreeViewColumn>, start_editing: bool) {
        unsafe {
            ffi::gtk_tree_view_set_cursor(GTK_TREE_VIEW(self.pointer),
                                          path.unwrap_pointer(),
                                          if focus_column.is_none() { ::std::ptr::null_mut() } else { focus_column.unwrap().unwrap_pointer() },
                                          to_gboolean(start_editing))
        };
    }

    pub fn get_cursor(&self, path: Option<&mut ::TreePath>, focus_column: Option<&mut ::TreeViewColumn>) {
        unsafe { ffi::gtk_tree_view_get_cursor(GTK_TREE_VIEW(self.pointer),
            match path {
                Some(p) => &mut p.unwrap_pointer(),
                None => ::std::ptr::null_mut()
            },
            match focus_column {
                Some(f) => &mut f.unwrap_pointer(),
                None => ::std::ptr::null_mut()
            }) }
    }

    pub fn expand_row(&self, path: &TreePath, open_all: bool) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_expand_row(GTK_TREE_VIEW(self.pointer), path.unwrap_pointer(), to_gboolean(open_all)))
        }
    }

    pub fn collapse_row(&self, path: &TreePath) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_collapse_row(GTK_TREE_VIEW(self.pointer), path.unwrap_pointer()))
        }
    }

    pub fn append_column(&self, column: &::TreeViewColumn) -> i32 {
        unsafe { ffi::gtk_tree_view_append_column(GTK_TREE_VIEW(self.pointer),
                                                  column.unwrap_pointer()) }
    }
    
    pub fn remove_column(&self, column: &::TreeViewColumn) -> i32 {
        unsafe { 
            ffi::gtk_tree_view_remove_column(
                GTK_TREE_VIEW(self.pointer),
                column.unwrap_pointer()
            ) 
       }
    }
}

impl_drop!(TreeView);
impl_TraitWidget!(TreeView);

impl ::ContainerTrait for TreeView {}
impl ::ScrollableTrait for TreeView {}
