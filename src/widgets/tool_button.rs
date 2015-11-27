// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;

use glib::translate::ToGlibPtr;
use ffi;

struct_Widget!(ToolButton);

impl ToolButton {
    pub fn new<T: ::WidgetTrait>(icon_widget: Option<&T>, label: Option<&str>) -> Option<ToolButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            let icon_widget_ptr = match icon_widget {
                Some(i) => i.unwrap_widget(),
                None    => ptr::null_mut(),
            };
            ffi::gtk_tool_button_new(icon_widget_ptr, label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, ToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<ToolButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_tool_button_new_from_stock(stock_id.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, ToolButton)
    }
}

impl_drop!(ToolButton);
impl_TraitWidget!(ToolButton);

impl ::ContainerTrait for ToolButton {}
impl ::BinTrait for ToolButton {}
impl ::ToolItemTrait for ToolButton {}
impl ::ToolButtonTrait for ToolButton {}
