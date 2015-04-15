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

use Orientation;
use cast::GTK_ORIENTABLE;
use ffi;

pub trait OrientableTrait: ::WidgetTrait {
    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_orientable_get_orientation(GTK_ORIENTABLE(self.unwrap_widget()))
        }
    }

    fn set_orientation(&self, orientation: Orientation) -> () {
        unsafe {
            ffi::gtk_orientable_set_orientation(GTK_ORIENTABLE(self.unwrap_widget()), orientation)
        }
    }
}
