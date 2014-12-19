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

use std::ptr;

use gtk::{mod, ffi};

/// GtkScrolledWindow — Adds scrollbars to its child widget
struct_Widget!(ScrolledWindow);

impl ScrolledWindow {
    pub fn new(h_adjustment: Option<gtk::Adjustment>, v_adjustment: Option<gtk::Adjustment>) -> Option<ScrolledWindow> {

        let tmp_pointer = unsafe {
            ffi::gtk_scrolled_window_new(
                h_adjustment.map_or(ptr::null_mut(), |p| { p.get_pointer() }),
                v_adjustment.map_or(ptr::null_mut(), |p| { p.get_pointer() })
            )
        };

        check_pointer!(tmp_pointer, ScrolledWindow)
    }
}

impl_drop!(ScrolledWindow);
impl_TraitWidget!(ScrolledWindow);

impl gtk::ScrolledWindowTrait for ScrolledWindow {}
impl gtk::ContainerTrait for ScrolledWindow {}

impl_widget_events!(ScrolledWindow);
