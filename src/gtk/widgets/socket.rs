// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! GtkSocket — Container for widgets from other processes

use gtk::{self, ffi};
use gtk::FFIWidget;
//use gtk::cast::GTK_SOCKET;

struct_Widget!(Socket);

impl Socket {
    pub fn new() -> Option<Socket> {
        let tmp_pointer = unsafe { ffi::gtk_socket_new() };

        check_pointer!(tmp_pointer, Socket)
    }

    /*pub fn add_id(&self, window: Window) {
        unsafe { ffi::gtk_socket_add_id(GTK_SOCKET(self.get_widget()), window) };
    }

    pub fn get_id(&self) -> Window {
        unsafe { ffi::gtk_socket_get_id(GTK_SOCKET(self.get_widget())) };
    }

    pub fn get_plug_window(&self) -> GdkWindow {
        let tmp_pointer = unsafe { ffi::gtk_socket_get_plug_window(GTK_SOCKET(self.get_widget())) };

        // add end of code
    }*/
}

impl_drop!(Socket);
impl_TraitWidget!(Socket);

impl gtk::ContainerTrait for Socket {}

impl_widget_events!(Socket);