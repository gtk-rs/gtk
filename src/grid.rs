use ffi;
use glib::translate::*;
use glib::Value;
use IsA;
use Grid;
use Widget;

impl Grid {
    pub fn set_child_height<T: IsA<Widget>>(&self, item: &T, height: i32) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "height".to_glib_none().0,
                                                       Value::from(&height).to_glib_none().0) }
    }

    pub fn get_child_height<T: IsA<Widget>>(&self, child: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       child.to_glib_none().0,
                                                       "height".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<i32>().unwrap()
    }

    pub fn set_child_left_attach<T: IsA<Widget>>(&self, item: &T, left_attach: i32) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "left_attach".to_glib_none().0,
                                                       Value::from(&left_attach).to_glib_none().0) }
    }

    pub fn get_child_left_attach<T: IsA<Widget>>(&self, child: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       child.to_glib_none().0,
                                                       "left_attach".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<i32>().unwrap()
    }

    pub fn set_child_top_attach<T: IsA<Widget>>(&self, item: &T, top_attach: i32) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "top_attach".to_glib_none().0,
                                                       Value::from(&top_attach).to_glib_none().0) }
    }

    pub fn get_child_top_attach<T: IsA<Widget>>(&self, child: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       child.to_glib_none().0,
                                                       "top_attach".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<i32>().unwrap()
    }

    pub fn set_child_width<T: IsA<Widget>>(&self, item: &T, width: i32) {
        unsafe { ffi::gtk_container_child_set_property(self.to_glib_none().0,
                                                       item.to_glib_none().0,
                                                       "width".to_glib_none().0,
                                                       Value::from(&width).to_glib_none().0) }
    }

    pub fn get_child_width<T: IsA<Widget>>(&self, child: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe { ffi::gtk_container_child_get_property(self.to_glib_none().0,
                                                       child.to_glib_none().0,
                                                       "width".to_glib_none().0,
                                                       value.to_glib_none_mut().0); }
        value.get::<i32>().unwrap()
    }
}
