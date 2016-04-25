use Application;
use ffi;
use gio::ApplicationFlags;
use glib::translate::*;
use rt;

impl Application {
    #[cfg(feature = "v3_6")]
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Result<Application, ()> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib()))
                .ok_or(())
        }
    }

    #[cfg(not(feature = "v3_6"))]
    pub fn new(application_id: &str, flags: ApplicationFlags) -> Result<Application, ()> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib()))
                .ok_or(())
        }
    }
}
