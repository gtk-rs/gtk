// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_BIN;
use ffi;

pub trait BinTrait: ::WidgetTrait + ::ContainerTrait {
    fn get_child<T: ::WidgetTrait>(&self) ->  Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_bin_get_child(GTK_BIN(self.unwrap_widget()))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}
