// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// FIXME: add missing methods (3.12)

use ffi;
use cast::GTK_STACK;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::{to_bool, to_gboolean};

struct_Widget!(Stack);

impl Stack {
    pub fn new() -> Option<Stack> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_stack_new() };
        check_pointer!(tmp_pointer, Stack)
    }

    pub fn add_named<T: ::WidgetTrait>(&self, child: &T, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(GTK_STACK(self.pointer),
                                     child.unwrap_widget(),
                                     name.to_glib_none().0)
        }
    }

    pub fn add_titled<T: ::WidgetTrait>(&self, child: &T, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(GTK_STACK(self.pointer),
                                      child.unwrap_widget(),
                                      name.to_glib_none().0,
                                      title.to_glib_none().0)
        }
    }

    pub fn set_visible_child<T: ::WidgetTrait>(&self, child: &T) {
        unsafe {
            ffi::gtk_stack_set_visible_child(GTK_STACK(self.pointer),
                                             child.unwrap_widget())
        }
    }

    pub unsafe fn get_visible_child<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = ffi::gtk_stack_get_visible_child(GTK_STACK(self.pointer));
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(GTK_STACK(self.pointer),
                                                  name.to_glib_none().0)
        }
    }

    pub fn get_visible_child_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child_name(GTK_STACK(self.pointer)))
        }
    }

    pub fn set_visible_child_full(&self, name: &str, transition: ::StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(GTK_STACK(self.pointer),
                                                  name.to_glib_none().0,
                                                  transition)
        }
    }

    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(GTK_STACK(self.pointer), to_gboolean(homogeneous))
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_stack_get_homogeneous(GTK_STACK(self.pointer)))
        }
    }

    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(GTK_STACK(self.pointer), duration)
        }
    }

    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(GTK_STACK(self.pointer))
        }
    }

    pub fn set_transition_type(&self, transition: ::StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(GTK_STACK(self.pointer), transition)
        }
    }

    pub fn get_transition_type(&self) -> ::StackTransitionType {
        unsafe {
            ffi::gtk_stack_get_transition_type(GTK_STACK(self.pointer))
        }
    }
}

impl_drop!(Stack);
impl_TraitWidget!(Stack);

impl ::ContainerTrait for Stack {}
