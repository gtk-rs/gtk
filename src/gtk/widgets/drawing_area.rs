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

use gtk::ffi;

/// GtkDrawingArea — A widget for custom user interface elements
struct_Widget!(DrawingArea);

impl DrawingArea {
    pub fn new() -> Option<DrawingArea> {
        let tmp_pointer = unsafe { ffi::gtk_drawing_area_new() };
        check_pointer!(tmp_pointer, DrawingArea)
    }
}

impl_TraitWidget!(DrawingArea);

impl_widget_events!(DrawingArea);
