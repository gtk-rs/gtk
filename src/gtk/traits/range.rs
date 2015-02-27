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

use gtk::cast::GTK_RANGE;
use gtk::{self, ffi};

pub trait RangeTrait: gtk::WidgetTrait {
    fn set_adjustment(&mut self, adjustment: &gtk::Adjustment) -> () {
        unsafe {
            ffi::gtk_range_set_adjustment(GTK_RANGE(self.unwrap_widget()), adjustment.unwrap_pointer());
        }
    }

    fn get_adjustment(&self) -> gtk::Adjustment {
        unsafe {
            gtk::Adjustment::wrap_pointer(ffi::gtk_range_get_adjustment(GTK_RANGE(self.unwrap_widget())))
        }
    }
}