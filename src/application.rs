use Application;
use ffi;
use gio::ApplicationFlags;
use glib::object::IsA;
use glib::translate::*;
use rt;

pub trait ApplicationExtManual {
    #[cfg(feature = "v3_6")]
    fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Result<Application, ()>;

    #[cfg(not(feature = "v3_6"))]
    fn new(application_id: &str, flags: ApplicationFlags) -> Result<Application, ()>;
}

impl<O: IsA<Application>> ApplicationExtManual for O {
    #[cfg(feature = "v3_6")]
    fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Result<Application, ()> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib()))
                .ok_or(())
        }
    }

    #[cfg(not(feature = "v3_6"))]
    fn new(application_id: &str, flags: ApplicationFlags) -> Result<Application, ()> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib()))
                .ok_or(())
        }
    }
}
