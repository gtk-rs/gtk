// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

use traits::{GtkWidget};
use ffi;
use std;
use gtk;

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
        GtkWidget::wrap_widget(self.pointer)
    }
}