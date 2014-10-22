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

//! A widget for displaying both trees and lists

use gtk;
use gtk::ffi::{mod, FFIWidget};
use gtk::traits;
use gtk::cast::GTK_TREE_VIEW;
use gtk::widgets::TreeSelection;

/// TreeView — A widget for displaying both trees and lists
struct_Widget!(TreeView)

impl TreeView {
    pub fn new() -> Option<TreeView> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_new() };
        check_pointer!(tmp_pointer, TreeView)
    }

    pub fn new_with_model(model: &gtk::TreeModel) -> Option<TreeView> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_new_with_model(model.get_pointer()) };
        check_pointer!(tmp_pointer, TreeView)
    }

    pub fn get_headers_visible(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_headers_visible(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_headers_visible(&mut self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_visible(GTK_TREE_VIEW(self.pointer),
                                                   ffi::to_gboolean(visible))
        }
    }

    pub fn columns_autosize(&mut self) {
        unsafe {
            ffi::gtk_tree_view_columns_autosize(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn get_headers_clickable(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_headers_clickable(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_headers_clickable(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_clickable(GTK_TREE_VIEW(self.pointer),
                                                     ffi::to_gboolean(setting))
        }
    }

    pub fn get_rules_hint(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_rules_hint(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_rules_hint(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_rules_hint(GTK_TREE_VIEW(self.pointer),
                                              ffi::to_gboolean(setting))
        }
    }

    #[cfg(any(feature = "GTK_3_8", feature = "GTK_3_10", feature = "GTK_3_12", feature = "GTK_3_14"))]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_activate_on_single_click(GTK_TREE_VIEW(self.pointer)))
        }
    }

    #[cfg(any(feature = "GTK_3_8", feature = "GTK_3_10", feature = "GTK_3_12", feature = "GTK_3_14"))]
    pub fn set_activate_on_single_click(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_activate_on_single_click(GTK_TREE_VIEW(self.pointer),
                                                            ffi::to_gboolean(setting))
        }
    }

    #[cfg(any(feature = "GTK_3_4", feature = "GTK_3_6", feature = "GTK_3_8", feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    pub fn get_n_columns(&self) -> uint {
        unsafe {
            ffi::gtk_tree_view_get_n_columns(GTK_TREE_VIEW(self.pointer)) as uint
        }
    }

    pub fn scroll_to_point(&mut self, tree_x: i32, tree_y: i32) {
        unsafe {
            ffi::gtk_tree_view_scroll_to_point(GTK_TREE_VIEW(self.pointer), tree_x, tree_y)
        }
    }

    pub fn expand_all(&mut self) {
        unsafe {
            ffi::gtk_tree_view_expand_all(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn collapse_all(&mut self) {
        unsafe {
            ffi::gtk_tree_view_collapse_all(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_reorderable(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_reorderable(&mut self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_reorderable(GTK_TREE_VIEW(self.pointer),
                                               ffi::to_gboolean(reorderable))
        }
    }

    pub fn unset_rows_drag_source(&mut self) {
        unsafe {
            ffi::gtk_tree_view_unset_rows_drag_source(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn unset_rows_drag_dest(&mut self) {
        unsafe {
            ffi::gtk_tree_view_unset_rows_drag_dest(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn get_enable_search(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_enable_search(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_enable_search(&mut self, enable_search: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_search(GTK_TREE_VIEW(self.pointer),
                                                 ffi::to_gboolean(enable_search))
        }
    }

    pub fn get_search_column(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_search_column(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_search_column(&mut self, column: i32) {
        unsafe {
            ffi::gtk_tree_view_set_search_column(GTK_TREE_VIEW(self.pointer), column)
        }
    }

    pub fn get_search_entry(&self) -> gtk::Entry {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_tree_view_get_search_entry(GTK_TREE_VIEW(self.pointer))
                                 as *mut ffi::C_GtkWidget)
        }
    }

    pub fn set_search_entry(&mut self, entry: &mut gtk::Entry) {
        unsafe {
            ffi::gtk_tree_view_set_search_entry(GTK_TREE_VIEW(self.pointer),
                                                entry.get_widget() as *mut ffi::C_GtkEntry)
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
            ffi::to_bool(ffi::gtk_tree_view_get_fixed_height_mode(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_fixed_height_mode(&mut self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_fixed_height_mode(GTK_TREE_VIEW(self.pointer),
                                                     ffi::to_gboolean(enable))
        }
    }

    pub fn get_hover_selection(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_hover_selection(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_hover_selection(&mut self, hover: bool) {
        unsafe {
            ffi::gtk_tree_view_set_hover_selection(GTK_TREE_VIEW(self.pointer),
                                                   ffi::to_gboolean(hover))
        }
    }

    pub fn get_hover_expand(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_hover_expand(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_hover_expand(&mut self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_set_hover_expand(GTK_TREE_VIEW(self.pointer),
                                                ffi::to_gboolean(expand))
        }
    }

    pub fn get_rubber_banding(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_rubber_banding(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_rubber_banding(&mut self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_rubber_banding(GTK_TREE_VIEW(self.pointer),
                                                  ffi::to_gboolean(enable))
        }
    }

    pub fn is_rubber_banding_active(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_is_rubber_banding_active(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn get_grid_lines(&self) -> gtk::TreeViewGridLines {
        unsafe {
            ffi::gtk_tree_view_get_grid_lines(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_grid_lines(&mut self, grid_lines: gtk::TreeViewGridLines) {
        unsafe {
            ffi::gtk_tree_view_set_grid_lines(GTK_TREE_VIEW(self.pointer), grid_lines)
        }
    }

    pub fn get_enable_tree_lines(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_enable_tree_lines(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_enable_tree_lines(&mut self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_tree_lines(GTK_TREE_VIEW(self.pointer),
                                                     ffi::to_gboolean(enable))
        }
    }

    pub fn get_show_expanders(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_get_show_expanders(GTK_TREE_VIEW(self.pointer)))
        }
    }

    pub fn set_show_expanders(&mut self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_show_expanders(GTK_TREE_VIEW(self.pointer),
                                                  ffi::to_gboolean(enable))
        }
    }

    pub fn get_level_indentation(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_level_indentation(GTK_TREE_VIEW(self.pointer))
        }
    }

    pub fn set_level_indentation(&mut self, indentation: i32) {
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

    pub fn set_tooltip_column(&mut self, column: i32) {
        unsafe {
            ffi::gtk_tree_view_set_tooltip_column(GTK_TREE_VIEW(self.pointer),
                                                  column)
        }
    }

    pub fn get_model(&self) -> Option<gtk::TreeModel> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_get_model(GTK_TREE_VIEW(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::TreeModel::wrap_pointer(tmp_pointer))
        }
    }

    pub fn set_model(&mut self, model: &gtk::TreeModel) {
        unsafe {
            ffi::gtk_tree_view_set_model(GTK_TREE_VIEW(self.pointer),
                                         model.get_pointer())
        }
    }

    pub fn get_selection(&self) -> Option<TreeSelection> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_get_selection(GTK_TREE_VIEW(self.pointer)) };

        TreeSelection::wrap(tmp_pointer)
    }

    pub fn append_column(&mut self, column: &gtk::TreeViewColumn) -> i32 {
        unsafe { ffi::gtk_tree_view_append_column(GTK_TREE_VIEW(self.pointer),
                                                  column.get_pointer()) }
    }
}

impl_drop!(TreeView)
impl_TraitWidget!(TreeView)

impl traits::Container for TreeView {}
impl traits::Scrollable for TreeView {}
