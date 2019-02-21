use ffi;
use glib::translate::{from_glib, ToGlib};
use IconSize;
use ResponseType;

impl IconSize {
    pub fn unscaled() -> IconSize {
        skip_assert_initialized!();
        IconSize::__Unknown(-1)
    }
}

impl Into<i32> for IconSize {
    fn into(self) -> i32 {
        skip_assert_initialized!();
        self.to_glib() as i32
    }
}

impl From<i32> for IconSize {
    fn from(val: i32) -> Self {
        skip_assert_initialized!();
        from_glib(val as ffi::GtkIconSize)
    }
}

impl Into<i32> for ResponseType {
    fn into(self) -> i32 {
        skip_assert_initialized!();
        self.to_glib() as i32
    }
}

impl From<i32> for ResponseType {
    fn from(val: i32) -> Self {
        skip_assert_initialized!();
        from_glib(val as ffi::GtkResponseType)
    }
}
