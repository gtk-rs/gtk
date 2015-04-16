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
