use std::mem::transmute;
use std::boxed::Box as Box_;

use Assistant;
use ffi;
use glib::translate::*;
use glib::object::IsA;
use glib_ffi;

pub trait AssistantExtManual: 'static {
    fn set_forward_page_func<F: Fn(i32) -> i32 + 'static>(&self, f: F);
}

impl<O: IsA<Assistant>> AssistantExtManual for O {
    fn set_forward_page_func<F: Fn(i32) -> i32 + 'static>(&self, f: F) {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            ffi::gtk_assistant_set_forward_page_func(self.as_ref().to_glib_none().0,
                Some(forward_page_trampoline::<F>), Box_::into_raw(f) as *mut _, Some(destroy_closure::<F>))
       }
    }
}

unsafe extern "C" fn forward_page_trampoline<F: Fn(i32) -> i32 + 'static>(current_page: i32, f: glib_ffi::gpointer) -> i32 {
    let f: &F = transmute(f);
    f(current_page)
}

unsafe extern "C" fn destroy_closure<F: Fn(i32) -> i32 + 'static>(ptr: glib_ffi::gpointer) {
    Box_::<F>::from_raw(ptr as *mut _);
}
