// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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
