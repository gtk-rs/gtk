// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

struct_Widget!(ToolItem);

impl ToolItem {
    pub fn new() -> Option<ToolItem> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_tool_item_new() };
        check_pointer!(tmp_pointer, ToolItem)
    }
}

impl_drop!(ToolItem);
impl_TraitWidget!(ToolItem);

impl ::ContainerTrait for ToolItem {}
impl ::BinTrait for ToolItem {}
impl ::ToolItemTrait for ToolItem {}
