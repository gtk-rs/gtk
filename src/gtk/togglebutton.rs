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

//! A button to launch a font chooser dialog

use std::ptr;
use libc::{c_void};

use traits::{GtkWidget, GtkButton, GtkContainer, GtkToggleButton, Signal};
use ffi;
use std;

/**
* ToggleButton — A button to launch a font chooser dialog
*
* # Availables signals :
* * `toggled` : Run First
*/
pub struct ToggleButton {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl ToggleButton {
    pub fn new() -> Option<ToggleButton> {
        let tmp_pointer = unsafe { ffi::gtk_toggle_button_new() };
        check_pointer!(tmp_pointer, ToggleButton)
    }

    pub fn new_with_label(label: &str) -> Option<ToggleButton> {
        let tmp_pointer = unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_toggle_button_new_with_label(c_str)
            })
        };
        check_pointer!(tmp_pointer, ToggleButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<ToggleButton> {
        let tmp_pointer = unsafe {
            mnemonic.with_c_str(|c_str| {
                ffi::gtk_toggle_button_new_with_mnemonic(c_str)
            })
        };
        check_pointer!(tmp_pointer, ToggleButton)
    }

}

impl_GtkWidget!(ToggleButton)
redirect_callback!(ToggleButton)
redirect_callback_widget!(ToggleButton)
struct_signal!(ToggleButton)
impl_signals!(ToggleButton)

impl GtkContainer for ToggleButton {}
impl GtkButton for ToggleButton {}
impl GtkToggleButton for ToggleButton {}
