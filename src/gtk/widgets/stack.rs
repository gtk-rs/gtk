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

//! A stacking container

// FIXME: add missing methods (3.12)

use gtk::{self, ffi};
use gtk::cast::GTK_STACK;
use std::ffi::CString;
use glib::{to_bool, to_gboolean};

/// GtkStack — A stacking container
struct_Widget!(Stack);

impl Stack {
    pub fn new() -> Option<Stack> {
        let tmp_pointer = unsafe { ffi::gtk_stack_new() };
        check_pointer!(tmp_pointer, Stack)
    }

    pub fn add_named<T: gtk::WidgetTrait>(&mut self, child: &T, name: &str) {
        let c_str = CString::from_slice(name.as_bytes());

        unsafe {
            ffi::gtk_stack_add_named(GTK_STACK(self.pointer),
                                     child.get_widget(),
                                     c_str.as_ptr())
        }
    }

    pub fn add_titled<T: gtk::WidgetTrait>(&mut self, child: &T, name: &str, title: &str) {
        let c_name = CString::from_slice(name.as_bytes());
        let c_title = CString::from_slice(title.as_bytes());

        unsafe {
            ffi::gtk_stack_add_titled(GTK_STACK(self.pointer),
                                      child.get_widget(),
                                      c_name.as_ptr(),
                                      c_title.as_ptr())
        }
    }

    pub fn set_visible_child<T: gtk::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_stack_set_visible_child(GTK_STACK(self.pointer),
                                             child.get_widget())
        }
    }

    pub fn get_visible_child<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_stack_get_visible_child(GTK_STACK(self.pointer)) };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn set_visible_child_name(&mut self, name: &str) {
        let c_str = CString::from_slice(name.as_bytes());

        unsafe {
            ffi::gtk_stack_set_visible_child_name(GTK_STACK(self.pointer), c_str.as_ptr())
        }
    }

    pub fn get_visible_child_name(&self) -> Option<String> {
        let c_name = unsafe { ffi::gtk_stack_get_visible_child_name(GTK_STACK(self.pointer)) };

        if c_name.is_null() {
            None
        } else {
            unsafe { Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_name)).to_string()) }
        }
    }

    pub fn set_visible_child_full(&mut self, name: &str, transition: gtk::StackTransitionType) {
        let c_str = CString::from_slice(name.as_bytes());

        unsafe {
            ffi::gtk_stack_set_visible_child_full(GTK_STACK(self.pointer),
                                                  c_str.as_ptr(),
                                                  transition)
        }
    }

    pub fn set_homogeneous(&mut self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(GTK_STACK(self.pointer), to_gboolean(homogeneous))
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_stack_get_homogeneous(GTK_STACK(self.pointer)))
        }
    }

    pub fn set_transition_duration(&mut self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(GTK_STACK(self.pointer), duration)
        }
    }

    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(GTK_STACK(self.pointer))
        }
    }

    pub fn set_transition_type(&mut self, transition: gtk::StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(GTK_STACK(self.pointer), transition)
        }
    }

    pub fn get_transition_type(&self) -> gtk::StackTransitionType {
        unsafe {
            ffi::gtk_stack_get_transition_type(GTK_STACK(self.pointer))
        }
    }
}

impl_drop!(Stack);
impl_TraitWidget!(Stack);

impl gtk::ContainerTrait for Stack {}

impl_widget_events!(Stack);
