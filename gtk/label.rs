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

//! A widget that displays a small to medium amount of text

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkWidget, GtkMisc, GtkLabel, Signal};
use ffi;

/**
* Label — A widget that displays a small to medium amount of text
*
* # Available signals:
* * `activate-current-link` : Action
* * `activate-link` : Run Last
* * `copy-clipboard` : Action
* * `move-cursor` : Action
* * `populate-popup` : Run Last
*/
pub struct Label {
    priv pointer: 			*ffi::C_GtkWidget,
    priv can_drop: 			bool,
    priv signal_handlers: 	~[~SignalHandler]
}

impl Label {
    pub fn new(text: &str) -> Option<Label> {
        let tmp_pointer = unsafe { 
        	text.with_c_str(|c_str| {
        		ffi::gtk_label_new(c_str) 
        	}) 
        };
        check_pointer!(tmp_pointer, Label)
    }

    pub fn new_with_mnemonic(text: &str) -> Option<Label> {
        let tmp_pointer = unsafe { 
            text.with_c_str(|c_str| {
                ffi::gtk_label_new_with_mnemonic(c_str) 
            }) 
        };
        check_pointer!(tmp_pointer, Label)
    }
}

impl_GtkWidget!(Label)
redirect_callback!(Label)
redirect_callback_widget!(Label)
struct_signal!(Label)
impl_signals!(Label)

impl GtkMisc for Label {}
impl GtkLabel for Label {}