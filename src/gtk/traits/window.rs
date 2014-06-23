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

use std::str;
use ffi;
use gtk::traits::WidgetTrait;
use utils::cast::GTK_WINDOW;

pub trait WindowTrait : WidgetTrait {
    fn set_title(&mut self, title: &str) -> () {
        unsafe {
            title.with_c_str(|c_str| {
                ffi::gtk_window_set_title(GTK_WINDOW(self.get_widget()), c_str)
            });
        }
    }

    fn get_title(&self) -> Option<String> {
        let c_title = unsafe { ffi::gtk_window_get_title(GTK_WINDOW(self.get_widget())) };
        if c_title.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(c_title) })
        }
    }
}