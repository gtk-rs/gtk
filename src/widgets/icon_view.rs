// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::{GTK_ICON_VIEW, GTK_CELL_RENDERER};
use ffi;
use {TreeModel, TreePath};

/// GtkIconView — A widget which displays a list of icons in a grid

struct_Widget!(IconView);

impl IconView {
    pub fn new() -> Option<IconView> {
        let tmp_pointer = unsafe { ffi::gtk_icon_view_new() };
        check_pointer!(tmp_pointer, IconView)
    }

    /*pub fn new_with_area(area: &::CellArea) -> Option<IconView> {
        let tmp_pointer = unsafe { ffi::gtk_icon_view_new_with_area(::FFIWidget::unwrap(area)) };
        check_pointer!(tmp_pointer, IconView)
    }*/

    pub fn new_with_model(model: &TreeModel) -> Option<IconView> {
        let tmp_pointer = unsafe { ffi::gtk_icon_view_new_with_model(model.unwrap_pointer()) };
        check_pointer!(tmp_pointer, IconView)
    }

    pub fn set_model(&self, model: &TreeModel) {
        unsafe { ffi::gtk_icon_view_set_model(GTK_ICON_VIEW(self.pointer), model.unwrap_pointer()) }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        let tmp_pointer = unsafe { ffi::gtk_icon_view_get_model(GTK_ICON_VIEW(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe { ::gobject_ffi::g_object_ref(tmp_pointer as *mut _) };
            Some(TreeModel::wrap_pointer(tmp_pointer))
        }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_text_column(GTK_ICON_VIEW(self.pointer), column) }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_text_column(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_markup_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_markup_column(GTK_ICON_VIEW(self.pointer), column) }
    }

    pub fn get_markup_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_markup_column(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_pixbuf_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_pixbuf_column(GTK_ICON_VIEW(self.pointer), column) }
    }

    pub fn get_pixbuf_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_pixbuf_column(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath> {
        let tmp_pointer = unsafe { ffi::gtk_icon_view_get_path_at_pos(GTK_ICON_VIEW(self.pointer), x, y) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreePath::wrap_pointer(tmp_pointer))
        }
    }

    pub fn get_item_at_pos<T: ::CellRendererTrait>(&self, x: i32, y: i32, path: &TreePath, cell: &T) -> bool {
        match unsafe { ffi::gtk_icon_view_get_item_at_pos(GTK_ICON_VIEW(self.pointer), x, y, &mut path.unwrap_pointer(),
            &mut GTK_CELL_RENDERER(cell.unwrap_widget())) } {
            0 => false,
            _ => true
        }
    }

