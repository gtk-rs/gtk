// This file was generated by gir (1644ef1) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Alignment(Object<ffi::GtkAlignment>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_alignment_get_type(),
    }
}

impl Alignment {
    pub fn new(xalign: f32, yalign: f32, xscale: f32, yscale: f32) -> Alignment {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_alignment_new(xalign, yalign, xscale, yscale)).downcast_unchecked()
        }
    }

    pub fn get_padding(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut padding_top = mem::uninitialized();
            let mut padding_bottom = mem::uninitialized();
            let mut padding_left = mem::uninitialized();
            let mut padding_right = mem::uninitialized();
            ffi::gtk_alignment_get_padding(self.to_glib_none().0, &mut padding_top, &mut padding_bottom, &mut padding_left, &mut padding_right);
            (padding_top, padding_bottom, padding_left, padding_right)
        }
    }

    pub fn set(&self, xalign: f32, yalign: f32, xscale: f32, yscale: f32) {
        unsafe {
            ffi::gtk_alignment_set(self.to_glib_none().0, xalign, yalign, xscale, yscale);
        }
    }

    pub fn set_padding(&self, padding_top: u32, padding_bottom: u32, padding_left: u32, padding_right: u32) {
        unsafe {
            ffi::gtk_alignment_set_padding(self.to_glib_none().0, padding_top, padding_bottom, padding_left, padding_right);
        }
    }
}
