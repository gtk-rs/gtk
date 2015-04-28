// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container for arranging buttons

use {Orientation, ButtonBoxStyle};
use cast::GTK_BUTTONBOX;
use ffi;
use glib::{to_bool, to_gboolean};

/// ButtonBox — A container for arranging buttons
struct_Widget!(ButtonBox);

impl ButtonBox {
    pub fn new(orientation: Orientation) -> Option<ButtonBox> {
        let tmp_pointer = unsafe { ffi::gtk_button_box_new(orientation) };
        check_pointer!(tmp_pointer, ButtonBox)
    }

    pub fn get_layout(&self) -> ButtonBoxStyle {
        unsafe {
            ffi::gtk_button_box_get_layout(GTK_BUTTONBOX(self.pointer))
        }
    }

    pub fn get_child_secondary<T: ::WidgetTrait + ::ButtonTrait>(&self, child: &T) -> bool {
        unsafe { to_bool(ffi::gtk_button_box_get_child_secondary(GTK_BUTTONBOX(self.pointer), child.unwrap_widget())) }
    }

    pub fn get_child_non_homogeneous<T: ::WidgetTrait + ::ButtonTrait>(&self, child: &T) -> bool {
        unsafe { to_bool(ffi::gtk_button_box_get_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.unwrap_widget())) }
    }

    pub fn set_layout(&self, layout_style: ButtonBoxStyle) -> () {
        unsafe {
            ffi::gtk_button_box_set_layout(GTK_BUTTONBOX(self.pointer), layout_style)
        }
    }

    pub fn set_child_secondary<T: ::WidgetTrait + ::ButtonTrait>(&self, child: &T, is_secondary: bool) -> () {
        unsafe { ffi::gtk_button_box_set_child_secondary(GTK_BUTTONBOX(self.pointer), child.unwrap_widget(), to_gboolean(is_secondary)); }
    }

    pub fn set_child_non_homogeneous<T: ::WidgetTrait + ::ButtonTrait>(&self, child: &T, non_homogeneous: bool) -> () {
        unsafe { ffi::gtk_button_box_set_child_non_homogeneous(GTK_BUTTONBOX(self.pointer), child.unwrap_widget(), to_gboolean(non_homogeneous)); }
    }
}

impl_drop!(ButtonBox);
impl_TraitWidget!(ButtonBox);

impl ::ContainerTrait for ButtonBox {}
impl ::BoxTrait for ButtonBox {}
impl ::OrientableTrait for ButtonBox {}
