// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ptr;

use glib::translate::*;
use glib::types;
use ffi;

use mvc::cell_renderer::CellRenderer;
use mvc::tree_model::{TreeModel, TreePath};
use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use {
    IconViewDropPosition,
    Orientation,
    SelectionMode,
};

pub type IconView = Object<ffi::GtkIconView>;

unsafe impl Upcast<Widget> for IconView { }
unsafe impl Upcast<super::container::Container> for IconView { }
unsafe impl Upcast<super::scrollable::Scrollable> for IconView { }
unsafe impl Upcast<::mvc::cell_interfaces::CellLayout> for IconView { }
unsafe impl Upcast<::builder::Buildable> for IconView { }

impl IconView {
    pub fn new() -> IconView {
        unsafe { Widget::from_glib_none(ffi::gtk_icon_view_new()).downcast_unchecked() }
    }

    /*pub fn new_with_area(area: &::CellArea) -> IconView {
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new_with_area(::FFIWidget::unwrap(area))).downcast_unchecked()
        }
    }*/

    pub fn new_with_model<T: Upcast<TreeModel>>(model: &T) -> IconView {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_icon_view_new_with_model(model.upcast().to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn set_model<T: Upcast<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_icon_view_set_model(self.to_glib_none().0,
                model.map(|w| w.upcast()).to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_icon_view_get_model(self.to_glib_none().0)) }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_text_column(self.to_glib_none().0, column) }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_text_column(self.to_glib_none().0) }
    }

    pub fn set_markup_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_markup_column(self.to_glib_none().0, column) }
    }

    pub fn get_markup_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_markup_column(self.to_glib_none().0) }
    }

    pub fn set_pixbuf_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_pixbuf_column(self.to_glib_none().0, column) }
    }

    pub fn get_pixbuf_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_pixbuf_column(self.to_glib_none().0) }
    }

    pub fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath> {
        let tmp_pointer = unsafe { ffi::gtk_icon_view_get_path_at_pos(self.to_glib_none().0, x, y) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreePath::wrap_pointer(tmp_pointer))
        }
    }

    pub fn get_item_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ok = ffi::gtk_icon_view_get_item_at_pos(self.to_glib_none().0, x, y, &mut path,
                &mut cell);
            if from_glib(ok) {
                Some((TreePath::wrap_pointer(path), from_glib_full(cell)))
            }
            else {
                None
            }
        }
    }

    pub fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32, bx: &mut i32,
                                               by: &mut i32) {
        unsafe {
            ffi::gtk_icon_view_convert_widget_to_bin_window_coords(self.to_glib_none().0, wx, wy,
                bx, by)
        }
    }

    pub fn set_cursor<T: Upcast<CellRenderer>>(&self, path: &TreePath, cell: Option<&T>,
                                               start_edition: bool) {
        unsafe {
            ffi::gtk_icon_view_set_cursor(self.to_glib_none().0, path.unwrap_pointer(),
                cell.map(|c| c.upcast()).to_glib_none().0, start_edition.to_glib());
        }
    }

    pub fn get_cursor_path(&self) -> Option<TreePath> {
        unsafe {
            let mut path = ptr::null_mut();
            let ok = ffi::gtk_icon_view_get_cursor(self.to_glib_none().0, &mut path,
                ptr::null_mut());
            if from_glib(ok) { Some(TreePath::wrap_pointer(path)) } else { None }
        }
    }

    pub fn get_cursor_cell(&self) -> Option<CellRenderer> {
        unsafe {
            let mut cell = ptr::null_mut();
            let ok = ffi::gtk_icon_view_get_cursor(self.to_glib_none().0, ptr::null_mut(),
                &mut cell);
            if from_glib(ok) { Some(from_glib_none(cell)) } else { None }
        }
    }

    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe { ffi::gtk_icon_view_set_selection_mode(self.to_glib_none().0, mode) }
    }

    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe { ffi::gtk_icon_view_get_selection_mode(self.to_glib_none().0) }
    }

    pub fn set_item_orientation(&self, orientation: Orientation) {
        unsafe { ffi::gtk_icon_view_set_item_orientation(self.to_glib_none().0, orientation) }
    }

    pub fn get_item_orientation(&self) -> Orientation {
        unsafe { ffi::gtk_icon_view_get_item_orientation(self.to_glib_none().0) }
    }

    pub fn set_columns(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_columns(self.to_glib_none().0, column) }
    }

    pub fn get_columns(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_columns(self.to_glib_none().0) }
    }

    pub fn set_item_width(&self, item_width: i32) {
        unsafe { ffi::gtk_icon_view_set_item_width(self.to_glib_none().0, item_width) }
    }

    pub fn get_item_width(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_width(self.to_glib_none().0) }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe { ffi::gtk_icon_view_set_spacing(self.to_glib_none().0, spacing) }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_spacing(self.to_glib_none().0) }
    }

    pub fn set_row_spacing(&self, row_spacing: i32) {
        unsafe { ffi::gtk_icon_view_set_row_spacing(self.to_glib_none().0, row_spacing) }
    }

    pub fn get_row_spacing(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_row_spacing(self.to_glib_none().0) }
    }

    pub fn set_column_spacing(&self, column_spacing: i32) {
        unsafe { ffi::gtk_icon_view_set_column_spacing(self.to_glib_none().0, column_spacing) }
    }

    pub fn get_column_spacing(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_column_spacing(self.to_glib_none().0) }
    }

    pub fn set_margin(&self, margin: i32) {
        unsafe { ffi::gtk_icon_view_set_margin(self.to_glib_none().0, margin) }
    }

    pub fn get_margin(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_margin(self.to_glib_none().0) }
    }

    pub fn set_item_padding(&self, item_padding: i32) {
        unsafe { ffi::gtk_icon_view_set_item_padding(self.to_glib_none().0, item_padding) }
    }

    pub fn get_item_padding(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_padding(self.to_glib_none().0) }
    }

    #[cfg(gtk_3_8)]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_icon_view_set_activate_on_single_click(self.to_glib_none().0, single.to_glib())
        }
    }

    #[cfg(gtk_3_8)]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_icon_view_select_path(self.to_glib_none().0, path.unwrap_pointer()) }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_icon_view_unselect_path(self.to_glib_none().0, path.unwrap_pointer()) }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_icon_view_path_is_selected(self.to_glib_none().0, path.unwrap_pointer()))
        }
    }

    /*
    fn get_selected_items(&self) -> Vec<TreePath> {
        unsafe { Vec::from_glib_full(ffi::gtk_icon_view_get_selected_items(self.to_glib_none().0)) }
    }
    */

    pub fn select_all(&self) {
        unsafe { ffi::gtk_icon_view_select_all(self.to_glib_none().0) }
    }

    pub fn unselect_all(&self) {
        unsafe { ffi::gtk_icon_view_unselect_all(self.to_glib_none().0) }
    }

    pub fn item_activated(&self, path: &TreePath) {
        unsafe { ffi::gtk_icon_view_item_activated(self.to_glib_none().0, path.unwrap_pointer()) }
    }

    pub fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32) {
        unsafe {
            ffi::gtk_icon_view_scroll_to_path(self.to_glib_none().0, path.unwrap_pointer(),
                use_align.to_glib(), row_align, col_align) }
    }

    pub fn get_visible_range(&self) -> Option<(TreePath, TreePath)> {
        unsafe {
            let mut start_path = ptr::null_mut();
            let mut end_path = ptr::null_mut();
            let ok = ffi::gtk_icon_view_get_visible_range(self.to_glib_none().0, &mut start_path,
                &mut end_path);
            if from_glib(ok) {
                Some((TreePath::wrap_pointer(start_path), TreePath::wrap_pointer(end_path)))
            }
            else {
                None
            }
        }
    }

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_tooltip_column(self.to_glib_none().0, column) }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_tooltip_column(self.to_glib_none().0) }
    }

    pub fn get_item_row(&self, path: &TreePath) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_row(self.to_glib_none().0, path.unwrap_pointer()) }
    }

    pub fn get_item_column(&self, path: &TreePath) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_column(self.to_glib_none().0, path.unwrap_pointer()) }
    }

    pub fn unset_model_drag_source(&self) {
        unsafe { ffi::gtk_icon_view_unset_model_drag_source(self.to_glib_none().0) }
    }

    pub fn unset_model_drag_dest(&self) {
        unsafe { ffi::gtk_icon_view_unset_model_drag_dest(self.to_glib_none().0) }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe { ffi::gtk_icon_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib()) }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_icon_view_get_reorderable(self.to_glib_none().0)) }
    }

    pub fn set_drag_dest_item(&self, path: &TreePath, pos: IconViewDropPosition) {
        unsafe {
            ffi::gtk_icon_view_set_drag_dest_item(self.to_glib_none().0, path.unwrap_pointer(), pos)
        }
    }

    pub fn get_drag_dest_item(&self) -> (Option<TreePath>, IconViewDropPosition) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::zeroed();
            ffi::gtk_icon_view_get_drag_dest_item(self.to_glib_none().0, &mut path, &mut pos);
            let path = if path.is_null() { None } else { Some (TreePath::wrap_pointer(path)) };
            (path, pos)
        }
    }

    pub fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32)
            -> Option<(TreePath, IconViewDropPosition)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::zeroed();
            let ok = ffi::gtk_icon_view_get_dest_item_at_pos(self.to_glib_none().0, drag_x, drag_y,
                &mut path, &mut pos);
            if from_glib(ok) {
                Some((TreePath::wrap_pointer(path), pos))
            }
            else {
                None
            }
        }
    }
}

impl types::StaticType for IconView {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_icon_view_get_type()) }
    }
}
