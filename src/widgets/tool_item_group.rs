// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkToolItemGroup — A sub container used in a tool palette

use ffi;
use ToolItem;
use FFIWidget;
use cast::{GTK_TOOL_ITEM_GROUP, GTK_TOOL_ITEM};
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::{to_bool, to_gboolean};

struct_Widget!(ToolItemGroup);

impl ToolItemGroup {
    pub fn new(label: &str) -> Option<ToolItemGroup> {
        let tmp_pointer = unsafe {
            ffi::gtk_tool_item_group_new(label.to_glib_none().0)
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
        unsafe {
            from_glib_none(
                ffi::gtk_tool_item_group_get_label(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())))
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_tool_item_group_set_label(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), label.to_glib_none().0)
        }
    }

    pub fn get_label_widget<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_group_get_label_widget(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
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

    pub fn get_header_relief(&self) -> ::ReliefStyle {
        unsafe { ffi::gtk_tool_item_group_get_header_relief(GTK_TOOL_ITEM_GROUP(self.unwrap_widget())) }
    }

    pub fn set_header_relief(&self, style: ::ReliefStyle) {
        unsafe { ffi::gtk_tool_item_group_set_header_relief(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), style) }
    }

    pub fn insert(&self, item: &ToolItem, position: i32) {
        unsafe { ffi::gtk_tool_item_group_insert(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), GTK_TOOL_ITEM(item.unwrap_widget()),
            position as ::libc::c_int) }
    }

    pub fn set_label_widget<T: ::WidgetTrait>(&self, label_widget: &T) {
        unsafe { ffi::gtk_tool_item_group_set_label_widget(GTK_TOOL_ITEM_GROUP(self.unwrap_widget()), label_widget.unwrap_widget()) }
    }
}

impl_drop!(ToolItemGroup);
impl_TraitWidget!(ToolItemGroup);

impl ::ContainerTrait for ToolItemGroup {}
impl ::BinTrait for ToolItemGroup {}
