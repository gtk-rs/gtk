use std::mem;
use glib::translate::*;
use cairo_ffi::cairo_rectangle_int_t;

#[repr(C)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Uninitialized for Rectangle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

impl<'a> ToGlibPtr<'a, *const cairo_rectangle_int_t> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const cairo_rectangle_int_t, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const cairo_rectangle_int_t, self)
    }
}

impl<'a> ToGlibPtrMut<'a, *mut cairo_rectangle_int_t> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut cairo_rectangle_int_t, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut cairo_rectangle_int_t, self)
    }
}
