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

//! GtkTextView — Widget that displays a GtkTextBuffer

use gtk;
use gtk::TextBuffer;
use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk::traits;
use gtk::cast::{GTK_TEXT_VIEW, GTK_TEXT_BUFFER};

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

    pub fn set_buffer(&mut self, buffer: gtk::TextBuffer) -> () {
        unsafe {
            ffi::gtk_text_view_set_buffer(GTK_TEXT_VIEW(self.get_widget()), GTK_TEXT_BUFFER(buffer.get_widget()));
        }
    }

    pub fn get_buffer(&self) -> Option<gtk::TextBuffer> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_view_get_buffer(GTK_TEXT_VIEW(self.get_widget()))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn scroll_to_mark(&self, mark: &gtk::TextMark, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) {
        unsafe { ffi::gtk_text_view_scroll_to_mark(GTK_TEXT_VIEW(self.get_widget()), mark.get_pointer(), within_margin,
            ffi::to_gboolean(use_align), xalign, yalign) }
    }
}

impl_drop!(TextView)
impl_TraitWidget!(TextView)

impl traits::Scrollable for TextView {}

impl_widget_events!(TextView)
