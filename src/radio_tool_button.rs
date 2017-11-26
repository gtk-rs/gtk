use std::ptr;

use RadioToolButton;
use ToolItem;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib::Value;
use gobject_ffi;

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

    pub fn join_group<'a, P: Into<Option<&'a RadioToolButton>>>(&self, group: P) {
        let group = group.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "group".to_glib_none().0, Value::from(group).to_glib_none().0);
        }
    }
}
