// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A ToolItem containing a toggle button

use ffi;
use glib::translate::ToGlibPtr;

/// ToggleToolButton — A ToolItem containing a toggle button
struct_Widget!(ToggleToolButton);

impl ToggleToolButton {
    pub fn new() -> Option<ToggleToolButton> {
        let tmp_pointer = unsafe { ffi::gtk_toggle_tool_button_new() };
        check_pointer!(tmp_pointer, ToggleToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<ToggleToolButton> {
        let tmp_pointer = unsafe { ffi::gtk_toggle_tool_button_new_from_stock(stock_id.borrow_to_glib().0) };
        check_pointer!(tmp_pointer, ToggleToolButton)
    }
}

impl_drop!(ToggleToolButton);
impl_TraitWidget!(ToggleToolButton);

impl ::ContainerTrait for ToggleToolButton {}
impl ::BinTrait for ToggleToolButton {}
impl ::ToolItemTrait for ToggleToolButton {}
impl ::ToolButtonTrait for ToggleToolButton {}
impl ::ToggleToolButtonTrait for ToggleToolButton {}

impl_widget_events!(ToggleToolButton);
