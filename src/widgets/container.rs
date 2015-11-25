// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

use glib::object::Upcast;
use super::widget::Widget;
use ResizeMode;

glib_wrapper! {
    pub struct Container(Object<ffi::GtkContainer>): Widget, ::Buildable;

    match fn {
        get_type => || ffi::gtk_container_get_type(),
    }
}

pub trait ContainerExt {
    fn add<T: Upcast<Widget>>(&self, widget: &T);
    fn remove<T: Upcast<Widget>>(&self, widget: &T);
    fn get_resize_mode(&self) -> ResizeMode;
    fn set_resize_mode(&self, resize_mode: ResizeMode);
    fn get_border_width(&self) -> u32;
    fn set_border_width(&self, border_width: u32);
    fn get_children(&self) -> Vec<Widget>;
}

impl<O: Upcast<Container>> ContainerExt for O {
    fn add<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_container_add(
                self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    fn remove<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_container_remove(
                self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    fn get_resize_mode(&self) -> ResizeMode {
        unsafe {
            ffi::gtk_container_get_resize_mode(self.to_glib_none().0)
        }
    }

    fn set_resize_mode(&self, resize_mode: ResizeMode) {
        unsafe {
            ffi::gtk_container_set_resize_mode(self.to_glib_none().0, resize_mode);
        }
    }

    fn get_border_width(&self) -> u32 {
        unsafe {
            ffi::gtk_container_get_border_width(self.to_glib_none().0)
        }
    }

    fn set_border_width(&self, border_width: u32) {
        unsafe {
            ffi::gtk_container_set_border_width(self.to_glib_none().0, border_width);
        }
    }

    fn get_children(&self) -> Vec<Widget> {
        unsafe {
            Vec::from_glib_container(
                ffi::gtk_container_get_children(self.to_glib_none().0))
        }
    }
}
