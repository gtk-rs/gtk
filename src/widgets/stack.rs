// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use {
    StackTransitionType,
};

///////////////////////////////////////////////////////////////////////////////

/// A stacking container.
pub type Stack = Object<ffi::GtkStack>;

unsafe impl Upcast<Widget> for Stack { }
unsafe impl Upcast<super::container::Container> for Stack { }
unsafe impl Upcast<::builder::Buildable> for Stack { }

impl Stack {
    pub fn new() -> Stack {
        unsafe { Widget::from_glib_none(ffi::gtk_stack_new()).downcast_unchecked() }
    }

    pub fn add_named<T: Upcast<Widget>>(&self, child: &T, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(self.to_glib_none().0, child.upcast().to_glib_none().0,
                name.to_glib_none().0)
        }
    }

    pub fn add_titled<T: Upcast<Widget>>(&self, child: &T, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(self.to_glib_none().0, child.upcast().to_glib_none().0,
                name.to_glib_none().0, title.to_glib_none().0)
        }
    }

    pub fn set_visible_child<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_stack_set_visible_child(self.to_glib_none().0, child.upcast().to_glib_none().0)
        }
    }

    pub fn get_visible_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0)) }
    }

    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0)
        }
    }

    pub fn get_visible_child_name(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0)) }
    }

    pub fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(self.to_glib_none().0, name.to_glib_none().0,
                transition)
        }
    }

    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe { ffi::gtk_stack_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib()) }
    }

    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_homogeneous(self.to_glib_none().0)) }
    }

    pub fn set_transition_duration(&self, duration: u32) {
        unsafe { ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration) }
    }

    pub fn get_transition_duration(&self) -> u32 {
        unsafe { ffi::gtk_stack_get_transition_duration(self.to_glib_none().0) }
    }

    pub fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe { ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition) }
    }

    pub fn get_transition_type(&self) -> StackTransitionType {
        unsafe { ffi::gtk_stack_get_transition_type(self.to_glib_none().0) }
    }
}

impl types::StaticType for Stack {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_stack_get_type()) }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A controller for GtkStack.
pub type StackSwitcher = Object<ffi::GtkStackSwitcher>;

unsafe impl Upcast<Widget> for StackSwitcher { }
unsafe impl Upcast<super::container::Container> for StackSwitcher { }
unsafe impl Upcast<super::box_::Box> for StackSwitcher { }
unsafe impl Upcast<super::orientable::Orientable> for StackSwitcher { }
unsafe impl Upcast<::builder::Buildable> for StackSwitcher { }

impl StackSwitcher {
    pub fn new() -> StackSwitcher {
        unsafe { Widget::from_glib_none(ffi::gtk_stack_switcher_new()).downcast_unchecked() }
    }

    pub fn set_stack(&self, stack: &Stack) {
        unsafe { ffi::gtk_stack_switcher_set_stack(self.to_glib_none().0, stack.to_glib_none().0)
        }
    }

    pub fn get_stack(&self) -> Option<Stack> {
        unsafe { from_glib_none(ffi::gtk_stack_switcher_get_stack(self.to_glib_none().0)) }
    }
}

impl types::StaticType for StackSwitcher {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_stack_switcher_get_type()) }
    }
}
