// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{from_glib_none};
use ffi;
use cast::GTK_APP_CHOOSER;

pub trait AppChooserTrait: ::WidgetTrait {
    fn get_app_info(&self) -> Option<::AppInfo> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_get_app_info(GTK_APP_CHOOSER(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    fn get_content_info(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_app_chooser_get_content_type(GTK_APP_CHOOSER(self.unwrap_widget())))
        }
    }

    fn refresh(&self) -> () {
        unsafe { ffi::gtk_app_chooser_refresh(GTK_APP_CHOOSER(self.unwrap_widget())) }
    }
}
