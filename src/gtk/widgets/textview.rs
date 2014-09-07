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

use gtk;
use gtk::TextBuffer;
use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk::traits;
use gtk::cast::GTK_TEXT_BUFFER;

/// GtkTextView — Widget that displays a GtkTextBuffer

struct_Widget!(TextView)

impl TextView {
    pub fn new() -> Option<TextView> {
        let tmp_pointer = unsafe { ffi::gtk_text_view_new() };
        check_pointer!(tmp_pointer, TextView)
    }

    pub fn new_with_buffer(buffer: gtk::TextBuffer) -> Option<TextView> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_view_new_with_buffer(GTK_TEXT_BUFFER(buffer.get_widget()))
        };
        check_pointer!(tmp_pointer, TextView)
    }
}

impl_drop!(TextView)
impl_TraitWidget!(TextView)
impl traits::TextView for TextView {}
impl traits::Scrollable for TextView {}
