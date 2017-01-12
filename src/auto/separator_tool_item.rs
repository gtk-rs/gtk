// This file was generated by gir (1644ef1) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use ToolItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SeparatorToolItem(Object<ffi::GtkSeparatorToolItem>): ToolItem, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_separator_tool_item_get_type(),
    }
}

impl SeparatorToolItem {
    pub fn new() -> SeparatorToolItem {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_separator_tool_item_new()).downcast_unchecked()
        }
    }

    pub fn get_draw(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_separator_tool_item_get_draw(self.to_glib_none().0))
        }
    }

    pub fn set_draw(&self, draw: bool) {
        unsafe {
            ffi::gtk_separator_tool_item_set_draw(self.to_glib_none().0, draw.to_glib());
        }
    }
}
