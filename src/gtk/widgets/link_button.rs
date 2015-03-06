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

//! Create buttons bound to a URL

use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};
use gtk::cast::GTK_LINKBUTTON;
use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};

/// LinkButton — Create buttons bound to a URL
/*
* # Availables signals :
* * `activate-link` : Run Last
*/
struct_Widget!(LinkButton);

impl LinkButton {
    pub fn new(uri: &str) -> Option<LinkButton> {
        let tmp_pointer = unsafe {
            let mut tmp_uri = uri.to_tmp_for_borrow();
            ffi::gtk_link_button_new(tmp_uri.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, LinkButton)
    }

    pub fn new_with_label(uri: &str, label: &str) -> Option<LinkButton> {
        let tmp_pointer = unsafe {
            let mut tmp_uri = uri.to_tmp_for_borrow();
            let mut tmp_label = label.to_tmp_for_borrow();
            ffi::gtk_link_button_new_with_label(tmp_uri.as_ptr(), tmp_label.as_ptr())
        };
        check_pointer!(tmp_pointer, LinkButton)
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_link_button_get_uri(GTK_LINKBUTTON(self.pointer)))
        }
    }

    pub fn set_uri(&mut self, uri: &str) -> () {
        unsafe {
            let mut tmp_uri = uri.to_tmp_for_borrow();
            ffi::gtk_link_button_set_uri(GTK_LINKBUTTON(self.pointer), tmp_uri.as_ptr())
        }
    }

    pub fn set_visited(&mut self, visited: bool) -> () {
        unsafe { ffi::gtk_link_button_set_visited(GTK_LINKBUTTON(self.pointer), to_gboolean(visited)); }
    }

    pub fn get_visited(&self) -> bool {
        unsafe { to_bool(ffi::gtk_link_button_get_visited(GTK_LINKBUTTON(self.pointer))) }
    }
}

impl_drop!(LinkButton);
impl_TraitWidget!(LinkButton);

impl gtk::ContainerTrait for LinkButton {}
impl gtk::ButtonTrait for LinkButton {}

impl_widget_events!(LinkButton);
impl_button_events!(LinkButton);
