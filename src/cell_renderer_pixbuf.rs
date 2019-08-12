// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use CellRendererPixbuf;
use IconSize;

pub trait CellRendererPixbufExtManual: 'static {
    fn get_property_stock_size(&self) -> IconSize;

    fn set_property_stock_size(&self, stock_size: IconSize);
}

impl<O: IsA<CellRendererPixbuf> + IsA<glib::object::Object>> CellRendererPixbufExtManual for O {
    fn get_property_stock_size(&self) -> IconSize {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut _,
                "stock-size".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            from_glib(
                value
                    .get::<u32>()
                    .expect("Return Value for property `stock_size` getter")
                    .unwrap() as i32,
            )
        }
    }

    fn set_property_stock_size(&self, stock_size: IconSize) {
        unsafe {
            let value = Value::from(&(stock_size.to_glib() as u32));
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut _,
                "stock-size".to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}
