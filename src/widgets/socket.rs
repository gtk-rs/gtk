// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkSocket — Container for widgets from other processes

use ffi;
use FFIWidget;
//use cast::GTK_SOCKET;

struct_Widget!(Socket);

impl Socket {
    pub fn new() -> Option<Socket> {
        let tmp_pointer = unsafe { ffi::gtk_socket_new() };

        check_pointer!(tmp_pointer, Socket)
    }

    /*pub fn add_id(&self, window: Window) {
        unsafe { ffi::gtk_socket_add_id(GTK_SOCKET(self.unwrap_widget()), window) };
    }

    pub fn get_id(&self) -> Window {
        unsafe { ffi::gtk_socket_get_id(GTK_SOCKET(self.unwrap_widget())) };
    }

    pub fn get_plug_window(&self) -> GdkWindow {
        let tmp_pointer = unsafe { ffi::gtk_socket_get_plug_window(GTK_SOCKET(self.unwrap_widget())) };

        // add end of code
    }*/
}

impl_drop!(Socket);
impl_TraitWidget!(Socket);

impl ::ContainerTrait for Socket {}
