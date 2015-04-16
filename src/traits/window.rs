// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{FromGlibPtr, ToGlibPtr};
use ffi;
use glib::to_gboolean;
use cast::GTK_WINDOW;

pub trait WindowTrait : ::WidgetTrait {
    fn set_title(&self, title: &str) -> () {
        unsafe {
            ffi::gtk_window_set_title(GTK_WINDOW(self.unwrap_widget()), title.borrow_to_glib().0);
        }
    }

    fn set_decorated(&self, setting: bool) -> () {
        unsafe {
            ffi::gtk_window_set_decorated(GTK_WINDOW(self.unwrap_widget()), to_gboolean(setting));
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_window_get_title(GTK_WINDOW(self.unwrap_widget())))
        }
    }

    fn set_default_size(&self, width: i32, height: i32){
        unsafe {
            ffi::gtk_window_set_default_size(self.unwrap_widget(), width, height)
        }
    }

    fn set_window_position(&self, window_position: ::WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(GTK_WINDOW(self.unwrap_widget()), window_position);
        }
    }

    #[cfg(feature = "gtk_3_10")]
    fn set_titlebar<T: ::WidgetTrait>(&self, titlebar: &T) {
        unsafe {
            ffi::gtk_window_set_titlebar(GTK_WINDOW(self.unwrap_widget()), titlebar.unwrap_widget());
        }
    }
}
