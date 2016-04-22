use glib::translate::ToGlib;

impl Into<i32> for ::IconSize {
    fn into(self) -> i32 {
        skip_assert_initialized!();
        self.to_glib() as i32
    }
}

impl Into<i32> for ::ResponseType {
    fn into(self) -> i32 {
        skip_assert_initialized!();
        self.to_glib() as i32
    }
}
