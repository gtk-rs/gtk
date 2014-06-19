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

//! Create widgets with a discrete toggle button

use libc::{c_void};

use traits::{GtkWidget, GtkButton, GtkContainer, GtkToggleButton, Signal};
use ffi;

/// CheckButton — Create widgets with a discrete toggle button
pub struct CheckButton {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl CheckButton {
    pub fn new() -> Option<CheckButton> {
        let tmp_pointer = unsafe { ffi::gtk_check_button_new() };
        check_pointer!(tmp_pointer, CheckButton)
    }

    pub fn new_with_label(label: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_check_button_new_with_label(c_str)
            })
        };
        check_pointer!(tmp_pointer, CheckButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            mnemonic.with_c_str(|c_str| {
                ffi::gtk_check_button_new_with_mnemonic(c_str)
            })
        };
        check_pointer!(tmp_pointer, CheckButton)
    }

}

impl_GtkWidget!(CheckButton)
redirect_callback!(CheckButton)
redirect_callback_widget!(CheckButton)
struct_signal!(CheckButton)
impl_signals!(CheckButton)


impl GtkContainer for CheckButton {}
impl GtkButton for CheckButton {}
impl GtkToggleButton for CheckButton {}