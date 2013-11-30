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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.

//! Create buttons bound to a URL

use std::{ptr, str, cast};
use std::libc::c_void;

use traits::{GtkWidget, GtkButton, GtkContainer, Signal};
use utils::cast::GTK_LINKBUTTON;
use ffi;

/** 
* LinkButton — Create buttons bound to a URL
*
* # Availables signals :
* * `activate-link` : Run Last
*/
pub struct LinkButton {
	priv pointer: 			*ffi::C_GtkWidget,
	priv can_drop: 			bool,
    priv signal_handlers: 	~[~SignalHandler]
}

impl LinkButton {
	pub fn new(uri: &str) -> Option<LinkButton> {
		let tmp_pointer = unsafe { 
			uri.with_c_str(|c_str| {
				ffi::gtk_link_button_new(c_str) 
			}) 
		};
		check_pointer!(tmp_pointer, LinkButton)
	}

	pub fn new_with_label(uri: &str, label: &str) -> Option<LinkButton> {
		let tmp_pointer = unsafe { 
			uri.with_c_str(|c_uri| {
				label.with_c_str(|c_label| {
					ffi::gtk_link_button_new_with_label(c_uri, c_label) 
				}) 
			})
		};
		check_pointer!(tmp_pointer, LinkButton)
	}

	pub fn get_uri(&self) -> ~str {
		let c_str = unsafe { ffi::gtk_link_button_get_uri(GTK_LINKBUTTON(self.pointer)) };
		unsafe { str::raw::from_c_str(c_str) }
	}

	pub fn set_uri(&mut self, uri: &str) -> () {
		unsafe {
			uri.with_c_str(|c_str| { 
				ffi::gtk_link_button_set_uri(GTK_LINKBUTTON(self.pointer), c_str) 
			})
		}
	}

	pub fn set_visited(&mut self, visited: bool) -> () {
		match visited {
			true 	=> unsafe { ffi::gtk_link_button_set_visited(GTK_LINKBUTTON(self.pointer), ffi::Gtrue) },
			false 	=> unsafe { ffi::gtk_link_button_set_visited(GTK_LINKBUTTON(self.pointer), ffi::Gfalse) }
		}
	}

	pub fn get_visited(&self) -> bool {
		match unsafe { ffi::gtk_link_button_get_visited(GTK_LINKBUTTON(self.pointer)) } {
			ffi::Gfalse 	=> false,
			_				=> true
		}
	}
}

impl_GtkWidget!(LinkButton)
redirect_callback!(LinkButton)
redirect_callback_widget!(LinkButton)
struct_signal!(LinkButton)
impl_signals!(LinkButton)

impl GtkContainer for LinkButton {}
impl GtkButton for LinkButton {}

