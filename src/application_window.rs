use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use Application;
use ApplicationWindow;
use Widget;

impl ApplicationWindow {
    pub fn new<P: IsA<Application>>(application: &P) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_application_window_new(
                application.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}
