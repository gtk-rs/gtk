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

use ffi;

pub struct AboutDialog {
    pointer:   *ffi::C_GtkAboutDialog,
    can_drop:  bool
}

impl AboutDialog {
    pub fn new() -> Option<AboutDialog> {
        let tmp_pointer = unsafe { ffi::gtk_about_dialog_new() };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(Adjustment {
                pointer:    tmp_pointer,
                can_drop:   true
            })
        }
    }

    pub fn get_program_name(&self) -> String {
        unsafe { ffi::gtk_about_dialog_get_program_name() };
    }
}