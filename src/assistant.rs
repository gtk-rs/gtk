use std::mem::transmute;
use std::boxed::Box as Box_;

use Assistant;
use ffi;
use glib::translate::*;
use glib_ffi;

impl Assistant {
    pub fn set_forward_page_func<F: Fn(i32) -> i32 + 'static>(&self, f: F) {
        unsafe {
            let f: Box_<Box_<Fn(i32) -> i32 + 'static>> = Box_::new(Box_::new(f));
            ffi::gtk_assistant_set_forward_page_func(self.to_glib_none().0,
                Some(forward_page_trampoline), Box_::into_raw(f) as *mut _, Some(destroy_closure))
       }
    }
}

unsafe extern "C" fn forward_page_trampoline(current_page: i32, f: glib_ffi::gpointer) -> i32 {
    callback_guard!();
    let f: &Box_<Fn(i32) -> i32 + 'static> = transmute(f);
    f(current_page)
}

unsafe extern "C" fn destroy_closure(ptr: glib_ffi::gpointer) {
    callback_guard!();
    Box_::<Box_<Fn(i32) -> i32 + 'static>>::from_raw(ptr as *mut _);
}
