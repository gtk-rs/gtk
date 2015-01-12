// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use std::ffi::CString;
use gtk::{self, ffi};
use gtk::ffi::to_gboolean;
use gtk::cast::GTK_WINDOW;
use gtk::WindowPosition;

pub trait WindowTrait : gtk::WidgetTrait {
    fn set_title(&mut self, title: &str) -> () {
        unsafe {
            title.with_c_str(|c_str| {
                ffi::gtk_window_set_title(GTK_WINDOW(self.get_widget()), c_str)
            });
        }
    }

    fn set_decorated(&mut self, setting: bool) -> () {
        unsafe {
            ffi::gtk_window_set_decorated(GTK_WINDOW(self.get_widget()), to_gboolean(setting));
        }
    }

    fn get_title(&self) -> Option<String> {
        let c_title = unsafe { ffi::gtk_window_get_title(GTK_WINDOW(self.get_widget())) };
        if c_title.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(c_title as *const u8) })
        }
    }

    fn set_default_size(&self, width: i32, height: i32){
        unsafe {
            ffi::gtk_window_set_default_size(self.get_widget(), width, height)
        }
    }

    fn set_window_position(&self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(GTK_WINDOW(self.get_widget()), window_position);
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn set_titlebar<T: gtk::WidgetTrait>(&self, titlebar: &T) {
        unsafe {
            ffi::gtk_window_set_titlebar(GTK_WINDOW(self.get_widget()), titlebar.get_widget());
        }
    }
}
