use std::ptr;

use RadioToolButton;
use ToolItem;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

impl RadioToolButton {
    pub fn new() -> RadioToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new(ptr::null_mut())).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> RadioToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new_from_stock(ptr::null_mut(), stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}
