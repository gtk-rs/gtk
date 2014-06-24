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

// NB: This file is not actually packaged into rgtk

use gtk::ffi;
use gtk::traits;
use gtk;
use gtk::traits;

pub struct Widget {
    pointer:           *ffi::C_GtkWidget
}

impl Widget {
    pub fn wrap(pointer: *ffi::C_GtkWidget) -> Widget {
        Widget {
            pointer: pointer
        }
    }

    pub fn to_entry(self) -> gtk::Entry {
        traits::Widget::wrap_widget(self.pointer)
    }
}