// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkPopover — Context dependent bubbles

use ffi;
use cast::GTK_POPOVER;
use glib::{to_bool, to_gboolean};
// use std::string;

struct_Widget!(Popover);

impl Popover {
    pub fn new<T: ::WidgetTrait>(relative_to: &T) -> Option<Popover> {
        skip_assert_initialized!();
        let tmp_pointer = unsafe { ffi::gtk_popover_new(relative_to.unwrap_widget()) };
        check_pointer!(tmp_pointer, Popover)
    }

    pub fn set_relative_to<T: ::WidgetTrait>(&self, relative_to: &T) {
        unsafe { ffi::gtk_popover_set_relative_to(GTK_POPOVER(self.pointer), relative_to.unwrap_widget()) }
    }

    pub fn get_relative_to<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_popover_get_relative_to(GTK_POPOVER(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_position(&self, position: ::PositionType) {
        unsafe { ffi::gtk_popover_set_position(GTK_POPOVER(self.pointer), position) }
    }

    pub fn get_position(&self) -> ::PositionType {
        unsafe { ffi::gtk_popover_get_position(GTK_POPOVER(self.pointer)) }
    }

    pub fn set_modal(&self, modal: bool) {
        unsafe { ffi::gtk_popover_set_modal(GTK_POPOVER(self.pointer), to_gboolean(modal)) }
    }

    pub fn get_modal(&self) -> bool {
        unsafe { to_bool(ffi::gtk_popover_get_modal(GTK_POPOVER(self.pointer))) }
    }
}

impl_drop!(Popover);
impl_TraitWidget!(Popover);

impl ::ContainerTrait for Popover {}
impl ::BinTrait for Popover {}
