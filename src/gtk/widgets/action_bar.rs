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

//! Hide and show with animation

use gtk::cast::{GTK_ACTION_BAR};
use gtk::{self, ffi};

struct_Widget!(ActionBar);

impl ActionBar {
    pub fn new() -> Option<ActionBar> {
        let tmp_pointer = unsafe { ffi::gtk_action_bar_new() };
        check_pointer!(tmp_pointer, ActionBar)
    }

    pub fn get_center_widget<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_action_bar_get_center_widget(GTK_ACTION_BAR(self.pointer))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn set_center_widget<T: gtk::WidgetTrait>(&mut self, center_widget: &T) {
        unsafe {
            ffi::gtk_action_bar_set_center_widget(GTK_ACTION_BAR(self.pointer),
                                                  center_widget.get_widget())
        }
    }

    pub fn pack_start<T: gtk::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_start(GTK_ACTION_BAR(self.pointer),
                                           child.get_widget())
        }
    }

    pub fn pack_end<T: gtk::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_end(GTK_ACTION_BAR(self.pointer),
                                         child.get_widget())
        }
    }
}

impl_drop!(ActionBar);
impl_TraitWidget!(ActionBar);

impl gtk::ContainerTrait for ActionBar {}
impl gtk::BinTrait for ActionBar {}

impl_widget_events!(ActionBar);
