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

use libc::c_char;
use ffi;
use cast::GTK_TEXT_BUFFER;

pub trait TextBufferTrait: ::WidgetTrait {
    fn set_text(&self, text: &str) {
        unsafe {
            // Don't need a null-terminated string here
            ffi::gtk_text_buffer_set_text(
                GTK_TEXT_BUFFER(self.unwrap_widget()),
                text.as_ptr() as *const c_char,
                text.len() as i32)
        }
    }

}
