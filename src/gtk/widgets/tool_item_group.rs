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

//! GtkToolItemGroup — A sub container used in a tool palette

use gtk::{self, ffi, ToolItem};
use gtk::FFIWidget;
use gtk::cast::{GTK_TOOL_ITEM_GROUP, GTK_TOOL_ITEM};
use glib::translate::{ToGlibPtr, ToTmp};
use glib::{to_bool, to_gboolean};

struct_Widget!(ToolItemGroup);

impl ToolItemGroup {
    pub fn new(label: &str) -> Option<ToolItemGroup> {
        let tmp_pointer = unsafe {
            let mut tmp_label = label.to_tmp_for_borrow();
            ffi::gtk_tool_item_group_new(tmp_label.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, ToolItemGroup)
    }

    pub fn get_collapsed(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tool_item_group_get_collapsed(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()))) }
    }

    pub fn set_collapsed(&self, collapsed: bool) {
        unsafe { ffi::gtk_tool_item_group_set_collapsed(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), to_gboolean(collapsed)) }
    }

    pub fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_group_get_drop_item(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()),
            x as ::libc::c_int, y as ::libc::c_int) } as *mut ffi::C_GtkWidget;

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_item_position(&self, item: &ToolItem) -> i32 {
        unsafe { ffi::gtk_tool_item_group_get_item_position(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), GTK_TOOL_ITEM(item.unwrap_widget())) }
    }

    pub fn set_item_position(&self, item: &ToolItem, position: i32) {
        unsafe { ffi::gtk_tool_item_group_set_item_position(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), GTK_TOOL_ITEM(item.unwrap_widget()),
            position as ::libc::c_int) }
    }

    pub fn get_n_items(&self) -> u32 {
        unsafe { ffi::gtk_tool_item_group_get_n_items(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())) }
    }

    pub fn get_label(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_group_get_label(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe { Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp_pointer)).to_string()) }
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let mut tmp_label = label.to_tmp_for_borrow();
            ffi::gtk_tool_item_group_set_label(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), tmp_label.to_glib_ptr())
        }
    }

    pub fn get_label_widget<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_group_get_label_widget(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_nth_item(&self, index: u32) -> Option<ToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_group_get_nth_item(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), index) } as *mut ffi::C_GtkWidget;

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_header_relief(&self) -> gtk::ReliefStyle {
        unsafe { ffi::gtk_tool_item_group_get_header_relief(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())) }
    }

    pub fn set_header_relief(&self, style: gtk::ReliefStyle) {
        unsafe { ffi::gtk_tool_item_group_set_header_relief(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), style) }
    }

    pub fn insert(&self, item: &ToolItem, position: i32) {
        unsafe { ffi::gtk_tool_item_group_insert(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), GTK_TOOL_ITEM(item.unwrap_widget()),
            position as ::libc::c_int) }
    }

    pub fn set_label_widget<T: gtk::WidgetTrait>(&self, label_widget: &T) {
        unsafe { ffi::gtk_tool_item_group_set_label_widget(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), label_widget.unwrap_widget()) }
    }
}

impl_drop!(ToolItemGroup);
impl_TraitWidget!(ToolItemGroup);

impl gtk::ContainerTrait for ToolItemGroup {}
impl gtk::BinTrait for ToolItemGroup {}

impl_widget_events!(ToolItemGroup);
