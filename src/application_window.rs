use ApplicationWindow;
use Application;
use Widget;

use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;


impl ApplicationWindow {
    pub fn new<P: IsA<Application>>(application: &P) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_application_window_new(application.to_glib_none().0)).downcast_unchecked()
        }
    }
}
