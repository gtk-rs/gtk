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
use std::ffi::CString;
use c_str::FromCStr;

use gtk::cast::GTK_EXPANDER;
use gtk::{self, ffi};
use gtk::ffi::FFIWidget;

/// Expander — A container which can hide its child
struct_Widget!(Expander);

impl Expander {
    pub fn new(label: &str) -> Option<Expander> {
        let c_str = CString::from_slice(label.as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_expander_new(c_str.as_ptr())
        };
        check_pointer!(tmp_pointer, Expander)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<Expander> {
        let c_str = CString::from_slice(mnemonic.as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_expander_new_with_mnemonic(c_str.as_ptr())
        };
        check_pointer!(tmp_pointer, Expander)
    }


    pub fn set_expanded(&mut self, expanded: bool) -> () {
        match expanded {
            true    => unsafe { ffi::gtk_expander_set_expanded(GTK_EXPANDER(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_expander_set_expanded(GTK_EXPANDER(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_expanded(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_expanded(GTK_EXPANDER(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_use_underline(&mut self, use_underline: bool) -> () {
        match use_underline {
            true    => unsafe { ffi::gtk_expander_set_use_underline(GTK_EXPANDER(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_expander_set_use_underline(GTK_EXPANDER(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_use_underline(GTK_EXPANDER(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_use_markup(&mut self, use_markup: bool) -> () {
        match use_markup {
            true    => unsafe { ffi::gtk_expander_set_use_markup(GTK_EXPANDER(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_expander_set_use_markup(GTK_EXPANDER(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_use_markup(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_use_markup(GTK_EXPANDER(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_label_fill(&mut self, label_fill: bool) -> () {
        match label_fill {
            true    => unsafe { ffi::gtk_expander_set_label_fill(GTK_EXPANDER(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_expander_set_label_fill(GTK_EXPANDER(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_label_fill(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_label_fill(GTK_EXPANDER(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_resize_toplevel(&mut self, resize_toplevel: bool) -> () {
        match resize_toplevel {
            true    => unsafe { ffi::gtk_expander_set_resize_toplevel(GTK_EXPANDER(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_expander_set_resize_toplevel(GTK_EXPANDER(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_resize_toplevel(&self) -> bool {
        match unsafe { ffi::gtk_expander_get_resize_toplevel(GTK_EXPANDER(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn get_label(&self) -> String {
        unsafe {
            let c_str = ffi::gtk_expander_get_label(GTK_EXPANDER(self.pointer));
            FromCStr::from_raw_buf(c_str as *const u8)
        }
    }

    pub fn set_label(&mut self, label: &str) -> () {
        let c_str = CString::from_slice(label.as_bytes());
        unsafe {
            ffi::gtk_expander_set_label(GTK_EXPANDER(self.pointer), c_str.as_ptr());
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

    pub fn set_label_widget(&mut self, label: &gtk::Label) -> () {
        unsafe {
            ffi::gtk_expander_set_label_widget(GTK_EXPANDER(self.pointer), label.get_widget());
        }
    }

    pub fn get_label_widget(&mut self) -> gtk::Label {
        unsafe {
            ffi::FFIWidget::wrap(ffi::gtk_expander_get_label_widget(GTK_EXPANDER(self.pointer)))
        }
    }
}

impl_drop!(Expander);
impl_TraitWidget!(Expander);

impl gtk::ContainerTrait for Expander {}
impl gtk::BinTrait for Expander {}

impl_widget_events!(Expander);
