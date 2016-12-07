// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use ToolItem;
use ToolItemGroup;
use IsA;
use glib::translate::*;
use glib::{ToValue, Value};
use Widget;

impl ToolItemGroup {
    pub fn set_item_expand<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "expand".to_glib_none().0,
                                                       Value::from(&expand).to_glib_none().0) }
    }

    pub fn get_item_expand<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = false.to_value();
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "expand".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<bool>().unwrap()
    }

    pub fn set_item_fill<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, fill: bool) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "fill".to_glib_none().0,
                                                       Value::from(&fill).to_glib_none().0) }
    }

    pub fn get_item_fill<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = false.to_value();
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "fill".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<bool>().unwrap()
    }

    pub fn set_item_homogeneous<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, homogeneous: bool) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "homogeneous".to_glib_none().0,
                                                       Value::from(&homogeneous).to_glib_none().0) }
    }

    pub fn get_item_homogeneous<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = false.to_value();
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "homogeneous".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<bool>().unwrap()
    }

    pub fn set_item_new_row<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, new_row: bool) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "new-row".to_glib_none().0,
                                                       Value::from(&new_row).to_glib_none().0) }
    }

    pub fn get_item_new_row<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = false.to_value();
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "new-row".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<bool>().unwrap()
    }
}