    pub fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32, bx: &mut i32, by: &mut i32) {
        unsafe { ffi::gtk_icon_view_convert_widget_to_bin_window_coords(GTK_ICON_VIEW(self.pointer), wx, wy, bx, by) }
    }

    pub fn set_cursor<T: ::CellRendererTrait>(&self, path: &TreePath, cell: &T, start_edition: bool) {
        unsafe { ffi::gtk_icon_view_set_cursor(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer(), GTK_CELL_RENDERER(cell.unwrap_widget()),
            match start_edition {
                true => 1,
                false => 0
            }) }
    }

    pub fn get_cursor<T: ::CellRendererTrait>(&self, path: &TreePath, cell: &T) -> bool {
        match unsafe { ffi::gtk_icon_view_get_cursor(GTK_ICON_VIEW(self.pointer), &mut path.unwrap_pointer(),
            &mut GTK_CELL_RENDERER(cell.unwrap_widget())) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_selection_mode(&self, mode: ::SelectionMode) {
        unsafe { ffi::gtk_icon_view_set_selection_mode(GTK_ICON_VIEW(self.pointer), mode) }
    }

    pub fn get_selection_mode(&self) -> ::SelectionMode {
        unsafe { ffi::gtk_icon_view_get_selection_mode(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_item_orientation(&self, orientation: ::Orientation) {
        unsafe { ffi::gtk_icon_view_set_item_orientation(GTK_ICON_VIEW(self.pointer), orientation) }
    }

    pub fn get_item_orientation(&self) -> ::Orientation {
        unsafe { ffi::gtk_icon_view_get_item_orientation(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_columns(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_columns(GTK_ICON_VIEW(self.pointer), column) }
    }

    pub fn get_columns(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_columns(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_item_width(&self, item_width: i32) {
        unsafe { ffi::gtk_icon_view_set_item_width(GTK_ICON_VIEW(self.pointer), item_width) }
    }

    pub fn get_item_width(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_width(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe { ffi::gtk_icon_view_set_spacing(GTK_ICON_VIEW(self.pointer), spacing) }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_spacing(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_row_spacing(&self, row_spacing: i32) {
        unsafe { ffi::gtk_icon_view_set_row_spacing(GTK_ICON_VIEW(self.pointer), row_spacing) }
    }

    pub fn get_row_spacing(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_row_spacing(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_column_spacing(&self, column_spacing: i32) {
        unsafe { ffi::gtk_icon_view_set_column_spacing(GTK_ICON_VIEW(self.pointer), column_spacing) }
    }

    pub fn get_column_spacing(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_column_spacing(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_margin(&self, margin: i32) {
        unsafe { ffi::gtk_icon_view_set_margin(GTK_ICON_VIEW(self.pointer), margin) }
    }

    pub fn get_margin(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_margin(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_item_padding(&self, item_padding: i32) {
        unsafe { ffi::gtk_icon_view_set_item_padding(GTK_ICON_VIEW(self.pointer), item_padding) }
    }

    pub fn get_item_padding(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_padding(GTK_ICON_VIEW(self.pointer)) }
    }

    #[cfg(gtk_3_8)]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe { ffi::gtk_icon_view_set_activate_on_single_click(GTK_ICON_VIEW(self.pointer), if single == false {0} else {1}) }
    }

    #[cfg(gtk_3_8)]
    pub fn get_activate_on_single_click(&self) -> bool {
        match unsafe { ffi::gtk_icon_view_get_activate_on_single_click(GTK_ICON_VIEW(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_icon_view_select_path(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer()) }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe { ffi::gtk_icon_view_unselect_path(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer()) }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        match unsafe { ffi::gtk_icon_view_path_is_selected(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer()) } {
            0 => false,
            _ => true
        }
    }

    /*fn get_selected_items<T: traits::TreeRowReference>(&self) -> glib::List<T> {
        let tmp = unsafe { ffi::gtk_icon_view_get_selected_items(GTK_ICON_VIEW(self.pointer)) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let list: glib::List<*mut ffi::GtkWidget> = glib::GlibContainer::wrap(tmp);

            list.iter().map(|it| ::FFIWidget::wrap_widget(*it)).collect()
        }
    }*/

    pub fn select_all(&self) {
        unsafe { ffi::gtk_icon_view_select_all(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn unselect_all(&self) {
        unsafe { ffi::gtk_icon_view_unselect_all(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn item_activated(&self, path: &TreePath) {
        unsafe { ffi::gtk_icon_view_item_activated(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer()) }
    }

    pub fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32) {
        unsafe { ffi::gtk_icon_view_scroll_to_path(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer(),
            if use_align {1} else {0}, row_align, col_align) }
    }

    pub fn get_visible_range(&self, start_path: &TreePath, end_path: &TreePath) -> bool {
        match unsafe { ffi::gtk_icon_view_get_visible_range(GTK_ICON_VIEW(self.pointer), &mut start_path.unwrap_pointer(),
            &mut end_path.unwrap_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe { ffi::gtk_icon_view_set_tooltip_column(GTK_ICON_VIEW(self.pointer), column) }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe { ffi::gtk_icon_view_get_tooltip_column(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn get_item_row(&self, path: &TreePath) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_row(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer()) }
    }

    pub fn get_item_column(&self, path: &TreePath) -> i32 {
        unsafe { ffi::gtk_icon_view_get_item_column(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer()) }
    }

    pub fn unset_model_drag_source(&self) {
        unsafe { ffi::gtk_icon_view_unset_model_drag_source(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn unset_model_drag_dest(&self) {
        unsafe { ffi::gtk_icon_view_unset_model_drag_dest(GTK_ICON_VIEW(self.pointer)) }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe { ffi::gtk_icon_view_set_reorderable(GTK_ICON_VIEW(self.pointer), if reorderable {1} else {0}) }
    }

    pub fn get_reorderable(&self) -> bool {
        match unsafe { ffi::gtk_icon_view_get_reorderable(GTK_ICON_VIEW(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_drag_dest_item(&self, path: &TreePath, pos: ::IconViewDropPosition) {
        unsafe { ffi::gtk_icon_view_set_drag_dest_item(GTK_ICON_VIEW(self.pointer), path.unwrap_pointer(), pos) }
    }

    pub fn get_drag_dest_item(&self, path: &TreePath, pos: &mut ::IconViewDropPosition) {
        unsafe { ffi::gtk_icon_view_get_drag_dest_item(GTK_ICON_VIEW(self.pointer), &mut path.unwrap_pointer(), pos) }
    }

    pub fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32, path: &TreePath, pos: &mut ::IconViewDropPosition) {
        unsafe { ffi::gtk_icon_view_get_dest_item_at_pos(GTK_ICON_VIEW(self.pointer), drag_x, drag_y, &mut path.unwrap_pointer(), pos); }
    }
}

impl_drop!(IconView);
impl_TraitWidget!(IconView);

impl ::ScrollableTrait for IconView {}
impl ::CellLayoutTrait for IconView {}
