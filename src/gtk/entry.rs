// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A single line text entry field

use libc::{c_void};
use std::ptr;

use traits::{GtkWidget, GtkEntry, Signal};
use gtk;
use ffi;
use std;

/** 
* Entry — A single line text entry field
*
* # Availables signals :
* * `activate` : Action
* * `backspace` : Action
* * `copy-clipboard` : Action
* * `cut-clipboard` : Action
* * `delete-from-cursor` : Action
* * `icon-press` : Run Last
* * `icon-release` : Run Last
* * `insert-at-cursor` : Action
* * `move-cursor` : Action
* * `paste-clipboard` : Action
* * `populate-popup` : Run Last
* * `preedit-changed` : Action
* * `toggle-overwrite` : Action
*/
pub struct Entry {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl Entry {
    pub fn new() -> Option<Entry> {
        let tmp_pointer = unsafe { ffi::gtk_entry_new() };
        check_pointer!(tmp_pointer, Entry)
    }

    pub fn new_with_buffer(buffer: &gtk::EntryBuffer) -> Option<Entry> {
        let tmp_pointer = unsafe { ffi::gtk_entry_new_with_buffer(buffer.get_pointer()) };
        check_pointer!(tmp_pointer, Entry)
    }
}

impl_GtkWidget!(Entry)
redirect_callback!(Entry)
redirect_callback_widget!(Entry)
struct_signal!(Entry)
impl_signals!(Entry)

impl GtkEntry for Entry {}
