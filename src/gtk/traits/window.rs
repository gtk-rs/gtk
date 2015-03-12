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

use glib::translate::{FromGlibPtr, ToGlibPtr};
use gtk::{self, ffi};
use glib::to_gboolean;
use gtk::cast::GTK_WINDOW;
use gtk::WindowPosition;

pub trait WindowTrait : gtk::WidgetTrait {
    fn set_title(&mut self, title: &str) -> () {
        unsafe {
            ffi::gtk_window_set_title(GTK_WINDOW(self.unwrap_widget()), title.borrow_to_glib().0);
        }
    }

    fn set_decorated(&mut self, setting: bool) -> () {
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

    fn set_window_position(&self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(GTK_WINDOW(self.unwrap_widget()), window_position);
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn set_titlebar<T: gtk::WidgetTrait>(&self, titlebar: &T) {
        unsafe {
            ffi::gtk_window_set_titlebar(GTK_WINDOW(self.unwrap_widget()), titlebar.unwrap_widget());
        }
    }
}
