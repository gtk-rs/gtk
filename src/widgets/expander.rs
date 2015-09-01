// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container which can hide its child

use libc::c_int;

use glib::translate::{from_glib_none, ToGlibPtr};
use cast::GTK_EXPANDER;
use ffi;
use glib::{to_bool, to_gboolean};
use FFIWidget;

/// Expander — A container which can hide its child
struct_Widget!(Expander);

impl Expander {
    pub fn new(label: &str) -> Option<Expander> {
        let tmp_pointer = unsafe {
            ffi::gtk_expander_new(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Expander)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Expander> {
        let tmp_pointer = unsafe {
            ffi::gtk_expander_new_with_mnemonic(mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Expander)
    }


    pub fn set_expanded(&self, expanded: bool) -> () {
        unsafe { ffi::gtk_expander_set_expanded(GTK_EXPANDER(self.pointer), to_gboolean(expanded)); }
    }

    pub fn get_expanded(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_expanded(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_use_underline(&self, use_underline: bool) -> () {
        unsafe { ffi::gtk_expander_set_use_underline(GTK_EXPANDER(self.pointer), to_gboolean(use_underline)); }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_use_underline(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_use_markup(&self, use_markup: bool) -> () {
        unsafe { ffi::gtk_expander_set_use_markup(GTK_EXPANDER(self.pointer), to_gboolean(use_markup)); }
    }

    pub fn get_use_markup(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_use_markup(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_label_fill(&self, label_fill: bool) -> () {
        unsafe { ffi::gtk_expander_set_label_fill(GTK_EXPANDER(self.pointer), to_gboolean(label_fill)); }
    }

    pub fn get_label_fill(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_label_fill(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_resize_toplevel(&self, resize_toplevel: bool) -> () {
        unsafe { ffi::gtk_expander_set_resize_toplevel(GTK_EXPANDER(self.pointer), to_gboolean(resize_toplevel)); }
    }

    pub fn get_resize_toplevel(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_resize_toplevel(GTK_EXPANDER(self.pointer))) }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label(GTK_EXPANDER(self.pointer)))
        }
    }

    pub fn set_label(&self, label: &str) -> () {
        unsafe {
            ffi::gtk_expander_set_label(GTK_EXPANDER(self.pointer), label.to_glib_none().0);
        }
    }

    pub fn set_spacing(&self, spacing: i32) -> () {
        unsafe {
            ffi::gtk_expander_set_spacing(GTK_EXPANDER(self.pointer), spacing as c_int)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_expander_get_spacing(GTK_EXPANDER(self.pointer)) as i32
        }
    }

    pub fn set_label_widget(&self, label: &::Label) -> () {
        unsafe {
            ffi::gtk_expander_set_label_widget(GTK_EXPANDER(self.pointer), label.unwrap_widget());
        }
    }

    pub fn get_label_widget(&self) -> ::Label {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_expander_get_label_widget(GTK_EXPANDER(self.pointer)))
        }
    }
}

impl_drop!(Expander);
impl_TraitWidget!(Expander);

impl ::ContainerTrait for Expander {}
impl ::BinTrait for Expander {}
