// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget that emits a signal when clicked on

use glib;
use ffi;
use cast;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::{to_bool, to_gboolean};

pub struct TreeViewColumn {
    pointer: *mut ffi::GtkTreeViewColumn
}

impl TreeViewColumn {
    pub fn new() -> Option<TreeViewColumn> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_column_new() };
        check_pointer!(tmp_pointer, TreeViewColumn, G_OBJECT_FROM_TREE_VIEW_COLUMN)
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clear(self.pointer)
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_spacing(self.pointer, spacing)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_spacing(self.pointer)
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_visible(self.pointer, to_gboolean(visible))
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_get_visible(self.pointer))
        }
    }

    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_resizable(self.pointer, to_gboolean(resizable))
        }
    }

    pub fn get_resizable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_get_resizable(self.pointer))
        }
    }

    pub fn set_sizing(&self, ty: ::TreeViewColumnSizing) {
        unsafe {
            ffi::gtk_tree_view_column_set_sizing(self.pointer, ty)
        }
    }

    pub fn get_sizing(&self) -> ::TreeViewColumnSizing {
        unsafe {
            ffi::gtk_tree_view_column_get_sizing(self.pointer)
        }
    }

    pub fn get_x_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_x_offset(self.pointer)
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_width(self.pointer)
        }
    }

    pub fn get_fixed_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_fixed_width(self.pointer)
        }
    }

    pub fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_fixed_width(self.pointer, fixed_width)
        }
    }

    pub fn set_min_width(&self, min_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_min_width(self.pointer, min_width)
        }
    }

    pub fn set_max_width(&self, max_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_max_width(self.pointer, max_width)
        }
    }

    pub fn get_min_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_min_width(self.pointer)
        }
    }

    pub fn get_max_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_max_width(self.pointer)
        }
    }

    pub fn clicked(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clicked(self.pointer)
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_tree_view_column_set_title(self.pointer,
                                                title.to_glib_none().0);
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_title(self.pointer))
        }
    }

    pub fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_expand(self.pointer, to_gboolean(expand))
        }
    }

    pub fn get_expand(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_get_expand(self.pointer))
        }
    }

    pub fn set_clickable(&self, clickable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_clickable(self.pointer, to_gboolean(clickable))
        }
    }

    pub fn get_clickable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_get_clickable(self.pointer))
        }
    }

    pub fn set_widget<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe {
            ffi::gtk_tree_view_column_set_widget(self.pointer, widget.unwrap_widget())
        }
    }

    pub fn get_widget<T: ::WidgetTrait>(&self) -> T {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_tree_view_column_get_widget(self.pointer))
        }
    }

    pub fn set_alignment(&self, x_align: f32) {
        unsafe {
            ffi::gtk_tree_view_column_set_alignment(self.pointer, x_align)
        }
    }

    pub fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tree_view_column_get_alignment(self.pointer)
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_reorderable(self.pointer, to_gboolean(reorderable))
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_get_reorderable(self.pointer))
        }
    }

    pub fn get_sort_column_id(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_column_id(self.pointer)
        }
    }

    pub fn set_sort_column_id(&self, sort_column_id: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_column_id(self.pointer, sort_column_id)
        }
    }

    pub fn set_sort_indicator(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_indicator(self.pointer, to_gboolean(setting))
        }
    }

    pub fn get_sort_indicator(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_get_sort_indicator(self.pointer))
        }
    }

    pub fn set_sort_order(&self, order: ::SortType) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_order(self.pointer, order)
        }
    }

    pub fn get_sort_order(&self) -> ::SortType {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_order(self.pointer)
        }
    }

    pub fn column_cell_is_visible(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_tree_view_column_cell_is_visible(self.pointer))
        }
    }

    pub fn queue_resize(&self) {
        unsafe {
            ffi::gtk_tree_view_column_queue_resize(self.pointer)
        }
    }

    pub fn get_tree_view(&self) -> ::TreeView {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_tree_view_column_get_tree_view(self.pointer))
        }
    }

    pub fn get_button<T: ::WidgetTrait + ::ButtonTrait>(&self) -> T {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_tree_view_column_get_button(self.pointer))
        }
    }

    pub fn add_attribute<T: ::FFIWidget + ::CellRendererTrait>(&self, cell: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_tree_view_column_add_attribute(
                self.pointer,
                cast::GTK_CELL_RENDERER(cell.unwrap_widget()),
                attribute.to_glib_none().0,
                column)
        }
}

    pub fn clear_attributes<T: ::FFIWidget + ::CellRendererTrait>(&self, cell: &T) {
        unsafe { ffi::gtk_tree_view_column_clear_attributes(self.pointer,
                                                            cast::GTK_CELL_RENDERER(cell.unwrap_widget())) }
    }

    pub fn pack_start<T: ::FFIWidget + ::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe { ffi::gtk_tree_view_column_pack_start(self.pointer,
                                                      cast::GTK_CELL_RENDERER(cell.unwrap_widget()),
                                                      to_gboolean(expand)) }
    }

    pub fn pack_end<T: ::FFIWidget + ::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe { ffi::gtk_tree_view_column_pack_end(self.pointer,
                                                    cast::GTK_CELL_RENDERER(cell.unwrap_widget()),
                                                    to_gboolean(expand)) }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkTreeViewColumn {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(treeview_column: *mut ffi::GtkTreeViewColumn) -> TreeViewColumn {
        unsafe{
            ::glib::ffi::g_object_ref(treeview_column as *mut ::libc::c_void);
        }

        TreeViewColumn {
            pointer: treeview_column
        }
    }
}

impl glib::traits::FFIGObject for TreeViewColumn {
    fn unwrap_gobject(&self) -> *mut glib::ffi::GObject {
        ::cast::G_OBJECT_FROM_TREE_VIEW_COLUMN(self.pointer)
    }

    fn wrap_object(object: *mut glib::ffi::GObject) -> TreeViewColumn {
        TreeViewColumn { pointer: object as *mut ffi::GtkTreeViewColumn }
    }
}

impl Drop for TreeViewColumn {
    fn drop(&mut self) {
        unsafe {
            ::glib::ffi::g_object_unref(self.pointer as *mut ::libc::c_void);
        }
    }
}

impl Clone for TreeViewColumn {
    fn clone(&self) -> TreeViewColumn {
        let pointer = unsafe {
            ::glib::ffi::g_object_ref(self.pointer as *mut ::libc::c_void)
        };

        TreeViewColumn {
            pointer: pointer as *mut ffi::GtkTreeViewColumn
        }
    }
}
