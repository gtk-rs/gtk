use ffi;
use glib::object::IsA;
use libc::{c_char, c_int};
use Clipboard;

pub trait ClipboardExtManual {
    fn set_text(&self, text: &str);
}

impl<O: IsA<Clipboard>> ClipboardExtManual for O {
    fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_clipboard_set_text(self.to_glib_none().0, text.as_ptr() as *const c_char,
                                        text.len() as c_int);
        }
    }
}
