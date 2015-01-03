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

use gtk::{self, ffi};

struct_Widget!(Dialog);

impl Dialog {
    fn new() -> Option<Dialog> {
        let tmp_pointer = unsafe {
            ffi::gtk_dialog_new()
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    /*fn new_width_buttons(title: &str, parent: Window, DialogFlags: flags, first_button_text: &str, ...) -> Option<Dialog> {
        let tmp_pointer = unsafe {
            ffi::gtk_dialog_new_with_buttons();
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }*/
}

impl_drop!(Dialog);
impl_TraitWidget!(Dialog);

impl gtk::ContainerTrait for Dialog {}
impl gtk::BinTrait for Dialog {}
impl gtk::WindowTrait for Dialog {}
impl gtk::DialogTrait for Dialog {}

impl_widget_events!(Dialog);
