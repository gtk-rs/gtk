// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_RANGE;
use ffi;

pub trait RangeTrait: ::WidgetTrait {
    fn set_adjustment(&self, adjustment: &::Adjustment) -> () {
        unsafe {
            ffi::gtk_range_set_adjustment(GTK_RANGE(self.unwrap_widget()), adjustment.unwrap_pointer());
        }
    }

    fn get_adjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_range_get_adjustment(GTK_RANGE(self.unwrap_widget())))
        }
    }
}