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

use gtk::{mod, ffi};
use gtk::cast::GTK_TEXT_BUFFER;

pub trait TextBufferTrait: gtk::WidgetTrait {
    fn set_text(&self, text: String) {
        unsafe {
            text.as_slice().with_c_str(|c_str| {
                ffi::gtk_text_buffer_set_text(GTK_TEXT_BUFFER(self.get_widget()), c_str, text.len() as i32);
            })
        }
    }

}