// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! An adapter which makes widgets scrollable

use glib::translate::ToGlibPtr;
use cast::GTK_STATUSBAR;
use ffi;

/// GtkViewport — An adapter which makes widgets scrollable
struct_Widget!(StatusBar);

impl StatusBar {
    pub fn new() -> Option<StatusBar> {
        let tmp_pointer = unsafe { ffi::gtk_statusbar_new() };

        check_pointer!(tmp_pointer, StatusBar)
    }

    pub fn push(&self, context_id: u32, text: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_push(GTK_STATUSBAR(self.pointer),
                                    context_id,
                                    text.borrow_to_glib().0)
        }
    }

    pub fn pop(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_pop(GTK_STATUSBAR(self.pointer), context_id)
        }
    }

    pub fn remove(&self, context_id: u32, message_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove(GTK_STATUSBAR(self.pointer), context_id, message_id)
        }
    }

    pub fn remove_all(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove_all(GTK_STATUSBAR(self.pointer), context_id)
        }
    }

    pub fn get_message_area<T: ::WidgetTrait + ::BoxTrait>(&self) -> T {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_statusbar_get_message_area(GTK_STATUSBAR(self.pointer)))
        }
    }
}

impl_drop!(StatusBar);
impl_TraitWidget!(StatusBar);

impl ::ContainerTrait for StatusBar {}
impl ::BoxTrait for StatusBar {}
impl ::OrientableTrait for StatusBar {}
