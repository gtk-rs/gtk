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

//! A container which can hide its child

use libc::c_int;

use glib::translate::{FromGlibPtr, ToGlibPtr};
use cast::GTK_EXPANDER;
use ffi;
use glib::{to_bool, to_gboolean};
use FFIWidget;

/// Expander — A container which can hide its child
struct_Widget!(Expander);

impl Expander {
    pub fn new(label: &str) -> Option<Expander> {
        let tmp_pointer = unsafe {
            ffi::gtk_expander_new(label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Expander)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Expander> {
        let tmp_pointer = unsafe {
            ffi::gtk_expander_new_with_mnemonic(mnemonic.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Expander)
    }


    pub fn set_expanded(&mut self, expanded: bool) -> () {
        unsafe { ffi::gtk_expander_set_expanded(GTK_EXPANDER(self.pointer), to_gboolean(expanded)); }
    }

    pub fn get_expanded(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_expanded(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_use_underline(&mut self, use_underline: bool) -> () {
        unsafe { ffi::gtk_expander_set_use_underline(GTK_EXPANDER(self.pointer), to_gboolean(use_underline)); }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_use_underline(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_use_markup(&mut self, use_markup: bool) -> () {
        unsafe { ffi::gtk_expander_set_use_markup(GTK_EXPANDER(self.pointer), to_gboolean(use_markup)); }
    }

    pub fn get_use_markup(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_use_markup(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_label_fill(&mut self, label_fill: bool) -> () {
        unsafe { ffi::gtk_expander_set_label_fill(GTK_EXPANDER(self.pointer), to_gboolean(label_fill)); }
    }

    pub fn get_label_fill(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_label_fill(GTK_EXPANDER(self.pointer))) }
    }

    pub fn set_resize_toplevel(&mut self, resize_toplevel: bool) -> () {
        unsafe { ffi::gtk_expander_set_resize_toplevel(GTK_EXPANDER(self.pointer), to_gboolean(resize_toplevel)); }
    }

    pub fn get_resize_toplevel(&self) -> bool {
        unsafe { to_bool(ffi::gtk_expander_get_resize_toplevel(GTK_EXPANDER(self.pointer))) }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_expander_get_label(GTK_EXPANDER(self.pointer)))
        }
    }

    pub fn set_label(&mut self, label: &str) -> () {
        unsafe {
            ffi::gtk_expander_set_label(GTK_EXPANDER(self.pointer), label.borrow_to_glib().0);
        }
    }

    pub fn set_spacing(&mut self, spacing: i32) -> () {
        unsafe {
            ffi::gtk_expander_set_spacing(GTK_EXPANDER(self.pointer), spacing as c_int)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_expander_get_spacing(GTK_EXPANDER(self.pointer)) as i32
        }
    }

    pub fn set_label_widget(&mut self, label: &::Label) -> () {
        unsafe {
            ffi::gtk_expander_set_label_widget(GTK_EXPANDER(self.pointer), label.unwrap_widget());
        }
    }

    pub fn get_label_widget(&mut self) -> ::Label {
        unsafe {
            ::FFIWidget::wrap_widget(ffi::gtk_expander_get_label_widget(GTK_EXPANDER(self.pointer)))
        }
    }
}

impl_drop!(Expander);
impl_TraitWidget!(Expander);

impl ::ContainerTrait for Expander {}
impl ::BinTrait for Expander {}

impl_widget_events!(Expander);
