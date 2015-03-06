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

//! An adapter which makes widgets scrollable

use glib::translate::{ToGlibPtr, ToTmp};
use gtk::cast::GTK_STATUSBAR;
use gtk::{self, ffi};

/// GtkViewport — An adapter which makes widgets scrollable
struct_Widget!(StatusBar);

impl StatusBar {
    pub fn new() -> Option<StatusBar> {
        let tmp_pointer = unsafe { ffi::gtk_statusbar_new() };

        check_pointer!(tmp_pointer, StatusBar)
    }

    pub fn push(&mut self, context_id: u32, text: &str) -> u32 {
        unsafe {
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_statusbar_push(GTK_STATUSBAR(self.pointer),
                                    context_id,
                                    tmp_text.to_glib_ptr())
        }
    }

    pub fn pop(&mut self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_pop(GTK_STATUSBAR(self.pointer), context_id)
        }
    }

    pub fn remove(&mut self, context_id: u32, message_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove(GTK_STATUSBAR(self.pointer), context_id, message_id)
        }
    }

    pub fn remove_all(&mut self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove_all(GTK_STATUSBAR(self.pointer), context_id)
        }
    }

    pub fn get_message_area<T: gtk::WidgetTrait + gtk::BoxTrait>(&self) -> T {
        unsafe {
            gtk::FFIWidget::wrap_widget(ffi::gtk_statusbar_get_message_area(GTK_STATUSBAR(self.pointer)))
        }
    }
}

impl_drop!(StatusBar);
impl_TraitWidget!(StatusBar);

impl gtk::ContainerTrait for StatusBar {}
impl gtk::BoxTrait for StatusBar {}
impl gtk::OrientableTrait for StatusBar {}

impl_widget_events!(StatusBar);
