// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Hide and show with animation

use cast::{GTK_ACTION_BAR};
use ffi;

struct_Widget!(ActionBar);

impl ActionBar {
    pub fn new() -> Option<ActionBar> {
        let tmp_pointer = unsafe { ffi::gtk_action_bar_new() };
        check_pointer!(tmp_pointer, ActionBar)
    }

    pub fn get_center_widget<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_action_bar_get_center_widget(GTK_ACTION_BAR(self.pointer))
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_center_widget<T: ::WidgetTrait>(&self, center_widget: &T) {
        unsafe {
            ffi::gtk_action_bar_set_center_widget(GTK_ACTION_BAR(self.pointer),
                                                  center_widget.unwrap_widget())
        }
    }

    pub fn pack_start<T: ::WidgetTrait>(&self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_start(GTK_ACTION_BAR(self.pointer),
                                           child.unwrap_widget())
        }
    }

    pub fn pack_end<T: ::WidgetTrait>(&self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_end(GTK_ACTION_BAR(self.pointer),
                                         child.unwrap_widget())
        }
    }
}

impl_drop!(ActionBar);
impl_TraitWidget!(ActionBar);

impl ::ContainerTrait for ActionBar {}
impl ::BinTrait for ActionBar {}
