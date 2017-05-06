use ffi;
use gdk::DragContext;
use glib::object::IsA;


pub trait DragContextExtManual {
    fn drag_finish(&self, success: bool, del: bool, time_: u32);
}

impl<O: IsA<DragContext>> DragContextExtManual for O {
    fn drag_finish(&self, success: bool, del: bool, time_: u32) {
        unsafe { ffi::gtk_drag_finish(self.to_glib_none().0,
                                      success as i32,
                                      del as i32,
                                      time_) };
    }
}
