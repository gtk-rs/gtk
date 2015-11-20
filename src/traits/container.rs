// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_uint;

use cast::GTK_CONTAINER;
use ResizeMode;
use ffi;

pub trait ContainerTrait: ::WidgetTrait {
    fn add<'r, T: ::WidgetTrait>(&'r self, widget: &'r T) {
        unsafe {
            ffi::gtk_container_add(GTK_CONTAINER(self.unwrap_widget()), widget.unwrap_widget());
        }
    }

    fn remove<'r, T: ::WidgetTrait>(&'r self, widget: &'r T) {
        unsafe {
            ffi::gtk_container_remove(GTK_CONTAINER(self.unwrap_widget()), widget.unwrap_widget());
        }
    }

    fn get_resize_mode(&self) -> ResizeMode {
        unsafe {
            ffi::gtk_container_get_resize_mode(GTK_CONTAINER(self.unwrap_widget()))
        }
    }

    fn set_resize_mode(&self, resize_mode: ResizeMode) -> () {
        unsafe {
            ffi::gtk_container_set_resize_mode(GTK_CONTAINER(self.unwrap_widget()), resize_mode);
        }
    }

    fn get_border_width(&self) -> u32 {
        unsafe {
            ffi::gtk_container_get_border_width(GTK_CONTAINER(self.unwrap_widget())) as u32
        }
    }

    fn set_border_width(&self, border_width: u32) -> () {
        unsafe {
            ffi::gtk_container_set_border_width(GTK_CONTAINER(self.unwrap_widget()), border_width as c_uint);
        }
    }

}
