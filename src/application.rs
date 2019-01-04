use Application;
use ffi;
use gio::ApplicationFlags;
use glib::translate::*;
use rt;
use glib;

impl Application {
    #[cfg(feature = "v3_6")]
    pub fn new<'a, I: Into<Option<&'a str>>>(application_id: I, flags: ApplicationFlags) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.into().to_glib_none().0, flags.to_glib())
            )
            .ok_or_else(|| glib_bool_error!("Failed to create application"))
        }
    }

    #[cfg(not(feature = "v3_6"))]
    pub fn new(application_id: &str, flags: ApplicationFlags) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib())
            )
            .ok_or_else(|| glib_bool_error!("Failed to create application"))
        }
    }
}
