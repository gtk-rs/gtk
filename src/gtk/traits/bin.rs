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

use gtk::cast::GTK_BIN;
use gtk::traits;
use gtk::ffi;

pub trait Bin: traits::Widget + traits::Container {
    fn get_child<T: traits::Widget>(&self) ->  Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_bin_get_child(GTK_BIN(self.get_widget()))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(traits::Widget::wrap(tmp_pointer))
        }
    }
}
