// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! The base class of widgets that can be added to ToolShe

use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_SEPARATORTOOLITEM;

/// ToolItem — The base class of widgets that can be added to ToolShe
struct_Widget!(SeparatorToolItem);

impl SeparatorToolItem {
    pub fn new() -> Option<SeparatorToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_separator_tool_item_new() };
        check_pointer!(tmp_pointer, SeparatorToolItem)
    }

    pub fn set_draw(&self, draw: bool) -> () {
        unsafe { ffi::gtk_separator_tool_item_set_draw(GTK_SEPARATORTOOLITEM(self.pointer), to_gboolean(draw)); }
    }

    pub fn get_draw(&self) -> bool {
        unsafe { to_bool(ffi::gtk_separator_tool_item_get_draw(GTK_SEPARATORTOOLITEM(self.pointer))) }
    }
}

impl_drop!(SeparatorToolItem);
impl_TraitWidget!(SeparatorToolItem);

impl ::ContainerTrait for SeparatorToolItem {}
impl ::BinTrait for SeparatorToolItem {}
impl ::ToolItemTrait for SeparatorToolItem {}
