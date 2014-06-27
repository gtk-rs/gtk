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
use gtk;
use gtk::traits;

pub struct Widget {
    pointer: *mut ffi::C_GtkWidget
}

impl Widget {
    #[allow(visible_private_types)]
    pub fn wrap(pointer: *mut ffi::C_GtkWidget) -> Widget {
        Widget {
            pointer: pointer
        }
    }

    #[doc(hidden)]
    #[allow(visible_private_types)]
    pub fn get_widget(&self) -> *mut ffi::C_GtkWidget {
        self.pointer
    }

    pub fn to_entry(self) -> gtk::Entry {
        traits::Widget::wrap(self.pointer)
    }
}