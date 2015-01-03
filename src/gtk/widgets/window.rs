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

//! Toplevel which can contain other widgets

use gtk::{self, ffi};
use gtk::WindowType;

/**
* Window — Toplevel which can contain other widgets
*
* # Available signals:
* * `activate-default` : Action
* * `activate-focus` : Action
* * `keys-changed` : Run First
* * `set-focus` : Run Last
*/
struct_Widget!(Window);

impl Window {
    pub fn new(window_type: WindowType) -> Option<Window> {
        let tmp_pointer = unsafe { ffi::gtk_window_new(window_type) };
        check_pointer!(tmp_pointer, Window)
    }
}

impl_drop!(Window);
impl_TraitWidget!(Window);

impl gtk::ContainerTrait for Window {}
impl gtk::WindowTrait for Window {}
impl gtk::BinTrait for Window {}

impl_widget_events!(Window);
