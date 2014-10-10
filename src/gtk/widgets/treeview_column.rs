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

//! A widget that emits a signal when clicked on

use gtk::{mod, ffi, traits, cast};

pub struct TreeViewColumn {
    pointer: *mut ffi::C_GtkTreeViewColumn
}

impl TreeViewColumn {
    pub fn new() -> Option<TreeViewColumn> {
        let tmp_pointer = unsafe { ffi::gtk_tree_view_column_new() };
        check_pointer!(tmp_pointer, TreeViewColumn)
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::gtk_tree_view_column_clear(self.pointer)
        }
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_spacing(self.pointer, spacing)
        }
    }

    pub fn spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_spacing(self.pointer)
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_visible(self.pointer, ffi::to_gboolean(visible))
        }
    }

    pub fn visible(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_get_visible(self.pointer))
        }
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_resizable(self.pointer, ffi::to_gboolean(resizable))
        }
    }

    pub fn resizable(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_get_resizable(self.pointer))
        }
    }

    pub fn set_sizing(&mut self, ty: gtk::TreeViewColumnSizing) {
        unsafe {
            ffi::gtk_tree_view_column_set_sizing(self.pointer, ty)
        }
    }

    pub fn sizing(&self) -> gtk::TreeViewColumnSizing {
        unsafe {
            ffi::gtk_tree_view_column_get_sizing(self.pointer)
        }
    }

    pub fn x_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_x_offset(self.pointer)
        }
    }

    pub fn width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_width(self.pointer)
        }
    }

    pub fn fixed_width(&self) -> i32 {
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

    pub fn min_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_min_width(self.pointer)
        }
    }

    pub fn max_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_max_width(self.pointer)
        }
    }

    pub fn clicked(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clicked(self.pointer)
        }
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            title.with_c_str(|c_str| {
                ffi::gtk_tree_view_column_set_title(self.pointer, c_str)
            });
        }
    }

    pub fn title(&self) -> String {
        unsafe {
            ::std::string::raw::from_buf(ffi::gtk_tree_view_column_get_title(self.pointer) as *const u8)
        }
    }

    pub fn set_expand(&mut self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_expand(self.pointer, ffi::to_gboolean(expand))
        }
    }

    pub fn expand(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_get_expand(self.pointer))
        }
    }

    pub fn set_clickable(&mut self, clickable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_clickable(self.pointer, ffi::to_gboolean(clickable))
        }
    }

    pub fn clickable(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_get_clickable(self.pointer))
        }
    }

    pub fn set_widget(&mut self, widget: &traits::Widget) {
        unsafe {
            ffi::gtk_tree_view_column_set_widget(self.pointer, widget.get_widget())
        }
    }

    pub fn get_widget<T: traits::Widget>(&self) -> T {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_tree_view_column_get_widget(self.pointer))
        }
    }

    pub fn set_alignment(&mut self, x_align: f32) {
        unsafe {
            ffi::gtk_tree_view_column_set_alignment(self.pointer, x_align)
        }
    }

    pub fn alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tree_view_column_get_alignment(self.pointer)
        }
    }

    pub fn set_reorderable(&mut self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_reorderable(self.pointer, ffi::to_gboolean(reorderable))
        }
    }

    pub fn reorderable(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_get_reorderable(self.pointer))
        }
    }

    pub fn sort_column_id(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_column_id(self.pointer)
        }
    }

    pub fn set_sort_column_id(&mut self, sort_column_id: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_column_id(self.pointer, sort_column_id)
        }
    }

    pub fn set_sort_indicator(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_indicator(self.pointer, ffi::to_gboolean(setting))
        }
    }

    pub fn sort_indicator(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_get_sort_indicator(self.pointer))
        }
    }

    pub fn set_sort_order(&mut self, order: gtk::SortType) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_order(self.pointer, order)
        }
    }

    pub fn sort_order(&self) -> gtk::SortType {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_order(self.pointer)
        }
    }

    pub fn column_cell_is_visible(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_tree_view_column_cell_is_visible(self.pointer))
        }
    }

    pub fn queue_resize(&mut self) {
        unsafe {
            ffi::gtk_tree_view_column_queue_resize(self.pointer)
        }
    }

    pub fn get_tree_view(&self) -> gtk::TreeView {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_tree_view_column_get_tree_view(self.pointer))
        }
    }

    pub fn get_button<T: traits::Widget + traits::Button>(&self) -> T {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_tree_view_column_get_button(self.pointer))
        }
    }

    pub fn add_attribute<T: ffi::FFIWidget + traits::CellRenderer>(&self, cell: &T, attribute: &str, column: i32) {
        let attribute_c = attribute.to_c_str();
        unsafe { ffi::gtk_tree_view_column_add_attribute(self.pointer,
                                                         cast::GTK_CELL_RENDERER(cell.get_widget()),
                                                         attribute_c.as_ptr(),
                                                         column) }
    }

    pub fn clear_attributes<T: ffi::FFIWidget + traits::CellRenderer>(&self, cell: &T) {
        unsafe { ffi::gtk_tree_view_column_clear_attributes(self.pointer,
                                                            cast::GTK_CELL_RENDERER(cell.get_widget())) }
    }

    pub fn pack_start<T: ffi::FFIWidget + traits::CellRenderer>(&self, cell: &T, expand: bool) {
        unsafe { ffi::gtk_tree_view_column_pack_start(self.pointer,
                                                      cast::GTK_CELL_RENDERER(cell.get_widget()),
                                                      ffi::to_gboolean(expand)) }
    }

    pub fn pack_end<T: ffi::FFIWidget + traits::CellRenderer>(&self, cell: &T, expand: bool) {
        unsafe { ffi::gtk_tree_view_column_pack_end(self.pointer,
                                                    cast::GTK_CELL_RENDERER(cell.get_widget()),
                                                    ffi::to_gboolean(expand)) }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_GtkTreeViewColumn {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(treeview_column: *mut ffi::C_GtkTreeViewColumn) -> TreeViewColumn {
        unsafe{
            ::glib::ffi::g_object_ref(treeview_column as *mut ::glib::ffi::C_GObject);
        }

        TreeViewColumn {
            pointer: treeview_column
        }
    }
}

impl Drop for TreeViewColumn {
    fn drop(&mut self) {
        unsafe {
            ::glib::ffi::g_object_unref(self.pointer as *mut ::glib::ffi::C_GObject);
        }
    }
}

impl Clone for TreeViewColumn {
    fn clone(&self) -> TreeViewColumn {
        let pointer = unsafe {
            ::glib::ffi::g_object_ref(self.pointer as *mut ::glib::ffi::C_GObject)
        };

        TreeViewColumn {
            pointer: pointer as *mut ffi::C_GtkTreeViewColumn
        }
    }
}
