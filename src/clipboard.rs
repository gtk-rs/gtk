use ffi;
use glib::translate::*;
use libc::{c_char, c_int};
use Clipboard;

impl Clipboard {
    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_clipboard_set_text(self.to_glib_none().0, text.as_ptr() as *const c_char,
                                        text.len() as c_int);
        }
    }
}
